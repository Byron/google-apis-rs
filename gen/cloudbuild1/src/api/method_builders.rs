use super::*;
/// A builder providing access to all methods supported on *githubDotComWebhook* resources.
/// It is not used directly, but through the [`CloudBuild`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudbuild1 as cloudbuild1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudbuild1::{CloudBuild, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudBuild::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `receive(...)`
/// // to build up your call.
/// let rb = hub.github_dot_com_webhook();
/// # }
/// ```
pub struct GithubDotComWebhookMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudBuild<S>,
}

impl<'a, S> client::MethodsBuilder for GithubDotComWebhookMethods<'a, S> {}

impl<'a, S> GithubDotComWebhookMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// ReceiveGitHubDotComWebhook is called when the API receives a github.com webhook.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn receive(&self, request: HttpBody) -> GithubDotComWebhookReceiveCall<'a, S> {
        GithubDotComWebhookReceiveCall {
            hub: self.hub,
            _request: request,
            _webhook_key: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`CloudBuild`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudbuild1 as cloudbuild1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudbuild1::{CloudBuild, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudBuild::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `regional_webhook(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudBuild<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// ReceiveRegionalWebhook is called when the API receives a regional GitHub webhook.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `location` - Required. The location where the webhook should be sent.
    pub fn regional_webhook(&self, request: HttpBody, location: &str) -> LocationRegionalWebhookCall<'a, S> {
        LocationRegionalWebhookCall {
            hub: self.hub,
            _request: request,
            _location: location.to_string(),
            _webhook_key: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`CloudBuild`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudbuild1 as cloudbuild1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudbuild1::{CloudBuild, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudBuild::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudBuild<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, request: CancelOperationRequest, name: &str) -> OperationCancelCall<'a, S> {
        OperationCancelCall {
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
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudBuild`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudbuild1 as cloudbuild1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudbuild1::{CloudBuild, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudBuild::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `builds_approve(...)`, `builds_cancel(...)`, `builds_create(...)`, `builds_get(...)`, `builds_list(...)`, `builds_retry(...)`, `github_enterprise_configs_create(...)`, `github_enterprise_configs_delete(...)`, `github_enterprise_configs_get(...)`, `github_enterprise_configs_list(...)`, `github_enterprise_configs_patch(...)`, `locations_bitbucket_server_configs_connected_repositories_batch_create(...)`, `locations_bitbucket_server_configs_create(...)`, `locations_bitbucket_server_configs_delete(...)`, `locations_bitbucket_server_configs_get(...)`, `locations_bitbucket_server_configs_list(...)`, `locations_bitbucket_server_configs_patch(...)`, `locations_bitbucket_server_configs_remove_bitbucket_server_connected_repository(...)`, `locations_bitbucket_server_configs_repos_list(...)`, `locations_builds_approve(...)`, `locations_builds_cancel(...)`, `locations_builds_create(...)`, `locations_builds_get(...)`, `locations_builds_list(...)`, `locations_builds_retry(...)`, `locations_git_lab_configs_connected_repositories_batch_create(...)`, `locations_git_lab_configs_create(...)`, `locations_git_lab_configs_delete(...)`, `locations_git_lab_configs_get(...)`, `locations_git_lab_configs_list(...)`, `locations_git_lab_configs_patch(...)`, `locations_git_lab_configs_remove_git_lab_connected_repository(...)`, `locations_git_lab_configs_repos_list(...)`, `locations_github_enterprise_configs_create(...)`, `locations_github_enterprise_configs_delete(...)`, `locations_github_enterprise_configs_get(...)`, `locations_github_enterprise_configs_list(...)`, `locations_github_enterprise_configs_patch(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_triggers_create(...)`, `locations_triggers_delete(...)`, `locations_triggers_get(...)`, `locations_triggers_list(...)`, `locations_triggers_patch(...)`, `locations_triggers_run(...)`, `locations_triggers_webhook(...)`, `locations_worker_pools_create(...)`, `locations_worker_pools_delete(...)`, `locations_worker_pools_get(...)`, `locations_worker_pools_list(...)`, `locations_worker_pools_patch(...)`, `triggers_create(...)`, `triggers_delete(...)`, `triggers_get(...)`, `triggers_list(...)`, `triggers_patch(...)`, `triggers_run(...)` and `triggers_webhook(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudBuild<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Approves or rejects a pending build. If approved, the returned LRO will be analogous to the LRO returned from a CreateBuild call. If rejected, the returned LRO will be immediately done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the target build. For example: "projects/{$project_id}/builds/{$build_id}"
    pub fn builds_approve(&self, request: ApproveBuildRequest, name: &str) -> ProjectBuildApproveCall<'a, S> {
        ProjectBuildApproveCall {
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
    /// Cancels a build in progress.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. ID of the project.
    /// * `id` - Required. ID of the build.
    pub fn builds_cancel(&self, request: CancelBuildRequest, project_id: &str, id: &str) -> ProjectBuildCancelCall<'a, S> {
        ProjectBuildCancelCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. ID of the project.
    pub fn builds_create(&self, request: Build, project_id: &str) -> ProjectBuildCreateCall<'a, S> {
        ProjectBuildCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the project.
    /// * `id` - Required. ID of the build.
    pub fn builds_get(&self, project_id: &str, id: &str) -> ProjectBuildGetCall<'a, S> {
        ProjectBuildGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _id: id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the project.
    pub fn builds_list(&self, project_id: &str) -> ProjectBuildListCall<'a, S> {
        ProjectBuildListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _parent: Default::default(),
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
    /// Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Google Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket's lifecycle management settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. ID of the project.
    /// * `id` - Required. Build ID of the original build.
    pub fn builds_retry(&self, request: RetryBuildRequest, project_id: &str, id: &str) -> ProjectBuildRetryCall<'a, S> {
        ProjectBuildRetryCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create an association between a GCP project and a GitHub Enterprise server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}
    pub fn github_enterprise_configs_create(&self, request: GitHubEnterpriseConfig, parent: &str) -> ProjectGithubEnterpriseConfigCreateCall<'a, S> {
        ProjectGithubEnterpriseConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _project_id: Default::default(),
            _ghe_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an association between a GCP project and a GitHub Enterprise server.
    /// 
    /// # Arguments
    ///
    /// * `name` - This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    pub fn github_enterprise_configs_delete(&self, name: &str) -> ProjectGithubEnterpriseConfigDeleteCall<'a, S> {
        ProjectGithubEnterpriseConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _project_id: Default::default(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a GitHubEnterpriseConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    pub fn github_enterprise_configs_get(&self, name: &str) -> ProjectGithubEnterpriseConfigGetCall<'a, S> {
        ProjectGithubEnterpriseConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _project_id: Default::default(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all GitHubEnterpriseConfigs for a given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}
    pub fn github_enterprise_configs_list(&self, parent: &str) -> ProjectGithubEnterpriseConfigListCall<'a, S> {
        ProjectGithubEnterpriseConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an association between a GCP project and a GitHub Enterprise server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The full resource name for the GitHubEnterpriseConfig For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    pub fn github_enterprise_configs_patch(&self, request: GitHubEnterpriseConfig, name: &str) -> ProjectGithubEnterpriseConfigPatchCall<'a, S> {
        ProjectGithubEnterpriseConfigPatchCall {
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
    /// Batch connecting Bitbucket Server repositories to Cloud Build.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the `BitbucketServerConfig` that added connected repository. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{config}`
    pub fn locations_bitbucket_server_configs_connected_repositories_batch_create(&self, request: BatchCreateBitbucketServerConnectedRepositoriesRequest, parent: &str) -> ProjectLocationBitbucketServerConfigConnectedRepositoryBatchCreateCall<'a, S> {
        ProjectLocationBitbucketServerConfigConnectedRepositoryBatchCreateCall {
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
    /// List all repositories for a given `BitbucketServerConfig`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent resource.
    pub fn locations_bitbucket_server_configs_repos_list(&self, parent: &str) -> ProjectLocationBitbucketServerConfigRepoListCall<'a, S> {
        ProjectLocationBitbucketServerConfigRepoListCall {
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
    /// Creates a new `BitbucketServerConfig`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the parent resource.
    pub fn locations_bitbucket_server_configs_create(&self, request: BitbucketServerConfig, parent: &str) -> ProjectLocationBitbucketServerConfigCreateCall<'a, S> {
        ProjectLocationBitbucketServerConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _bitbucket_server_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a `BitbucketServerConfig`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The config resource name.
    pub fn locations_bitbucket_server_configs_delete(&self, name: &str) -> ProjectLocationBitbucketServerConfigDeleteCall<'a, S> {
        ProjectLocationBitbucketServerConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a `BitbucketServerConfig`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The config resource name.
    pub fn locations_bitbucket_server_configs_get(&self, name: &str) -> ProjectLocationBitbucketServerConfigGetCall<'a, S> {
        ProjectLocationBitbucketServerConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all `BitbucketServerConfigs` for a given project. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent resource.
    pub fn locations_bitbucket_server_configs_list(&self, parent: &str) -> ProjectLocationBitbucketServerConfigListCall<'a, S> {
        ProjectLocationBitbucketServerConfigListCall {
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
    /// Updates an existing `BitbucketServerConfig`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name for the config.
    pub fn locations_bitbucket_server_configs_patch(&self, request: BitbucketServerConfig, name: &str) -> ProjectLocationBitbucketServerConfigPatchCall<'a, S> {
        ProjectLocationBitbucketServerConfigPatchCall {
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
    /// Remove a Bitbucket Server repository from a given BitbucketServerConfig's connected repositories. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `config` - Required. The name of the `BitbucketServerConfig` to remove a connected repository. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{config}`
    pub fn locations_bitbucket_server_configs_remove_bitbucket_server_connected_repository(&self, request: RemoveBitbucketServerConnectedRepositoryRequest, config: &str) -> ProjectLocationBitbucketServerConfigRemoveBitbucketServerConnectedRepositoryCall<'a, S> {
        ProjectLocationBitbucketServerConfigRemoveBitbucketServerConnectedRepositoryCall {
            hub: self.hub,
            _request: request,
            _config: config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Approves or rejects a pending build. If approved, the returned LRO will be analogous to the LRO returned from a CreateBuild call. If rejected, the returned LRO will be immediately done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the target build. For example: "projects/{$project_id}/builds/{$build_id}"
    pub fn locations_builds_approve(&self, request: ApproveBuildRequest, name: &str) -> ProjectLocationBuildApproveCall<'a, S> {
        ProjectLocationBuildApproveCall {
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
    /// Cancels a build in progress.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the `Build` to cancel. Format: `projects/{project}/locations/{location}/builds/{build}`
    pub fn locations_builds_cancel(&self, request: CancelBuildRequest, name: &str) -> ProjectLocationBuildCancelCall<'a, S> {
        ProjectLocationBuildCancelCall {
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
    /// Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The parent resource where this build will be created. Format: `projects/{project}/locations/{location}`
    pub fn locations_builds_create(&self, request: Build, parent: &str) -> ProjectLocationBuildCreateCall<'a, S> {
        ProjectLocationBuildCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the `Build` to retrieve. Format: `projects/{project}/locations/{location}/builds/{build}`
    pub fn locations_builds_get(&self, name: &str) -> ProjectLocationBuildGetCall<'a, S> {
        ProjectLocationBuildGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _project_id: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent of the collection of `Builds`. Format: `projects/{project}/locations/{location}`
    pub fn locations_builds_list(&self, parent: &str) -> ProjectLocationBuildListCall<'a, S> {
        ProjectLocationBuildListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _project_id: Default::default(),
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
    /// Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Google Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket's lifecycle management settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the `Build` to retry. Format: `projects/{project}/locations/{location}/builds/{build}`
    pub fn locations_builds_retry(&self, request: RetryBuildRequest, name: &str) -> ProjectLocationBuildRetryCall<'a, S> {
        ProjectLocationBuildRetryCall {
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
    /// Batch connecting GitLab repositories to Cloud Build. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the `GitLabConfig` that adds connected repositories. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}`
    pub fn locations_git_lab_configs_connected_repositories_batch_create(&self, request: BatchCreateGitLabConnectedRepositoriesRequest, parent: &str) -> ProjectLocationGitLabConfigConnectedRepositoryBatchCreateCall<'a, S> {
        ProjectLocationGitLabConfigConnectedRepositoryBatchCreateCall {
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
    /// List all repositories for a given `GitLabConfig`. This API is experimental
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent resource.
    pub fn locations_git_lab_configs_repos_list(&self, parent: &str) -> ProjectLocationGitLabConfigRepoListCall<'a, S> {
        ProjectLocationGitLabConfigRepoListCall {
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
    /// Creates a new `GitLabConfig`. This API is experimental
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the parent resource.
    pub fn locations_git_lab_configs_create(&self, request: GitLabConfig, parent: &str) -> ProjectLocationGitLabConfigCreateCall<'a, S> {
        ProjectLocationGitLabConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _gitlab_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a `GitLabConfig`. This API is experimental
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The config resource name.
    pub fn locations_git_lab_configs_delete(&self, name: &str) -> ProjectLocationGitLabConfigDeleteCall<'a, S> {
        ProjectLocationGitLabConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a `GitLabConfig`. This API is experimental
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The config resource name.
    pub fn locations_git_lab_configs_get(&self, name: &str) -> ProjectLocationGitLabConfigGetCall<'a, S> {
        ProjectLocationGitLabConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all `GitLabConfigs` for a given project. This API is experimental
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent resource
    pub fn locations_git_lab_configs_list(&self, parent: &str) -> ProjectLocationGitLabConfigListCall<'a, S> {
        ProjectLocationGitLabConfigListCall {
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
    /// Updates an existing `GitLabConfig`. This API is experimental
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name for the config.
    pub fn locations_git_lab_configs_patch(&self, request: GitLabConfig, name: &str) -> ProjectLocationGitLabConfigPatchCall<'a, S> {
        ProjectLocationGitLabConfigPatchCall {
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
    /// Remove a GitLab repository from a given GitLabConfig's connected repositories. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `config` - Required. The name of the `GitLabConfig` to remove a connected repository. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}`
    pub fn locations_git_lab_configs_remove_git_lab_connected_repository(&self, request: RemoveGitLabConnectedRepositoryRequest, config: &str) -> ProjectLocationGitLabConfigRemoveGitLabConnectedRepositoryCall<'a, S> {
        ProjectLocationGitLabConfigRemoveGitLabConnectedRepositoryCall {
            hub: self.hub,
            _request: request,
            _config: config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create an association between a GCP project and a GitHub Enterprise server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}
    pub fn locations_github_enterprise_configs_create(&self, request: GitHubEnterpriseConfig, parent: &str) -> ProjectLocationGithubEnterpriseConfigCreateCall<'a, S> {
        ProjectLocationGithubEnterpriseConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _project_id: Default::default(),
            _ghe_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an association between a GCP project and a GitHub Enterprise server.
    /// 
    /// # Arguments
    ///
    /// * `name` - This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    pub fn locations_github_enterprise_configs_delete(&self, name: &str) -> ProjectLocationGithubEnterpriseConfigDeleteCall<'a, S> {
        ProjectLocationGithubEnterpriseConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _project_id: Default::default(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a GitHubEnterpriseConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    pub fn locations_github_enterprise_configs_get(&self, name: &str) -> ProjectLocationGithubEnterpriseConfigGetCall<'a, S> {
        ProjectLocationGithubEnterpriseConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _project_id: Default::default(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all GitHubEnterpriseConfigs for a given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}
    pub fn locations_github_enterprise_configs_list(&self, parent: &str) -> ProjectLocationGithubEnterpriseConfigListCall<'a, S> {
        ProjectLocationGithubEnterpriseConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an association between a GCP project and a GitHub Enterprise server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The full resource name for the GitHubEnterpriseConfig For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    pub fn locations_github_enterprise_configs_patch(&self, request: GitHubEnterpriseConfig, name: &str) -> ProjectLocationGithubEnterpriseConfigPatchCall<'a, S> {
        ProjectLocationGithubEnterpriseConfigPatchCall {
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
    /// Creates a new `BuildTrigger`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The parent resource where this trigger will be created. Format: `projects/{project}/locations/{location}`
    pub fn locations_triggers_create(&self, request: BuildTrigger, parent: &str) -> ProjectLocationTriggerCreateCall<'a, S> {
        ProjectLocationTriggerCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a `BuildTrigger` by its project ID and trigger ID. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the `Trigger` to delete. Format: `projects/{project}/locations/{location}/triggers/{trigger}`
    pub fn locations_triggers_delete(&self, name: &str) -> ProjectLocationTriggerDeleteCall<'a, S> {
        ProjectLocationTriggerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _trigger_id: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a `BuildTrigger`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the `Trigger` to retrieve. Format: `projects/{project}/locations/{location}/triggers/{trigger}`
    pub fn locations_triggers_get(&self, name: &str) -> ProjectLocationTriggerGetCall<'a, S> {
        ProjectLocationTriggerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _trigger_id: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists existing `BuildTrigger`s. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent of the collection of `Triggers`. Format: `projects/{project}/locations/{location}`
    pub fn locations_triggers_list(&self, parent: &str) -> ProjectLocationTriggerListCall<'a, S> {
        ProjectLocationTriggerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _project_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a `BuildTrigger` by its project ID and trigger ID. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceName` - The `Trigger` name with format: `projects/{project}/locations/{location}/triggers/{trigger}`, where {trigger} is a unique identifier generated by the service.
    pub fn locations_triggers_patch(&self, request: BuildTrigger, resource_name: &str) -> ProjectLocationTriggerPatchCall<'a, S> {
        ProjectLocationTriggerPatchCall {
            hub: self.hub,
            _request: request,
            _resource_name: resource_name.to_string(),
            _trigger_id: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a `BuildTrigger` at a particular source revision. To run a regional or global trigger, use the POST request that includes the location endpoint in the path (ex. v1/projects/{projectId}/locations/{region}/triggers/{triggerId}:run). The POST request that does not include the location endpoint in the path can only be used when running global triggers.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the `Trigger` to run. Format: `projects/{project}/locations/{location}/triggers/{trigger}`
    pub fn locations_triggers_run(&self, request: RunBuildTriggerRequest, name: &str) -> ProjectLocationTriggerRunCall<'a, S> {
        ProjectLocationTriggerRunCall {
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
    /// ReceiveTriggerWebhook [Experimental] is called when the API receives a webhook request targeted at a specific trigger.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the `ReceiveTriggerWebhook` to retrieve. Format: `projects/{project}/locations/{location}/triggers/{trigger}`
    pub fn locations_triggers_webhook(&self, request: HttpBody, name: &str) -> ProjectLocationTriggerWebhookCall<'a, S> {
        ProjectLocationTriggerWebhookCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _trigger: Default::default(),
            _secret: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a `WorkerPool`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource where this worker pool will be created. Format: `projects/{project}/locations/{location}`.
    pub fn locations_worker_pools_create(&self, request: WorkerPool, parent: &str) -> ProjectLocationWorkerPoolCreateCall<'a, S> {
        ProjectLocationWorkerPoolCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _worker_pool_id: Default::default(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a `WorkerPool`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the `WorkerPool` to delete. Format: `projects/{project}/locations/{location}/workerPools/{workerPool}`.
    pub fn locations_worker_pools_delete(&self, name: &str) -> ProjectLocationWorkerPoolDeleteCall<'a, S> {
        ProjectLocationWorkerPoolDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _etag: Default::default(),
            _allow_missing: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns details of a `WorkerPool`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the `WorkerPool` to retrieve. Format: `projects/{project}/locations/{location}/workerPools/{workerPool}`.
    pub fn locations_worker_pools_get(&self, name: &str) -> ProjectLocationWorkerPoolGetCall<'a, S> {
        ProjectLocationWorkerPoolGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists `WorkerPool`s.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent of the collection of `WorkerPools`. Format: `projects/{project}/locations/{location}`.
    pub fn locations_worker_pools_list(&self, parent: &str) -> ProjectLocationWorkerPoolListCall<'a, S> {
        ProjectLocationWorkerPoolListCall {
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
    /// Updates a `WorkerPool`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the `WorkerPool`, with format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. The value of `{worker_pool}` is provided by `worker_pool_id` in `CreateWorkerPool` request and the value of `{location}` is determined by the endpoint accessed.
    pub fn locations_worker_pools_patch(&self, request: WorkerPool, name: &str) -> ProjectLocationWorkerPoolPatchCall<'a, S> {
        ProjectLocationWorkerPoolPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new `BuildTrigger`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. ID of the project for which to configure automatic builds.
    pub fn triggers_create(&self, request: BuildTrigger, project_id: &str) -> ProjectTriggerCreateCall<'a, S> {
        ProjectTriggerCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a `BuildTrigger` by its project ID and trigger ID. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the project that owns the trigger.
    /// * `triggerId` - Required. ID of the `BuildTrigger` to delete.
    pub fn triggers_delete(&self, project_id: &str, trigger_id: &str) -> ProjectTriggerDeleteCall<'a, S> {
        ProjectTriggerDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _trigger_id: trigger_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a `BuildTrigger`. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the project that owns the trigger.
    /// * `triggerId` - Required. Identifier (`id` or `name`) of the `BuildTrigger` to get.
    pub fn triggers_get(&self, project_id: &str, trigger_id: &str) -> ProjectTriggerGetCall<'a, S> {
        ProjectTriggerGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _trigger_id: trigger_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists existing `BuildTrigger`s. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the project for which to list BuildTriggers.
    pub fn triggers_list(&self, project_id: &str) -> ProjectTriggerListCall<'a, S> {
        ProjectTriggerListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _parent: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a `BuildTrigger` by its project ID and trigger ID. This API is experimental.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. ID of the project that owns the trigger.
    /// * `triggerId` - Required. ID of the `BuildTrigger` to update.
    pub fn triggers_patch(&self, request: BuildTrigger, project_id: &str, trigger_id: &str) -> ProjectTriggerPatchCall<'a, S> {
        ProjectTriggerPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _trigger_id: trigger_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a `BuildTrigger` at a particular source revision. To run a regional or global trigger, use the POST request that includes the location endpoint in the path (ex. v1/projects/{projectId}/locations/{region}/triggers/{triggerId}:run). The POST request that does not include the location endpoint in the path can only be used when running global triggers.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. ID of the project.
    /// * `triggerId` - Required. ID of the trigger.
    pub fn triggers_run(&self, request: RepoSource, project_id: &str, trigger_id: &str) -> ProjectTriggerRunCall<'a, S> {
        ProjectTriggerRunCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _trigger_id: trigger_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// ReceiveTriggerWebhook [Experimental] is called when the API receives a webhook request targeted at a specific trigger.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project in which the specified trigger lives
    /// * `trigger` - Name of the trigger to run the payload against
    pub fn triggers_webhook(&self, request: HttpBody, project_id: &str, trigger: &str) -> ProjectTriggerWebhookCall<'a, S> {
        ProjectTriggerWebhookCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _trigger: trigger.to_string(),
            _secret: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`CloudBuild`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudbuild1 as cloudbuild1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudbuild1::{CloudBuild, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudBuild::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `webhook(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudBuild<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// ReceiveWebhook is called when the API receives a GitHub webhook.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn webhook(&self, request: HttpBody) -> MethodWebhookCall<'a, S> {
        MethodWebhookCall {
            hub: self.hub,
            _request: request,
            _webhook_key: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



