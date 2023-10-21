use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudSourceRepositories`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_sourcerepo1 as sourcerepo1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use sourcerepo1::{CloudSourceRepositories, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudSourceRepositories::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_config(...)`, `repos_create(...)`, `repos_delete(...)`, `repos_get(...)`, `repos_get_iam_policy(...)`, `repos_list(...)`, `repos_patch(...)`, `repos_set_iam_policy(...)`, `repos_sync(...)`, `repos_test_iam_permissions(...)` and `update_config(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudSourceRepositories<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a repo in the given project with the given name. If the named repository already exists, `CreateRepo` returns `ALREADY_EXISTS`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The project in which to create the repo. Values are of the form `projects/`.
    pub fn repos_create(&self, request: Repo, parent: &str) -> ProjectRepoCreateCall<'a, S> {
        ProjectRepoCreateCall {
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
    /// Deletes a repo.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the repo to delete. Values are of the form `projects//repos/`.
    pub fn repos_delete(&self, name: &str) -> ProjectRepoDeleteCall<'a, S> {
        ProjectRepoDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a repo.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the requested repository. Values are of the form `projects//repos/`.
    pub fn repos_get(&self, name: &str) -> ProjectRepoGetCall<'a, S> {
        ProjectRepoGetCall {
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
    pub fn repos_get_iam_policy(&self, resource: &str) -> ProjectRepoGetIamPolicyCall<'a, S> {
        ProjectRepoGetIamPolicyCall {
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
    /// Returns all repos belonging to a project. The sizes of the repos are not set by ListRepos. To get the size of a repo, use GetRepo.
    /// 
    /// # Arguments
    ///
    /// * `name` - The project ID whose repos should be listed. Values are of the form `projects/`.
    pub fn repos_list(&self, name: &str) -> ProjectRepoListCall<'a, S> {
        ProjectRepoListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information about a repo.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the requested repository. Values are of the form `projects//repos/`.
    pub fn repos_patch(&self, request: UpdateRepoRequest, name: &str) -> ProjectRepoPatchCall<'a, S> {
        ProjectRepoPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn repos_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectRepoSetIamPolicyCall<'a, S> {
        ProjectRepoSetIamPolicyCall {
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
    /// Synchronize a connected repo. The response contains SyncRepoMetadata in the metadata field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the repo to synchronize. Values are of the form `projects//repos/`.
    pub fn repos_sync(&self, request: SyncRepoRequest, name: &str) -> ProjectRepoSyncCall<'a, S> {
        ProjectRepoSyncCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn repos_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectRepoTestIamPermissionCall<'a, S> {
        ProjectRepoTestIamPermissionCall {
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
    /// Returns the Cloud Source Repositories configuration of the project.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the requested project. Values are of the form `projects/`.
    pub fn get_config(&self, name: &str) -> ProjectGetConfigCall<'a, S> {
        ProjectGetConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Cloud Source Repositories configuration of the project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the requested project. Values are of the form `projects/`.
    pub fn update_config(&self, request: UpdateProjectConfigRequest, name: &str) -> ProjectUpdateConfigCall<'a, S> {
        ProjectUpdateConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



