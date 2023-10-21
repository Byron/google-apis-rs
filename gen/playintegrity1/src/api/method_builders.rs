use super::*;
/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`PlayIntegrity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playintegrity1 as playintegrity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use playintegrity1::{PlayIntegrity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlayIntegrity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `decode_integrity_token(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayIntegrity<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Decodes the integrity token and returns the token payload.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` -  Package name of the app the attached integrity token belongs to.
    pub fn decode_integrity_token(&self, request: DecodeIntegrityTokenRequest, package_name: &str) -> MethodDecodeIntegrityTokenCall<'a, S> {
        MethodDecodeIntegrityTokenCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



