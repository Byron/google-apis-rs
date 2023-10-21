use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Transcoder`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_transcoder1_beta1 as transcoder1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use transcoder1_beta1::{Transcoder, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Transcoder::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_job_templates_create(...)`, `locations_job_templates_delete(...)`, `locations_job_templates_get(...)`, `locations_job_templates_list(...)`, `locations_jobs_create(...)`, `locations_jobs_delete(...)`, `locations_jobs_get(...)` and `locations_jobs_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Transcoder<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a job template in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent location to create this job template. Format: `projects/{project}/locations/{location}`
    pub fn locations_job_templates_create(&self, request: JobTemplate, parent: &str) -> ProjectLocationJobTemplateCreateCall<'a, S> {
        ProjectLocationJobTemplateCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _job_template_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a job template.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the job template to delete. `projects/{project}/locations/{location}/jobTemplates/{job_template}`
    pub fn locations_job_templates_delete(&self, name: &str) -> ProjectLocationJobTemplateDeleteCall<'a, S> {
        ProjectLocationJobTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the job template data.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the job template to retrieve. Format: `projects/{project}/locations/{location}/jobTemplates/{job_template}`
    pub fn locations_job_templates_get(&self, name: &str) -> ProjectLocationJobTemplateGetCall<'a, S> {
        ProjectLocationJobTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists job templates in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent location from which to retrieve the collection of job templates. Format: `projects/{project}/locations/{location}`
    pub fn locations_job_templates_list(&self, parent: &str) -> ProjectLocationJobTemplateListCall<'a, S> {
        ProjectLocationJobTemplateListCall {
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
    /// Creates a job in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent location to create and process this job. Format: `projects/{project}/locations/{location}`
    pub fn locations_jobs_create(&self, request: Job, parent: &str) -> ProjectLocationJobCreateCall<'a, S> {
        ProjectLocationJobCreateCall {
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
    /// Deletes a job.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the job to delete. Format: `projects/{project}/locations/{location}/jobs/{job}`
    pub fn locations_jobs_delete(&self, name: &str) -> ProjectLocationJobDeleteCall<'a, S> {
        ProjectLocationJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the job data.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the job to retrieve. Format: `projects/{project}/locations/{location}/jobs/{job}`
    pub fn locations_jobs_get(&self, name: &str) -> ProjectLocationJobGetCall<'a, S> {
        ProjectLocationJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists jobs in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Format: `projects/{project}/locations/{location}`
    pub fn locations_jobs_list(&self, parent: &str) -> ProjectLocationJobListCall<'a, S> {
        ProjectLocationJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



