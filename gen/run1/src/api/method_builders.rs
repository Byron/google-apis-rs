use super::*;
/// A builder providing access to all methods supported on *namespace* resources.
/// It is not used directly, but through the [`CloudRun`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_run1 as run1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use run1::{CloudRun, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudRun::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `authorizeddomains_list(...)`, `configurations_get(...)`, `configurations_list(...)`, `domainmappings_create(...)`, `domainmappings_delete(...)`, `domainmappings_get(...)`, `domainmappings_list(...)`, `executions_cancel(...)`, `executions_delete(...)`, `executions_get(...)`, `executions_list(...)`, `jobs_create(...)`, `jobs_delete(...)`, `jobs_get(...)`, `jobs_list(...)`, `jobs_replace_job(...)`, `jobs_run(...)`, `revisions_delete(...)`, `revisions_get(...)`, `revisions_list(...)`, `routes_get(...)`, `routes_list(...)`, `services_create(...)`, `services_delete(...)`, `services_get(...)`, `services_list(...)`, `services_replace_service(...)`, `tasks_get(...)` and `tasks_list(...)`
/// // to build up your call.
/// let rb = hub.namespaces();
/// # }
/// ```
pub struct NamespaceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudRun<S>,
}

impl<'a, S> client::MethodsBuilder for NamespaceMethods<'a, S> {}

impl<'a, S> NamespaceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List authorized domains.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the parent Project resource. Example: `projects/myproject`.
    pub fn authorizeddomains_list(&self, parent: &str) -> NamespaceAuthorizeddomainListCall<'a, S> {
        NamespaceAuthorizeddomainListCall {
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
    /// Get information about a configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the configuration to retrieve. For Cloud Run, replace {namespace_id} with the project ID or number.
    pub fn configurations_get(&self, name: &str) -> NamespaceConfigurationGetCall<'a, S> {
        NamespaceConfigurationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List configurations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the configurations should be listed. For Cloud Run, replace {namespace_id} with the project ID or number.
    pub fn configurations_list(&self, parent: &str) -> NamespaceConfigurationListCall<'a, S> {
        NamespaceConfigurationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new domain mapping.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The namespace in which the domain mapping should be created. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn domainmappings_create(&self, request: DomainMapping, parent: &str) -> NamespaceDomainmappingCreateCall<'a, S> {
        NamespaceDomainmappingCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a domain mapping.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the domain mapping to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn domainmappings_delete(&self, name: &str) -> NamespaceDomainmappingDeleteCall<'a, S> {
        NamespaceDomainmappingDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _dry_run: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a domain mapping.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the domain mapping to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn domainmappings_get(&self, name: &str) -> NamespaceDomainmappingGetCall<'a, S> {
        NamespaceDomainmappingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all domain mappings.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the domain mappings should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn domainmappings_list(&self, parent: &str) -> NamespaceDomainmappingListCall<'a, S> {
        NamespaceDomainmappingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancel an execution.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the execution to cancel. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn executions_cancel(&self, request: CancelExecutionRequest, name: &str) -> NamespaceExecutionCancelCall<'a, S> {
        NamespaceExecutionCancelCall {
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
    /// Delete an execution.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the execution to delete. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn executions_delete(&self, name: &str) -> NamespaceExecutionDeleteCall<'a, S> {
        NamespaceExecutionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about an execution.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the execution to retrieve. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn executions_get(&self, name: &str) -> NamespaceExecutionGetCall<'a, S> {
        NamespaceExecutionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List executions.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The namespace from which the executions should be listed. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn executions_list(&self, parent: &str) -> NamespaceExecutionListCall<'a, S> {
        NamespaceExecutionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The namespace in which the job should be created. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn jobs_create(&self, request: Job, parent: &str) -> NamespaceJobCreateCall<'a, S> {
        NamespaceJobCreateCall {
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
    /// Delete a job.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the job to delete. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn jobs_delete(&self, name: &str) -> NamespaceJobDeleteCall<'a, S> {
        NamespaceJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a job.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the job to retrieve. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn jobs_get(&self, name: &str) -> NamespaceJobGetCall<'a, S> {
        NamespaceJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List jobs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The namespace from which the jobs should be listed. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn jobs_list(&self, parent: &str) -> NamespaceJobListCall<'a, S> {
        NamespaceJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replace a job. Only the spec and metadata labels and annotations are modifiable. After the Replace request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the job being replaced. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn jobs_replace_job(&self, request: Job, name: &str) -> NamespaceJobReplaceJobCall<'a, S> {
        NamespaceJobReplaceJobCall {
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
    /// Trigger creation of a new execution of this job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the job to run. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn jobs_run(&self, request: RunJobRequest, name: &str) -> NamespaceJobRunCall<'a, S> {
        NamespaceJobRunCall {
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
    /// Delete a revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the revision to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn revisions_delete(&self, name: &str) -> NamespaceRevisionDeleteCall<'a, S> {
        NamespaceRevisionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _dry_run: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the revision to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn revisions_get(&self, name: &str) -> NamespaceRevisionGetCall<'a, S> {
        NamespaceRevisionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List revisions.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the revisions should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn revisions_list(&self, parent: &str) -> NamespaceRevisionListCall<'a, S> {
        NamespaceRevisionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a route.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the route to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn routes_get(&self, name: &str) -> NamespaceRouteGetCall<'a, S> {
        NamespaceRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List routes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the routes should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn routes_list(&self, parent: &str) -> NamespaceRouteListCall<'a, S> {
        NamespaceRouteListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Service. Service creation will trigger a new deployment. Use GetService, and check service.status to determine if the Service is ready.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`
    pub fn services_create(&self, request: Service, parent: &str) -> NamespaceServiceCreateCall<'a, S> {
        NamespaceServiceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the provided service. This will cause the Service to stop serving traffic and will delete all associated Revisions.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully qualified name of the service to delete. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`
    pub fn services_delete(&self, name: &str) -> NamespaceServiceDeleteCall<'a, S> {
        NamespaceServiceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _dry_run: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully qualified name of the service to retrieve. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`
    pub fn services_get(&self, name: &str) -> NamespaceServiceGetCall<'a, S> {
        NamespaceServiceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists services for the given project and region.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent from where the resources should be listed. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`
    pub fn services_list(&self, parent: &str) -> NamespaceServiceListCall<'a, S> {
        NamespaceServiceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces a service. Only the spec and metadata labels and annotations are modifiable. After the Update request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The fully qualified name of the service to replace. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`
    pub fn services_replace_service(&self, request: Service, name: &str) -> NamespaceServiceReplaceServiceCall<'a, S> {
        NamespaceServiceReplaceServiceCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a task.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the task to retrieve. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn tasks_get(&self, name: &str) -> NamespaceTaskGetCall<'a, S> {
        NamespaceTaskGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List tasks.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The namespace from which the tasks should be listed. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn tasks_list(&self, parent: &str) -> NamespaceTaskListCall<'a, S> {
        NamespaceTaskListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudRun`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_run1 as run1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use run1::{CloudRun, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudRun::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `authorizeddomains_list(...)`, `locations_authorizeddomains_list(...)`, `locations_configurations_get(...)`, `locations_configurations_list(...)`, `locations_domainmappings_create(...)`, `locations_domainmappings_delete(...)`, `locations_domainmappings_get(...)`, `locations_domainmappings_list(...)`, `locations_jobs_get_iam_policy(...)`, `locations_jobs_set_iam_policy(...)`, `locations_jobs_test_iam_permissions(...)`, `locations_list(...)`, `locations_revisions_delete(...)`, `locations_revisions_get(...)`, `locations_revisions_list(...)`, `locations_routes_get(...)`, `locations_routes_list(...)`, `locations_services_create(...)`, `locations_services_delete(...)`, `locations_services_get(...)`, `locations_services_get_iam_policy(...)`, `locations_services_list(...)`, `locations_services_replace_service(...)`, `locations_services_set_iam_policy(...)` and `locations_services_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudRun<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List authorized domains.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the parent Project resource. Example: `projects/myproject`.
    pub fn authorizeddomains_list(&self, parent: &str) -> ProjectAuthorizeddomainListCall<'a, S> {
        ProjectAuthorizeddomainListCall {
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
    /// List authorized domains.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the parent Project resource. Example: `projects/myproject`.
    pub fn locations_authorizeddomains_list(&self, parent: &str) -> ProjectLocationAuthorizeddomainListCall<'a, S> {
        ProjectLocationAuthorizeddomainListCall {
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
    /// Get information about a configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the configuration to retrieve. For Cloud Run, replace {namespace_id} with the project ID or number.
    pub fn locations_configurations_get(&self, name: &str) -> ProjectLocationConfigurationGetCall<'a, S> {
        ProjectLocationConfigurationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List configurations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the configurations should be listed. For Cloud Run, replace {namespace_id} with the project ID or number.
    pub fn locations_configurations_list(&self, parent: &str) -> ProjectLocationConfigurationListCall<'a, S> {
        ProjectLocationConfigurationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new domain mapping.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The namespace in which the domain mapping should be created. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_domainmappings_create(&self, request: DomainMapping, parent: &str) -> ProjectLocationDomainmappingCreateCall<'a, S> {
        ProjectLocationDomainmappingCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a domain mapping.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the domain mapping to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_domainmappings_delete(&self, name: &str) -> ProjectLocationDomainmappingDeleteCall<'a, S> {
        ProjectLocationDomainmappingDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _dry_run: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a domain mapping.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the domain mapping to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_domainmappings_get(&self, name: &str) -> ProjectLocationDomainmappingGetCall<'a, S> {
        ProjectLocationDomainmappingGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all domain mappings.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the domain mappings should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_domainmappings_list(&self, parent: &str) -> ProjectLocationDomainmappingListCall<'a, S> {
        ProjectLocationDomainmappingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the IAM Access Control policy currently in effect for the given job. This result does not include any inherited policies.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_jobs_get_iam_policy(&self, resource: &str) -> ProjectLocationJobGetIamPolicyCall<'a, S> {
        ProjectLocationJobGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the IAM Access control policy for the specified job. Overwrites any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_jobs_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationJobSetIamPolicyCall<'a, S> {
        ProjectLocationJobSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified job. There are no permissions required for making this API call.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_jobs_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationJobTestIamPermissionCall<'a, S> {
        ProjectLocationJobTestIamPermissionCall {
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
    /// Delete a revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the revision to delete. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_revisions_delete(&self, name: &str) -> ProjectLocationRevisionDeleteCall<'a, S> {
        ProjectLocationRevisionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _dry_run: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the revision to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_revisions_get(&self, name: &str) -> ProjectLocationRevisionGetCall<'a, S> {
        ProjectLocationRevisionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List revisions.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the revisions should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_revisions_list(&self, parent: &str) -> ProjectLocationRevisionListCall<'a, S> {
        ProjectLocationRevisionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get information about a route.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the route to retrieve. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_routes_get(&self, name: &str) -> ProjectLocationRouteGetCall<'a, S> {
        ProjectLocationRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List routes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The namespace from which the routes should be listed. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
    pub fn locations_routes_list(&self, parent: &str) -> ProjectLocationRouteListCall<'a, S> {
        ProjectLocationRouteListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Service. Service creation will trigger a new deployment. Use GetService, and check service.status to determine if the Service is ready.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`
    pub fn locations_services_create(&self, request: Service, parent: &str) -> ProjectLocationServiceCreateCall<'a, S> {
        ProjectLocationServiceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the provided service. This will cause the Service to stop serving traffic and will delete all associated Revisions.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully qualified name of the service to delete. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`
    pub fn locations_services_delete(&self, name: &str) -> ProjectLocationServiceDeleteCall<'a, S> {
        ProjectLocationServiceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _propagation_policy: Default::default(),
            _kind: Default::default(),
            _dry_run: Default::default(),
            _api_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The fully qualified name of the service to retrieve. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`
    pub fn locations_services_get(&self, name: &str) -> ProjectLocationServiceGetCall<'a, S> {
        ProjectLocationServiceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM Access Control policy currently in effect for the given Cloud Run service. This result does not include any inherited policies.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceGetIamPolicyCall<'a, S> {
        ProjectLocationServiceGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists services for the given project and region.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent from where the resources should be listed. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`
    pub fn locations_services_list(&self, parent: &str) -> ProjectLocationServiceListCall<'a, S> {
        ProjectLocationServiceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _watch: Default::default(),
            _resource_version: Default::default(),
            _limit: Default::default(),
            _label_selector: Default::default(),
            _include_uninitialized: Default::default(),
            _field_selector: Default::default(),
            _continue_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces a service. Only the spec and metadata labels and annotations are modifiable. After the Update request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The fully qualified name of the service to replace. It can be any of the following forms: * `namespaces/{project_id_or_number}/services/{service_name}` (only when the `endpoint` is regional) * `projects/{project_id_or_number}/locations/{region}/services/{service_name}` * `projects/{project_id_or_number}/regions/{region}/services/{service_name}`
    pub fn locations_services_replace_service(&self, request: Service, name: &str) -> ProjectLocationServiceReplaceServiceCall<'a, S> {
        ProjectLocationServiceReplaceServiceCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _dry_run: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the IAM Access control policy for the specified Service. Overwrites any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceSetIamPolicyCall<'a, S> {
        ProjectLocationServiceSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified Project. There are no permissions required for making this API call.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_services_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceTestIamPermissionCall<'a, S> {
        ProjectLocationServiceTestIamPermissionCall {
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
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
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
}



