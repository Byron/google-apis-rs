use std;
use std::error;
use std::fmt::{self, Display};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use hyper;
use hyper::header::{
    Authorization, Bearer, ContentLength, ContentType, Header, HeaderFormat, Headers, UserAgent,
};
use hyper::http::h1::LINE_ENDING;
use hyper::method::Method;
use hyper::status::StatusCode;
use mime::{Attr, Mime, SubLevel, TopLevel, Value};
use oauth2::{self, Retry, TokenType};

use serde_json as json;

/// Identifies the Hub. There is only one per library, this trait is supposed
/// to make intended use more explicit.
/// The hub allows to access all resource methods more easily.
pub trait Hub {}

/// Identifies types for building methods of a particular resource type
pub trait MethodsBuilder {}

/// Identifies types which represent builders for a particular resource method
pub trait CallBuilder {}

/// Identifies types which can be inserted and deleted.
/// Types with this trait are most commonly used by clients of this API.
pub trait Resource {}

/// Identifies types which are used in API responses.
pub trait ResponseResult {}

/// Identifies types which are used in API requests.
pub trait RequestValue {}

/// Identifies types which are not actually used by the API
/// This might be a bug within the google API schema.
pub trait UnusedType {}

/// Identifies types which are only used as part of other types, which
/// usually are carrying the `Resource` trait.
pub trait Part {}

/// Identifies types which are only used by other types internally.
/// They have no special meaning, this trait just marks them for completeness.
pub trait NestedType {}

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
    pub error_description: Option<String>,
}

/// A utility to represent detailed errors we might see in case there are BadRequests.
/// The latter happen if the sent parameters or request structures are unsound
#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub error: ServerError,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerError {
    pub errors: Vec<ServerMessage>,
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerMessage {
    pub domain: String,
    pub reason: String,
    pub message: String,
    #[serde(rename = "locationType")]
    pub location_type: Option<String>,
    pub location: Option<String>,
}

#[derive(Copy, Clone)]
pub struct DummyNetworkStream;

impl Read for DummyNetworkStream {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Write for DummyNetworkStream {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
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

    fn set_read_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
        Ok(())
    }

    fn set_write_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
        Ok(())
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
    /// request was successful. That way, the delegate may easily maintain a clean state
    /// between various API calls.
    fn begin(&mut self, MethodInfo) {}

    /// Called whenever there is an [HttpError](hyper::Error), usually if there are network problems.
    ///
    /// If you choose to retry after a duration, the duration should be chosen using the
    /// [exponential backoff algorithm](http://en.wikipedia.org/wiki/Exponential_backoff).
    ///
    /// Return retry information.
    fn http_error(&mut self, &hyper::Error) -> Retry {
        Retry::Abort
    }

    /// Called whenever there is the need for your applications API key after
    /// the official authenticator implementation didn't provide one, for some reason.
    /// If this method returns None as well, the underlying operation will fail
    fn api_key(&mut self) -> Option<String> {
        None
    }

    /// Called whenever the Authenticator didn't yield a token. The delegate
    /// may attempt to provide one, or just take it as a general information about the
    /// impending failure.
    /// The given Error provides information about why the token couldn't be acquired in the
    /// first place
    fn token(&mut self, err: &dyn error::Error) -> Option<oauth2::Token> {
        let _ = err;
        None
    }

    /// Called during resumable uploads to provide a URL for the impending upload.
    /// It was saved after a previous call to `store_upload_url(...)`, and if not None,
    /// will be used instead of asking the server for a new upload URL.
    /// This is useful in case a previous resumable upload was aborted/canceled, but should now
    /// be resumed.
    /// The returned URL will be used exactly once - if it fails again and the delegate allows
    /// to retry, we will ask the server for a new upload URL.
    fn upload_url(&mut self) -> Option<String> {
        None
    }

    /// Called after we have retrieved a new upload URL for a resumable upload to store it
    /// in case we fail or cancel. That way, we can attempt to resume the upload later,
    /// see `upload_url()`.
    /// It will also be called with None after a successful upload, which allows the delegate
    /// to forget the URL. That way, we will not attempt to resume an upload that has already
    /// finished.
    fn store_upload_url(&mut self, url: Option<&str>) {
        let _ = url;
    }

    /// Called whenever a server response could not be decoded from json.
    /// It's for informational purposes only, the caller will return with an error
    /// accordingly.
    ///
    /// # Arguments
    ///
    /// * `json_encoded_value` - The json-encoded value which failed to decode.
    /// * `json_decode_error`  - The decoder error
    fn response_json_decode_error(
        &mut self,
        json_encoded_value: &str,
        json_decode_error: &json::Error,
    ) {
        let _ = json_encoded_value;
        let _ = json_decode_error;
    }

    /// Called whenever the http request returns with a non-success status code.
    /// This can involve authentication issues, or anything else that very much
    /// depends on the used API method.
    /// The delegate should check the status, header and decoded json error to decide
    /// whether to retry or not. In the latter case, the underlying call will fail.
    ///
    /// If you choose to retry after a duration, the duration should be chosen using the
    /// [exponential backoff algorithm](http://en.wikipedia.org/wiki/Exponential_backoff).
    fn http_failure(
        &mut self,
        _: &hyper::client::Response,
        Option<JsonServerError>,
        _: Option<ServerError>,
    ) -> Retry {
        Retry::Abort
    }

    /// Called prior to sending the main request of the given method. It can be used to time
    /// the call or to print progress information.
    /// It's also useful as you can be sure that a request will definitely be made.
    fn pre_request(&mut self) {}

    /// Return the size of each chunk of a resumable upload.
    /// Must be a power of two, with 1<<18 being the smallest allowed chunk size.
    /// Will be called once before starting any resumable upload.
    fn chunk_size(&mut self) -> u64 {
        1 << 23
    }

    /// Called before the given chunk is uploaded to the server.
    /// If true is returned, the upload will be interrupted.
    /// However, it may be resumable if you stored the upload URL in a previous call
    /// to `store_upload_url()`
    fn cancel_chunk_upload(&mut self, chunk: &ContentRange) -> bool {
        let _ = chunk;
        false
    }

    /// Called before the API request method returns, in every case. It can be used to clean up
    /// internal state between calls to the API.
    /// This call always has a matching call to `begin(...)`.
    ///
    /// # Arguments
    ///
    /// * `is_success` - a true value indicates the operation was successful. If false, you should
    ///                  discard all values stored during `store_upload_url`.
    fn finished(&mut self, is_success: bool) {
        let _ = is_success;
    }
}

/// A delegate with a conservative default implementation, which is used if no other delegate is
/// set.
#[derive(Default)]
pub struct DefaultDelegate;

impl Delegate for DefaultDelegate {}

#[derive(Debug)]
pub enum Error {
    /// The http connection failed
    HttpError(hyper::Error),

    /// An attempt was made to upload a resource with size stored in field `.0`
    /// even though the maximum upload size is what is stored in field `.1`.
    UploadSizeLimitExceeded(u64, u64),

    /// Represents information about a request that was not understood by the server.
    /// Details are included.
    BadRequest(ErrorResponse),

    /// We needed an API key for authentication, but didn't obtain one.
    /// Neither through the authenticator, nor through the Delegate.
    MissingAPIKey,

    /// We required a Token, but didn't get one from the Authenticator
    MissingToken(Box<dyn error::Error>),

    /// The delgate instructed to cancel the operation
    Cancelled,

    /// An additional, free form field clashed with one of the built-in optional ones
    FieldClash(&'static str),

    /// Shows that we failed to decode the server response.
    /// This can happen if the protocol changes in conjunction with strict json decoding.
    JsonDecodeError(String, json::Error),

    /// Indicates an HTTP repsonse with a non-success status code
    Failure(hyper::client::Response),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HttpError(ref err) => err.fmt(f),
            Error::UploadSizeLimitExceeded(ref resource_size, ref max_size) => writeln!(
                f,
                "The media size {} exceeds the maximum allowed upload size of {}",
                resource_size, max_size
            ),
            Error::MissingAPIKey => {
                (writeln!(
                    f,
                    "The application's API key was not found in the configuration"
                ))
                .ok();
                writeln!(
                    f,
                    "It is used as there are no Scopes defined for this method."
                )
            }
            Error::BadRequest(ref err) => {
                writeln!(f, "Bad Request ({}): {}", err.error.code, err.error.message)?;
                for err in err.error.errors.iter() {
                    writeln!(
                        f,
                        "    {}: {}, {}{}",
                        err.domain,
                        err.message,
                        err.reason,
                        match &err.location {
                            &Some(ref loc) => format!("@{}", loc),
                            &None => String::new(),
                        }
                    )?;
                }
                Ok(())
            }
            Error::MissingToken(ref err) => {
                writeln!(f, "Token retrieval failed with error: {}", err)
            }
            Error::Cancelled => writeln!(f, "Operation cancelled by delegate"),
            Error::FieldClash(field) => writeln!(
                f,
                "The custom parameter '{}' is already provided natively by the CallBuilder.",
                field
            ),
            Error::JsonDecodeError(ref json_str, ref err) => writeln!(f, "{}: {}", err, json_str),
            Error::Failure(ref response) => {
                writeln!(f, "Http status indicates failure: {:?}", response)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::HttpError(ref err) => err.source(),
            Error::JsonDecodeError(_, ref err) => err.source(),
            _ => None,
        }
    }
}

/// A universal result type used as return for all calls.
pub type Result<T> = std::result::Result<T, Error>;

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
    raw_parts: Vec<(Headers, &'a mut dyn Read)>,
    current_part: Option<(Cursor<Vec<u8>>, &'a mut dyn Read)>,
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
    ///
    /// `headers` - identifying the body of the part. It's similar to the header
    ///             in an ordinary single-part call, and should thus contain the
    ///             same information.
    /// `reader`  - a reader providing the part's body
    /// `size`    - the amount of bytes provided by the reader. It will be put onto the header as
    ///             content-size.
    /// `mime`    - It will be put onto the content type
    pub fn add_part(
        &mut self,
        reader: &'a mut dyn Read,
        size: u64,
        mime_type: Mime,
    ) -> &mut MultiPartReader<'a> {
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
            vec![(
                Attr::Ext("boundary".to_string()),
                Value::Ext(BOUNDARY.to_string()),
            )],
        )
    }

    /// Returns true if we are totally used
    fn is_depleted(&self) -> bool {
        self.raw_parts.len() == 0
            && self.current_part.is_none()
            && self.last_part_boundary.is_none()
    }

    /// Returns true if we are handling our last part
    fn is_last_part(&self) -> bool {
        self.raw_parts.len() == 0 && self.current_part.is_some()
    }
}

impl<'a> Read for MultiPartReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match (
            self.raw_parts.len(),
            self.current_part.is_none(),
            self.last_part_boundary.is_none(),
        ) {
            (_, _, false) => {
                let br = self
                    .last_part_boundary
                    .as_mut()
                    .unwrap()
                    .read(buf)
                    .unwrap_or(0);
                if br < buf.len() {
                    self.last_part_boundary = None;
                }
                return Ok(br);
            }
            (0, true, true) => return Ok(0),
            (n, true, _) if n > 0 => {
                let (headers, reader) = self.raw_parts.remove(0);
                let mut c = Cursor::new(Vec::<u8>::new());
                (write!(
                    &mut c,
                    "{}--{}{}{}{}",
                    LINE_ENDING, BOUNDARY, LINE_ENDING, headers, LINE_ENDING
                ))
                .unwrap();
                c.seek(SeekFrom::Start(0)).unwrap();
                self.current_part = Some((c, reader));
            }
            _ => {}
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
                            format!("{}--{}--", LINE_ENDING, BOUNDARY).into_bytes(),
                        ))
                    }
                    // We are depleted - this can trigger the next part to come in
                    self.current_part = None;
                }
                let mut total_bytes_read = hb + bytes_read;
                while total_bytes_read < buf.len() && !self.is_depleted() {
                    match self.read(&mut buf[total_bytes_read..]) {
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
///
/// Generated via rustc --pretty expanded -Z unstable-options, and manually
/// processed to be more readable.
#[derive(PartialEq, Debug, Clone)]
pub struct XUploadContentType(pub Mime);

impl ::std::ops::Deref for XUploadContentType {
    type Target = Mime;
    fn deref<'a>(&'a self) -> &'a Mime {
        &self.0
    }
}
impl ::std::ops::DerefMut for XUploadContentType {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Mime {
        &mut self.0
    }
}
impl Header for XUploadContentType {
    fn header_name() -> &'static str {
        "X-Upload-Content-Type"
    }
    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<Self> {
        hyper::header::parsing::from_one_raw_str(raw).map(XUploadContentType)
    }
}
impl HeaderFormat for XUploadContentType {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}
impl Display for XUploadContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Chunk {
    pub first: u64,
    pub last: u64,
}

impl fmt::Display for Chunk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (write!(fmt, "{}-{}", self.first, self.last)).ok();
        Ok(())
    }
}

impl FromStr for Chunk {
    type Err = &'static str;

    /// NOTE: only implements `%i-%i`, not `*`
    fn from_str(s: &str) -> std::result::Result<Chunk, &'static str> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Expected two parts: %i-%i");
        }
        Ok(Chunk {
            first: match FromStr::from_str(parts[0]) {
                Ok(d) => d,
                _ => return Err("Couldn't parse 'first' as digit"),
            },
            last: match FromStr::from_str(parts[1]) {
                Ok(d) => d,
                _ => return Err("Couldn't parse 'last' as digit"),
            },
        })
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
    fn parse_header(_: &[Vec<u8>]) -> hyper::error::Result<Self> {
        Err(hyper::error::Error::Method)
    }
}

impl HeaderFormat for ContentRange {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("bytes ")?;
        match self.range {
            Some(ref c) => c.fmt(fmt)?,
            None => fmt.write_str("*")?,
        }
        (write!(fmt, "/{}", self.total_length)).ok();
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RangeResponseHeader(pub Chunk);

impl Header for RangeResponseHeader {
    fn header_name() -> &'static str {
        "Range"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<Self> {
        if raw.len() > 0 {
            let v = &raw[0];
            if let Ok(s) = std::str::from_utf8(v) {
                const PREFIX: &'static str = "bytes ";
                if s.starts_with(PREFIX) {
                    if let Ok(c) = <Chunk as FromStr>::from_str(&s[PREFIX.len()..]) {
                        return Ok(RangeResponseHeader(c));
                    }
                }
            }
        }
        Err(hyper::error::Error::Method)
    }
}

impl HeaderFormat for RangeResponseHeader {
    /// No implmentation necessary, we just need to parse
    fn fmt_header(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
}

/// A utility type to perform a resumable upload from start to end.
pub struct ResumableUploadHelper<'a, A: 'a> {
    pub client: &'a mut hyper::client::Client,
    pub delegate: &'a mut dyn Delegate,
    pub start_at: Option<u64>,
    pub auth: &'a mut A,
    pub user_agent: &'a str,
    pub auth_header: Authorization<Bearer>,
    pub url: &'a str,
    pub reader: &'a mut dyn ReadSeek,
    pub media_type: Mime,
    pub content_length: u64,
}

impl<'a, A> ResumableUploadHelper<'a, A>
where
    A: oauth2::GetToken,
{
    fn query_transfer_status(
        &mut self,
    ) -> std::result::Result<u64, hyper::Result<hyper::client::Response>> {
        loop {
            match self
                .client
                .post(self.url)
                .header(UserAgent(self.user_agent.to_string()))
                .header(ContentRange {
                    range: None,
                    total_length: self.content_length,
                })
                .header(self.auth_header.clone())
                .send()
            {
                Ok(r) => {
                    // 308 = resume-incomplete == PermanentRedirect
                    let headers = r.headers.clone();
                    let h: &RangeResponseHeader = match headers.get() {
                        Some(hh) if r.status == StatusCode::PermanentRedirect => hh,
                        None | Some(_) => {
                            if let Retry::After(d) = self.delegate.http_failure(&r, None, None) {
                                sleep(d);
                                continue;
                            }
                            return Err(Ok(r));
                        }
                    };
                    return Ok(h.0.last);
                }
                Err(err) => {
                    if let Retry::After(d) = self.delegate.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    return Err(Err(err));
                }
            }
        }
    }

    /// returns None if operation was cancelled by delegate, or the HttpResult.
    /// It can be that we return the result just because we didn't understand the status code -
    /// caller should check for status himself before assuming it's OK to use
    pub fn upload(&mut self) -> Option<hyper::Result<hyper::client::Response>> {
        let mut start = match self.start_at {
            Some(s) => s,
            None => match self.query_transfer_status() {
                Ok(s) => s,
                Err(result) => return Some(result),
            },
        };

        const MIN_CHUNK_SIZE: u64 = 1 << 18;
        let chunk_size = match self.delegate.chunk_size() {
            cs if cs > MIN_CHUNK_SIZE => cs,
            _ => MIN_CHUNK_SIZE,
        };

        self.reader.seek(SeekFrom::Start(start)).unwrap();
        loop {
            let request_size = match self.content_length - start {
                rs if rs > chunk_size => chunk_size,
                rs => rs,
            };

            let mut section_reader = self.reader.take(request_size);
            let range_header = ContentRange {
                range: Some(Chunk {
                    first: start,
                    last: start + request_size - 1,
                }),
                total_length: self.content_length,
            };
            start += request_size;
            if self.delegate.cancel_chunk_upload(&range_header) {
                return None;
            }
            let res = self
                .client
                .post(self.url)
                .header(range_header)
                .header(ContentType(self.media_type.clone()))
                .header(UserAgent(self.user_agent.to_string()))
                .body(&mut section_reader)
                .send();
            match res {
                Ok(mut res) => {
                    if res.status == StatusCode::PermanentRedirect {
                        continue;
                    }
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let Retry::After(d) = self.delegate.http_failure(
                            &res,
                            json::from_str(&json_err).ok(),
                            json::from_str(&json_err).ok(),
                        ) {
                            sleep(d);
                            continue;
                        }
                    }
                    return Some(Ok(res));
                }
                Err(err) => {
                    if let Retry::After(d) = self.delegate.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    return Some(Err(err));
                }
            }
        }
    }
}

// Copy of src/rust/cli/client.rs
// TODO(ST): Allow sharing common code between program types
pub fn remove_json_null_values(value: &mut json::value::Value) {
    match *value {
        json::value::Value::Object(ref mut map) => {
            let mut for_removal = Vec::new();

            for (key, mut value) in map.iter_mut() {
                if value.is_null() {
                    for_removal.push(key.clone());
                } else {
                    remove_json_null_values(&mut value);
                }
            }

            for key in &for_removal {
                map.remove(key);
            }
        }
        json::value::Value::Array(ref mut arr) => {
            let mut i = 0;
            while i < arr.len() {
                if arr[i].is_null() {
                    arr.remove(i);
                } else {
                    remove_json_null_values(&mut arr[i]);
                    i += 1;
                }
            }
        }
        _ => {}
    }
}
