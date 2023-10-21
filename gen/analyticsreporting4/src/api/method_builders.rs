use super::*;
/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the [`AnalyticsReporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analyticsreporting4 as analyticsreporting4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analyticsreporting4::{AnalyticsReporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AnalyticsReporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AnalyticsReporting<S>,
}

impl<'a, S> client::MethodsBuilder for ReportMethods<'a, S> {}

impl<'a, S> ReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the Analytics data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_get(&self, request: GetReportsRequest) -> ReportBatchGetCall<'a, S> {
        ReportBatchGetCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userActivity* resources.
/// It is not used directly, but through the [`AnalyticsReporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_analyticsreporting4 as analyticsreporting4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use analyticsreporting4::{AnalyticsReporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AnalyticsReporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.user_activity();
/// # }
/// ```
pub struct UserActivityMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AnalyticsReporting<S>,
}

impl<'a, S> client::MethodsBuilder for UserActivityMethods<'a, S> {}

impl<'a, S> UserActivityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns User Activity data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: SearchUserActivityRequest) -> UserActivitySearchCall<'a, S> {
        UserActivitySearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



