use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Datapipelines`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datapipelines1 as datapipelines1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datapipelines1::{Datapipelines, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Datapipelines::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_pipelines_create(...)`, `locations_pipelines_delete(...)`, `locations_pipelines_get(...)`, `locations_pipelines_jobs_list(...)`, `locations_pipelines_list(...)`, `locations_pipelines_patch(...)`, `locations_pipelines_run(...)` and `locations_pipelines_stop(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Datapipelines<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists jobs for a given pipeline. Throws a "FORBIDDEN" error if the caller doesn't have permission to access it.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`.
    pub fn locations_pipelines_jobs_list(&self, parent: &str) -> ProjectLocationPipelineJobListCall<'a, S> {
        ProjectLocationPipelineJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a pipeline. For a batch pipeline, you can pass scheduler information. Data Pipelines uses the scheduler information to create an internal scheduler that runs jobs periodically. If the internal scheduler is not configured, you can use RunPipeline to run jobs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`.
    pub fn locations_pipelines_create(&self, request: GoogleCloudDatapipelinesV1Pipeline, parent: &str) -> ProjectLocationPipelineCreateCall<'a, S> {
        ProjectLocationPipelineCreateCall {
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
    /// Deletes a pipeline. If a scheduler job is attached to the pipeline, it will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`.
    pub fn locations_pipelines_delete(&self, name: &str) -> ProjectLocationPipelineDeleteCall<'a, S> {
        ProjectLocationPipelineDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up a single pipeline. Returns a "NOT_FOUND" error if no such pipeline exists. Returns a "FORBIDDEN" error if the caller doesn't have permission to access it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`.
    pub fn locations_pipelines_get(&self, name: &str) -> ProjectLocationPipelineGetCall<'a, S> {
        ProjectLocationPipelineGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists pipelines. Returns a "FORBIDDEN" error if the caller doesn't have permission to access it.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`.
    pub fn locations_pipelines_list(&self, parent: &str) -> ProjectLocationPipelineListCall<'a, S> {
        ProjectLocationPipelineListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates a pipeline. If successful, the updated Pipeline is returned. Returns `NOT_FOUND` if the pipeline doesn't exist. If UpdatePipeline does not return successfully, you can retry the UpdatePipeline request until you receive a successful response.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects). * `LOCATION_ID` is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling `google.cloud.location.Locations.ListLocations`. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in [App Engine regions](https://cloud.google.com/about/locations#region). * `PIPELINE_ID` is the ID of the pipeline. Must be unique for the selected project and location.
    pub fn locations_pipelines_patch(&self, request: GoogleCloudDatapipelinesV1Pipeline, name: &str) -> ProjectLocationPipelinePatchCall<'a, S> {
        ProjectLocationPipelinePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a job for the specified pipeline directly. You can use this method when the internal scheduler is not configured and you want to trigger the job directly or through an external system. Returns a "NOT_FOUND" error if the pipeline doesn't exist. Returns a "FORBIDDEN" error if the user doesn't have permission to access the pipeline or run jobs for the pipeline.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`.
    pub fn locations_pipelines_run(&self, request: GoogleCloudDatapipelinesV1RunPipelineRequest, name: &str) -> ProjectLocationPipelineRunCall<'a, S> {
        ProjectLocationPipelineRunCall {
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
    /// Freezes pipeline execution permanently. If there's a corresponding scheduler entry, it's deleted, and the pipeline state is changed to "ARCHIVED". However, pipeline metadata is retained.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`.
    pub fn locations_pipelines_stop(&self, request: GoogleCloudDatapipelinesV1StopPipelineRequest, name: &str) -> ProjectLocationPipelineStopCall<'a, S> {
        ProjectLocationPipelineStopCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



