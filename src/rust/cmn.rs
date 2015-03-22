use std::marker::MarkerTrait;
use std::io::{self, Read, Seek, Cursor, Write, SeekFrom};
use std;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::thread::sleep;

use mime::{Mime, TopLevel, SubLevel, Attr, Value};
use oauth2::{TokenType, Retry, self};
use hyper;
use hyper::header::{ContentType, ContentLength, Headers, UserAgent, Authorization, Header,
                    HeaderFormat};
use hyper::http::LINE_ENDING;
use hyper::method::Method;
use hyper::status::StatusCode;

use serde;

/// Identifies the Hub. There is only one per library, this trait is supposed
/// to make intended use more explicit.
/// The hub allows to access all resource methods more easily.
pub trait Hub: MarkerTrait {}

/// Identifies types for building methods of a particular resource type
pub trait ResourceMethodsBuilder: MarkerTrait {}

/// Identifies types which represent builders for a particular resource method
pub trait CallBuilder: MarkerTrait {}

/// Identifies types which can be inserted and deleted.
/// Types with this trait are most commonly used by clients of this API.
pub trait Resource: MarkerTrait {}

/// Identifies types which are used in API responses.
pub trait ResponseResult: MarkerTrait {}

/// Identifies types which are used in API requests.
pub trait RequestValue: MarkerTrait {}

/// Identifies types which are not actually used by the API
/// This might be a bug within the google API schema.
pub trait UnusedType: MarkerTrait {}

/// Identifies types which are only used as part of other types, which 
/// usually are carrying the `Resource` trait.
pub trait Part: MarkerTrait {}

/// Identifies types which are only used by other types internally.
/// They have no special meaning, this trait just marks them for completeness.
pub trait NestedType: MarkerTrait {}

/// A utility to specify reader types which provide seeking capabilities too
pub trait ReadSeek: Seek + Read {}
impl<T: Seek + Read> ReadSeek for T {}

/// A trait for all types that can convert themselves into a *parts* string
pub trait ToParts {
    fn to_parts(&self) -> String;
}


/// A utility type which can decode a server response that indicates error
#[derive(Deserialize)]
pub struct JsonServerError {
    pub error: String,
    pub error_description: Option<String>
}

#[derive(Copy, Clone)]
pub struct DummyNetworkStream;

impl Read for DummyNetworkStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Write for DummyNetworkStream {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl hyper::net::NetworkStream for DummyNetworkStream {
    fn peer_addr(&mut self) -> io::Result<std::net::SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }
}


/// A trait specifying functionality to help controlling any request performed by the API.
/// The trait has a conservative default implementation.
///
/// It contains methods to deal with all common issues, as well with the ones related to 
/// uploading media
pub trait Delegate {

    /// Called at the beginning of any API request. The delegate should store the method
    /// information if he is interesting in knowing more context when further calls to it
    /// are made.
    /// The matching `finished()` call will always be made, no matter whether or not the API
    /// request was sucessfull. That way, the delgate may easily maintain a clean state 
    /// between various API calls.
    fn begin(&mut self, MethodInfo) {}

    /// Called whenever there is an [HttpError](http://hyperium.github.io/hyper/hyper/error/enum.HttpError.html), usually if there are network problems.
    /// 
    /// Return retry information.
    fn http_error(&mut self, &hyper::HttpError) -> Retry {
        Retry::Abort
    }

    /// Called whenever there is the need for your applications API key after 
    /// the official authenticator implementation didn't provide one, for some reason.
    /// If this method returns None as well, the underlying operation will fail
    fn api_key(&mut self) -> Option<String> {
        None
    }

    /// Called whenever the Authenticator didn't yield a token. The delegate
    /// may attempt to provide one, or just take is a general information about the
    /// pending impending failure
    fn token(&mut self) -> Option<oauth2::Token> {
        None
    }

    /// Called during resumable uploads to provide a URL for the impending upload.
    /// It was saved after a previous call to `store_upload_url(...)`, and if not None,
    /// will be used instead of asking the server for a new upload URL.
    /// This is useful in case a previous resumable upload was aborted/cancelled, but should now
    /// be resumed.
    /// The returned URL will be used exactly once - if it fails again and the delegate allows
    /// to retry, we will ask the server for a new upload URL.
    fn upload_url(&mut self) -> Option<String> {
        None
    }

    /// Called after we have retrieved a new upload URL for a resumable upload to store it
    /// in case we fail or cancel. That way, we can attempt to resume the upload later, 
    /// see `upload_url()`.
    fn store_upload_url(&mut self, url: &str) {
        let _ = url;
    }

    /// Called whenever a server response could not be decoded from json.
    /// It's for informational purposes only, the caller will return with an error
    /// accordingly.
    /// 
    /// # Arguments
    ///
    /// `json_encoded_value` - The json-encoded value which failed to decode.
    /// `json_decode_error` - The decoder error
    fn response_json_decode_error(&mut self, json_encoded_value: &str, json_decode_error: &serde::json::Error) {
        let _ = json_encoded_value;
        let _ = json_decode_error;
    }

    /// Called whenever the http request returns with a non-success status code.
    /// This can involve authentication issues, or anything else that very much 
    /// depends on the used API method.
    /// The delegate should check the status, header and decoded json error to decide
    /// whether to retry or not. In the latter case, the underlying call will fail.
    fn http_failure(&mut self, _: &hyper::client::Response, Option<JsonServerError>) -> Retry {
        Retry::Abort
    }

    /// Called prior to sending the main request of the given method. It can be used to time 
    /// the call or to print progress information.
    /// It's also useful as you can be sure that a request will definitely be made.
    fn pre_request(&mut self) { }


    /// Called before the API request method returns, in every case. It can be used to clean up
    /// internal state between calls to the API.
    /// This call always has a matching call to `begin(...)`.
    ///
    /// # Arguments
    /// 
    /// `is_success` - a true value indicates the operation was successful. If false, you should
    ///                discard all values stored during `store_upload_url`.
    fn finished(&mut self, is_success: bool) {
        let _ = is_success;
    }
}

/// A delegate with a conservative default implementation, which is used if no other delegate is
/// set.
#[derive(Default)]
pub struct DefaultDelegate;

impl Delegate for DefaultDelegate {}


/// A universal result type used as return for all action method results.
pub enum Result<T = ()> {
    /// The http connection failed
    HttpError(hyper::HttpError),

    /// An attempt was made to upload a resource with size stored in field `.0`
    /// even though the maximum upload size is what is stored in field `.1`.
    UploadSizeLimitExceeded(u64, u64),

    /// We needed an API key for authentication, but didn't obtain one.
    /// Neither through the authenticator, nor through the Delegate.
    MissingAPIKey,

    /// We required a Token, but didn't get one from the Authenticator
    MissingToken,

    /// An additional, free form field clashed with one of the built-in optional ones
    FieldClash(&'static str),

    /// Shows that we failed to decode the server response.
    /// This can happen if the protocol changes in conjunction with strict json decoding.
    JsonDecodeError(serde::json::Error),

    /// Indicates an HTTP repsonse with a non-success status code
    Failure(hyper::client::Response),

    /// It worked !
    Success(T),
}

/// Contains information about an API request.
pub struct MethodInfo {
    pub id: &'static str,
    pub http_method: Method,
}

const BOUNDARY: &'static str = "MDuXWGyeE33QFXGchb2VFWc4Z7945d";

/// Provides a `Read` interface that converts multiple parts into the protocol
/// identified by [RFC2387](https://tools.ietf.org/html/rfc2387).
/// **Note**: This implementation is just as rich as it needs to be to perform uploads
/// to google APIs, and might not be a fully-featured implementation.
#[derive(Default)]
pub struct MultiPartReader<'a> {
    raw_parts: Vec<(Headers, &'a mut Read)>,
    current_part: Option<(Cursor<Vec<u8>>, &'a mut Read)>,
    last_part_boundary: Option<Cursor<Vec<u8>>>,
}

impl<'a> MultiPartReader<'a> {

    /// Reserve memory for exactly the given amount of parts
    pub fn reserve_exact(&mut self, cap: usize) {
        self.raw_parts.reserve_exact(cap);
    }

    /// Add a new part to the queue of parts to be read on the first `read` call.
    ///
    /// # Arguments
    /// `headers` - identifying the body of the part. It's similar to the header
    ///             in an ordinary single-part call, and should thus contain the
    ///             same information.
    /// `reader`  - a reader providing the part's body
    /// `size`    - the amount of bytes provided by the reader. It will be put onto the header as
    ///             content-size.
    /// `mime`    - It will be put onto the content type
    /// # Panics
    ///
    /// If this method is called after the first `read` call, it will panic
    pub fn add_part(&mut self, reader: &'a mut Read, size: u64, mime_type: Mime) -> &mut MultiPartReader<'a> {
        let mut headers = Headers::new();
        headers.set(ContentType(mime_type));
        headers.set(ContentLength(size));
        self.raw_parts.push((headers, reader));
        self
    }

    /// Returns the mime-type representing our multi-part message.
    /// Use it with the ContentType header.
    pub fn mime_type(&self) -> Mime {
        Mime(
            TopLevel::Multipart,
            SubLevel::Ext("Related".to_string()),
            vec![(Attr::Ext("boundary".to_string()), Value::Ext(BOUNDARY.to_string()))],
        )
    }

    /// Returns true if we are totally used
    fn is_depleted(&self) -> bool {
        self.raw_parts.len() == 0 && self.current_part.is_none() && self.last_part_boundary.is_none()
    }

    /// Returns true if we are handling our last part
    fn is_last_part(&self) -> bool {
        self.raw_parts.len() == 0 && self.current_part.is_some()
    }
}

impl<'a> Read for MultiPartReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match (self.raw_parts.len(), 
               self.current_part.is_none(), 
               self.last_part_boundary.is_none()) {
            (_, _, false) => {
                let br = self.last_part_boundary.as_mut().unwrap().read(buf).unwrap_or(0);
                if br < buf.len() {
                    self.last_part_boundary = None;
                }
                return Ok(br)
            },
            (0, true, true) => return Ok(0),
            (n, true, _) if n > 0 => {
                let (headers, reader) = self.raw_parts.remove(0);
                let mut c = Cursor::new(Vec::<u8>::new());
                write!(&mut c, "{}--{}{}{}{}", LINE_ENDING, BOUNDARY, LINE_ENDING, 
                                               headers, LINE_ENDING).unwrap();
                c.seek(SeekFrom::Start(0)).unwrap();
                self.current_part = Some((c, reader));
            }
            _ => {},
        }

        // read headers as long as possible
        let (hb, rr) = {
            let &mut (ref mut c, ref mut reader) = self.current_part.as_mut().unwrap();
            let b = c.read(buf).unwrap_or(0);
            (b, reader.read(&mut buf[b..]))
        };
        
        match rr {
            Ok(bytes_read) => {
                if hb < buf.len() && bytes_read == 0 {
                    if self.is_last_part() {
                        // before clearing the last part, we will add the boundary that 
                        // will be written last
                        self.last_part_boundary = Some(Cursor::new(
                                                        format!("{}--{}", LINE_ENDING, BOUNDARY).into_bytes()))
                    }
                    // We are depleted - this can trigger the next part to come in
                    self.current_part = None;
                }
                let mut total_bytes_read = hb + bytes_read;
                while total_bytes_read < buf.len() && !self.is_depleted() {
                    match self.read(&mut buf[total_bytes_read ..]) {
                        Ok(br) => total_bytes_read += br,
                        Err(err) => return Err(err),
                    }
                }
                Ok(total_bytes_read)
            }
            Err(err) => {
                // fail permanently
                self.current_part = None;
                self.last_part_boundary = None;
                self.raw_parts.clear();
                Err(err)
            }
        }
    }
}


/// The `X-Upload-Content-Type` header.
#[derive(Clone, PartialEq, Debug)]
pub struct XUploadContentType(pub Mime);

impl_header!(XUploadContentType,
             "X-Upload-Content-Type",
             Mime);

#[derive(Clone, PartialEq, Debug)]
pub struct Chunk {
    pub first: u64,
    pub last: u64
}

impl fmt::Display for Chunk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}-{}", self.first, self.last).ok();
        Ok(())
    }
}

impl FromStr for Chunk {
    type Err = &'static str;

    /// NOTE: only implements `%i-%i`, not `*`
    fn from_str(s: &str) -> std::result::Result<Chunk, &'static str> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Expected two parts: %i-%i")
        }
        Ok(
            Chunk {
                first: match FromStr::from_str(parts[0]) {
                    Ok(d) => d,
                    _ => return Err("Couldn't parse 'first' as digit")
                },
                last: match FromStr::from_str(parts[1]) {
                    Ok(d) => d,
                    _ => return Err("Couldn't parse 'last' as digit")
                }
            }
        )
    }
}

/// Implements the Content-Range header, for serialization only
#[derive(Clone, PartialEq, Debug)]
pub struct ContentRange {
    pub range: Option<Chunk>,
    pub total_length: u64,
}

impl Header for ContentRange {
    fn header_name() -> &'static str {
        "Content-Range"
    }

    /// We are not parsable, as parsing is done by the `Range` header
    fn parse_header(raw: &[Vec<u8>]) -> Option<ContentRange> {
        None
    }
}


impl HeaderFormat for ContentRange {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt.write_str("bytes "));
        match self.range {
            Some(ref c) => try!(c.fmt(fmt)),
            None => try!(fmt.write_str("*"))
        }
        write!(fmt, "/{}", self.total_length).ok();
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RangeResponseHeader(pub Chunk);

impl Header for RangeResponseHeader {
    fn header_name() -> &'static str {
        "Range"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<RangeResponseHeader> {
        match raw {
            [ref v] => {
                if let Ok(s) = std::str::from_utf8(v) {
                    const PREFIX: &'static str = "bytes=";
                    if s.starts_with(PREFIX) {
                        let c: Chunk = match FromStr::from_str(&s[PREFIX.len()..]) {
                            Ok(c) => c,
                            _ => return None
                        };
                        return  Some(RangeResponseHeader(c))
                    }
                }
                None
            },
            _ => None
        }
    }
}

impl HeaderFormat for RangeResponseHeader {
    /// No implmentation necessary, we just need to parse
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
}

/// A utility type to perform a resumable upload from start to end.
pub struct ResumableUploadHelper<'a, NC: 'a, A: 'a> {
    pub client: &'a mut hyper::client::Client<NC>,
    pub delegate: &'a mut Delegate,
    pub start_at: Option<u64>,
    pub auth: &'a mut A,
    pub user_agent: &'a str,
    pub auth_header: Authorization<oauth2::Scheme>,
    pub url: &'a str,
    pub reader: &'a mut ReadSeek,
    pub media_type: Mime,
    pub content_length: u64
}

impl<'a, NC, A> ResumableUploadHelper<'a, NC, A>
    where NC: hyper::net::NetworkConnector,
          A: oauth2::GetToken {

    fn query_transfer_status(&'a mut self) -> (Option<u64>, hyper::HttpResult<hyper::client::Response>) {
        loop {
            match self.client.post(self.url)
                .header(UserAgent(self.user_agent.to_string()))
                .header(ContentRange { range: None, total_length: self.content_length })
                .header(self.auth_header.clone())
                .send() {
                Ok(r) => {
                    // 308 = resume-incomplete == PermanentRedirect
                    let headers = r.headers.clone();
                    let h: &RangeResponseHeader = match headers.get() {
                        Some(hh) if r.status == StatusCode::PermanentRedirect => hh,
                        None|Some(_) => {
                            if let Retry::After(d) = self.delegate.http_failure(&r, None) {
                                sleep(d);
                                continue;
                            }
                            return (None, Ok(r))
                        }
                    };
                    return (Some(h.0.last), Ok(r))
                }
                Err(err) => {
                    if let Retry::After(d) = self.delegate.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    return (None, Err(err))
                }
            }
        }
    }

    pub fn upload(&'a mut self) -> hyper::HttpResult<hyper::client::Response> {
        let start = match self.start_at {
            Some(s) => s,
            None => match self.query_transfer_status() {
                (Some(s), _) => s,
                (_, result) => return result
            }
        };
        Err(hyper::error::HttpError::HttpStatusError)
    }
}