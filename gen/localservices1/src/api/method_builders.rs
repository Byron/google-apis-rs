use super::*;
/// A builder providing access to all methods supported on *accountReport* resources.
/// It is not used directly, but through the [`Localservices`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_localservices1 as localservices1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.account_reports();
/// # }
/// ```
pub struct AccountReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Localservices<S>,
}

impl<'a, S> client::MethodsBuilder for AccountReportMethods<'a, S> {}

impl<'a, S> AccountReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get account reports containing aggregate account data of all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
    pub fn search(&self) -> AccountReportSearchCall<'a, S> {
        AccountReportSearchCall {
            hub: self.hub,
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *detailedLeadReport* resources.
/// It is not used directly, but through the [`Localservices`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_localservices1 as localservices1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.detailed_lead_reports();
/// # }
/// ```
pub struct DetailedLeadReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Localservices<S>,
}

impl<'a, S> client::MethodsBuilder for DetailedLeadReportMethods<'a, S> {}

impl<'a, S> DetailedLeadReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get detailed lead reports containing leads that have been received by all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
    pub fn search(&self) -> DetailedLeadReportSearchCall<'a, S> {
        DetailedLeadReportSearchCall {
            hub: self.hub,
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



