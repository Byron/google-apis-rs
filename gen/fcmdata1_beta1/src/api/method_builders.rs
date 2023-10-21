use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Fcmdata`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fcmdata1_beta1 as fcmdata1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fcmdata1_beta1::{Fcmdata, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fcmdata::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `android_apps_delivery_data_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Fcmdata<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List aggregate delivery data for the given Android application.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The application for which to list delivery data. Format: `projects/{project_id}/androidApps/{app_id}`
    pub fn android_apps_delivery_data_list(&self, parent: &str) -> ProjectAndroidAppDeliveryDataListCall<'a, S> {
        ProjectAndroidAppDeliveryDataListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



