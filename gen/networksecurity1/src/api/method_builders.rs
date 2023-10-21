use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`NetworkSecurity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_networksecurity1 as networksecurity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use networksecurity1::{NetworkSecurity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = NetworkSecurity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_authorization_policies_create(...)`, `locations_authorization_policies_delete(...)`, `locations_authorization_policies_get(...)`, `locations_authorization_policies_get_iam_policy(...)`, `locations_authorization_policies_list(...)`, `locations_authorization_policies_patch(...)`, `locations_authorization_policies_set_iam_policy(...)`, `locations_authorization_policies_test_iam_permissions(...)`, `locations_client_tls_policies_create(...)`, `locations_client_tls_policies_delete(...)`, `locations_client_tls_policies_get(...)`, `locations_client_tls_policies_get_iam_policy(...)`, `locations_client_tls_policies_list(...)`, `locations_client_tls_policies_patch(...)`, `locations_client_tls_policies_set_iam_policy(...)`, `locations_client_tls_policies_test_iam_permissions(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_server_tls_policies_create(...)`, `locations_server_tls_policies_delete(...)`, `locations_server_tls_policies_get(...)`, `locations_server_tls_policies_get_iam_policy(...)`, `locations_server_tls_policies_list(...)`, `locations_server_tls_policies_patch(...)`, `locations_server_tls_policies_set_iam_policy(...)` and `locations_server_tls_policies_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a NetworkSecurity<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new AuthorizationPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the AuthorizationPolicy. Must be in the format `projects/{project}/locations/{location}`.
    pub fn locations_authorization_policies_create(&self, request: AuthorizationPolicy, parent: &str) -> ProjectLocationAuthorizationPolicyCreateCall<'a, S> {
        ProjectLocationAuthorizationPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _authorization_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single AuthorizationPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AuthorizationPolicy to delete. Must be in the format `projects/{project}/locations/{location}/authorizationPolicies/*`.
    pub fn locations_authorization_policies_delete(&self, name: &str) -> ProjectLocationAuthorizationPolicyDeleteCall<'a, S> {
        ProjectLocationAuthorizationPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single AuthorizationPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AuthorizationPolicy to get. Must be in the format `projects/{project}/locations/{location}/authorizationPolicies/*`.
    pub fn locations_authorization_policies_get(&self, name: &str) -> ProjectLocationAuthorizationPolicyGetCall<'a, S> {
        ProjectLocationAuthorizationPolicyGetCall {
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
    pub fn locations_authorization_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationAuthorizationPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationAuthorizationPolicyGetIamPolicyCall {
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
    /// Lists AuthorizationPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the AuthorizationPolicies should be listed, specified in the format `projects/{project}/locations/{location}`.
    pub fn locations_authorization_policies_list(&self, parent: &str) -> ProjectLocationAuthorizationPolicyListCall<'a, S> {
        ProjectLocationAuthorizationPolicyListCall {
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
    /// Updates the parameters of a single AuthorizationPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the AuthorizationPolicy resource. It matches pattern `projects/{project}/locations/{location}/authorizationPolicies/`.
    pub fn locations_authorization_policies_patch(&self, request: AuthorizationPolicy, name: &str) -> ProjectLocationAuthorizationPolicyPatchCall<'a, S> {
        ProjectLocationAuthorizationPolicyPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_authorization_policies_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationAuthorizationPolicySetIamPolicyCall<'a, S> {
        ProjectLocationAuthorizationPolicySetIamPolicyCall {
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
    pub fn locations_authorization_policies_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationAuthorizationPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationAuthorizationPolicyTestIamPermissionCall {
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
    /// Creates a new ClientTlsPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the ClientTlsPolicy. Must be in the format `projects/*/locations/{location}`.
    pub fn locations_client_tls_policies_create(&self, request: ClientTlsPolicy, parent: &str) -> ProjectLocationClientTlsPolicyCreateCall<'a, S> {
        ProjectLocationClientTlsPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _client_tls_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ClientTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ClientTlsPolicy to delete. Must be in the format `projects/*/locations/{location}/clientTlsPolicies/*`.
    pub fn locations_client_tls_policies_delete(&self, name: &str) -> ProjectLocationClientTlsPolicyDeleteCall<'a, S> {
        ProjectLocationClientTlsPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ClientTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ClientTlsPolicy to get. Must be in the format `projects/*/locations/{location}/clientTlsPolicies/*`.
    pub fn locations_client_tls_policies_get(&self, name: &str) -> ProjectLocationClientTlsPolicyGetCall<'a, S> {
        ProjectLocationClientTlsPolicyGetCall {
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
    pub fn locations_client_tls_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationClientTlsPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationClientTlsPolicyGetIamPolicyCall {
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
    /// Lists ClientTlsPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the ClientTlsPolicies should be listed, specified in the format `projects/*/locations/{location}`.
    pub fn locations_client_tls_policies_list(&self, parent: &str) -> ProjectLocationClientTlsPolicyListCall<'a, S> {
        ProjectLocationClientTlsPolicyListCall {
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
    /// Updates the parameters of a single ClientTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the ClientTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/clientTlsPolicies/{client_tls_policy}`
    pub fn locations_client_tls_policies_patch(&self, request: ClientTlsPolicy, name: &str) -> ProjectLocationClientTlsPolicyPatchCall<'a, S> {
        ProjectLocationClientTlsPolicyPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_client_tls_policies_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationClientTlsPolicySetIamPolicyCall<'a, S> {
        ProjectLocationClientTlsPolicySetIamPolicyCall {
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
    pub fn locations_client_tls_policies_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationClientTlsPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationClientTlsPolicyTestIamPermissionCall {
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
    /// Creates a new ServerTlsPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the ServerTlsPolicy. Must be in the format `projects/*/locations/{location}`.
    pub fn locations_server_tls_policies_create(&self, request: ServerTlsPolicy, parent: &str) -> ProjectLocationServerTlsPolicyCreateCall<'a, S> {
        ProjectLocationServerTlsPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _server_tls_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ServerTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ServerTlsPolicy to delete. Must be in the format `projects/*/locations/{location}/serverTlsPolicies/*`.
    pub fn locations_server_tls_policies_delete(&self, name: &str) -> ProjectLocationServerTlsPolicyDeleteCall<'a, S> {
        ProjectLocationServerTlsPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ServerTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ServerTlsPolicy to get. Must be in the format `projects/*/locations/{location}/serverTlsPolicies/*`.
    pub fn locations_server_tls_policies_get(&self, name: &str) -> ProjectLocationServerTlsPolicyGetCall<'a, S> {
        ProjectLocationServerTlsPolicyGetCall {
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
    pub fn locations_server_tls_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationServerTlsPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationServerTlsPolicyGetIamPolicyCall {
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
    /// Lists ServerTlsPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the ServerTlsPolicies should be listed, specified in the format `projects/*/locations/{location}`.
    pub fn locations_server_tls_policies_list(&self, parent: &str) -> ProjectLocationServerTlsPolicyListCall<'a, S> {
        ProjectLocationServerTlsPolicyListCall {
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
    /// Updates the parameters of a single ServerTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the ServerTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}`
    pub fn locations_server_tls_policies_patch(&self, request: ServerTlsPolicy, name: &str) -> ProjectLocationServerTlsPolicyPatchCall<'a, S> {
        ProjectLocationServerTlsPolicyPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_server_tls_policies_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationServerTlsPolicySetIamPolicyCall<'a, S> {
        ProjectLocationServerTlsPolicySetIamPolicyCall {
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
    pub fn locations_server_tls_policies_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationServerTlsPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationServerTlsPolicyTestIamPermissionCall {
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



