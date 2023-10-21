use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudRuntimeConfig`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_runtimeconfig1_beta1 as runtimeconfig1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use runtimeconfig1_beta1::{CloudRuntimeConfig, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudRuntimeConfig::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `configs_create(...)`, `configs_delete(...)`, `configs_get(...)`, `configs_get_iam_policy(...)`, `configs_list(...)`, `configs_operations_get(...)`, `configs_operations_test_iam_permissions(...)`, `configs_set_iam_policy(...)`, `configs_test_iam_permissions(...)`, `configs_update(...)`, `configs_variables_create(...)`, `configs_variables_delete(...)`, `configs_variables_get(...)`, `configs_variables_list(...)`, `configs_variables_test_iam_permissions(...)`, `configs_variables_update(...)`, `configs_variables_watch(...)`, `configs_waiters_create(...)`, `configs_waiters_delete(...)`, `configs_waiters_get(...)`, `configs_waiters_list(...)` and `configs_waiters_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudRuntimeConfig<S>,
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
    pub fn configs_operations_get(&self, name: &str) -> ProjectConfigOperationGetCall<'a, S> {
        ProjectConfigOperationGetCall {
            hub: self.hub,
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
    pub fn configs_operations_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectConfigOperationTestIamPermissionCall<'a, S> {
        ProjectConfigOperationTestIamPermissionCall {
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
    /// Creates a variable within the given configuration. You cannot create a variable with a name that is a prefix of an existing variable name, or a name that has an existing variable name as a prefix. To learn more about creating a variable, read the [Setting and Getting Data](https://cloud.google.com/deployment-manager/runtime-configurator/set-and-get-variables) documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The path to the RutimeConfig resource that this variable should belong to. The configuration must exist beforehand; the path must be in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    pub fn configs_variables_create(&self, request: Variable, parent: &str) -> ProjectConfigVariableCreateCall<'a, S> {
        ProjectConfigVariableCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a variable or multiple variables. If you specify a variable name, then that variable is deleted. If you specify a prefix and `recursive` is true, then all variables with that prefix are deleted. You must set a `recursive` to true if you delete variables by prefix.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the variable to delete, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIABLE_NAME]`
    pub fn configs_variables_delete(&self, name: &str) -> ProjectConfigVariableDeleteCall<'a, S> {
        ProjectConfigVariableDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _recursive: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a single variable.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the variable to return, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIBLE_NAME]`
    pub fn configs_variables_get(&self, name: &str) -> ProjectConfigVariableGetCall<'a, S> {
        ProjectConfigVariableGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists variables within given a configuration, matching any provided filters. This only lists variable names, not the values, unless `return_values` is true, in which case only variables that user has IAM permission to GetVariable will be returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The path to the RuntimeConfig resource for which you want to list variables. The configuration must exist beforehand; the path must be in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    pub fn configs_variables_list(&self, parent: &str) -> ProjectConfigVariableListCall<'a, S> {
        ProjectConfigVariableListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _return_values: Default::default(),
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn configs_variables_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectConfigVariableTestIamPermissionCall<'a, S> {
        ProjectConfigVariableTestIamPermissionCall {
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
    /// Updates an existing variable with a new value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the variable to update, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIABLE_NAME]`
    pub fn configs_variables_update(&self, request: Variable, name: &str) -> ProjectConfigVariableUpdateCall<'a, S> {
        ProjectConfigVariableUpdateCall {
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
    /// Watches a specific variable and waits for a change in the variable’s value. When there is a change, this method returns the new value or times out. If a variable is deleted while being watched, the `variableState` state is set to `DELETED` and the method returns the last known variable `value`. If you set the deadline for watching to a larger value than internal timeout (60 seconds), the current variable value is returned and the `variableState` will be `VARIABLE_STATE_UNSPECIFIED`. To learn more about creating a watcher, read the [Watching a Variable for Changes](https://cloud.google.com/deployment-manager/runtime-configurator/watching-a-variable) documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the variable to watch, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    pub fn configs_variables_watch(&self, request: WatchVariableRequest, name: &str) -> ProjectConfigVariableWatchCall<'a, S> {
        ProjectConfigVariableWatchCall {
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
    /// Creates a Waiter resource. This operation returns a long-running Operation resource which can be polled for completion. However, a waiter with the given name will exist (and can be retrieved) prior to the operation completing. If the operation fails, the failed Waiter resource will still exist and must be deleted prior to subsequent creation attempts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The path to the configuration that will own the waiter. The configuration must exist beforehand; the path must be in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`.
    pub fn configs_waiters_create(&self, request: Waiter, parent: &str) -> ProjectConfigWaiterCreateCall<'a, S> {
        ProjectConfigWaiterCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the waiter with the specified name.
    /// 
    /// # Arguments
    ///
    /// * `name` - The Waiter resource to delete, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/waiters/[WAITER_NAME]`
    pub fn configs_waiters_delete(&self, name: &str) -> ProjectConfigWaiterDeleteCall<'a, S> {
        ProjectConfigWaiterDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a single waiter.
    /// 
    /// # Arguments
    ///
    /// * `name` - The fully-qualified name of the Waiter resource object to retrieve, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/waiters/[WAITER_NAME]`
    pub fn configs_waiters_get(&self, name: &str) -> ProjectConfigWaiterGetCall<'a, S> {
        ProjectConfigWaiterGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List waiters within the given configuration.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The path to the configuration for which you want to get a list of waiters. The configuration must exist beforehand; the path must be in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    pub fn configs_waiters_list(&self, parent: &str) -> ProjectConfigWaiterListCall<'a, S> {
        ProjectConfigWaiterListCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn configs_waiters_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectConfigWaiterTestIamPermissionCall<'a, S> {
        ProjectConfigWaiterTestIamPermissionCall {
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
    /// Creates a new RuntimeConfig resource. The configuration name must be unique within project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The [project ID](https://support.google.com/cloud/answer/6158840?hl=en&ref_topic=6158848) for this request, in the format `projects/[PROJECT_ID]`.
    pub fn configs_create(&self, request: RuntimeConfig, parent: &str) -> ProjectConfigCreateCall<'a, S> {
        ProjectConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a RuntimeConfig resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - The RuntimeConfig resource to delete, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    pub fn configs_delete(&self, name: &str) -> ProjectConfigDeleteCall<'a, S> {
        ProjectConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a RuntimeConfig resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the RuntimeConfig resource to retrieve, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    pub fn configs_get(&self, name: &str) -> ProjectConfigGetCall<'a, S> {
        ProjectConfigGetCall {
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
    pub fn configs_get_iam_policy(&self, resource: &str) -> ProjectConfigGetIamPolicyCall<'a, S> {
        ProjectConfigGetIamPolicyCall {
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
    /// Lists all the RuntimeConfig resources within project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The [project ID](https://support.google.com/cloud/answer/6158840?hl=en&ref_topic=6158848) for this request, in the format `projects/[PROJECT_ID]`.
    pub fn configs_list(&self, parent: &str) -> ProjectConfigListCall<'a, S> {
        ProjectConfigListCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn configs_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectConfigSetIamPolicyCall<'a, S> {
        ProjectConfigSetIamPolicyCall {
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
    pub fn configs_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectConfigTestIamPermissionCall<'a, S> {
        ProjectConfigTestIamPermissionCall {
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
    /// Updates a RuntimeConfig resource. The configuration must exist beforehand.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the RuntimeConfig resource to update, in the format: `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    pub fn configs_update(&self, request: RuntimeConfig, name: &str) -> ProjectConfigUpdateCall<'a, S> {
        ProjectConfigUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



