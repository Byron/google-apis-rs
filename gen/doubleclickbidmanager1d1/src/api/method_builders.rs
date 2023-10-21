use super::*;
/// A builder providing access to all methods supported on *query* resources.
/// It is not used directly, but through the [`DoubleClickBidManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_doubleclickbidmanager1d1 as doubleclickbidmanager1d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use doubleclickbidmanager1d1::{DoubleClickBidManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DoubleClickBidManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `createquery(...)`, `deletequery(...)`, `getquery(...)`, `listqueries(...)` and `runquery(...)`
/// // to build up your call.
/// let rb = hub.queries();
/// # }
/// ```
pub struct QueryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DoubleClickBidManager<S>,
}

impl<'a, S> client::MethodsBuilder for QueryMethods<'a, S> {}

impl<'a, S> QueryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a query.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn createquery(&self, request: Query) -> QueryCreatequeryCall<'a, S> {
        QueryCreatequeryCall {
            hub: self.hub,
            _request: request,
            _asynchronous: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a stored query as well as the associated stored reports.
    /// 
    /// # Arguments
    ///
    /// * `queryId` - Query ID to delete.
    pub fn deletequery(&self, query_id: i64) -> QueryDeletequeryCall<'a, S> {
        QueryDeletequeryCall {
            hub: self.hub,
            _query_id: query_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a stored query.
    /// 
    /// # Arguments
    ///
    /// * `queryId` - Query ID to retrieve.
    pub fn getquery(&self, query_id: i64) -> QueryGetqueryCall<'a, S> {
        QueryGetqueryCall {
            hub: self.hub,
            _query_id: query_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves stored queries.
    pub fn listqueries(&self) -> QueryListqueryCall<'a, S> {
        QueryListqueryCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a stored query to generate a report.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `queryId` - Query ID to run.
    pub fn runquery(&self, request: RunQueryRequest, query_id: i64) -> QueryRunqueryCall<'a, S> {
        QueryRunqueryCall {
            hub: self.hub,
            _request: request,
            _query_id: query_id,
            _asynchronous: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the [`DoubleClickBidManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_doubleclickbidmanager1d1 as doubleclickbidmanager1d1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use doubleclickbidmanager1d1::{DoubleClickBidManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DoubleClickBidManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `listreports(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DoubleClickBidManager<S>,
}

impl<'a, S> client::MethodsBuilder for ReportMethods<'a, S> {}

impl<'a, S> ReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves stored reports.
    /// 
    /// # Arguments
    ///
    /// * `queryId` - Query ID with which the reports are associated.
    pub fn listreports(&self, query_id: i64) -> ReportListreportCall<'a, S> {
        ReportListreportCall {
            hub: self.hub,
            _query_id: query_id,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



