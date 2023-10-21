use super::*;
/// A builder providing access to all methods supported on *googleServiceAccount* resources.
/// It is not used directly, but through the [`Storagetransfer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storagetransfer1 as storagetransfer1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storagetransfer1::{Storagetransfer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storagetransfer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.google_service_accounts();
/// # }
/// ```
pub struct GoogleServiceAccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storagetransfer<S>,
}

impl<'a, S> client::MethodsBuilder for GoogleServiceAccountMethods<'a, S> {}

impl<'a, S> GoogleServiceAccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the Google service account that is used by Storage Transfer Service to access buckets in the project where transfers run or in other projects. Each Google service account is associated with one Google Cloud project. Users should add this service account to the Google Cloud Storage bucket ACLs to grant access to Storage Transfer Service. This service account is created and owned by Storage Transfer Service and can only be used by Storage Transfer Service.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud project that the Google service account is associated with.
    pub fn get(&self, project_id: &str) -> GoogleServiceAccountGetCall<'a, S> {
        GoogleServiceAccountGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Storagetransfer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storagetransfer1 as storagetransfer1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storagetransfer1::{Storagetransfer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storagetransfer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `agent_pools_create(...)`, `agent_pools_delete(...)`, `agent_pools_get(...)`, `agent_pools_list(...)` and `agent_pools_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storagetransfer<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an agent pool resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The ID of the Google Cloud project that owns the agent pool.
    pub fn agent_pools_create(&self, request: AgentPool, project_id: &str) -> ProjectAgentPoolCreateCall<'a, S> {
        ProjectAgentPoolCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _agent_pool_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an agent pool.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the agent pool to delete.
    pub fn agent_pools_delete(&self, name: &str) -> ProjectAgentPoolDeleteCall<'a, S> {
        ProjectAgentPoolDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an agent pool.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the agent pool to get.
    pub fn agent_pools_get(&self, name: &str) -> ProjectAgentPoolGetCall<'a, S> {
        ProjectAgentPoolGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists agent pools.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. The ID of the Google Cloud project that owns the job.
    pub fn agent_pools_list(&self, project_id: &str) -> ProjectAgentPoolListCall<'a, S> {
        ProjectAgentPoolListCall {
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
    /// Updates an existing agent pool resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Specifies a unique string that identifies the agent pool. Format: `projects/{project_id}/agentPools/{agent_pool_id}`
    pub fn agent_pools_patch(&self, request: AgentPool, name: &str) -> ProjectAgentPoolPatchCall<'a, S> {
        ProjectAgentPoolPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *transferJob* resources.
/// It is not used directly, but through the [`Storagetransfer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storagetransfer1 as storagetransfer1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storagetransfer1::{Storagetransfer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storagetransfer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)`, `patch(...)` and `run(...)`
/// // to build up your call.
/// let rb = hub.transfer_jobs();
/// # }
/// ```
pub struct TransferJobMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storagetransfer<S>,
}

impl<'a, S> client::MethodsBuilder for TransferJobMethods<'a, S> {}

impl<'a, S> TransferJobMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a transfer job that runs periodically.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: TransferJob) -> TransferJobCreateCall<'a, S> {
        TransferJobCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a transfer job. Deleting a transfer job sets its status to DELETED.
    /// 
    /// # Arguments
    ///
    /// * `jobName` - Required. The job to delete.
    /// * `projectId` - Required. The ID of the Google Cloud project that owns the job.
    pub fn delete(&self, job_name: &str, project_id: &str) -> TransferJobDeleteCall<'a, S> {
        TransferJobDeleteCall {
            hub: self.hub,
            _job_name: job_name.to_string(),
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a transfer job.
    /// 
    /// # Arguments
    ///
    /// * `jobName` - Required. The job to get.
    /// * `projectId` - Required. The ID of the Google Cloud project that owns the job.
    pub fn get(&self, job_name: &str, project_id: &str) -> TransferJobGetCall<'a, S> {
        TransferJobGetCall {
            hub: self.hub,
            _job_name: job_name.to_string(),
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists transfer jobs.
    /// 
    /// # Arguments
    ///
    /// * `filter` - Required. A list of query parameters specified as JSON text in the form of: `{"projectId":"my_project_id", "jobNames":["jobid1","jobid2",...], "jobStatuses":["status1","status2",...]}` Since `jobNames` and `jobStatuses` support multiple values, their values must be specified with array notation. `projectId` is required. `jobNames` and `jobStatuses` are optional. The valid values for `jobStatuses` are case-insensitive: ENABLED, DISABLED, and DELETED.
    pub fn list(&self, filter: &str) -> TransferJobListCall<'a, S> {
        TransferJobListCall {
            hub: self.hub,
            _filter: filter.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a transfer job. Updating a job's transfer spec does not affect transfer operations that are running already. **Note:** The job's status field can be modified using this RPC (for example, to set a job's status to DELETED, DISABLED, or ENABLED).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `jobName` - Required. The name of job to update.
    pub fn patch(&self, request: UpdateTransferJobRequest, job_name: &str) -> TransferJobPatchCall<'a, S> {
        TransferJobPatchCall {
            hub: self.hub,
            _request: request,
            _job_name: job_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts a new operation for the specified transfer job. A `TransferJob` has a maximum of one active `TransferOperation`. If this method is called while a `TransferOperation` is active, an error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `jobName` - Required. The name of the transfer job.
    pub fn run(&self, request: RunTransferJobRequest, job_name: &str) -> TransferJobRunCall<'a, S> {
        TransferJobRunCall {
            hub: self.hub,
            _request: request,
            _job_name: job_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *transferOperation* resources.
/// It is not used directly, but through the [`Storagetransfer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storagetransfer1 as storagetransfer1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storagetransfer1::{Storagetransfer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storagetransfer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `get(...)`, `list(...)`, `pause(...)` and `resume(...)`
/// // to build up your call.
/// let rb = hub.transfer_operations();
/// # }
/// ```
pub struct TransferOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storagetransfer<S>,
}

impl<'a, S> client::MethodsBuilder for TransferOperationMethods<'a, S> {}

impl<'a, S> TransferOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a transfer. Use the transferOperations.get method to check if the cancellation succeeded or if the operation completed despite the `cancel` request. When you cancel an operation, the currently running transfer is interrupted. For recurring transfer jobs, the next instance of the transfer job will still run. For example, if your job is configured to run every day at 1pm and you cancel Monday's operation at 1:05pm, Monday's transfer will stop. However, a transfer job will still be attempted on Tuesday. This applies only to currently running operations. If an operation is not currently running, `cancel` does nothing. *Caution:* Canceling a transfer job can leave your data in an unknown state. We recommend that you restore the state at both the destination and the source after the `cancel` request completes so that your data is in a consistent state. When you cancel a job, the next job computes a delta of files and may repair any inconsistent state. For instance, if you run a job every day, and today's job found 10 new files and transferred five files before you canceled the job, tomorrow's transfer operation will compute a new delta with the five files that were not copied today plus any new files discovered tomorrow.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, request: CancelOperationRequest, name: &str) -> TransferOperationCancelCall<'a, S> {
        TransferOperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> TransferOperationGetCall<'a, S> {
        TransferOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists transfer operations. Operations are ordered by their creation time in reverse chronological order.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the type being listed; must be `transferOperations`.
    /// * `filter` - Required. A list of query parameters specified as JSON text in the form of: `{"projectId":"my_project_id", "jobNames":["jobid1","jobid2",...], "operationNames":["opid1","opid2",...], "transferStatuses":["status1","status2",...]}` Since `jobNames`, `operationNames`, and `transferStatuses` support multiple values, they must be specified with array notation. `projectId` is required. `jobNames`, `operationNames`, and `transferStatuses` are optional. The valid values for `transferStatuses` are case-insensitive: IN_PROGRESS, PAUSED, SUCCESS, FAILED, and ABORTED.
    pub fn list(&self, name: &str, filter: &str) -> TransferOperationListCall<'a, S> {
        TransferOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _filter: filter.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Pauses a transfer operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the transfer operation.
    pub fn pause(&self, request: PauseTransferOperationRequest, name: &str) -> TransferOperationPauseCall<'a, S> {
        TransferOperationPauseCall {
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
    /// Resumes a transfer operation that is paused.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the transfer operation.
    pub fn resume(&self, request: ResumeTransferOperationRequest, name: &str) -> TransferOperationResumeCall<'a, S> {
        TransferOperationResumeCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



