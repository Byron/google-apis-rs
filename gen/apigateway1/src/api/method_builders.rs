use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Apigateway`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_apigateway1 as apigateway1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use apigateway1::{Apigateway, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Apigateway::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_apis_configs_create(...)`, `locations_apis_configs_delete(...)`, `locations_apis_configs_get(...)`, `locations_apis_configs_get_iam_policy(...)`, `locations_apis_configs_list(...)`, `locations_apis_configs_patch(...)`, `locations_apis_configs_set_iam_policy(...)`, `locations_apis_configs_test_iam_permissions(...)`, `locations_apis_create(...)`, `locations_apis_delete(...)`, `locations_apis_get(...)`, `locations_apis_get_iam_policy(...)`, `locations_apis_list(...)`, `locations_apis_patch(...)`, `locations_apis_set_iam_policy(...)`, `locations_apis_test_iam_permissions(...)`, `locations_gateways_create(...)`, `locations_gateways_delete(...)`, `locations_gateways_get(...)`, `locations_gateways_get_iam_policy(...)`, `locations_gateways_list(...)`, `locations_gateways_patch(...)`, `locations_gateways_set_iam_policy(...)`, `locations_gateways_test_iam_permissions(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)` and `locations_operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Apigateway<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ApiConfig in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource of the API Config, of the form: `projects/*/locations/global/apis/*`
    pub fn locations_apis_configs_create(&self, request: ApigatewayApiConfig, parent: &str) -> ProjectLocationApiConfigCreateCall<'a, S> {
        ProjectLocationApiConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _api_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ApiConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/global/apis/*/configs/*`
    pub fn locations_apis_configs_delete(&self, name: &str) -> ProjectLocationApiConfigDeleteCall<'a, S> {
        ProjectLocationApiConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ApiConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/global/apis/*/configs/*`
    pub fn locations_apis_configs_get(&self, name: &str) -> ProjectLocationApiConfigGetCall<'a, S> {
        ProjectLocationApiConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
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
    pub fn locations_apis_configs_get_iam_policy(&self, resource: &str) -> ProjectLocationApiConfigGetIamPolicyCall<'a, S> {
        ProjectLocationApiConfigGetIamPolicyCall {
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
    /// Lists ApiConfigs in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the API Config, of the form: `projects/*/locations/global/apis/*`
    pub fn locations_apis_configs_list(&self, parent: &str) -> ProjectLocationApiConfigListCall<'a, S> {
        ProjectLocationApiConfigListCall {
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
    /// Updates the parameters of a single ApiConfig.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the API Config. Format: projects/{project}/locations/global/apis/{api}/configs/{api_config}
    pub fn locations_apis_configs_patch(&self, request: ApigatewayApiConfig, name: &str) -> ProjectLocationApiConfigPatchCall<'a, S> {
        ProjectLocationApiConfigPatchCall {
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
    pub fn locations_apis_configs_set_iam_policy(&self, request: ApigatewaySetIamPolicyRequest, resource: &str) -> ProjectLocationApiConfigSetIamPolicyCall<'a, S> {
        ProjectLocationApiConfigSetIamPolicyCall {
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
    pub fn locations_apis_configs_test_iam_permissions(&self, request: ApigatewayTestIamPermissionsRequest, resource: &str) -> ProjectLocationApiConfigTestIamPermissionCall<'a, S> {
        ProjectLocationApiConfigTestIamPermissionCall {
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
    /// Creates a new Api in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource of the API, of the form: `projects/*/locations/global`
    pub fn locations_apis_create(&self, request: ApigatewayApi, parent: &str) -> ProjectLocationApiCreateCall<'a, S> {
        ProjectLocationApiCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _api_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Api.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/global/apis/*`
    pub fn locations_apis_delete(&self, name: &str) -> ProjectLocationApiDeleteCall<'a, S> {
        ProjectLocationApiDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Api.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/global/apis/*`
    pub fn locations_apis_get(&self, name: &str) -> ProjectLocationApiGetCall<'a, S> {
        ProjectLocationApiGetCall {
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
    pub fn locations_apis_get_iam_policy(&self, resource: &str) -> ProjectLocationApiGetIamPolicyCall<'a, S> {
        ProjectLocationApiGetIamPolicyCall {
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
    /// Lists Apis in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the API, of the form: `projects/*/locations/global`
    pub fn locations_apis_list(&self, parent: &str) -> ProjectLocationApiListCall<'a, S> {
        ProjectLocationApiListCall {
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
    /// Updates the parameters of a single Api.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the API. Format: projects/{project}/locations/global/apis/{api}
    pub fn locations_apis_patch(&self, request: ApigatewayApi, name: &str) -> ProjectLocationApiPatchCall<'a, S> {
        ProjectLocationApiPatchCall {
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
    pub fn locations_apis_set_iam_policy(&self, request: ApigatewaySetIamPolicyRequest, resource: &str) -> ProjectLocationApiSetIamPolicyCall<'a, S> {
        ProjectLocationApiSetIamPolicyCall {
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
    pub fn locations_apis_test_iam_permissions(&self, request: ApigatewayTestIamPermissionsRequest, resource: &str) -> ProjectLocationApiTestIamPermissionCall<'a, S> {
        ProjectLocationApiTestIamPermissionCall {
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
    /// Creates a new Gateway in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource of the Gateway, of the form: `projects/*/locations/*`
    pub fn locations_gateways_create(&self, request: ApigatewayGateway, parent: &str) -> ProjectLocationGatewayCreateCall<'a, S> {
        ProjectLocationGatewayCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _gateway_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Gateway.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/gateways/*`
    pub fn locations_gateways_delete(&self, name: &str) -> ProjectLocationGatewayDeleteCall<'a, S> {
        ProjectLocationGatewayDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Gateway.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/gateways/*`
    pub fn locations_gateways_get(&self, name: &str) -> ProjectLocationGatewayGetCall<'a, S> {
        ProjectLocationGatewayGetCall {
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
    pub fn locations_gateways_get_iam_policy(&self, resource: &str) -> ProjectLocationGatewayGetIamPolicyCall<'a, S> {
        ProjectLocationGatewayGetIamPolicyCall {
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
    /// Lists Gateways in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the Gateway, of the form: `projects/*/locations/*`
    pub fn locations_gateways_list(&self, parent: &str) -> ProjectLocationGatewayListCall<'a, S> {
        ProjectLocationGatewayListCall {
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
    /// Updates the parameters of a single Gateway.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the Gateway. Format: projects/{project}/locations/{location}/gateways/{gateway}
    pub fn locations_gateways_patch(&self, request: ApigatewayGateway, name: &str) -> ProjectLocationGatewayPatchCall<'a, S> {
        ProjectLocationGatewayPatchCall {
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
    pub fn locations_gateways_set_iam_policy(&self, request: ApigatewaySetIamPolicyRequest, resource: &str) -> ProjectLocationGatewaySetIamPolicyCall<'a, S> {
        ProjectLocationGatewaySetIamPolicyCall {
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
    pub fn locations_gateways_test_iam_permissions(&self, request: ApigatewayTestIamPermissionsRequest, resource: &str) -> ProjectLocationGatewayTestIamPermissionCall<'a, S> {
        ProjectLocationGatewayTestIamPermissionCall {
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
    pub fn locations_operations_cancel(&self, request: ApigatewayCancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
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



