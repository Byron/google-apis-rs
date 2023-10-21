use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Datastore`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datastore1_beta3 as datastore1_beta3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datastore1_beta3::{Datastore, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Datastore::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `allocate_ids(...)`, `begin_transaction(...)`, `commit(...)`, `lookup(...)`, `reserve_ids(...)`, `rollback(...)`, `run_aggregation_query(...)` and `run_query(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Datastore<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Allocates IDs for the given keys, which is useful for referencing an entity before it is inserted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn allocate_ids(&self, request: AllocateIdsRequest, project_id: &str) -> ProjectAllocateIdCall<'a, S> {
        ProjectAllocateIdCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Begins a new transaction.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn begin_transaction(&self, request: BeginTransactionRequest, project_id: &str) -> ProjectBeginTransactionCall<'a, S> {
        ProjectBeginTransactionCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commits a transaction, optionally creating, deleting or modifying some entities.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn commit(&self, request: CommitRequest, project_id: &str) -> ProjectCommitCall<'a, S> {
        ProjectCommitCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up entities by key.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn lookup(&self, request: LookupRequest, project_id: &str) -> ProjectLookupCall<'a, S> {
        ProjectLookupCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Prevents the supplied keys' IDs from being auto-allocated by Cloud Datastore.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn reserve_ids(&self, request: ReserveIdsRequest, project_id: &str) -> ProjectReserveIdCall<'a, S> {
        ProjectReserveIdCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rolls back a transaction.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn rollback(&self, request: RollbackRequest, project_id: &str) -> ProjectRollbackCall<'a, S> {
        ProjectRollbackCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs an aggregation query.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn run_aggregation_query(&self, request: RunAggregationQueryRequest, project_id: &str) -> ProjectRunAggregationQueryCall<'a, S> {
        ProjectRunAggregationQueryCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Queries for entities.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the project against which to make the request.
    pub fn run_query(&self, request: RunQueryRequest, project_id: &str) -> ProjectRunQueryCall<'a, S> {
        ProjectRunQueryCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



