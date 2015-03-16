use std::marker::MarkerTrait;
use std::io::{Read, Seek};

use oauth2;
use hyper;

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
