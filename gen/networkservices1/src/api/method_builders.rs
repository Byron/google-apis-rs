use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`NetworkServices`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_networkservices1 as networkservices1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use networkservices1::{NetworkServices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = NetworkServices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_edge_cache_keysets_get_iam_policy(...)`, `locations_edge_cache_keysets_set_iam_policy(...)`, `locations_edge_cache_keysets_test_iam_permissions(...)`, `locations_edge_cache_origins_get_iam_policy(...)`, `locations_edge_cache_origins_set_iam_policy(...)`, `locations_edge_cache_origins_test_iam_permissions(...)`, `locations_edge_cache_services_get_iam_policy(...)`, `locations_edge_cache_services_set_iam_policy(...)`, `locations_edge_cache_services_test_iam_permissions(...)`, `locations_endpoint_policies_create(...)`, `locations_endpoint_policies_delete(...)`, `locations_endpoint_policies_get(...)`, `locations_endpoint_policies_get_iam_policy(...)`, `locations_endpoint_policies_list(...)`, `locations_endpoint_policies_patch(...)`, `locations_endpoint_policies_set_iam_policy(...)`, `locations_endpoint_policies_test_iam_permissions(...)`, `locations_gateways_create(...)`, `locations_gateways_delete(...)`, `locations_gateways_get(...)`, `locations_gateways_get_iam_policy(...)`, `locations_gateways_list(...)`, `locations_gateways_patch(...)`, `locations_gateways_set_iam_policy(...)`, `locations_gateways_test_iam_permissions(...)`, `locations_get(...)`, `locations_grpc_routes_create(...)`, `locations_grpc_routes_delete(...)`, `locations_grpc_routes_get(...)`, `locations_grpc_routes_list(...)`, `locations_grpc_routes_patch(...)`, `locations_http_routes_create(...)`, `locations_http_routes_delete(...)`, `locations_http_routes_get(...)`, `locations_http_routes_list(...)`, `locations_http_routes_patch(...)`, `locations_list(...)`, `locations_meshes_create(...)`, `locations_meshes_delete(...)`, `locations_meshes_get(...)`, `locations_meshes_get_iam_policy(...)`, `locations_meshes_list(...)`, `locations_meshes_patch(...)`, `locations_meshes_set_iam_policy(...)`, `locations_meshes_test_iam_permissions(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_service_bindings_create(...)`, `locations_service_bindings_delete(...)`, `locations_service_bindings_get(...)`, `locations_service_bindings_get_iam_policy(...)`, `locations_service_bindings_list(...)`, `locations_service_bindings_set_iam_policy(...)`, `locations_service_bindings_test_iam_permissions(...)`, `locations_tcp_routes_create(...)`, `locations_tcp_routes_delete(...)`, `locations_tcp_routes_get(...)`, `locations_tcp_routes_list(...)`, `locations_tcp_routes_patch(...)`, `locations_tls_routes_create(...)`, `locations_tls_routes_delete(...)`, `locations_tls_routes_get(...)`, `locations_tls_routes_list(...)` and `locations_tls_routes_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a NetworkServices<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_edge_cache_keysets_get_iam_policy(&self, resource: &str) -> ProjectLocationEdgeCacheKeysetGetIamPolicyCall<'a, S> {
        ProjectLocationEdgeCacheKeysetGetIamPolicyCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_edge_cache_keysets_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationEdgeCacheKeysetSetIamPolicyCall<'a, S> {
        ProjectLocationEdgeCacheKeysetSetIamPolicyCall {
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
    pub fn locations_edge_cache_keysets_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEdgeCacheKeysetTestIamPermissionCall<'a, S> {
        ProjectLocationEdgeCacheKeysetTestIamPermissionCall {
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
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_edge_cache_origins_get_iam_policy(&self, resource: &str) -> ProjectLocationEdgeCacheOriginGetIamPolicyCall<'a, S> {
        ProjectLocationEdgeCacheOriginGetIamPolicyCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_edge_cache_origins_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationEdgeCacheOriginSetIamPolicyCall<'a, S> {
        ProjectLocationEdgeCacheOriginSetIamPolicyCall {
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
    pub fn locations_edge_cache_origins_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEdgeCacheOriginTestIamPermissionCall<'a, S> {
        ProjectLocationEdgeCacheOriginTestIamPermissionCall {
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
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_edge_cache_services_get_iam_policy(&self, resource: &str) -> ProjectLocationEdgeCacheServiceGetIamPolicyCall<'a, S> {
        ProjectLocationEdgeCacheServiceGetIamPolicyCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_edge_cache_services_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationEdgeCacheServiceSetIamPolicyCall<'a, S> {
        ProjectLocationEdgeCacheServiceSetIamPolicyCall {
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
    pub fn locations_edge_cache_services_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEdgeCacheServiceTestIamPermissionCall<'a, S> {
        ProjectLocationEdgeCacheServiceTestIamPermissionCall {
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
    /// Creates a new EndpointPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the EndpointPolicy. Must be in the format `projects/*/locations/global`.
    pub fn locations_endpoint_policies_create(&self, request: EndpointPolicy, parent: &str) -> ProjectLocationEndpointPolicyCreateCall<'a, S> {
        ProjectLocationEndpointPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _endpoint_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single EndpointPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the EndpointPolicy to delete. Must be in the format `projects/*/locations/global/endpointPolicies/*`.
    pub fn locations_endpoint_policies_delete(&self, name: &str) -> ProjectLocationEndpointPolicyDeleteCall<'a, S> {
        ProjectLocationEndpointPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single EndpointPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the EndpointPolicy to get. Must be in the format `projects/*/locations/global/endpointPolicies/*`.
    pub fn locations_endpoint_policies_get(&self, name: &str) -> ProjectLocationEndpointPolicyGetCall<'a, S> {
        ProjectLocationEndpointPolicyGetCall {
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
    pub fn locations_endpoint_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationEndpointPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationEndpointPolicyGetIamPolicyCall {
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
    /// Lists EndpointPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the EndpointPolicies should be listed, specified in the format `projects/*/locations/global`.
    pub fn locations_endpoint_policies_list(&self, parent: &str) -> ProjectLocationEndpointPolicyListCall<'a, S> {
        ProjectLocationEndpointPolicyListCall {
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
    /// Updates the parameters of a single EndpointPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the EndpointPolicy resource. It matches pattern `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`.
    pub fn locations_endpoint_policies_patch(&self, request: EndpointPolicy, name: &str) -> ProjectLocationEndpointPolicyPatchCall<'a, S> {
        ProjectLocationEndpointPolicyPatchCall {
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
    pub fn locations_endpoint_policies_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationEndpointPolicySetIamPolicyCall<'a, S> {
        ProjectLocationEndpointPolicySetIamPolicyCall {
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
    pub fn locations_endpoint_policies_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationEndpointPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationEndpointPolicyTestIamPermissionCall {
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
    /// * `parent` - Required. The parent resource of the Gateway. Must be in the format `projects/*/locations/*`.
    pub fn locations_gateways_create(&self, request: Gateway, parent: &str) -> ProjectLocationGatewayCreateCall<'a, S> {
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
    /// * `name` - Required. A name of the Gateway to delete. Must be in the format `projects/*/locations/*/gateways/*`.
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
    /// * `name` - Required. A name of the Gateway to get. Must be in the format `projects/*/locations/*/gateways/*`.
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
    /// * `parent` - Required. The project and location from which the Gateways should be listed, specified in the format `projects/*/locations/*`.
    pub fn locations_gateways_list(&self, parent: &str) -> ProjectLocationGatewayListCall<'a, S> {
        ProjectLocationGatewayListCall {
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
    /// Updates the parameters of a single Gateway.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the Gateway resource. It matches pattern `projects/*/locations/*/gateways/`.
    pub fn locations_gateways_patch(&self, request: Gateway, name: &str) -> ProjectLocationGatewayPatchCall<'a, S> {
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
    pub fn locations_gateways_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationGatewaySetIamPolicyCall<'a, S> {
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
    pub fn locations_gateways_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationGatewayTestIamPermissionCall<'a, S> {
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
    /// Creates a new GrpcRoute in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the GrpcRoute. Must be in the format `projects/*/locations/global`.
    pub fn locations_grpc_routes_create(&self, request: GrpcRoute, parent: &str) -> ProjectLocationGrpcRouteCreateCall<'a, S> {
        ProjectLocationGrpcRouteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _grpc_route_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single GrpcRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the GrpcRoute to delete. Must be in the format `projects/*/locations/global/grpcRoutes/*`.
    pub fn locations_grpc_routes_delete(&self, name: &str) -> ProjectLocationGrpcRouteDeleteCall<'a, S> {
        ProjectLocationGrpcRouteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single GrpcRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the GrpcRoute to get. Must be in the format `projects/*/locations/global/grpcRoutes/*`.
    pub fn locations_grpc_routes_get(&self, name: &str) -> ProjectLocationGrpcRouteGetCall<'a, S> {
        ProjectLocationGrpcRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists GrpcRoutes in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the GrpcRoutes should be listed, specified in the format `projects/*/locations/global`.
    pub fn locations_grpc_routes_list(&self, parent: &str) -> ProjectLocationGrpcRouteListCall<'a, S> {
        ProjectLocationGrpcRouteListCall {
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
    /// Updates the parameters of a single GrpcRoute.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the GrpcRoute resource. It matches pattern `projects/*/locations/global/grpcRoutes/`
    pub fn locations_grpc_routes_patch(&self, request: GrpcRoute, name: &str) -> ProjectLocationGrpcRoutePatchCall<'a, S> {
        ProjectLocationGrpcRoutePatchCall {
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
    /// Creates a new HttpRoute in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the HttpRoute. Must be in the format `projects/*/locations/global`.
    pub fn locations_http_routes_create(&self, request: HttpRoute, parent: &str) -> ProjectLocationHttpRouteCreateCall<'a, S> {
        ProjectLocationHttpRouteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _http_route_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single HttpRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the HttpRoute to delete. Must be in the format `projects/*/locations/global/httpRoutes/*`.
    pub fn locations_http_routes_delete(&self, name: &str) -> ProjectLocationHttpRouteDeleteCall<'a, S> {
        ProjectLocationHttpRouteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single HttpRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the HttpRoute to get. Must be in the format `projects/*/locations/global/httpRoutes/*`.
    pub fn locations_http_routes_get(&self, name: &str) -> ProjectLocationHttpRouteGetCall<'a, S> {
        ProjectLocationHttpRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists HttpRoute in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the HttpRoutes should be listed, specified in the format `projects/*/locations/global`.
    pub fn locations_http_routes_list(&self, parent: &str) -> ProjectLocationHttpRouteListCall<'a, S> {
        ProjectLocationHttpRouteListCall {
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
    /// Updates the parameters of a single HttpRoute.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the HttpRoute resource. It matches pattern `projects/*/locations/global/httpRoutes/http_route_name>`.
    pub fn locations_http_routes_patch(&self, request: HttpRoute, name: &str) -> ProjectLocationHttpRoutePatchCall<'a, S> {
        ProjectLocationHttpRoutePatchCall {
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
    /// Creates a new Mesh in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the Mesh. Must be in the format `projects/*/locations/global`.
    pub fn locations_meshes_create(&self, request: Mesh, parent: &str) -> ProjectLocationMeshCreateCall<'a, S> {
        ProjectLocationMeshCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _mesh_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Mesh.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the Mesh to delete. Must be in the format `projects/*/locations/global/meshes/*`.
    pub fn locations_meshes_delete(&self, name: &str) -> ProjectLocationMeshDeleteCall<'a, S> {
        ProjectLocationMeshDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Mesh.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the Mesh to get. Must be in the format `projects/*/locations/global/meshes/*`.
    pub fn locations_meshes_get(&self, name: &str) -> ProjectLocationMeshGetCall<'a, S> {
        ProjectLocationMeshGetCall {
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
    pub fn locations_meshes_get_iam_policy(&self, resource: &str) -> ProjectLocationMeshGetIamPolicyCall<'a, S> {
        ProjectLocationMeshGetIamPolicyCall {
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
    /// Lists Meshes in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the Meshes should be listed, specified in the format `projects/*/locations/global`.
    pub fn locations_meshes_list(&self, parent: &str) -> ProjectLocationMeshListCall<'a, S> {
        ProjectLocationMeshListCall {
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
    /// Updates the parameters of a single Mesh.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the Mesh resource. It matches pattern `projects/*/locations/global/meshes/`.
    pub fn locations_meshes_patch(&self, request: Mesh, name: &str) -> ProjectLocationMeshPatchCall<'a, S> {
        ProjectLocationMeshPatchCall {
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
    pub fn locations_meshes_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationMeshSetIamPolicyCall<'a, S> {
        ProjectLocationMeshSetIamPolicyCall {
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
    pub fn locations_meshes_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationMeshTestIamPermissionCall<'a, S> {
        ProjectLocationMeshTestIamPermissionCall {
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
    /// Creates a new ServiceBinding in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the ServiceBinding. Must be in the format `projects/*/locations/global`.
    pub fn locations_service_bindings_create(&self, request: ServiceBinding, parent: &str) -> ProjectLocationServiceBindingCreateCall<'a, S> {
        ProjectLocationServiceBindingCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_binding_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ServiceBinding.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ServiceBinding to delete. Must be in the format `projects/*/locations/global/serviceBindings/*`.
    pub fn locations_service_bindings_delete(&self, name: &str) -> ProjectLocationServiceBindingDeleteCall<'a, S> {
        ProjectLocationServiceBindingDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ServiceBinding.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ServiceBinding to get. Must be in the format `projects/*/locations/global/serviceBindings/*`.
    pub fn locations_service_bindings_get(&self, name: &str) -> ProjectLocationServiceBindingGetCall<'a, S> {
        ProjectLocationServiceBindingGetCall {
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
    pub fn locations_service_bindings_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceBindingGetIamPolicyCall<'a, S> {
        ProjectLocationServiceBindingGetIamPolicyCall {
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
    /// Lists ServiceBinding in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the ServiceBindings should be listed, specified in the format `projects/*/locations/global`.
    pub fn locations_service_bindings_list(&self, parent: &str) -> ProjectLocationServiceBindingListCall<'a, S> {
        ProjectLocationServiceBindingListCall {
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
    pub fn locations_service_bindings_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceBindingSetIamPolicyCall<'a, S> {
        ProjectLocationServiceBindingSetIamPolicyCall {
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
    pub fn locations_service_bindings_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceBindingTestIamPermissionCall<'a, S> {
        ProjectLocationServiceBindingTestIamPermissionCall {
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
    /// Creates a new TcpRoute in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the TcpRoute. Must be in the format `projects/*/locations/global`.
    pub fn locations_tcp_routes_create(&self, request: TcpRoute, parent: &str) -> ProjectLocationTcpRouteCreateCall<'a, S> {
        ProjectLocationTcpRouteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _tcp_route_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single TcpRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the TcpRoute to delete. Must be in the format `projects/*/locations/global/tcpRoutes/*`.
    pub fn locations_tcp_routes_delete(&self, name: &str) -> ProjectLocationTcpRouteDeleteCall<'a, S> {
        ProjectLocationTcpRouteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single TcpRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the TcpRoute to get. Must be in the format `projects/*/locations/global/tcpRoutes/*`.
    pub fn locations_tcp_routes_get(&self, name: &str) -> ProjectLocationTcpRouteGetCall<'a, S> {
        ProjectLocationTcpRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists TcpRoute in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the TcpRoutes should be listed, specified in the format `projects/*/locations/global`.
    pub fn locations_tcp_routes_list(&self, parent: &str) -> ProjectLocationTcpRouteListCall<'a, S> {
        ProjectLocationTcpRouteListCall {
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
    /// Updates the parameters of a single TcpRoute.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the TcpRoute resource. It matches pattern `projects/*/locations/global/tcpRoutes/tcp_route_name>`.
    pub fn locations_tcp_routes_patch(&self, request: TcpRoute, name: &str) -> ProjectLocationTcpRoutePatchCall<'a, S> {
        ProjectLocationTcpRoutePatchCall {
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
    /// Creates a new TlsRoute in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the TlsRoute. Must be in the format `projects/*/locations/global`.
    pub fn locations_tls_routes_create(&self, request: TlsRoute, parent: &str) -> ProjectLocationTlsRouteCreateCall<'a, S> {
        ProjectLocationTlsRouteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _tls_route_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single TlsRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the TlsRoute to delete. Must be in the format `projects/*/locations/global/tlsRoutes/*`.
    pub fn locations_tls_routes_delete(&self, name: &str) -> ProjectLocationTlsRouteDeleteCall<'a, S> {
        ProjectLocationTlsRouteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single TlsRoute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the TlsRoute to get. Must be in the format `projects/*/locations/global/tlsRoutes/*`.
    pub fn locations_tls_routes_get(&self, name: &str) -> ProjectLocationTlsRouteGetCall<'a, S> {
        ProjectLocationTlsRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists TlsRoute in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the TlsRoutes should be listed, specified in the format `projects/*/locations/global`.
    pub fn locations_tls_routes_list(&self, parent: &str) -> ProjectLocationTlsRouteListCall<'a, S> {
        ProjectLocationTlsRouteListCall {
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
    /// Updates the parameters of a single TlsRoute.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the TlsRoute resource. It matches pattern `projects/*/locations/global/tlsRoutes/tls_route_name>`.
    pub fn locations_tls_routes_patch(&self, request: TlsRoute, name: &str) -> ProjectLocationTlsRoutePatchCall<'a, S> {
        ProjectLocationTlsRoutePatchCall {
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



