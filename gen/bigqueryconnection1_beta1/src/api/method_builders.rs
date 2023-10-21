use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`BigQueryConnectionService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigqueryconnection1_beta1 as bigqueryconnection1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigqueryconnection1_beta1::{BigQueryConnectionService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = BigQueryConnectionService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_connections_create(...)`, `locations_connections_delete(...)`, `locations_connections_get(...)`, `locations_connections_get_iam_policy(...)`, `locations_connections_list(...)`, `locations_connections_patch(...)`, `locations_connections_set_iam_policy(...)`, `locations_connections_test_iam_permissions(...)` and `locations_connections_update_credential(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a BigQueryConnectionService<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new connection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. Must be in the format `projects/{project_id}/locations/{location_id}`
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
    /// Deletes connection and associated credential.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the deleted connection, for example: `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
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
    /// Returns specified connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the requested connection, for example: `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    pub fn locations_connections_get(&self, name: &str) -> ProjectLocationConnectionGetCall<'a, S> {
        ProjectLocationConnectionGetCall {
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
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_connections_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectLocationConnectionGetIamPolicyCall<'a, S> {
        ProjectLocationConnectionGetIamPolicyCall {
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
    /// Returns a list of connections in the given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. Must be in the form: `projects/{project_id}/locations/{location_id}`
    pub fn locations_connections_list(&self, parent: &str) -> ProjectLocationConnectionListCall<'a, S> {
        ProjectLocationConnectionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified connection. For security reasons, also resets credential if connection properties are in the update field mask.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the connection to update, for example: `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
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
    /// Sets the credential for the specified connection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the connection, for example: `projects/{project_id}/locations/{location_id}/connections/{connection_id}/credential`
    pub fn locations_connections_update_credential(&self, request: ConnectionCredential, name: &str) -> ProjectLocationConnectionUpdateCredentialCall<'a, S> {
        ProjectLocationConnectionUpdateCredentialCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



