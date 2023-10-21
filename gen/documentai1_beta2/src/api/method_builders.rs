use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Document`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_documentai1_beta2 as documentai1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use documentai1_beta2::{Document, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Document::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `documents_batch_process(...)`, `documents_process(...)`, `locations_documents_batch_process(...)`, `locations_documents_process(...)`, `locations_operations_get(...)` and `operations_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Document<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically.
    pub fn documents_batch_process(&self, request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest, parent: &str) -> ProjectDocumentBatchProcesCall<'a, S> {
        ProjectDocumentBatchProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
    pub fn documents_process(&self, request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest, parent: &str) -> ProjectDocumentProcesCall<'a, S> {
        ProjectDocumentProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically.
    pub fn locations_documents_batch_process(&self, request: GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest, parent: &str) -> ProjectLocationDocumentBatchProcesCall<'a, S> {
        ProjectLocationDocumentBatchProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Processes a single document.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method.
    pub fn locations_documents_process(&self, request: GoogleCloudDocumentaiV1beta2ProcessDocumentRequest, parent: &str) -> ProjectLocationDocumentProcesCall<'a, S> {
        ProjectLocationDocumentProcesCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, S> {
        ProjectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



