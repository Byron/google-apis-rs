use std::marker::MarkerTrait;
use std::io::{self, Read, Seek, Cursor, Write, SeekFrom};

use mime::{Mime, TopLevel, SubLevel, Attr, Value};
use oauth2;
use hyper;
use hyper::header::{ContentType, ContentLength, Headers};
use hyper::http::LINE_ENDING;
use hyper::method::Method;

/// Identifies the Hub. There is only one per library, this trait is supposed
/// to make intended use more explicit.
/// The hub allows to access all resource methods more easily.
pub trait Hub: MarkerTrait {}

/// Identifies types for building methods of a particular resource type
pub trait ResourceMethodsBuilder: MarkerTrait {}

/// Identifies types which represent builders for a particular resource method
pub trait MethodBuilder: MarkerTrait {}

/// Identifies types which can be inserted and deleted.
/// Types with this trait are most commonly used by clients of this API.
pub trait Resource: MarkerTrait {}

/// Identifies types which are used in API responses.
pub trait ResponseResult: MarkerTrait {}

/// Identifies types which are used in API requests.
pub trait RequestValue: MarkerTrait {}

/// Identifies types which are only used as part of other types, which 
/// usually are carrying the `Resource` trait.
pub trait Part: MarkerTrait {}

/// Identifies types which are only used by other types internally.
/// They have no special meaning, this trait just marks them for completeness.
pub trait NestedType: MarkerTrait {}

/// A utility to specify reader types which provide seeking capabilities too
pub trait ReadSeek: Seek + Read {}
impl<T: Seek + Read> ReadSeek for T {}


/// A utility type which can decode a server response that indicates error
#[derive(RustcDecodable)]
pub struct JsonServerError {
    error: String,
    error_description: Option<String>
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
    fn http_error(&mut self, &hyper::HttpError) -> oauth2::Retry {
        oauth2::Retry::Abort
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

    /// Called whenever the http request returns with a non-success status code.
    /// This can involve authentication issues, or anything else that very much 
    /// depends on the used API method.
    /// The delegate should check the status, header and decoded json error to decide
    /// whether to retry or not. In the latter case, the underlying call will fail.
    fn http_failure(&mut self, _: &hyper::client::Response, JsonServerError) -> oauth2::Retry {
        oauth2::Retry::Abort
    }

    /// Called prior to sending the main request of the given method. It can be used to time 
    /// the call or to print progress information.
    /// It's also useful as you can be sure that a request will definitely be made.
    fn pre_request(&mut self) { }


    /// Called before the API request method returns, in every case. It can be used to clean up
    /// internal state between calls to the API.
    /// This call always has a matching call to `begin(...)`.
    fn finished(&mut self) {}
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

    /// We needed an API key for authentication, but didn't obtain one.
    /// Neither through the authenticator, nor through the Delegate.
    MissingAPIKey,

    /// We required a Token, but didn't get one from the Authenticator
    MissingToken,

    /// An additional, free form field clashed with one of the built-in optional ones
    FieldClash(&'static str),

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