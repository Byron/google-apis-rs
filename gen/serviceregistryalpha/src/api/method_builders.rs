use super::*;
/// A builder providing access to all methods supported on *endpoint* resources.
/// It is not used directly, but through the [`ServiceRegistry`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_serviceregistryalpha as serviceregistryalpha;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use serviceregistryalpha::{ServiceRegistry, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ServiceRegistry::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.endpoints();
/// # }
/// ```
pub struct EndpointMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ServiceRegistry<S>,
}

impl<'a, S> client::MethodsBuilder for EndpointMethods<'a, S> {}

impl<'a, S> EndpointMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an endpoint.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `endpoint` - The name of the endpoint for this request.
    pub fn delete(&self, project: &str, endpoint: &str) -> EndpointDeleteCall<'a, S> {
        EndpointDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _endpoint: endpoint.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an endpoint.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `endpoint` - The name of the endpoint for this request.
    pub fn get(&self, project: &str, endpoint: &str) -> EndpointGetCall<'a, S> {
        EndpointGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _endpoint: endpoint.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an endpoint.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    pub fn insert(&self, request: Endpoint, project: &str) -> EndpointInsertCall<'a, S> {
        EndpointInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists endpoints for a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    pub fn list(&self, project: &str) -> EndpointListCall<'a, S> {
        EndpointListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an endpoint. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    /// * `endpoint` - The name of the endpoint for this request.
    pub fn patch(&self, request: Endpoint, project: &str, endpoint: &str) -> EndpointPatchCall<'a, S> {
        EndpointPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _endpoint: endpoint.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an endpoint.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    /// * `endpoint` - The name of the endpoint for this request.
    pub fn update(&self, request: Endpoint, project: &str, endpoint: &str) -> EndpointUpdateCall<'a, S> {
        EndpointUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _endpoint: endpoint.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`ServiceRegistry`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_serviceregistryalpha as serviceregistryalpha;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use serviceregistryalpha::{ServiceRegistry, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ServiceRegistry::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ServiceRegistry<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a specific operation.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    /// * `operation` - The name of the operation for this request.
    pub fn get(&self, project: &str, operation: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all operations for a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID for this request.
    pub fn list(&self, project: &str) -> OperationListCall<'a, S> {
        OperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



