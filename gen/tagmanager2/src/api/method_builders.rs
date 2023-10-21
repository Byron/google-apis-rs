use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`TagManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_tagmanager2 as tagmanager2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use tagmanager2::{TagManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = TagManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `containers_combine(...)`, `containers_create(...)`, `containers_delete(...)`, `containers_destinations_get(...)`, `containers_destinations_link(...)`, `containers_destinations_list(...)`, `containers_environments_create(...)`, `containers_environments_delete(...)`, `containers_environments_get(...)`, `containers_environments_list(...)`, `containers_environments_reauthorize(...)`, `containers_environments_update(...)`, `containers_get(...)`, `containers_list(...)`, `containers_lookup(...)`, `containers_move_tag_id(...)`, `containers_snippet(...)`, `containers_update(...)`, `containers_version_headers_latest(...)`, `containers_version_headers_list(...)`, `containers_versions_delete(...)`, `containers_versions_get(...)`, `containers_versions_live(...)`, `containers_versions_publish(...)`, `containers_versions_set_latest(...)`, `containers_versions_undelete(...)`, `containers_versions_update(...)`, `containers_workspaces_built_in_variables_create(...)`, `containers_workspaces_built_in_variables_delete(...)`, `containers_workspaces_built_in_variables_list(...)`, `containers_workspaces_built_in_variables_revert(...)`, `containers_workspaces_clients_create(...)`, `containers_workspaces_clients_delete(...)`, `containers_workspaces_clients_get(...)`, `containers_workspaces_clients_list(...)`, `containers_workspaces_clients_revert(...)`, `containers_workspaces_clients_update(...)`, `containers_workspaces_create(...)`, `containers_workspaces_create_version(...)`, `containers_workspaces_delete(...)`, `containers_workspaces_folders_create(...)`, `containers_workspaces_folders_delete(...)`, `containers_workspaces_folders_entities(...)`, `containers_workspaces_folders_get(...)`, `containers_workspaces_folders_list(...)`, `containers_workspaces_folders_move_entities_to_folder(...)`, `containers_workspaces_folders_revert(...)`, `containers_workspaces_folders_update(...)`, `containers_workspaces_get(...)`, `containers_workspaces_get_status(...)`, `containers_workspaces_gtag_config_create(...)`, `containers_workspaces_gtag_config_delete(...)`, `containers_workspaces_gtag_config_get(...)`, `containers_workspaces_gtag_config_list(...)`, `containers_workspaces_gtag_config_update(...)`, `containers_workspaces_list(...)`, `containers_workspaces_quick_preview(...)`, `containers_workspaces_resolve_conflict(...)`, `containers_workspaces_sync(...)`, `containers_workspaces_tags_create(...)`, `containers_workspaces_tags_delete(...)`, `containers_workspaces_tags_get(...)`, `containers_workspaces_tags_list(...)`, `containers_workspaces_tags_revert(...)`, `containers_workspaces_tags_update(...)`, `containers_workspaces_templates_create(...)`, `containers_workspaces_templates_delete(...)`, `containers_workspaces_templates_get(...)`, `containers_workspaces_templates_list(...)`, `containers_workspaces_templates_revert(...)`, `containers_workspaces_templates_update(...)`, `containers_workspaces_triggers_create(...)`, `containers_workspaces_triggers_delete(...)`, `containers_workspaces_triggers_get(...)`, `containers_workspaces_triggers_list(...)`, `containers_workspaces_triggers_revert(...)`, `containers_workspaces_triggers_update(...)`, `containers_workspaces_update(...)`, `containers_workspaces_variables_create(...)`, `containers_workspaces_variables_delete(...)`, `containers_workspaces_variables_get(...)`, `containers_workspaces_variables_list(...)`, `containers_workspaces_variables_revert(...)`, `containers_workspaces_variables_update(...)`, `containers_workspaces_zones_create(...)`, `containers_workspaces_zones_delete(...)`, `containers_workspaces_zones_get(...)`, `containers_workspaces_zones_list(...)`, `containers_workspaces_zones_revert(...)`, `containers_workspaces_zones_update(...)`, `get(...)`, `list(...)`, `update(...)`, `user_permissions_create(...)`, `user_permissions_delete(...)`, `user_permissions_get(...)`, `user_permissions_list(...)` and `user_permissions_update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a TagManager<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Destination.
    /// 
    /// # Arguments
    ///
    /// * `path` - Google Tag Destination's API relative path. Example: accounts/{account_id}/containers/{container_id}/destinations/{destination_link_id}
    pub fn containers_destinations_get(&self, path: &str) -> AccountContainerDestinationGetCall<'a, S> {
        AccountContainerDestinationGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a Destination to this Container and removes it from the Container to which it is currently linked.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_destinations_link(&self, parent: &str) -> AccountContainerDestinationLinkCall<'a, S> {
        AccountContainerDestinationLinkCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _destination_id: Default::default(),
            _allow_user_permission_feature_update: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Destinations linked to a GTM Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_destinations_list(&self, parent: &str) -> AccountContainerDestinationListCall<'a, S> {
        AccountContainerDestinationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_environments_create(&self, request: Environment, parent: &str) -> AccountContainerEnvironmentCreateCall<'a, S> {
        AccountContainerEnvironmentCreateCall {
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
    /// Deletes a GTM Environment.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}
    pub fn containers_environments_delete(&self, path: &str) -> AccountContainerEnvironmentDeleteCall<'a, S> {
        AccountContainerEnvironmentDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Environment.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}
    pub fn containers_environments_get(&self, path: &str) -> AccountContainerEnvironmentGetCall<'a, S> {
        AccountContainerEnvironmentGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Environments of a GTM Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_environments_list(&self, parent: &str) -> AccountContainerEnvironmentListCall<'a, S> {
        AccountContainerEnvironmentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Re-generates the authorization code for a GTM Environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}
    pub fn containers_environments_reauthorize(&self, request: Environment, path: &str) -> AccountContainerEnvironmentReauthorizeCall<'a, S> {
        AccountContainerEnvironmentReauthorizeCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Environment's API relative path. Example: accounts/{account_id}/containers/{container_id}/environments/{environment_id}
    pub fn containers_environments_update(&self, request: Environment, path: &str) -> AccountContainerEnvironmentUpdateCall<'a, S> {
        AccountContainerEnvironmentUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest container version header
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_version_headers_latest(&self, parent: &str) -> AccountContainerVersionHeaderLatestCall<'a, S> {
        AccountContainerVersionHeaderLatestCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Container Versions of a GTM Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_version_headers_list(&self, parent: &str) -> AccountContainerVersionHeaderListCall<'a, S> {
        AccountContainerVersionHeaderListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Container Version.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}
    pub fn containers_versions_delete(&self, path: &str) -> AccountContainerVersionDeleteCall<'a, S> {
        AccountContainerVersionDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Container Version.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}
    pub fn containers_versions_get(&self, path: &str) -> AccountContainerVersionGetCall<'a, S> {
        AccountContainerVersionGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _container_version_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the live (i.e. published) container version
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_versions_live(&self, parent: &str) -> AccountContainerVersionLiveCall<'a, S> {
        AccountContainerVersionLiveCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Publishes a Container Version.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}
    pub fn containers_versions_publish(&self, path: &str) -> AccountContainerVersionPublishCall<'a, S> {
        AccountContainerVersionPublishCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the latest version used for synchronization of workspaces when detecting conflicts and errors.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}
    pub fn containers_versions_set_latest(&self, path: &str) -> AccountContainerVersionSetLatestCall<'a, S> {
        AccountContainerVersionSetLatestCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a Container Version.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}
    pub fn containers_versions_undelete(&self, path: &str) -> AccountContainerVersionUndeleteCall<'a, S> {
        AccountContainerVersionUndeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Container Version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM ContainerVersion's API relative path. Example: accounts/{account_id}/containers/{container_id}/versions/{version_id}
    pub fn containers_versions_update(&self, request: ContainerVersion, path: &str) -> AccountContainerVersionUpdateCall<'a, S> {
        AccountContainerVersionUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates one or more GTM Built-In Variables.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_built_in_variables_create(&self, parent: &str) -> AccountContainerWorkspaceBuiltInVariableCreateCall<'a, S> {
        AccountContainerWorkspaceBuiltInVariableCreateCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes one or more GTM Built-In Variables.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM BuiltInVariable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/built_in_variables
    pub fn containers_workspaces_built_in_variables_delete(&self, path: &str) -> AccountContainerWorkspaceBuiltInVariableDeleteCall<'a, S> {
        AccountContainerWorkspaceBuiltInVariableDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _type_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the enabled Built-In Variables of a GTM Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_built_in_variables_list(&self, parent: &str) -> AccountContainerWorkspaceBuiltInVariableListCall<'a, S> {
        AccountContainerWorkspaceBuiltInVariableListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Built-In Variables in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM BuiltInVariable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/built_in_variables
    pub fn containers_workspaces_built_in_variables_revert(&self, path: &str) -> AccountContainerWorkspaceBuiltInVariableRevertCall<'a, S> {
        AccountContainerWorkspaceBuiltInVariableRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _type_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Client.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_clients_create(&self, request: Client, parent: &str) -> AccountContainerWorkspaceClientCreateCall<'a, S> {
        AccountContainerWorkspaceClientCreateCall {
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
    /// Deletes a GTM Client.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}
    pub fn containers_workspaces_clients_delete(&self, path: &str) -> AccountContainerWorkspaceClientDeleteCall<'a, S> {
        AccountContainerWorkspaceClientDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Client.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}
    pub fn containers_workspaces_clients_get(&self, path: &str) -> AccountContainerWorkspaceClientGetCall<'a, S> {
        AccountContainerWorkspaceClientGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Clients of a GTM container workspace.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_clients_list(&self, parent: &str) -> AccountContainerWorkspaceClientListCall<'a, S> {
        AccountContainerWorkspaceClientListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Client in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}
    pub fn containers_workspaces_clients_revert(&self, path: &str) -> AccountContainerWorkspaceClientRevertCall<'a, S> {
        AccountContainerWorkspaceClientRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Client.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Client's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/clients/{client_id}
    pub fn containers_workspaces_clients_update(&self, request: Client, path: &str) -> AccountContainerWorkspaceClientUpdateCall<'a, S> {
        AccountContainerWorkspaceClientUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Folder.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_folders_create(&self, request: Folder, parent: &str) -> AccountContainerWorkspaceFolderCreateCall<'a, S> {
        AccountContainerWorkspaceFolderCreateCall {
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
    /// Deletes a GTM Folder.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}
    pub fn containers_workspaces_folders_delete(&self, path: &str) -> AccountContainerWorkspaceFolderDeleteCall<'a, S> {
        AccountContainerWorkspaceFolderDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all entities in a GTM Folder.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}
    pub fn containers_workspaces_folders_entities(&self, path: &str) -> AccountContainerWorkspaceFolderEntityCall<'a, S> {
        AccountContainerWorkspaceFolderEntityCall {
            hub: self.hub,
            _path: path.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Folder.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}
    pub fn containers_workspaces_folders_get(&self, path: &str) -> AccountContainerWorkspaceFolderGetCall<'a, S> {
        AccountContainerWorkspaceFolderGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Folders of a Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_folders_list(&self, parent: &str) -> AccountContainerWorkspaceFolderListCall<'a, S> {
        AccountContainerWorkspaceFolderListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves entities to a GTM Folder.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}
    pub fn containers_workspaces_folders_move_entities_to_folder(&self, request: Folder, path: &str) -> AccountContainerWorkspaceFolderMoveEntitiesToFolderCall<'a, S> {
        AccountContainerWorkspaceFolderMoveEntitiesToFolderCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _variable_id: Default::default(),
            _trigger_id: Default::default(),
            _tag_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Folder in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}
    pub fn containers_workspaces_folders_revert(&self, path: &str) -> AccountContainerWorkspaceFolderRevertCall<'a, S> {
        AccountContainerWorkspaceFolderRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Folder.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Folder's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/folders/{folder_id}
    pub fn containers_workspaces_folders_update(&self, request: Folder, path: &str) -> AccountContainerWorkspaceFolderUpdateCall<'a, S> {
        AccountContainerWorkspaceFolderUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Google tag config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_gtag_config_create(&self, request: GtagConfig, parent: &str) -> AccountContainerWorkspaceGtagConfigCreateCall<'a, S> {
        AccountContainerWorkspaceGtagConfigCreateCall {
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
    /// Deletes a Google tag config.
    /// 
    /// # Arguments
    ///
    /// * `path` - Google tag config's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/gtag_config/{gtag_config_id}
    pub fn containers_workspaces_gtag_config_delete(&self, path: &str) -> AccountContainerWorkspaceGtagConfigDeleteCall<'a, S> {
        AccountContainerWorkspaceGtagConfigDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Google tag config.
    /// 
    /// # Arguments
    ///
    /// * `path` - Google tag config's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/gtag_config/{gtag_config_id}
    pub fn containers_workspaces_gtag_config_get(&self, path: &str) -> AccountContainerWorkspaceGtagConfigGetCall<'a, S> {
        AccountContainerWorkspaceGtagConfigGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Google tag configs in a Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_gtag_config_list(&self, parent: &str) -> AccountContainerWorkspaceGtagConfigListCall<'a, S> {
        AccountContainerWorkspaceGtagConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Google tag config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - Google tag config's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/gtag_config/{gtag_config_id}
    pub fn containers_workspaces_gtag_config_update(&self, request: GtagConfig, path: &str) -> AccountContainerWorkspaceGtagConfigUpdateCall<'a, S> {
        AccountContainerWorkspaceGtagConfigUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_tags_create(&self, request: Tag, parent: &str) -> AccountContainerWorkspaceTagCreateCall<'a, S> {
        AccountContainerWorkspaceTagCreateCall {
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
    /// Deletes a GTM Tag.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}
    pub fn containers_workspaces_tags_delete(&self, path: &str) -> AccountContainerWorkspaceTagDeleteCall<'a, S> {
        AccountContainerWorkspaceTagDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Tag.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}
    pub fn containers_workspaces_tags_get(&self, path: &str) -> AccountContainerWorkspaceTagGetCall<'a, S> {
        AccountContainerWorkspaceTagGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Tags of a Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_tags_list(&self, parent: &str) -> AccountContainerWorkspaceTagListCall<'a, S> {
        AccountContainerWorkspaceTagListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Tag in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}
    pub fn containers_workspaces_tags_revert(&self, path: &str) -> AccountContainerWorkspaceTagRevertCall<'a, S> {
        AccountContainerWorkspaceTagRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Tag's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/tags/{tag_id}
    pub fn containers_workspaces_tags_update(&self, request: Tag, path: &str) -> AccountContainerWorkspaceTagUpdateCall<'a, S> {
        AccountContainerWorkspaceTagUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Custom Template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_templates_create(&self, request: CustomTemplate, parent: &str) -> AccountContainerWorkspaceTemplateCreateCall<'a, S> {
        AccountContainerWorkspaceTemplateCreateCall {
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
    /// Deletes a GTM Template.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}
    pub fn containers_workspaces_templates_delete(&self, path: &str) -> AccountContainerWorkspaceTemplateDeleteCall<'a, S> {
        AccountContainerWorkspaceTemplateDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Template.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}
    pub fn containers_workspaces_templates_get(&self, path: &str) -> AccountContainerWorkspaceTemplateGetCall<'a, S> {
        AccountContainerWorkspaceTemplateGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Templates of a GTM container workspace.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_templates_list(&self, parent: &str) -> AccountContainerWorkspaceTemplateListCall<'a, S> {
        AccountContainerWorkspaceTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Template in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}
    pub fn containers_workspaces_templates_revert(&self, path: &str) -> AccountContainerWorkspaceTemplateRevertCall<'a, S> {
        AccountContainerWorkspaceTemplateRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Custom Template's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/templates/{template_id}
    pub fn containers_workspaces_templates_update(&self, request: CustomTemplate, path: &str) -> AccountContainerWorkspaceTemplateUpdateCall<'a, S> {
        AccountContainerWorkspaceTemplateUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Trigger.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_triggers_create(&self, request: Trigger, parent: &str) -> AccountContainerWorkspaceTriggerCreateCall<'a, S> {
        AccountContainerWorkspaceTriggerCreateCall {
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
    /// Deletes a GTM Trigger.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}
    pub fn containers_workspaces_triggers_delete(&self, path: &str) -> AccountContainerWorkspaceTriggerDeleteCall<'a, S> {
        AccountContainerWorkspaceTriggerDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Trigger.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}
    pub fn containers_workspaces_triggers_get(&self, path: &str) -> AccountContainerWorkspaceTriggerGetCall<'a, S> {
        AccountContainerWorkspaceTriggerGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Triggers of a Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_triggers_list(&self, parent: &str) -> AccountContainerWorkspaceTriggerListCall<'a, S> {
        AccountContainerWorkspaceTriggerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Trigger in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}
    pub fn containers_workspaces_triggers_revert(&self, path: &str) -> AccountContainerWorkspaceTriggerRevertCall<'a, S> {
        AccountContainerWorkspaceTriggerRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Trigger.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Trigger's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/triggers/{trigger_id}
    pub fn containers_workspaces_triggers_update(&self, request: Trigger, path: &str) -> AccountContainerWorkspaceTriggerUpdateCall<'a, S> {
        AccountContainerWorkspaceTriggerUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Variable.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_variables_create(&self, request: Variable, parent: &str) -> AccountContainerWorkspaceVariableCreateCall<'a, S> {
        AccountContainerWorkspaceVariableCreateCall {
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
    /// Deletes a GTM Variable.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}
    pub fn containers_workspaces_variables_delete(&self, path: &str) -> AccountContainerWorkspaceVariableDeleteCall<'a, S> {
        AccountContainerWorkspaceVariableDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Variable.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}
    pub fn containers_workspaces_variables_get(&self, path: &str) -> AccountContainerWorkspaceVariableGetCall<'a, S> {
        AccountContainerWorkspaceVariableGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Variables of a Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_variables_list(&self, parent: &str) -> AccountContainerWorkspaceVariableListCall<'a, S> {
        AccountContainerWorkspaceVariableListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Variable in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}
    pub fn containers_workspaces_variables_revert(&self, path: &str) -> AccountContainerWorkspaceVariableRevertCall<'a, S> {
        AccountContainerWorkspaceVariableRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Variable.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Variable's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/variables/{variable_id}
    pub fn containers_workspaces_variables_update(&self, request: Variable, path: &str) -> AccountContainerWorkspaceVariableUpdateCall<'a, S> {
        AccountContainerWorkspaceVariableUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Zone.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_zones_create(&self, request: Zone, parent: &str) -> AccountContainerWorkspaceZoneCreateCall<'a, S> {
        AccountContainerWorkspaceZoneCreateCall {
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
    /// Deletes a GTM Zone.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}
    pub fn containers_workspaces_zones_delete(&self, path: &str) -> AccountContainerWorkspaceZoneDeleteCall<'a, S> {
        AccountContainerWorkspaceZoneDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Zone.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}
    pub fn containers_workspaces_zones_get(&self, path: &str) -> AccountContainerWorkspaceZoneGetCall<'a, S> {
        AccountContainerWorkspaceZoneGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Zones of a GTM container workspace.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_zones_list(&self, parent: &str) -> AccountContainerWorkspaceZoneListCall<'a, S> {
        AccountContainerWorkspaceZoneListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts changes to a GTM Zone in a GTM Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}
    pub fn containers_workspaces_zones_revert(&self, path: &str) -> AccountContainerWorkspaceZoneRevertCall<'a, S> {
        AccountContainerWorkspaceZoneRevertCall {
            hub: self.hub,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Zone.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Zone's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}/zones/{zone_id}
    pub fn containers_workspaces_zones_update(&self, request: Zone, path: &str) -> AccountContainerWorkspaceZoneUpdateCall<'a, S> {
        AccountContainerWorkspaceZoneUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Workspace.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_workspaces_create(&self, request: Workspace, parent: &str) -> AccountContainerWorkspaceCreateCall<'a, S> {
        AccountContainerWorkspaceCreateCall {
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
    /// Creates a Container Version from the entities present in the workspace, deletes the workspace, and sets the base container version to the newly created version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_create_version(&self, request: CreateContainerVersionRequestVersionOptions, path: &str) -> AccountContainerWorkspaceCreateVersionCall<'a, S> {
        AccountContainerWorkspaceCreateVersionCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_delete(&self, path: &str) -> AccountContainerWorkspaceDeleteCall<'a, S> {
        AccountContainerWorkspaceDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_get(&self, path: &str) -> AccountContainerWorkspaceGetCall<'a, S> {
        AccountContainerWorkspaceGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds conflicting and modified entities in the workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_get_status(&self, path: &str) -> AccountContainerWorkspaceGetStatuCall<'a, S> {
        AccountContainerWorkspaceGetStatuCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Workspaces that belong to a GTM Container.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM parent Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_workspaces_list(&self, parent: &str) -> AccountContainerWorkspaceListCall<'a, S> {
        AccountContainerWorkspaceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Quick previews a workspace by creating a fake container version from all entities in the provided workspace.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_quick_preview(&self, path: &str) -> AccountContainerWorkspaceQuickPreviewCall<'a, S> {
        AccountContainerWorkspaceQuickPreviewCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resolves a merge conflict for a workspace entity by updating it to the resolved entity passed in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_resolve_conflict(&self, request: Entity, path: &str) -> AccountContainerWorkspaceResolveConflictCall<'a, S> {
        AccountContainerWorkspaceResolveConflictCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Syncs a workspace to the latest container version by updating all unmodified workspace entities and displaying conflicts for modified entities.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_sync(&self, path: &str) -> AccountContainerWorkspaceSyncCall<'a, S> {
        AccountContainerWorkspaceSyncCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Workspace.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Workspace's API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
    pub fn containers_workspaces_update(&self, request: Workspace, path: &str) -> AccountContainerWorkspaceUpdateCall<'a, S> {
        AccountContainerWorkspaceUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Combines Containers.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_combine(&self, path: &str) -> AccountContainerCombineCall<'a, S> {
        AccountContainerCombineCall {
            hub: self.hub,
            _path: path.to_string(),
            _setting_source: Default::default(),
            _container_id: Default::default(),
            _allow_user_permission_feature_update: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Container.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Account's API relative path. Example: accounts/{account_id}.
    pub fn containers_create(&self, request: Container, parent: &str) -> AccountContainerCreateCall<'a, S> {
        AccountContainerCreateCall {
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
    /// Deletes a Container.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_delete(&self, path: &str) -> AccountContainerDeleteCall<'a, S> {
        AccountContainerDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Container.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_get(&self, path: &str) -> AccountContainerGetCall<'a, S> {
        AccountContainerGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Containers that belongs to a GTM Account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Account's API relative path. Example: accounts/{account_id}.
    pub fn containers_list(&self, parent: &str) -> AccountContainerListCall<'a, S> {
        AccountContainerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up a Container by destination ID.
    pub fn containers_lookup(&self) -> AccountContainerLookupCall<'a, S> {
        AccountContainerLookupCall {
            hub: self.hub,
            _destination_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Move Tag ID out of a Container.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_move_tag_id(&self, path: &str) -> AccountContainerMoveTagIdCall<'a, S> {
        AccountContainerMoveTagIdCall {
            hub: self.hub,
            _path: path.to_string(),
            _tag_name: Default::default(),
            _tag_id: Default::default(),
            _copy_users: Default::default(),
            _copy_terms_of_service: Default::default(),
            _copy_settings: Default::default(),
            _allow_user_permission_feature_update: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the tagging snippet for a Container.
    /// 
    /// # Arguments
    ///
    /// * `path` - Container snippet's API relative path. Example: accounts/{account_id}/containers/{container_id}:snippet
    pub fn containers_snippet(&self, path: &str) -> AccountContainerSnippetCall<'a, S> {
        AccountContainerSnippetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Container.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Container's API relative path. Example: accounts/{account_id}/containers/{container_id}
    pub fn containers_update(&self, request: Container, path: &str) -> AccountContainerUpdateCall<'a, S> {
        AccountContainerUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a user's Account & Container access.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - GTM Account's API relative path. Example: accounts/{account_id}
    pub fn user_permissions_create(&self, request: UserPermission, parent: &str) -> AccountUserPermissionCreateCall<'a, S> {
        AccountUserPermissionCreateCall {
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
    /// Removes a user from the account, revoking access to it and all of its containers.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM UserPermission's API relative path. Example: accounts/{account_id}/user_permissions/{user_permission_id}
    pub fn user_permissions_delete(&self, path: &str) -> AccountUserPermissionDeleteCall<'a, S> {
        AccountUserPermissionDeleteCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a user's Account & Container access.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM UserPermission's API relative path. Example: accounts/{account_id}/user_permissions/{user_permission_id}
    pub fn user_permissions_get(&self, path: &str) -> AccountUserPermissionGetCall<'a, S> {
        AccountUserPermissionGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all users that have access to the account along with Account and Container user access granted to each of them.
    /// 
    /// # Arguments
    ///
    /// * `parent` - GTM Account's API relative path. Example: accounts/{account_id}
    pub fn user_permissions_list(&self, parent: &str) -> AccountUserPermissionListCall<'a, S> {
        AccountUserPermissionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a user's Account & Container access.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM UserPermission's API relative path. Example: accounts/{account_id}/user_permissions/{user_permission_id}
    pub fn user_permissions_update(&self, request: UserPermission, path: &str) -> AccountUserPermissionUpdateCall<'a, S> {
        AccountUserPermissionUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Account.
    /// 
    /// # Arguments
    ///
    /// * `path` - GTM Account's API relative path. Example: accounts/{account_id}
    pub fn get(&self, path: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _path: path.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Accounts that a user has access to.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _include_google_tags: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `path` - GTM Account's API relative path. Example: accounts/{account_id}
    pub fn update(&self, request: Account, path: &str) -> AccountUpdateCall<'a, S> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request,
            _path: path.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



