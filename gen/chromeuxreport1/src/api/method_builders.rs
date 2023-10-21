use super::*;
/// A builder providing access to all methods supported on *record* resources.
/// It is not used directly, but through the [`ChromeUXReport`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromeuxreport1 as chromeuxreport1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use chromeuxreport1::{ChromeUXReport, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromeUXReport::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `query_record(...)`
/// // to build up your call.
/// let rb = hub.records();
/// # }
/// ```
pub struct RecordMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ChromeUXReport<S>,
}

impl<'a, S> client::MethodsBuilder for RecordMethods<'a, S> {}

impl<'a, S> RecordMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Queries the Chrome User Experience for a single `record` for a given site. Returns a `record` that contains one or more `metrics` corresponding to performance data about the requested site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn query_record(&self, request: QueryRequest) -> RecordQueryRecordCall<'a, S> {
        RecordQueryRecordCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



