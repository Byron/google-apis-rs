use super::*;
/// A builder providing access to all methods supported on *url* resources.
/// It is not used directly, but through the [`Urlshortener`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_urlshortener1 as urlshortener1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use urlshortener1::{Urlshortener, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Urlshortener::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.url();
/// # }
/// ```
pub struct UrlMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Urlshortener<S>,
}

impl<'a, S> client::MethodsBuilder for UrlMethods<'a, S> {}

impl<'a, S> UrlMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Expands a short URL or gets creation time and analytics.
    /// 
    /// # Arguments
    ///
    /// * `shortUrl` - The short URL, including the protocol.
    pub fn get(&self, short_url: &str) -> UrlGetCall<'a, S> {
        UrlGetCall {
            hub: self.hub,
            _short_url: short_url.to_string(),
            _projection: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new short URL.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Url) -> UrlInsertCall<'a, S> {
        UrlInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of URLs shortened by a user.
    pub fn list(&self) -> UrlListCall<'a, S> {
        UrlListCall {
            hub: self.hub,
            _start_token: Default::default(),
            _projection: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



