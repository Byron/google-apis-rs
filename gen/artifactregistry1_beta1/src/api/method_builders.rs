use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`ArtifactRegistry`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_artifactregistry1_beta1 as artifactregistry1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use artifactregistry1_beta1::{ArtifactRegistry, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ArtifactRegistry::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_list(...)`, `locations_operations_get(...)`, `locations_repositories_create(...)`, `locations_repositories_delete(...)`, `locations_repositories_files_get(...)`, `locations_repositories_files_list(...)`, `locations_repositories_get(...)`, `locations_repositories_get_iam_policy(...)`, `locations_repositories_list(...)`, `locations_repositories_packages_delete(...)`, `locations_repositories_packages_get(...)`, `locations_repositories_packages_list(...)`, `locations_repositories_packages_tags_create(...)`, `locations_repositories_packages_tags_delete(...)`, `locations_repositories_packages_tags_get(...)`, `locations_repositories_packages_tags_list(...)`, `locations_repositories_packages_tags_patch(...)`, `locations_repositories_packages_versions_delete(...)`, `locations_repositories_packages_versions_get(...)`, `locations_repositories_packages_versions_list(...)`, `locations_repositories_patch(...)`, `locations_repositories_set_iam_policy(...)` and `locations_repositories_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ArtifactRegistry<S>,
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
    /// Gets a file.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the file to retrieve.
    pub fn locations_repositories_files_get(&self, name: &str) -> ProjectLocationRepositoryFileGetCall<'a, S> {
        ProjectLocationRepositoryFileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists files.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the repository whose files will be listed. For example: "projects/p1/locations/us-central1/repositories/repo1
    pub fn locations_repositories_files_list(&self, parent: &str) -> ProjectLocationRepositoryFileListCall<'a, S> {
        ProjectLocationRepositoryFileListCall {
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
    /// Creates a tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the parent resource where the tag will be created.
    pub fn locations_repositories_packages_tags_create(&self, request: Tag, parent: &str) -> ProjectLocationRepositoryPackageTagCreateCall<'a, S> {
        ProjectLocationRepositoryPackageTagCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _tag_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a tag.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the tag to delete.
    pub fn locations_repositories_packages_tags_delete(&self, name: &str) -> ProjectLocationRepositoryPackageTagDeleteCall<'a, S> {
        ProjectLocationRepositoryPackageTagDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a tag.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the tag to retrieve.
    pub fn locations_repositories_packages_tags_get(&self, name: &str) -> ProjectLocationRepositoryPackageTagGetCall<'a, S> {
        ProjectLocationRepositoryPackageTagGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists tags.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the parent resource whose tags will be listed.
    pub fn locations_repositories_packages_tags_list(&self, parent: &str) -> ProjectLocationRepositoryPackageTagListCall<'a, S> {
        ProjectLocationRepositoryPackageTagListCall {
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
    /// Updates a tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded.
    pub fn locations_repositories_packages_tags_patch(&self, request: Tag, name: &str) -> ProjectLocationRepositoryPackageTagPatchCall<'a, S> {
        ProjectLocationRepositoryPackageTagPatchCall {
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
    /// Deletes a version and all of its content. The returned operation will complete once the version has been deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the version to delete.
    pub fn locations_repositories_packages_versions_delete(&self, name: &str) -> ProjectLocationRepositoryPackageVersionDeleteCall<'a, S> {
        ProjectLocationRepositoryPackageVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a version
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the version to retrieve.
    pub fn locations_repositories_packages_versions_get(&self, name: &str) -> ProjectLocationRepositoryPackageVersionGetCall<'a, S> {
        ProjectLocationRepositoryPackageVersionGetCall {
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
    /// Lists versions.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the parent resource whose versions will be listed.
    pub fn locations_repositories_packages_versions_list(&self, parent: &str) -> ProjectLocationRepositoryPackageVersionListCall<'a, S> {
        ProjectLocationRepositoryPackageVersionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a package and all of its versions and tags. The returned operation will complete once the package has been deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the package to delete.
    pub fn locations_repositories_packages_delete(&self, name: &str) -> ProjectLocationRepositoryPackageDeleteCall<'a, S> {
        ProjectLocationRepositoryPackageDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a package.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the package to retrieve.
    pub fn locations_repositories_packages_get(&self, name: &str) -> ProjectLocationRepositoryPackageGetCall<'a, S> {
        ProjectLocationRepositoryPackageGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists packages.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource whose packages will be listed.
    pub fn locations_repositories_packages_list(&self, parent: &str) -> ProjectLocationRepositoryPackageListCall<'a, S> {
        ProjectLocationRepositoryPackageListCall {
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
    /// Creates a repository. The returned Operation will finish once the repository has been created. Its response will be the created Repository.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource where the repository will be created.
    pub fn locations_repositories_create(&self, request: Repository, parent: &str) -> ProjectLocationRepositoryCreateCall<'a, S> {
        ProjectLocationRepositoryCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _repository_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a repository and all of its contents. The returned Operation will finish once the repository has been deleted. It will not have any Operation metadata and will return a google.protobuf.Empty response.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the repository to delete.
    pub fn locations_repositories_delete(&self, name: &str) -> ProjectLocationRepositoryDeleteCall<'a, S> {
        ProjectLocationRepositoryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a repository.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the repository to retrieve.
    pub fn locations_repositories_get(&self, name: &str) -> ProjectLocationRepositoryGetCall<'a, S> {
        ProjectLocationRepositoryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM policy for a given resource.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_repositories_get_iam_policy(&self, resource: &str) -> ProjectLocationRepositoryGetIamPolicyCall<'a, S> {
        ProjectLocationRepositoryGetIamPolicyCall {
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
    /// Lists repositories.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource whose repositories will be listed.
    pub fn locations_repositories_list(&self, parent: &str) -> ProjectLocationRepositoryListCall<'a, S> {
        ProjectLocationRepositoryListCall {
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
    /// Updates a repository.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the repository, for example: "projects/p1/locations/us-central1/repositories/repo1".
    pub fn locations_repositories_patch(&self, request: Repository, name: &str) -> ProjectLocationRepositoryPatchCall<'a, S> {
        ProjectLocationRepositoryPatchCall {
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
    /// Updates the IAM policy for a given resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_repositories_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationRepositorySetIamPolicyCall<'a, S> {
        ProjectLocationRepositorySetIamPolicyCall {
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
    /// Tests if the caller has a list of permissions on a resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_repositories_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationRepositoryTestIamPermissionCall<'a, S> {
        ProjectLocationRepositoryTestIamPermissionCall {
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



