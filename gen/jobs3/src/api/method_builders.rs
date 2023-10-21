use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudTalentSolution`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_jobs3 as jobs3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use jobs3::{CloudTalentSolution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudTalentSolution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `client_events_create(...)`, `companies_create(...)`, `companies_delete(...)`, `companies_get(...)`, `companies_list(...)`, `companies_patch(...)`, `complete(...)`, `jobs_batch_delete(...)`, `jobs_create(...)`, `jobs_delete(...)`, `jobs_get(...)`, `jobs_list(...)`, `jobs_patch(...)`, `jobs_search(...)` and `jobs_search_for_alert(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudTalentSolution<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Report events issued when end user interacts with customer's application that uses Cloud Talent Solution. You may inspect the created events in [self service tools](https://console.cloud.google.com/talent-solution/overview). [Learn more](https://cloud.google.com/talent-solution/docs/management-tools) about self service tools.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent project name.
    pub fn client_events_create(&self, request: CreateClientEventRequest, parent: &str) -> ProjectClientEventCreateCall<'a, S> {
        ProjectClientEventCreateCall {
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
    /// Creates a new company entity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the project under which the company is created. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn companies_create(&self, request: CreateCompanyRequest, parent: &str) -> ProjectCompanyCreateCall<'a, S> {
        ProjectCompanyCreateCall {
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
    /// Deletes specified company. Prerequisite: The company has no jobs associated with it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the company to be deleted. The format is "projects/{project_id}/companies/{company_id}", for example, "projects/api-test-project/companies/foo".
    pub fn companies_delete(&self, name: &str) -> ProjectCompanyDeleteCall<'a, S> {
        ProjectCompanyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves specified company.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the company to be retrieved. The format is "projects/{project_id}/companies/{company_id}", for example, "projects/api-test-project/companies/foo".
    pub fn companies_get(&self, name: &str) -> ProjectCompanyGetCall<'a, S> {
        ProjectCompanyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all companies associated with the service account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the project under which the company is created. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn companies_list(&self, parent: &str) -> ProjectCompanyListCall<'a, S> {
        ProjectCompanyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _require_open_jobs: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates specified company. Company names can't be updated. To update a company name, delete the company and all jobs associated with it, and only then re-create them.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required during company update. The resource name for a company. This is generated by the service when a company is created. The format is "projects/{project_id}/companies/{company_id}", for example, "projects/api-test-project/companies/foo".
    pub fn companies_patch(&self, request: UpdateCompanyRequest, name: &str) -> ProjectCompanyPatchCall<'a, S> {
        ProjectCompanyPatchCall {
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
    /// Deletes a list of Jobs by filter.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the project under which the job is created. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn jobs_batch_delete(&self, request: BatchDeleteJobsRequest, parent: &str) -> ProjectJobBatchDeleteCall<'a, S> {
        ProjectJobBatchDeleteCall {
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
    /// Creates a new job. Typically, the job becomes searchable within 10 seconds, but it may take up to 5 minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the project under which the job is created. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn jobs_create(&self, request: CreateJobRequest, parent: &str) -> ProjectJobCreateCall<'a, S> {
        ProjectJobCreateCall {
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
    /// Deletes the specified job. Typically, the job becomes unsearchable within 10 seconds, but it may take up to 5 minutes.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the job to be deleted. The format is "projects/{project_id}/jobs/{job_id}", for example, "projects/api-test-project/jobs/1234".
    pub fn jobs_delete(&self, name: &str) -> ProjectJobDeleteCall<'a, S> {
        ProjectJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified job, whose status is OPEN or recently EXPIRED within the last 90 days.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the job to retrieve. The format is "projects/{project_id}/jobs/{job_id}", for example, "projects/api-test-project/jobs/1234".
    pub fn jobs_get(&self, name: &str) -> ProjectJobGetCall<'a, S> {
        ProjectJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists jobs by filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the project under which the job is created. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn jobs_list(&self, parent: &str) -> ProjectJobListCall<'a, S> {
        ProjectJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _job_view: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates specified job. Typically, updated contents become visible in search results within 10 seconds, but it may take up to 5 minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required during job update. The resource name for the job. This is generated by the service when a job is created. The format is "projects/{project_id}/jobs/{job_id}", for example, "projects/api-test-project/jobs/1234". Use of this field in job queries and API calls is preferred over the use of requisition_id since this value is unique.
    pub fn jobs_patch(&self, request: UpdateJobRequest, name: &str) -> ProjectJobPatchCall<'a, S> {
        ProjectJobPatchCall {
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
    /// Searches for jobs using the provided SearchJobsRequest. This call constrains the visibility of jobs present in the database, and only returns jobs that the caller has permission to search against.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the project to search within. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn jobs_search(&self, request: SearchJobsRequest, parent: &str) -> ProjectJobSearchCall<'a, S> {
        ProjectJobSearchCall {
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
    /// Searches for jobs using the provided SearchJobsRequest. This API call is intended for the use case of targeting passive job seekers (for example, job seekers who have signed up to receive email alerts about potential job opportunities), and has different algorithmic adjustments that are targeted to passive job seekers. This call constrains the visibility of jobs present in the database, and only returns jobs the caller has permission to search against.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the project to search within. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn jobs_search_for_alert(&self, request: SearchJobsRequest, parent: &str) -> ProjectJobSearchForAlertCall<'a, S> {
        ProjectJobSearchForAlertCall {
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
    /// Completes the specified prefix with keyword suggestions. Intended for use by a job search auto-complete search box.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of project the completion is performed within. The format is "projects/{project_id}", for example, "projects/api-test-project".
    pub fn complete(&self, name: &str) -> ProjectCompleteCall<'a, S> {
        ProjectCompleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _type_: Default::default(),
            _scope: Default::default(),
            _query: Default::default(),
            _page_size: Default::default(),
            _language_codes: Default::default(),
            _language_code: Default::default(),
            _company_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



