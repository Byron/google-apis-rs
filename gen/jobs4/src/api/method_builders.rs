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
/// extern crate google_jobs4 as jobs4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use jobs4::{CloudTalentSolution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudTalentSolution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `operations_get(...)`, `tenants_client_events_create(...)`, `tenants_companies_create(...)`, `tenants_companies_delete(...)`, `tenants_companies_get(...)`, `tenants_companies_list(...)`, `tenants_companies_patch(...)`, `tenants_complete_query(...)`, `tenants_create(...)`, `tenants_delete(...)`, `tenants_get(...)`, `tenants_jobs_batch_create(...)`, `tenants_jobs_batch_delete(...)`, `tenants_jobs_batch_update(...)`, `tenants_jobs_create(...)`, `tenants_jobs_delete(...)`, `tenants_jobs_get(...)`, `tenants_jobs_list(...)`, `tenants_jobs_patch(...)`, `tenants_jobs_search(...)`, `tenants_jobs_search_for_alert(...)`, `tenants_list(...)` and `tenants_patch(...)`
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
    /// Report events issued when end user interacts with customer's application that uses Cloud Talent Solution. You may inspect the created events in [self service tools](https://console.cloud.google.com/talent-solution/overview). [Learn more](https://cloud.google.com/talent-solution/docs/management-tools) about self service tools.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the tenant under which the event is created. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    pub fn tenants_client_events_create(&self, request: ClientEvent, parent: &str) -> ProjectTenantClientEventCreateCall<'a, S> {
        ProjectTenantClientEventCreateCall {
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
    /// * `parent` - Required. Resource name of the tenant under which the company is created. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    pub fn tenants_companies_create(&self, request: Company, parent: &str) -> ProjectTenantCompanyCreateCall<'a, S> {
        ProjectTenantCompanyCreateCall {
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
    /// * `name` - Required. The resource name of the company to be deleted. The format is "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for example, "projects/foo/tenants/bar/companies/baz".
    pub fn tenants_companies_delete(&self, name: &str) -> ProjectTenantCompanyDeleteCall<'a, S> {
        ProjectTenantCompanyDeleteCall {
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
    /// * `name` - Required. The resource name of the company to be retrieved. The format is "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for example, "projects/api-test-project/tenants/foo/companies/bar".
    pub fn tenants_companies_get(&self, name: &str) -> ProjectTenantCompanyGetCall<'a, S> {
        ProjectTenantCompanyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all companies associated with the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the tenant under which the company is created. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    pub fn tenants_companies_list(&self, parent: &str) -> ProjectTenantCompanyListCall<'a, S> {
        ProjectTenantCompanyListCall {
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
    /// Updates specified company.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required during company update. The resource name for a company. This is generated by the service when a company is created. The format is "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for example, "projects/foo/tenants/bar/companies/baz".
    pub fn tenants_companies_patch(&self, request: Company, name: &str) -> ProjectTenantCompanyPatchCall<'a, S> {
        ProjectTenantCompanyPatchCall {
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
    /// Begins executing a batch create jobs operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the tenant under which the job is created. The format is "projects/{project_id}/tenants/{tenant_id}". For example, "projects/foo/tenants/bar".
    pub fn tenants_jobs_batch_create(&self, request: BatchCreateJobsRequest, parent: &str) -> ProjectTenantJobBatchCreateCall<'a, S> {
        ProjectTenantJobBatchCreateCall {
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
    /// Begins executing a batch delete jobs operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the tenant under which the job is created. The format is "projects/{project_id}/tenants/{tenant_id}". For example, "projects/foo/tenants/bar". The parent of all of the jobs specified in `names` must match this field.
    pub fn tenants_jobs_batch_delete(&self, request: BatchDeleteJobsRequest, parent: &str) -> ProjectTenantJobBatchDeleteCall<'a, S> {
        ProjectTenantJobBatchDeleteCall {
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
    /// Begins executing a batch update jobs operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the tenant under which the job is created. The format is "projects/{project_id}/tenants/{tenant_id}". For example, "projects/foo/tenants/bar".
    pub fn tenants_jobs_batch_update(&self, request: BatchUpdateJobsRequest, parent: &str) -> ProjectTenantJobBatchUpdateCall<'a, S> {
        ProjectTenantJobBatchUpdateCall {
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
    /// * `parent` - Required. The resource name of the tenant under which the job is created. The format is "projects/{project_id}/tenants/{tenant_id}". For example, "projects/foo/tenants/bar".
    pub fn tenants_jobs_create(&self, request: Job, parent: &str) -> ProjectTenantJobCreateCall<'a, S> {
        ProjectTenantJobCreateCall {
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
    /// * `name` - Required. The resource name of the job to be deleted. The format is "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For example, "projects/foo/tenants/bar/jobs/baz".
    pub fn tenants_jobs_delete(&self, name: &str) -> ProjectTenantJobDeleteCall<'a, S> {
        ProjectTenantJobDeleteCall {
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
    /// * `name` - Required. The resource name of the job to retrieve. The format is "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For example, "projects/foo/tenants/bar/jobs/baz".
    pub fn tenants_jobs_get(&self, name: &str) -> ProjectTenantJobGetCall<'a, S> {
        ProjectTenantJobGetCall {
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
    /// * `parent` - Required. The resource name of the tenant under which the job is created. The format is "projects/{project_id}/tenants/{tenant_id}". For example, "projects/foo/tenants/bar".
    pub fn tenants_jobs_list(&self, parent: &str) -> ProjectTenantJobListCall<'a, S> {
        ProjectTenantJobListCall {
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
    /// * `name` - Required during job update. The resource name for the job. This is generated by the service when a job is created. The format is "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For example, "projects/foo/tenants/bar/jobs/baz". Use of this field in job queries and API calls is preferred over the use of requisition_id since this value is unique.
    pub fn tenants_jobs_patch(&self, request: Job, name: &str) -> ProjectTenantJobPatchCall<'a, S> {
        ProjectTenantJobPatchCall {
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
    /// Searches for jobs using the provided SearchJobsRequest. This call constrains the visibility of jobs present in the database, and only returns jobs that the caller has permission to search against.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the tenant to search within. The format is "projects/{project_id}/tenants/{tenant_id}". For example, "projects/foo/tenants/bar".
    pub fn tenants_jobs_search(&self, request: SearchJobsRequest, parent: &str) -> ProjectTenantJobSearchCall<'a, S> {
        ProjectTenantJobSearchCall {
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
    /// Searches for jobs using the provided SearchJobsRequest. This API call is intended for the use case of targeting passive job seekers (for example, job seekers who have signed up to receive email alerts about potential job opportunities), it has different algorithmic adjustments that are designed to specifically target passive job seekers. This call constrains the visibility of jobs present in the database, and only returns jobs the caller has permission to search against.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the tenant to search within. The format is "projects/{project_id}/tenants/{tenant_id}". For example, "projects/foo/tenants/bar".
    pub fn tenants_jobs_search_for_alert(&self, request: SearchJobsRequest, parent: &str) -> ProjectTenantJobSearchForAlertCall<'a, S> {
        ProjectTenantJobSearchForAlertCall {
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
    /// * `tenant` - Required. Resource name of tenant the completion is performed within. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    pub fn tenants_complete_query(&self, tenant: &str) -> ProjectTenantCompleteQueryCall<'a, S> {
        ProjectTenantCompleteQueryCall {
            hub: self.hub,
            _tenant: tenant.to_string(),
            _type_: Default::default(),
            _scope: Default::default(),
            _query: Default::default(),
            _page_size: Default::default(),
            _language_codes: Default::default(),
            _company: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new tenant entity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the project under which the tenant is created. The format is "projects/{project_id}", for example, "projects/foo".
    pub fn tenants_create(&self, request: Tenant, parent: &str) -> ProjectTenantCreateCall<'a, S> {
        ProjectTenantCreateCall {
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
    /// Deletes specified tenant.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the tenant to be deleted. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    pub fn tenants_delete(&self, name: &str) -> ProjectTenantDeleteCall<'a, S> {
        ProjectTenantDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves specified tenant.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the tenant to be retrieved. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    pub fn tenants_get(&self, name: &str) -> ProjectTenantGetCall<'a, S> {
        ProjectTenantGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all tenants associated with the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the project under which the tenant is created. The format is "projects/{project_id}", for example, "projects/foo".
    pub fn tenants_list(&self, parent: &str) -> ProjectTenantListCall<'a, S> {
        ProjectTenantListCall {
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
    /// Updates specified tenant.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required during tenant update. The resource name for a tenant. This is generated by the service when a tenant is created. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    pub fn tenants_patch(&self, request: Tenant, name: &str) -> ProjectTenantPatchCall<'a, S> {
        ProjectTenantPatchCall {
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



