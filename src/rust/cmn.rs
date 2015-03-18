use std::marker::MarkerTrait;
use std::io::{self, Read, Seek, Cursor};

use mime::{Mime, TopLevel, SubLevel, Attr, Value};
use oauth2;
use hyper;
use hyper::header::{ContentType, ContentLength, Headers};

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
    fn pre_request(&mut self, method_name: &str) { let _ = method_name; }
}

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

const BOUNDARY: &'static str = "MDuXWGyeE33QFXGchb2VFWc4Z7945d";

/// Provides a `Read` interface that converts multiple parts into the protocol
/// identified by [RFC2387](https://tools.ietf.org/html/rfc2387).
/// **Note**: This implementation is just as rich as it needs to be to perform uploads
/// to google APIs, and might not be a fully-featured implementation.
#[derive(Default)]
pub struct MultiPartReader<'a> {
    raw_parts: Vec<(Headers, &'a mut Read)>,
    current_part: Option<(Cursor<Vec<u8>>, &'a mut Read)>,
}

impl<'a> MultiPartReader<'a> {

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
    pub fn add_part(mut self, reader: &'a mut Read, size: u64, mime_type: &Mime) -> MultiPartReader<'a> {
        let mut headers = Headers::new();
        headers.set(ContentType(mime_type.clone()));
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
}

impl<'a> Read for MultiPartReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::from_os_error(0))
    }
}