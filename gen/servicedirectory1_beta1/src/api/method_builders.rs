use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`ServiceDirectory`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_servicedirectory1_beta1 as servicedirectory1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use servicedirectory1_beta1::{ServiceDirectory, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ServiceDirectory::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_list(...)`, `locations_namespaces_create(...)`, `locations_namespaces_delete(...)`, `locations_namespaces_get(...)`, `locations_namespaces_get_iam_policy(...)`, `locations_namespaces_list(...)`, `locations_namespaces_patch(...)`, `locations_namespaces_service_workloads_get_iam_policy(...)`, `locations_namespaces_service_workloads_set_iam_policy(...)`, `locations_namespaces_service_workloads_test_iam_permissions(...)`, `locations_namespaces_services_create(...)`, `locations_namespaces_services_delete(...)`, `locations_namespaces_services_endpoints_create(...)`, `locations_namespaces_services_endpoints_delete(...)`, `locations_namespaces_services_endpoints_get(...)`, `locations_namespaces_services_endpoints_list(...)`, `locations_namespaces_services_endpoints_patch(...)`, `locations_namespaces_services_get(...)`, `locations_namespaces_services_get_iam_policy(...)`, `locations_namespaces_services_list(...)`, `locations_namespaces_services_patch(...)`, `locations_namespaces_services_resolve(...)`, `locations_namespaces_services_set_iam_policy(...)`, `locations_namespaces_services_test_iam_permissions(...)`, `locations_namespaces_set_iam_policy(...)`, `locations_namespaces_test_iam_permissions(...)`, `locations_registration_policies_get_iam_policy(...)`, `locations_registration_policies_set_iam_policy(...)` and `locations_registration_policies_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ServiceDirectory<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_service_workloads_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationNamespaceServiceWorkloadGetIamPolicyCall<'a, S> {
        ProjectLocationNamespaceServiceWorkloadGetIamPolicyCall {
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
    /// Sets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_service_workloads_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationNamespaceServiceWorkloadSetIamPolicyCall<'a, S> {
        ProjectLocationNamespaceServiceWorkloadSetIamPolicyCall {
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
    /// Tests IAM permissions for a resource (namespace, service or service workload only).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_service_workloads_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationNamespaceServiceWorkloadTestIamPermissionCall<'a, S> {
        ProjectLocationNamespaceServiceWorkloadTestIamPermissionCall {
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
    /// Creates an endpoint, and returns the new endpoint.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the service that this endpoint provides.
    pub fn locations_namespaces_services_endpoints_create(&self, request: Endpoint, parent: &str) -> ProjectLocationNamespaceServiceEndpointCreateCall<'a, S> {
        ProjectLocationNamespaceServiceEndpointCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _endpoint_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an endpoint.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the endpoint to delete.
    pub fn locations_namespaces_services_endpoints_delete(&self, name: &str) -> ProjectLocationNamespaceServiceEndpointDeleteCall<'a, S> {
        ProjectLocationNamespaceServiceEndpointDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an endpoint.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the endpoint to get.
    pub fn locations_namespaces_services_endpoints_get(&self, name: &str) -> ProjectLocationNamespaceServiceEndpointGetCall<'a, S> {
        ProjectLocationNamespaceServiceEndpointGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all endpoints.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the service whose endpoints you'd like to list.
    pub fn locations_namespaces_services_endpoints_list(&self, parent: &str) -> ProjectLocationNamespaceServiceEndpointListCall<'a, S> {
        ProjectLocationNamespaceServiceEndpointListCall {
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
    /// Updates an endpoint.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name for the endpoint in the format `projects/*/locations/*/namespaces/*/services/*/endpoints/*`.
    pub fn locations_namespaces_services_endpoints_patch(&self, request: Endpoint, name: &str) -> ProjectLocationNamespaceServiceEndpointPatchCall<'a, S> {
        ProjectLocationNamespaceServiceEndpointPatchCall {
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
    /// Creates a service, and returns the new service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the namespace this service will belong to.
    pub fn locations_namespaces_services_create(&self, request: Service, parent: &str) -> ProjectLocationNamespaceServiceCreateCall<'a, S> {
        ProjectLocationNamespaceServiceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a service. This also deletes all endpoints associated with the service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the service to delete.
    pub fn locations_namespaces_services_delete(&self, name: &str) -> ProjectLocationNamespaceServiceDeleteCall<'a, S> {
        ProjectLocationNamespaceServiceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a service.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the service to get.
    pub fn locations_namespaces_services_get(&self, name: &str) -> ProjectLocationNamespaceServiceGetCall<'a, S> {
        ProjectLocationNamespaceServiceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_services_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationNamespaceServiceGetIamPolicyCall<'a, S> {
        ProjectLocationNamespaceServiceGetIamPolicyCall {
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
    /// Lists all services belonging to a namespace.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the namespace whose services you'd like to list.
    pub fn locations_namespaces_services_list(&self, parent: &str) -> ProjectLocationNamespaceServiceListCall<'a, S> {
        ProjectLocationNamespaceServiceListCall {
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
    /// Updates a service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name for the service in the format `projects/*/locations/*/namespaces/*/services/*`.
    pub fn locations_namespaces_services_patch(&self, request: Service, name: &str) -> ProjectLocationNamespaceServicePatchCall<'a, S> {
        ProjectLocationNamespaceServicePatchCall {
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
    /// Returns a service and its associated endpoints. Resolving a service is not considered an active developer method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the service to resolve.
    pub fn locations_namespaces_services_resolve(&self, request: ResolveServiceRequest, name: &str) -> ProjectLocationNamespaceServiceResolveCall<'a, S> {
        ProjectLocationNamespaceServiceResolveCall {
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
    /// Sets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_services_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationNamespaceServiceSetIamPolicyCall<'a, S> {
        ProjectLocationNamespaceServiceSetIamPolicyCall {
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
    /// Tests IAM permissions for a resource (namespace, service or service workload only).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_services_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationNamespaceServiceTestIamPermissionCall<'a, S> {
        ProjectLocationNamespaceServiceTestIamPermissionCall {
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
    /// Creates a namespace, and returns the new namespace.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the project and location the namespace will be created in.
    pub fn locations_namespaces_create(&self, request: Namespace, parent: &str) -> ProjectLocationNamespaceCreateCall<'a, S> {
        ProjectLocationNamespaceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _namespace_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a namespace. This also deletes all services and endpoints in the namespace.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the namespace to delete.
    pub fn locations_namespaces_delete(&self, name: &str) -> ProjectLocationNamespaceDeleteCall<'a, S> {
        ProjectLocationNamespaceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a namespace.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the namespace to retrieve.
    pub fn locations_namespaces_get(&self, name: &str) -> ProjectLocationNamespaceGetCall<'a, S> {
        ProjectLocationNamespaceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationNamespaceGetIamPolicyCall<'a, S> {
        ProjectLocationNamespaceGetIamPolicyCall {
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
    /// Lists all namespaces.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the project and location whose namespaces you'd like to list.
    pub fn locations_namespaces_list(&self, parent: &str) -> ProjectLocationNamespaceListCall<'a, S> {
        ProjectLocationNamespaceListCall {
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
    /// Updates a namespace.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name for the namespace in the format `projects/*/locations/*/namespaces/*`.
    pub fn locations_namespaces_patch(&self, request: Namespace, name: &str) -> ProjectLocationNamespacePatchCall<'a, S> {
        ProjectLocationNamespacePatchCall {
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
    /// Sets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationNamespaceSetIamPolicyCall<'a, S> {
        ProjectLocationNamespaceSetIamPolicyCall {
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
    /// Tests IAM permissions for a resource (namespace, service or service workload only).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_namespaces_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationNamespaceTestIamPermissionCall<'a, S> {
        ProjectLocationNamespaceTestIamPermissionCall {
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
    /// Gets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registration_policies_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationRegistrationPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationRegistrationPolicyGetIamPolicyCall {
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
    /// Sets the IAM Policy for a resource
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registration_policies_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationRegistrationPolicySetIamPolicyCall<'a, S> {
        ProjectLocationRegistrationPolicySetIamPolicyCall {
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
    /// Tests IAM permissions for a resource (namespace, service or service workload only).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_registration_policies_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationRegistrationPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationRegistrationPolicyTestIamPermissionCall {
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



