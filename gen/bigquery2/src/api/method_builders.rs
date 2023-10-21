use super::*;
/// A builder providing access to all methods supported on *dataset* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.datasets();
/// # }
/// ```
pub struct DatasetMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for DatasetMethods<'a, S> {}

impl<'a, S> DatasetMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the dataset being deleted
    /// * `datasetId` - Dataset ID of dataset being deleted
    pub fn delete(&self, project_id: &str, dataset_id: &str) -> DatasetDeleteCall<'a, S> {
        DatasetDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delete_contents: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the dataset specified by datasetID.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the requested dataset
    /// * `datasetId` - Dataset ID of the requested dataset
    pub fn get(&self, project_id: &str, dataset_id: &str) -> DatasetGetCall<'a, S> {
        DatasetGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new empty dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the new dataset
    pub fn insert(&self, request: Dataset, project_id: &str) -> DatasetInsertCall<'a, S> {
        DatasetInsertCall {
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
    /// Lists all datasets in the specified project to which you have been granted the READER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the datasets to be listed
    pub fn list(&self, project_id: &str) -> DatasetListCall<'a, S> {
        DatasetListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _all: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the dataset being updated
    /// * `datasetId` - Dataset ID of the dataset being updated
    pub fn patch(&self, request: Dataset, project_id: &str, dataset_id: &str) -> DatasetPatchCall<'a, S> {
        DatasetPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the dataset being updated
    /// * `datasetId` - Dataset ID of the dataset being updated
    pub fn update(&self, request: Dataset, project_id: &str, dataset_id: &str) -> DatasetUpdateCall<'a, S> {
        DatasetUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *job* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)`, `get_query_results(...)`, `insert(...)`, `list(...)` and `query(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for JobMethods<'a, S> {}

impl<'a, S> JobMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - [Required] Project ID of the job to cancel
    /// * `jobId` - [Required] Job ID of the job to cancel
    pub fn cancel(&self, project_id: &str, job_id: &str) -> JobCancelCall<'a, S> {
        JobCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _location: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests the deletion of the metadata of a job. This call returns when the job's metadata is deleted.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the job for which metadata is to be deleted.
    /// * `jobId` - Required. Job ID of the job for which metadata is to be deleted. If this is a parent job which has child jobs, the metadata from all child jobs will be deleted as well. Direct deletion of the metadata of child jobs is not allowed.
    pub fn delete(&self, project_id: &str, job_id: &str) -> JobDeleteCall<'a, S> {
        JobDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _location: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - [Required] Project ID of the requested job
    /// * `jobId` - [Required] Job ID of the requested job
    pub fn get(&self, project_id: &str, job_id: &str) -> JobGetCall<'a, S> {
        JobGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _location: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the results of a query job.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - [Required] Project ID of the query job
    /// * `jobId` - [Required] Job ID of the query job
    pub fn get_query_results(&self, project_id: &str, job_id: &str) -> JobGetQueryResultCall<'a, S> {
        JobGetQueryResultCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _timeout_ms: Default::default(),
            _start_index: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _location: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts a new asynchronous job. Requires the Can View project role.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the project that will be billed for the job
    pub fn insert(&self, request: Job, project_id: &str) -> JobInsertCall<'a, S> {
        JobInsertCall {
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
    /// Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the jobs to list
    pub fn list(&self, project_id: &str) -> JobListCall<'a, S> {
        JobListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _state_filter: Default::default(),
            _projection: Default::default(),
            _parent_job_id: Default::default(),
            _page_token: Default::default(),
            _min_creation_time: Default::default(),
            _max_results: Default::default(),
            _max_creation_time: Default::default(),
            _all_users: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the project billed for the query
    pub fn query(&self, request: QueryRequest, project_id: &str) -> JobQueryCall<'a, S> {
        JobQueryCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *model* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.models();
/// # }
/// ```
pub struct ModelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for ModelMethods<'a, S> {}

impl<'a, S> ModelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the model specified by modelId from the dataset.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the model to delete.
    /// * `datasetId` - Required. Dataset ID of the model to delete.
    /// * `modelId` - Required. Model ID of the model to delete.
    pub fn delete(&self, project_id: &str, dataset_id: &str, model_id: &str) -> ModelDeleteCall<'a, S> {
        ModelDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _model_id: model_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified model resource by model ID.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the requested model.
    /// * `datasetId` - Required. Dataset ID of the requested model.
    /// * `modelId` - Required. Model ID of the requested model.
    pub fn get(&self, project_id: &str, dataset_id: &str, model_id: &str) -> ModelGetCall<'a, S> {
        ModelGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _model_id: model_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all models in the specified dataset. Requires the READER dataset role. After retrieving the list of models, you can get information about a particular model by calling the models.get method.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the models to list.
    /// * `datasetId` - Required. Dataset ID of the models to list.
    pub fn list(&self, project_id: &str, dataset_id: &str) -> ModelListCall<'a, S> {
        ModelListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patch specific fields in the specified model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the model to patch.
    /// * `datasetId` - Required. Dataset ID of the model to patch.
    /// * `modelId` - Required. Model ID of the model to patch.
    pub fn patch(&self, request: Model, project_id: &str, dataset_id: &str, model_id: &str) -> ModelPatchCall<'a, S> {
        ModelPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _model_id: model_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_service_account(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the email address of the service account for your project used for interactions with Google Cloud KMS.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID for which the service account is requested.
    pub fn get_service_account(&self, project_id: &str) -> ProjectGetServiceAccountCall<'a, S> {
        ProjectGetServiceAccountCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all projects to which you have been granted any project role.
    pub fn list(&self) -> ProjectListCall<'a, S> {
        ProjectListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *routine* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.routines();
/// # }
/// ```
pub struct RoutineMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for RoutineMethods<'a, S> {}

impl<'a, S> RoutineMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the routine specified by routineId from the dataset.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the routine to delete
    /// * `datasetId` - Required. Dataset ID of the routine to delete
    /// * `routineId` - Required. Routine ID of the routine to delete
    pub fn delete(&self, project_id: &str, dataset_id: &str, routine_id: &str) -> RoutineDeleteCall<'a, S> {
        RoutineDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _routine_id: routine_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified routine resource by routine ID.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the requested routine
    /// * `datasetId` - Required. Dataset ID of the requested routine
    /// * `routineId` - Required. Routine ID of the requested routine
    pub fn get(&self, project_id: &str, dataset_id: &str, routine_id: &str) -> RoutineGetCall<'a, S> {
        RoutineGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _routine_id: routine_id.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new routine in the dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the new routine
    /// * `datasetId` - Required. Dataset ID of the new routine
    pub fn insert(&self, request: Routine, project_id: &str, dataset_id: &str) -> RoutineInsertCall<'a, S> {
        RoutineInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all routines in the specified dataset. Requires the READER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the routines to list
    /// * `datasetId` - Required. Dataset ID of the routines to list
    pub fn list(&self, project_id: &str, dataset_id: &str) -> RoutineListCall<'a, S> {
        RoutineListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing routine. The update method replaces the entire Routine resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the routine to update
    /// * `datasetId` - Required. Dataset ID of the routine to update
    /// * `routineId` - Required. Routine ID of the routine to update
    pub fn update(&self, request: Routine, project_id: &str, dataset_id: &str, routine_id: &str) -> RoutineUpdateCall<'a, S> {
        RoutineUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _routine_id: routine_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *rowAccessPolicy* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_iam_policy(...)`, `list(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.row_access_policies();
/// # }
/// ```
pub struct RowAccessPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for RowAccessPolicyMethods<'a, S> {}

impl<'a, S> RowAccessPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> RowAccessPolicyGetIamPolicyCall<'a, S> {
        RowAccessPolicyGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all row access policies on the specified table.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the row access policies to list.
    /// * `datasetId` - Required. Dataset ID of row access policies to list.
    /// * `tableId` - Required. Table ID of the table to list row access policies.
    pub fn list(&self, project_id: &str, dataset_id: &str, table_id: &str) -> RowAccessPolicyListCall<'a, S> {
        RowAccessPolicyListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> RowAccessPolicySetIamPolicyCall<'a, S> {
        RowAccessPolicySetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> RowAccessPolicyTestIamPermissionCall<'a, S> {
        RowAccessPolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *tabledata* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert_all(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.tabledata();
/// # }
/// ```
pub struct TabledataMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for TabledataMethods<'a, S> {}

impl<'a, S> TabledataMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the destination table.
    /// * `datasetId` - Dataset ID of the destination table.
    /// * `tableId` - Table ID of the destination table.
    pub fn insert_all(&self, request: TableDataInsertAllRequest, project_id: &str, dataset_id: &str, table_id: &str) -> TabledataInsertAllCall<'a, S> {
        TabledataInsertAllCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves table data from a specified set of rows. Requires the READER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the table to read
    /// * `datasetId` - Dataset ID of the table to read
    /// * `tableId` - Table ID of the table to read
    pub fn list(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TabledataListCall<'a, S> {
        TabledataListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _start_index: Default::default(),
            _selected_fields: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *table* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.tables();
/// # }
/// ```
pub struct TableMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for TableMethods<'a, S> {}

impl<'a, S> TableMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the table to delete
    /// * `datasetId` - Dataset ID of the table to delete
    /// * `tableId` - Table ID of the table to delete
    pub fn delete(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TableDeleteCall<'a, S> {
        TableDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the requested table
    /// * `datasetId` - Dataset ID of the requested table
    /// * `tableId` - Table ID of the requested table
    pub fn get(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TableGetCall<'a, S> {
        TableGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _view: Default::default(),
            _selected_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> TableGetIamPolicyCall<'a, S> {
        TableGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new, empty table in the dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the new table
    /// * `datasetId` - Dataset ID of the new table
    pub fn insert(&self, request: Table, project_id: &str, dataset_id: &str) -> TableInsertCall<'a, S> {
        TableInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all tables in the specified dataset. Requires the READER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the tables to list
    /// * `datasetId` - Dataset ID of the tables to list
    pub fn list(&self, project_id: &str, dataset_id: &str) -> TableListCall<'a, S> {
        TableListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the table to update
    /// * `datasetId` - Dataset ID of the table to update
    /// * `tableId` - Table ID of the table to update
    pub fn patch(&self, request: Table, project_id: &str, dataset_id: &str, table_id: &str) -> TablePatchCall<'a, S> {
        TablePatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _autodetect_schema: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> TableSetIamPolicyCall<'a, S> {
        TableSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> TableTestIamPermissionCall<'a, S> {
        TableTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of the table to update
    /// * `datasetId` - Dataset ID of the table to update
    /// * `tableId` - Table ID of the table to update
    pub fn update(&self, request: Table, project_id: &str, dataset_id: &str, table_id: &str) -> TableUpdateCall<'a, S> {
        TableUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _autodetect_schema: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



