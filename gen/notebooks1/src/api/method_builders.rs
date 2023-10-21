use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`AIPlatformNotebooks`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_notebooks1 as notebooks1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use notebooks1::{AIPlatformNotebooks, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AIPlatformNotebooks::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_environments_create(...)`, `locations_environments_delete(...)`, `locations_environments_get(...)`, `locations_environments_list(...)`, `locations_executions_create(...)`, `locations_executions_delete(...)`, `locations_executions_get(...)`, `locations_executions_list(...)`, `locations_get(...)`, `locations_instances_create(...)`, `locations_instances_delete(...)`, `locations_instances_diagnose(...)`, `locations_instances_get(...)`, `locations_instances_get_iam_policy(...)`, `locations_instances_get_instance_health(...)`, `locations_instances_is_upgradeable(...)`, `locations_instances_list(...)`, `locations_instances_register(...)`, `locations_instances_report(...)`, `locations_instances_reset(...)`, `locations_instances_rollback(...)`, `locations_instances_set_accelerator(...)`, `locations_instances_set_iam_policy(...)`, `locations_instances_set_labels(...)`, `locations_instances_set_machine_type(...)`, `locations_instances_start(...)`, `locations_instances_stop(...)`, `locations_instances_test_iam_permissions(...)`, `locations_instances_update_config(...)`, `locations_instances_update_metadata_items(...)`, `locations_instances_update_shielded_instance_config(...)`, `locations_instances_upgrade(...)`, `locations_instances_upgrade_internal(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_runtimes_create(...)`, `locations_runtimes_delete(...)`, `locations_runtimes_diagnose(...)`, `locations_runtimes_get(...)`, `locations_runtimes_get_iam_policy(...)`, `locations_runtimes_list(...)`, `locations_runtimes_patch(...)`, `locations_runtimes_refresh_runtime_token_internal(...)`, `locations_runtimes_report_event(...)`, `locations_runtimes_reset(...)`, `locations_runtimes_set_iam_policy(...)`, `locations_runtimes_start(...)`, `locations_runtimes_stop(...)`, `locations_runtimes_switch(...)`, `locations_runtimes_test_iam_permissions(...)`, `locations_runtimes_upgrade(...)`, `locations_schedules_create(...)`, `locations_schedules_delete(...)`, `locations_schedules_get(...)`, `locations_schedules_list(...)` and `locations_schedules_trigger(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AIPlatformNotebooks<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Format: `projects/{project_id}/locations/{location}`
    pub fn locations_environments_create(&self, request: Environment, parent: &str) -> ProjectLocationEnvironmentCreateCall<'a, S> {
        ProjectLocationEnvironmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _environment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/environments/{environment_id}`
    pub fn locations_environments_delete(&self, name: &str) -> ProjectLocationEnvironmentDeleteCall<'a, S> {
        ProjectLocationEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/environments/{environment_id}`
    pub fn locations_environments_get(&self, name: &str) -> ProjectLocationEnvironmentGetCall<'a, S> {
        ProjectLocationEnvironmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists environments in a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Format: `projects/{project_id}/locations/{location}`
    pub fn locations_environments_list(&self, parent: &str) -> ProjectLocationEnvironmentListCall<'a, S> {
        ProjectLocationEnvironmentListCall {
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
    /// Creates a new Execution in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_executions_create(&self, request: Execution, parent: &str) -> ProjectLocationExecutionCreateCall<'a, S> {
        ProjectLocationExecutionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _execution_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes execution
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/executions/{execution_id}`
    pub fn locations_executions_delete(&self, name: &str) -> ProjectLocationExecutionDeleteCall<'a, S> {
        ProjectLocationExecutionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of executions
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/executions/{execution_id}`
    pub fn locations_executions_get(&self, name: &str) -> ProjectLocationExecutionGetCall<'a, S> {
        ProjectLocationExecutionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists executions in a given project and location
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_executions_list(&self, parent: &str) -> ProjectLocationExecutionListCall<'a, S> {
        ProjectLocationExecutionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Instance in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_instances_create(&self, request: Instance, parent: &str) -> ProjectLocationInstanceCreateCall<'a, S> {
        ProjectLocationInstanceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _instance_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_delete(&self, name: &str) -> ProjectLocationInstanceDeleteCall<'a, S> {
        ProjectLocationInstanceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Diagnostic File and runs Diagnostic Tool given an Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_diagnose(&self, request: DiagnoseInstanceRequest, name: &str) -> ProjectLocationInstanceDiagnoseCall<'a, S> {
        ProjectLocationInstanceDiagnoseCall {
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
    /// Gets details of a single Instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_get(&self, name: &str) -> ProjectLocationInstanceGetCall<'a, S> {
        ProjectLocationInstanceGetCall {
            hub: self.hub,
            _name: name.to_string(),
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_instances_get_iam_policy(&self, resource: &str) -> ProjectLocationInstanceGetIamPolicyCall<'a, S> {
        ProjectLocationInstanceGetIamPolicyCall {
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
    /// Check if a notebook instance is healthy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_get_instance_health(&self, name: &str) -> ProjectLocationInstanceGetInstanceHealthCall<'a, S> {
        ProjectLocationInstanceGetInstanceHealthCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Check if a notebook instance is upgradable.
    /// 
    /// # Arguments
    ///
    /// * `notebookInstance` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_is_upgradeable(&self, notebook_instance: &str) -> ProjectLocationInstanceIsUpgradeableCall<'a, S> {
        ProjectLocationInstanceIsUpgradeableCall {
            hub: self.hub,
            _notebook_instance: notebook_instance.to_string(),
            _type_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists instances in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_instances_list(&self, parent: &str) -> ProjectLocationInstanceListCall<'a, S> {
        ProjectLocationInstanceListCall {
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
    /// Registers an existing legacy notebook instance to the Notebooks API server. Legacy instances are instances created with the legacy Compute Engine calls. They are not manageable by the Notebooks API out of the box. This call makes these instances manageable by the Notebooks API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_instances_register(&self, request: RegisterInstanceRequest, parent: &str) -> ProjectLocationInstanceRegisterCall<'a, S> {
        ProjectLocationInstanceRegisterCall {
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
    /// Allows notebook instances to report their latest instance information to the Notebooks API server. The server will merge the reported information to the instance metadata store. Do not use this method directly.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_report(&self, request: ReportInstanceInfoRequest, name: &str) -> ProjectLocationInstanceReportCall<'a, S> {
        ProjectLocationInstanceReportCall {
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
    /// Resets a notebook instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_reset(&self, request: ResetInstanceRequest, name: &str) -> ProjectLocationInstanceResetCall<'a, S> {
        ProjectLocationInstanceResetCall {
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
    /// Rollbacks a notebook instance to the previous version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_rollback(&self, request: RollbackInstanceRequest, name: &str) -> ProjectLocationInstanceRollbackCall<'a, S> {
        ProjectLocationInstanceRollbackCall {
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
    /// Updates the guest accelerators of a single Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_set_accelerator(&self, request: SetInstanceAcceleratorRequest, name: &str) -> ProjectLocationInstanceSetAcceleratorCall<'a, S> {
        ProjectLocationInstanceSetAcceleratorCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_instances_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationInstanceSetIamPolicyCall<'a, S> {
        ProjectLocationInstanceSetIamPolicyCall {
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
    /// Replaces all the labels of an Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_set_labels(&self, request: SetInstanceLabelsRequest, name: &str) -> ProjectLocationInstanceSetLabelCall<'a, S> {
        ProjectLocationInstanceSetLabelCall {
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
    /// Updates the machine type of a single Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_set_machine_type(&self, request: SetInstanceMachineTypeRequest, name: &str) -> ProjectLocationInstanceSetMachineTypeCall<'a, S> {
        ProjectLocationInstanceSetMachineTypeCall {
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
    /// Starts a notebook instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_start(&self, request: StartInstanceRequest, name: &str) -> ProjectLocationInstanceStartCall<'a, S> {
        ProjectLocationInstanceStartCall {
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
    /// Stops a notebook instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_stop(&self, request: StopInstanceRequest, name: &str) -> ProjectLocationInstanceStopCall<'a, S> {
        ProjectLocationInstanceStopCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_instances_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationInstanceTestIamPermissionCall<'a, S> {
        ProjectLocationInstanceTestIamPermissionCall {
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
    /// Update Notebook Instance configurations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_update_config(&self, request: UpdateInstanceConfigRequest, name: &str) -> ProjectLocationInstanceUpdateConfigCall<'a, S> {
        ProjectLocationInstanceUpdateConfigCall {
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
    /// Add/update metadata items for an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_update_metadata_items(&self, request: UpdateInstanceMetadataItemsRequest, name: &str) -> ProjectLocationInstanceUpdateMetadataItemCall<'a, S> {
        ProjectLocationInstanceUpdateMetadataItemCall {
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
    /// Updates the Shielded instance configuration of a single Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_update_shielded_instance_config(&self, request: UpdateShieldedInstanceConfigRequest, name: &str) -> ProjectLocationInstanceUpdateShieldedInstanceConfigCall<'a, S> {
        ProjectLocationInstanceUpdateShieldedInstanceConfigCall {
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
    /// Upgrades a notebook instance to the latest version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_upgrade(&self, request: UpgradeInstanceRequest, name: &str) -> ProjectLocationInstanceUpgradeCall<'a, S> {
        ProjectLocationInstanceUpgradeCall {
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
    /// Allows notebook instances to call this endpoint to upgrade themselves. Do not use this method directly.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_upgrade_internal(&self, request: UpgradeInstanceInternalRequest, name: &str) -> ProjectLocationInstanceUpgradeInternalCall<'a, S> {
        ProjectLocationInstanceUpgradeInternalCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
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
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn locations_operations_delete(&self, name: &str) -> ProjectLocationOperationDeleteCall<'a, S> {
        ProjectLocationOperationDeleteCall {
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
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
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
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
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
    /// Creates a new Runtime in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_runtimes_create(&self, request: Runtime, parent: &str) -> ProjectLocationRuntimeCreateCall<'a, S> {
        ProjectLocationRuntimeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _runtime_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Runtime.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_delete(&self, name: &str) -> ProjectLocationRuntimeDeleteCall<'a, S> {
        ProjectLocationRuntimeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Diagnostic File and runs Diagnostic Tool given a Runtime.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtimes_id}`
    pub fn locations_runtimes_diagnose(&self, request: DiagnoseRuntimeRequest, name: &str) -> ProjectLocationRuntimeDiagnoseCall<'a, S> {
        ProjectLocationRuntimeDiagnoseCall {
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
    /// Gets details of a single Runtime. The location must be a regional endpoint rather than zonal.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_get(&self, name: &str) -> ProjectLocationRuntimeGetCall<'a, S> {
        ProjectLocationRuntimeGetCall {
            hub: self.hub,
            _name: name.to_string(),
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_runtimes_get_iam_policy(&self, resource: &str) -> ProjectLocationRuntimeGetIamPolicyCall<'a, S> {
        ProjectLocationRuntimeGetIamPolicyCall {
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
    /// Lists Runtimes in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_runtimes_list(&self, parent: &str) -> ProjectLocationRuntimeListCall<'a, S> {
        ProjectLocationRuntimeListCall {
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
    /// Update Notebook Runtime configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the runtime. Format: `projects/{project}/locations/{location}/runtimes/{runtimeId}`
    pub fn locations_runtimes_patch(&self, request: Runtime, name: &str) -> ProjectLocationRuntimePatchCall<'a, S> {
        ProjectLocationRuntimePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an access token for the consumer service account that the customer attached to the runtime. Only accessible from the tenant instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_refresh_runtime_token_internal(&self, request: RefreshRuntimeTokenInternalRequest, name: &str) -> ProjectLocationRuntimeRefreshRuntimeTokenInternalCall<'a, S> {
        ProjectLocationRuntimeRefreshRuntimeTokenInternalCall {
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
    /// Report and process a runtime event.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_report_event(&self, request: ReportRuntimeEventRequest, name: &str) -> ProjectLocationRuntimeReportEventCall<'a, S> {
        ProjectLocationRuntimeReportEventCall {
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
    /// Resets a Managed Notebook Runtime.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_reset(&self, request: ResetRuntimeRequest, name: &str) -> ProjectLocationRuntimeResetCall<'a, S> {
        ProjectLocationRuntimeResetCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_runtimes_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationRuntimeSetIamPolicyCall<'a, S> {
        ProjectLocationRuntimeSetIamPolicyCall {
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
    /// Starts a Managed Notebook Runtime. Perform "Start" on GPU instances; "Resume" on CPU instances See: https://cloud.google.com/compute/docs/instances/stop-start-instance https://cloud.google.com/compute/docs/instances/suspend-resume-instance
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_start(&self, request: StartRuntimeRequest, name: &str) -> ProjectLocationRuntimeStartCall<'a, S> {
        ProjectLocationRuntimeStartCall {
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
    /// Stops a Managed Notebook Runtime. Perform "Stop" on GPU instances; "Suspend" on CPU instances See: https://cloud.google.com/compute/docs/instances/stop-start-instance https://cloud.google.com/compute/docs/instances/suspend-resume-instance
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_stop(&self, request: StopRuntimeRequest, name: &str) -> ProjectLocationRuntimeStopCall<'a, S> {
        ProjectLocationRuntimeStopCall {
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
    /// Switch a Managed Notebook Runtime.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_switch(&self, request: SwitchRuntimeRequest, name: &str) -> ProjectLocationRuntimeSwitchCall<'a, S> {
        ProjectLocationRuntimeSwitchCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_runtimes_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationRuntimeTestIamPermissionCall<'a, S> {
        ProjectLocationRuntimeTestIamPermissionCall {
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
    /// Upgrades a Managed Notebook Runtime to the latest version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    pub fn locations_runtimes_upgrade(&self, request: UpgradeRuntimeRequest, name: &str) -> ProjectLocationRuntimeUpgradeCall<'a, S> {
        ProjectLocationRuntimeUpgradeCall {
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
    /// Creates a new Scheduled Notebook in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_schedules_create(&self, request: Schedule, parent: &str) -> ProjectLocationScheduleCreateCall<'a, S> {
        ProjectLocationScheduleCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _schedule_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes schedule and all underlying jobs
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    pub fn locations_schedules_delete(&self, name: &str) -> ProjectLocationScheduleDeleteCall<'a, S> {
        ProjectLocationScheduleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of schedule
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    pub fn locations_schedules_get(&self, name: &str) -> ProjectLocationScheduleGetCall<'a, S> {
        ProjectLocationScheduleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists schedules in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Format: `parent=projects/{project_id}/locations/{location}`
    pub fn locations_schedules_list(&self, parent: &str) -> ProjectLocationScheduleListCall<'a, S> {
        ProjectLocationScheduleListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Triggers execution of an existing schedule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Format: `parent=projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    pub fn locations_schedules_trigger(&self, request: TriggerScheduleRequest, name: &str) -> ProjectLocationScheduleTriggerCall<'a, S> {
        ProjectLocationScheduleTriggerCall {
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
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



