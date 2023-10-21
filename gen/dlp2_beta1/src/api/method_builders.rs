use super::*;
/// A builder providing access to all methods supported on *content* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2_beta1::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `deidentify(...)`, `inspect(...)` and `redact(...)`
/// // to build up your call.
/// let rb = hub.content();
/// # }
/// ```
pub struct ContentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for ContentMethods<'a, S> {}

impl<'a, S> ContentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// De-identifies potentially sensitive info from a list of strings.
    /// This method has limits on input size and output size.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn deidentify(&self, request: GooglePrivacyDlpV2beta1DeidentifyContentRequest) -> ContentDeidentifyCall<'a, S> {
        ContentDeidentifyCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds potentially sensitive info in a list of strings.
    /// This method has limits on input size, processing time, and output size.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn inspect(&self, request: GooglePrivacyDlpV2beta1InspectContentRequest) -> ContentInspectCall<'a, S> {
        ContentInspectCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Redacts potentially sensitive info from a list of strings.
    /// This method has limits on input size, processing time, and output size.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn redact(&self, request: GooglePrivacyDlpV2beta1RedactContentRequest) -> ContentRedactCall<'a, S> {
        ContentRedactCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *rootCategory* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2_beta1::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `info_types_list(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.root_categories();
/// # }
/// ```
pub struct RootCategoryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for RootCategoryMethods<'a, S> {}

impl<'a, S> RootCategoryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns sensitive information types for given category.
    /// 
    /// # Arguments
    ///
    /// * `category` - Category name as returned by ListRootCategories.
    pub fn info_types_list(&self, category: &str) -> RootCategoryInfoTypeListCall<'a, S> {
        RootCategoryInfoTypeListCall {
            hub: self.hub,
            _category: category.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of root categories of sensitive information.
    pub fn list(&self) -> RootCategoryListCall<'a, S> {
        RootCategoryListCall {
            hub: self.hub,
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *riskAnalysi* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2_beta1::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `operations_cancel(...)`, `operations_delete(...)`, `operations_get(...)` and `operations_list(...)`
/// // to build up your call.
/// let rb = hub.risk_analysis();
/// # }
/// ```
pub struct RiskAnalysiMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for RiskAnalysiMethods<'a, S> {}

impl<'a, S> RiskAnalysiMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels an operation. Use the `inspect.operations.get` to check whether the cancellation succeeded or the operation completed despite cancellation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> RiskAnalysiOperationCancelCall<'a, S> {
        RiskAnalysiOperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// This method is not supported and the server returns `UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn operations_delete(&self, name: &str) -> RiskAnalysiOperationDeleteCall<'a, S> {
        RiskAnalysiOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the list of long running operations.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> RiskAnalysiOperationListCall<'a, S> {
        RiskAnalysiOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> RiskAnalysiOperationGetCall<'a, S> {
        RiskAnalysiOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dataSource* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2_beta1::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analyze(...)`
/// // to build up your call.
/// let rb = hub.data_source();
/// # }
/// ```
pub struct DataSourceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for DataSourceMethods<'a, S> {}

impl<'a, S> DataSourceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Schedules a job to compute risk analysis metrics over content in a Google
    /// Cloud Platform repository.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn analyze(&self, request: GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest) -> DataSourceAnalyzeCall<'a, S> {
        DataSourceAnalyzeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *inspect* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2_beta1 as dlp2_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2_beta1::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `operations_cancel(...)`, `operations_create(...)`, `operations_delete(...)`, `operations_get(...)`, `operations_list(...)` and `results_findings_list(...)`
/// // to build up your call.
/// let rb = hub.inspect();
/// # }
/// ```
pub struct InspectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for InspectMethods<'a, S> {}

impl<'a, S> InspectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Schedules a job scanning content in a Google Cloud Platform data
    /// repository.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn operations_create(&self, request: GooglePrivacyDlpV2beta1CreateInspectOperationRequest) -> InspectOperationCreateCall<'a, S> {
        InspectOperationCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels an operation. Use the `inspect.operations.get` to check whether the cancellation succeeded or the operation completed despite cancellation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> InspectOperationCancelCall<'a, S> {
        InspectOperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// This method is not supported and the server returns `UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn operations_delete(&self, name: &str) -> InspectOperationDeleteCall<'a, S> {
        InspectOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the list of long running operations.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> InspectOperationListCall<'a, S> {
        InspectOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> InspectOperationGetCall<'a, S> {
        InspectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns list of results for given inspect operation result set id.
    /// 
    /// # Arguments
    ///
    /// * `name` - Identifier of the results set returned as metadata of
    ///            the longrunning operation created by a call to InspectDataSource.
    ///            Should be in the format of `inspect/results/{id}`.
    pub fn results_findings_list(&self, name: &str) -> InspectResultFindingListCall<'a, S> {
        InspectResultFindingListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



