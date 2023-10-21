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
/// extern crate google_datastore1 as datastore1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datastore1::{Datastore, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Datastore::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `allocate_ids(...)`, `begin_transaction(...)`, `commit(...)`, `export(...)`, `import(...)`, `indexes_create(...)`, `indexes_delete(...)`, `indexes_get(...)`, `indexes_list(...)`, `lookup(...)`, `operations_cancel(...)`, `operations_delete(...)`, `operations_get(...)`, `operations_list(...)`, `reserve_ids(...)`, `rollback(...)`, `run_aggregation_query(...)` and `run_query(...)`
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
    /// Creates the specified index. A newly created index's initial state is `CREATING`. On completion of the returned google.longrunning.Operation, the state will be `READY`. If the index already exists, the call will return an `ALREADY_EXISTS` status. During index creation, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, removing the index with delete, then re-creating the index with create. Indexes with a single property cannot be created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID against which to make the request.
    pub fn indexes_create(&self, request: GoogleDatastoreAdminV1Index, project_id: &str) -> ProjectIndexCreateCall<'a, S> {
        ProjectIndexCreateCall {
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
    /// Deletes an existing index. An index can only be deleted if it is in a `READY` or `ERROR` state. On successful execution of the request, the index will be in a `DELETING` state. And on completion of the returned google.longrunning.Operation, the index will be removed. During index deletion, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, followed by calling delete again.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID against which to make the request.
    /// * `indexId` - The resource ID of the index to delete.
    pub fn indexes_delete(&self, project_id: &str, index_id: &str) -> ProjectIndexDeleteCall<'a, S> {
        ProjectIndexDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _index_id: index_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an index.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID against which to make the request.
    /// * `indexId` - The resource ID of the index to get.
    pub fn indexes_get(&self, project_id: &str, index_id: &str) -> ProjectIndexGetCall<'a, S> {
        ProjectIndexGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _index_id: index_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the indexes that match the specified filters. Datastore uses an eventually consistent query to fetch the list of indexes and may occasionally return stale results.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID against which to make the request.
    pub fn indexes_list(&self, project_id: &str) -> ProjectIndexListCall<'a, S> {
        ProjectIndexListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, name: &str) -> ProjectOperationCancelCall<'a, S> {
        ProjectOperationCancelCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn operations_delete(&self, name: &str) -> ProjectOperationDeleteCall<'a, S> {
        ProjectOperationDeleteCall {
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> ProjectOperationListCall<'a, S> {
        ProjectOperationListCall {
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
    /// Exports a copy of all or a subset of entities from Google Cloud Datastore to another storage system, such as Google Cloud Storage. Recent updates to entities may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID against which to make the request.
    pub fn export(&self, request: GoogleDatastoreAdminV1ExportEntitiesRequest, project_id: &str) -> ProjectExportCall<'a, S> {
        ProjectExportCall {
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
    /// Imports entities into Google Cloud Datastore. Existing entities with the same key are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportEntities operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Datastore.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID against which to make the request.
    pub fn import(&self, request: GoogleDatastoreAdminV1ImportEntitiesRequest, project_id: &str) -> ProjectImportCall<'a, S> {
        ProjectImportCall {
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



