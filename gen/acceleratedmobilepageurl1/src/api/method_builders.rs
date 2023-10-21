use super::*;
/// A builder providing access to all methods supported on *ampUrl* resources.
/// It is not used directly, but through the [`Acceleratedmobilepageurl`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_acceleratedmobilepageurl1 as acceleratedmobilepageurl1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use acceleratedmobilepageurl1::{Acceleratedmobilepageurl, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Acceleratedmobilepageurl::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get(...)`
/// // to build up your call.
/// let rb = hub.amp_urls();
/// # }
/// ```
pub struct AmpUrlMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Acceleratedmobilepageurl<S>,
}

impl<'a, S> client::MethodsBuilder for AmpUrlMethods<'a, S> {}

impl<'a, S> AmpUrlMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns AMP URL(s) and equivalent [AMP Cache URL(s)](https://developers.google.com/amp/cache/overview#amp-cache-url-format).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_get(&self, request: BatchGetAmpUrlsRequest) -> AmpUrlBatchGetCall<'a, S> {
        AmpUrlBatchGetCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



