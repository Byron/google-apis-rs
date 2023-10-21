use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Connectors`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_connectors1 as connectors1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use connectors1::{Connectors, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Connectors::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_connections_connection_schema_metadata_refresh(...)`, `locations_connections_create(...)`, `locations_connections_delete(...)`, `locations_connections_get(...)`, `locations_connections_get_connection_schema_metadata(...)`, `locations_connections_get_iam_policy(...)`, `locations_connections_list(...)`, `locations_connections_patch(...)`, `locations_connections_runtime_action_schemas_list(...)`, `locations_connections_runtime_entity_schemas_list(...)`, `locations_connections_set_iam_policy(...)`, `locations_connections_test_iam_permissions(...)`, `locations_get(...)`, `locations_get_runtime_config(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_providers_connectors_get(...)`, `locations_providers_connectors_list(...)`, `locations_providers_connectors_versions_get(...)`, `locations_providers_connectors_versions_list(...)`, `locations_providers_get(...)`, `locations_providers_get_iam_policy(...)`, `locations_providers_list(...)`, `locations_providers_set_iam_policy(...)` and `locations_providers_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Connectors<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Refresh runtime schema of a connection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name. Format: projects/{project}/locations/{location}/connections/{connection}/connectionSchemaMetadata
    pub fn locations_connections_connection_schema_metadata_refresh(&self, request: RefreshConnectionSchemaMetadataRequest, name: &str) -> ProjectLocationConnectionConnectionSchemaMetadataRefreshCall<'a, S> {
        ProjectLocationConnectionConnectionSchemaMetadataRefreshCall {
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
    /// List schema of a runtime actions filtered by action name.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of RuntimeActionSchema Format: projects/{project}/locations/{location}/connections/{connection}
    pub fn locations_connections_runtime_action_schemas_list(&self, parent: &str) -> ProjectLocationConnectionRuntimeActionSchemaListCall<'a, S> {
        ProjectLocationConnectionRuntimeActionSchemaListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// List schema of a runtime entities filtered by entity name.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of RuntimeEntitySchema Format: projects/{project}/locations/{location}/connections/{connection}
    pub fn locations_connections_runtime_entity_schemas_list(&self, parent: &str) -> ProjectLocationConnectionRuntimeEntitySchemaListCall<'a, S> {
        ProjectLocationConnectionRuntimeEntitySchemaListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Creates a new Connection in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource of the Connection, of the form: `projects/*/locations/*`
    pub fn locations_connections_create(&self, request: Connection, parent: &str) -> ProjectLocationConnectionCreateCall<'a, S> {
        ProjectLocationConnectionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _connection_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/connections/*`
    pub fn locations_connections_delete(&self, name: &str) -> ProjectLocationConnectionDeleteCall<'a, S> {
        ProjectLocationConnectionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/connections/*`
    pub fn locations_connections_get(&self, name: &str) -> ProjectLocationConnectionGetCall<'a, S> {
        ProjectLocationConnectionGetCall {
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
    /// Gets schema metadata of a connection. SchemaMetadata is a singleton resource for each connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Connection name Format: projects/{project}/locations/{location}/connections/{connection}/connectionSchemaMetadata
    pub fn locations_connections_get_connection_schema_metadata(&self, name: &str) -> ProjectLocationConnectionGetConnectionSchemaMetadataCall<'a, S> {
        ProjectLocationConnectionGetConnectionSchemaMetadataCall {
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
    pub fn locations_connections_get_iam_policy(&self, resource: &str) -> ProjectLocationConnectionGetIamPolicyCall<'a, S> {
        ProjectLocationConnectionGetIamPolicyCall {
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
    /// Lists Connections in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the Connection, of the form: `projects/*/locations/*`
    pub fn locations_connections_list(&self, parent: &str) -> ProjectLocationConnectionListCall<'a, S> {
        ProjectLocationConnectionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Updates the parameters of a single Connection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the Connection. Format: projects/{project}/locations/{location}/connections/{connection}
    pub fn locations_connections_patch(&self, request: Connection, name: &str) -> ProjectLocationConnectionPatchCall<'a, S> {
        ProjectLocationConnectionPatchCall {
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
    pub fn locations_connections_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationConnectionSetIamPolicyCall<'a, S> {
        ProjectLocationConnectionSetIamPolicyCall {
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
    pub fn locations_connections_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationConnectionTestIamPermissionCall<'a, S> {
        ProjectLocationConnectionTestIamPermissionCall {
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
    /// Gets details of a single connector version.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/providers/*/connectors/*/versions/*` Only global location is supported for ConnectorVersion resource.
    pub fn locations_providers_connectors_versions_get(&self, name: &str) -> ProjectLocationProviderConnectorVersionGetCall<'a, S> {
        ProjectLocationProviderConnectorVersionGetCall {
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
    /// Lists Connector Versions in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the connectors, of the form: `projects/*/locations/*/providers/*/connectors/*` Only global location is supported for ConnectorVersion resource.
    pub fn locations_providers_connectors_versions_list(&self, parent: &str) -> ProjectLocationProviderConnectorVersionListCall<'a, S> {
        ProjectLocationProviderConnectorVersionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Connector.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/providers/*/connectors/*` Only global location is supported for Connector resource.
    pub fn locations_providers_connectors_get(&self, name: &str) -> ProjectLocationProviderConnectorGetCall<'a, S> {
        ProjectLocationProviderConnectorGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Connectors in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the connectors, of the form: `projects/*/locations/*/providers/*` Only global location is supported for Connector resource.
    pub fn locations_providers_connectors_list(&self, parent: &str) -> ProjectLocationProviderConnectorListCall<'a, S> {
        ProjectLocationProviderConnectorListCall {
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
    /// Gets details of a provider.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/providers/*` Only global location is supported for Provider resource.
    pub fn locations_providers_get(&self, name: &str) -> ProjectLocationProviderGetCall<'a, S> {
        ProjectLocationProviderGetCall {
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
    pub fn locations_providers_get_iam_policy(&self, resource: &str) -> ProjectLocationProviderGetIamPolicyCall<'a, S> {
        ProjectLocationProviderGetIamPolicyCall {
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
    /// Lists Providers in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the API, of the form: `projects/*/locations/*` Only global location is supported for Provider resource.
    pub fn locations_providers_list(&self, parent: &str) -> ProjectLocationProviderListCall<'a, S> {
        ProjectLocationProviderListCall {
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
    pub fn locations_providers_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationProviderSetIamPolicyCall<'a, S> {
        ProjectLocationProviderSetIamPolicyCall {
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
    pub fn locations_providers_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationProviderTestIamPermissionCall<'a, S> {
        ProjectLocationProviderTestIamPermissionCall {
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
    /// Gets the runtimeConfig of a location. RuntimeConfig is a singleton resource for each location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the form: `projects/*/locations/*/runtimeConfig`
    pub fn locations_get_runtime_config(&self, name: &str) -> ProjectLocationGetRuntimeConfigCall<'a, S> {
        ProjectLocationGetRuntimeConfigCall {
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



