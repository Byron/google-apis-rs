use super::*;
/// A builder providing access to all methods supported on *api* resources.
/// It is not used directly, but through the [`Discovery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_discovery1 as discovery1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use discovery1::{Discovery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Discovery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_rest(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.apis();
/// # }
/// ```
pub struct ApiMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Discovery<S>,
}

impl<'a, S> client::MethodsBuilder for ApiMethods<'a, S> {}

impl<'a, S> ApiMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the description of a particular version of an api.
    /// 
    /// # Arguments
    ///
    /// * `api` - The name of the API.
    /// * `version` - The version of the API.
    pub fn get_rest(&self, api: &str, version: &str) -> ApiGetRestCall<'a, S> {
        ApiGetRestCall {
            hub: self.hub,
            _api: api.to_string(),
            _version: version.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the list of APIs supported at this endpoint.
    pub fn list(&self) -> ApiListCall<'a, S> {
        ApiListCall {
            hub: self.hub,
            _preferred: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



