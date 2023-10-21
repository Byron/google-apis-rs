use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`SecretManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_secretmanager1 as secretmanager1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use secretmanager1::{SecretManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SecretManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_list(...)`, `secrets_add_version(...)`, `secrets_create(...)`, `secrets_delete(...)`, `secrets_get(...)`, `secrets_get_iam_policy(...)`, `secrets_list(...)`, `secrets_patch(...)`, `secrets_set_iam_policy(...)`, `secrets_test_iam_permissions(...)`, `secrets_versions_access(...)`, `secrets_versions_destroy(...)`, `secrets_versions_disable(...)`, `secrets_versions_enable(...)`, `secrets_versions_get(...)` and `secrets_versions_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SecretManager<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accesses a SecretVersion. This call returns the secret data. `projects/*/secrets/*/versions/latest` is an alias to the most recently created SecretVersion.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the SecretVersion in the format `projects/*/secrets/*/versions/*`. `projects/*/secrets/*/versions/latest` is an alias to the most recently created SecretVersion.
    pub fn secrets_versions_access(&self, name: &str) -> ProjectSecretVersionAccesCall<'a, S> {
        ProjectSecretVersionAccesCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Destroys a SecretVersion. Sets the state of the SecretVersion to DESTROYED and irrevocably destroys the secret data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the SecretVersion to destroy in the format `projects/*/secrets/*/versions/*`.
    pub fn secrets_versions_destroy(&self, request: DestroySecretVersionRequest, name: &str) -> ProjectSecretVersionDestroyCall<'a, S> {
        ProjectSecretVersionDestroyCall {
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
    /// Disables a SecretVersion. Sets the state of the SecretVersion to DISABLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the SecretVersion to disable in the format `projects/*/secrets/*/versions/*`.
    pub fn secrets_versions_disable(&self, request: DisableSecretVersionRequest, name: &str) -> ProjectSecretVersionDisableCall<'a, S> {
        ProjectSecretVersionDisableCall {
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
    /// Enables a SecretVersion. Sets the state of the SecretVersion to ENABLED.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the SecretVersion to enable in the format `projects/*/secrets/*/versions/*`.
    pub fn secrets_versions_enable(&self, request: EnableSecretVersionRequest, name: &str) -> ProjectSecretVersionEnableCall<'a, S> {
        ProjectSecretVersionEnableCall {
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
    /// Gets metadata for a SecretVersion. `projects/*/secrets/*/versions/latest` is an alias to the most recently created SecretVersion.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the SecretVersion in the format `projects/*/secrets/*/versions/*`. `projects/*/secrets/*/versions/latest` is an alias to the most recently created SecretVersion.
    pub fn secrets_versions_get(&self, name: &str) -> ProjectSecretVersionGetCall<'a, S> {
        ProjectSecretVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists SecretVersions. This call does not return secret data.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the Secret associated with the SecretVersions to list, in the format `projects/*/secrets/*`.
    pub fn secrets_versions_list(&self, parent: &str) -> ProjectSecretVersionListCall<'a, S> {
        ProjectSecretVersionListCall {
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
    /// Creates a new SecretVersion containing secret data and attaches it to an existing Secret.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the Secret to associate with the SecretVersion in the format `projects/*/secrets/*`.
    pub fn secrets_add_version(&self, request: AddSecretVersionRequest, parent: &str) -> ProjectSecretAddVersionCall<'a, S> {
        ProjectSecretAddVersionCall {
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
    /// Creates a new Secret containing no SecretVersions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the project to associate with the Secret, in the format `projects/*`.
    pub fn secrets_create(&self, request: Secret, parent: &str) -> ProjectSecretCreateCall<'a, S> {
        ProjectSecretCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _secret_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Secret.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Secret to delete in the format `projects/*/secrets/*`.
    pub fn secrets_delete(&self, name: &str) -> ProjectSecretDeleteCall<'a, S> {
        ProjectSecretDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets metadata for a given Secret.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Secret, in the format `projects/*/secrets/*`.
    pub fn secrets_get(&self, name: &str) -> ProjectSecretGetCall<'a, S> {
        ProjectSecretGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a secret. Returns empty policy if the secret exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn secrets_get_iam_policy(&self, resource: &str) -> ProjectSecretGetIamPolicyCall<'a, S> {
        ProjectSecretGetIamPolicyCall {
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
    /// Lists Secrets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the project associated with the Secrets, in the format `projects/*`.
    pub fn secrets_list(&self, parent: &str) -> ProjectSecretListCall<'a, S> {
        ProjectSecretListCall {
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
    /// Updates metadata of an existing Secret.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the Secret in the format `projects/*/secrets/*`.
    pub fn secrets_patch(&self, request: Secret, name: &str) -> ProjectSecretPatchCall<'a, S> {
        ProjectSecretPatchCall {
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
    /// Sets the access control policy on the specified secret. Replaces any existing policy. Permissions on SecretVersions are enforced according to the policy set on the associated Secret.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn secrets_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectSecretSetIamPolicyCall<'a, S> {
        ProjectSecretSetIamPolicyCall {
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
    /// Returns permissions that a caller has for the specified secret. If the secret does not exist, this call returns an empty set of permissions, not a NOT_FOUND error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn secrets_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectSecretTestIamPermissionCall<'a, S> {
        ProjectSecretTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



