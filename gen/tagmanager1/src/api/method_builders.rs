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
/// extern crate google_tagmanager1 as tagmanager1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use tagmanager1::{TagManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = TagManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `containers_create(...)`, `containers_delete(...)`, `containers_environments_create(...)`, `containers_environments_delete(...)`, `containers_environments_get(...)`, `containers_environments_list(...)`, `containers_environments_update(...)`, `containers_folders_create(...)`, `containers_folders_delete(...)`, `containers_folders_entities_list(...)`, `containers_folders_get(...)`, `containers_folders_list(...)`, `containers_folders_update(...)`, `containers_get(...)`, `containers_list(...)`, `containers_move_folders_update(...)`, `containers_reauthorize_environments_update(...)`, `containers_tags_create(...)`, `containers_tags_delete(...)`, `containers_tags_get(...)`, `containers_tags_list(...)`, `containers_tags_update(...)`, `containers_triggers_create(...)`, `containers_triggers_delete(...)`, `containers_triggers_get(...)`, `containers_triggers_list(...)`, `containers_triggers_update(...)`, `containers_update(...)`, `containers_variables_create(...)`, `containers_variables_delete(...)`, `containers_variables_get(...)`, `containers_variables_list(...)`, `containers_variables_update(...)`, `containers_versions_create(...)`, `containers_versions_delete(...)`, `containers_versions_get(...)`, `containers_versions_list(...)`, `containers_versions_publish(...)`, `containers_versions_restore(...)`, `containers_versions_undelete(...)`, `containers_versions_update(...)`, `get(...)`, `list(...)`, `permissions_create(...)`, `permissions_delete(...)`, `permissions_get(...)`, `permissions_list(...)`, `permissions_update(...)` and `update(...)`
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
    /// Creates a GTM Environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_environments_create(&self, request: Environment, account_id: &str, container_id: &str) -> AccountContainerEnvironmentCreateCall<'a, S> {
        AccountContainerEnvironmentCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `environmentId` - The GTM Environment ID.
    pub fn containers_environments_delete(&self, account_id: &str, container_id: &str, environment_id: &str) -> AccountContainerEnvironmentDeleteCall<'a, S> {
        AccountContainerEnvironmentDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _environment_id: environment_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `environmentId` - The GTM Environment ID.
    pub fn containers_environments_get(&self, account_id: &str, container_id: &str, environment_id: &str) -> AccountContainerEnvironmentGetCall<'a, S> {
        AccountContainerEnvironmentGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _environment_id: environment_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_environments_list(&self, account_id: &str, container_id: &str) -> AccountContainerEnvironmentListCall<'a, S> {
        AccountContainerEnvironmentListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `environmentId` - The GTM Environment ID.
    pub fn containers_environments_update(&self, request: Environment, account_id: &str, container_id: &str, environment_id: &str) -> AccountContainerEnvironmentUpdateCall<'a, S> {
        AccountContainerEnvironmentUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _environment_id: environment_id.to_string(),
            _fingerprint: Default::default(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `folderId` - The GTM Folder ID.
    pub fn containers_folders_entities_list(&self, account_id: &str, container_id: &str, folder_id: &str) -> AccountContainerFolderEntityListCall<'a, S> {
        AccountContainerFolderEntityListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _folder_id: folder_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_folders_create(&self, request: Folder, account_id: &str, container_id: &str) -> AccountContainerFolderCreateCall<'a, S> {
        AccountContainerFolderCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `folderId` - The GTM Folder ID.
    pub fn containers_folders_delete(&self, account_id: &str, container_id: &str, folder_id: &str) -> AccountContainerFolderDeleteCall<'a, S> {
        AccountContainerFolderDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _folder_id: folder_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `folderId` - The GTM Folder ID.
    pub fn containers_folders_get(&self, account_id: &str, container_id: &str, folder_id: &str) -> AccountContainerFolderGetCall<'a, S> {
        AccountContainerFolderGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _folder_id: folder_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_folders_list(&self, account_id: &str, container_id: &str) -> AccountContainerFolderListCall<'a, S> {
        AccountContainerFolderListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `folderId` - The GTM Folder ID.
    pub fn containers_folders_update(&self, request: Folder, account_id: &str, container_id: &str, folder_id: &str) -> AccountContainerFolderUpdateCall<'a, S> {
        AccountContainerFolderUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _folder_id: folder_id.to_string(),
            _fingerprint: Default::default(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `folderId` - The GTM Folder ID.
    pub fn containers_move_folders_update(&self, request: Folder, account_id: &str, container_id: &str, folder_id: &str) -> AccountContainerMoveFolderUpdateCall<'a, S> {
        AccountContainerMoveFolderUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _folder_id: folder_id.to_string(),
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
    /// Re-generates the authorization code for a GTM Environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `environmentId` - The GTM Environment ID.
    pub fn containers_reauthorize_environments_update(&self, request: Environment, account_id: &str, container_id: &str, environment_id: &str) -> AccountContainerReauthorizeEnvironmentUpdateCall<'a, S> {
        AccountContainerReauthorizeEnvironmentUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _environment_id: environment_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_tags_create(&self, request: Tag, account_id: &str, container_id: &str) -> AccountContainerTagCreateCall<'a, S> {
        AccountContainerTagCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `tagId` - The GTM Tag ID.
    pub fn containers_tags_delete(&self, account_id: &str, container_id: &str, tag_id: &str) -> AccountContainerTagDeleteCall<'a, S> {
        AccountContainerTagDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _tag_id: tag_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `tagId` - The GTM Tag ID.
    pub fn containers_tags_get(&self, account_id: &str, container_id: &str, tag_id: &str) -> AccountContainerTagGetCall<'a, S> {
        AccountContainerTagGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _tag_id: tag_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_tags_list(&self, account_id: &str, container_id: &str) -> AccountContainerTagListCall<'a, S> {
        AccountContainerTagListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `tagId` - The GTM Tag ID.
    pub fn containers_tags_update(&self, request: Tag, account_id: &str, container_id: &str, tag_id: &str) -> AccountContainerTagUpdateCall<'a, S> {
        AccountContainerTagUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _tag_id: tag_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_triggers_create(&self, request: Trigger, account_id: &str, container_id: &str) -> AccountContainerTriggerCreateCall<'a, S> {
        AccountContainerTriggerCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `triggerId` - The GTM Trigger ID.
    pub fn containers_triggers_delete(&self, account_id: &str, container_id: &str, trigger_id: &str) -> AccountContainerTriggerDeleteCall<'a, S> {
        AccountContainerTriggerDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _trigger_id: trigger_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `triggerId` - The GTM Trigger ID.
    pub fn containers_triggers_get(&self, account_id: &str, container_id: &str, trigger_id: &str) -> AccountContainerTriggerGetCall<'a, S> {
        AccountContainerTriggerGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _trigger_id: trigger_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_triggers_list(&self, account_id: &str, container_id: &str) -> AccountContainerTriggerListCall<'a, S> {
        AccountContainerTriggerListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `triggerId` - The GTM Trigger ID.
    pub fn containers_triggers_update(&self, request: Trigger, account_id: &str, container_id: &str, trigger_id: &str) -> AccountContainerTriggerUpdateCall<'a, S> {
        AccountContainerTriggerUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _trigger_id: trigger_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_variables_create(&self, request: Variable, account_id: &str, container_id: &str) -> AccountContainerVariableCreateCall<'a, S> {
        AccountContainerVariableCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `variableId` - The GTM Variable ID.
    pub fn containers_variables_delete(&self, account_id: &str, container_id: &str, variable_id: &str) -> AccountContainerVariableDeleteCall<'a, S> {
        AccountContainerVariableDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _variable_id: variable_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `variableId` - The GTM Variable ID.
    pub fn containers_variables_get(&self, account_id: &str, container_id: &str, variable_id: &str) -> AccountContainerVariableGetCall<'a, S> {
        AccountContainerVariableGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _variable_id: variable_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_variables_list(&self, account_id: &str, container_id: &str) -> AccountContainerVariableListCall<'a, S> {
        AccountContainerVariableListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `variableId` - The GTM Variable ID.
    pub fn containers_variables_update(&self, request: Variable, account_id: &str, container_id: &str, variable_id: &str) -> AccountContainerVariableUpdateCall<'a, S> {
        AccountContainerVariableUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _variable_id: variable_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Container Version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_versions_create(&self, request: CreateContainerVersionRequestVersionOptions, account_id: &str, container_id: &str) -> AccountContainerVersionCreateCall<'a, S> {
        AccountContainerVersionCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `containerVersionId` - The GTM Container Version ID.
    pub fn containers_versions_delete(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionDeleteCall<'a, S> {
        AccountContainerVersionDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `containerVersionId` - The GTM Container Version ID. Specify published to retrieve the currently published version.
    pub fn containers_versions_get(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionGetCall<'a, S> {
        AccountContainerVersionGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_versions_list(&self, account_id: &str, container_id: &str) -> AccountContainerVersionListCall<'a, S> {
        AccountContainerVersionListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _include_deleted: Default::default(),
            _headers: Default::default(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `containerVersionId` - The GTM Container Version ID.
    pub fn containers_versions_publish(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionPublishCall<'a, S> {
        AccountContainerVersionPublishCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores a Container Version. This will overwrite the container's current configuration (including its variables, triggers and tags). The operation will not have any effect on the version that is being served (i.e. the published version).
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `containerVersionId` - The GTM Container Version ID.
    pub fn containers_versions_restore(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionRestoreCall<'a, S> {
        AccountContainerVersionRestoreCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `containerVersionId` - The GTM Container Version ID.
    pub fn containers_versions_undelete(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionUndeleteCall<'a, S> {
        AccountContainerVersionUndeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    /// * `containerVersionId` - The GTM Container Version ID.
    pub fn containers_versions_update(&self, request: ContainerVersion, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionUpdateCall<'a, S> {
        AccountContainerVersionUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _fingerprint: Default::default(),
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
    /// * `accountId` - The GTM Account ID.
    pub fn containers_create(&self, request: Container, account_id: &str) -> AccountContainerCreateCall<'a, S> {
        AccountContainerCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_delete(&self, account_id: &str, container_id: &str) -> AccountContainerDeleteCall<'a, S> {
        AccountContainerDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_get(&self, account_id: &str, container_id: &str) -> AccountContainerGetCall<'a, S> {
        AccountContainerGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    pub fn containers_list(&self, account_id: &str) -> AccountContainerListCall<'a, S> {
        AccountContainerListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `containerId` - The GTM Container ID.
    pub fn containers_update(&self, request: Container, account_id: &str, container_id: &str) -> AccountContainerUpdateCall<'a, S> {
        AccountContainerUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a user's Account & Container Permissions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The GTM Account ID.
    pub fn permissions_create(&self, request: UserAccess, account_id: &str) -> AccountPermissionCreateCall<'a, S> {
        AccountPermissionCreateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    /// * `permissionId` - The GTM User ID.
    pub fn permissions_delete(&self, account_id: &str, permission_id: &str) -> AccountPermissionDeleteCall<'a, S> {
        AccountPermissionDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _permission_id: permission_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a user's Account & Container Permissions.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The GTM Account ID.
    /// * `permissionId` - The GTM User ID.
    pub fn permissions_get(&self, account_id: &str, permission_id: &str) -> AccountPermissionGetCall<'a, S> {
        AccountPermissionGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _permission_id: permission_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all users that have access to the account along with Account and Container Permissions granted to each of them.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - The GTM Account ID.
    pub fn permissions_list(&self, account_id: &str) -> AccountPermissionListCall<'a, S> {
        AccountPermissionListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a user's Account & Container Permissions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The GTM Account ID.
    /// * `permissionId` - The GTM User ID.
    pub fn permissions_update(&self, request: UserAccess, account_id: &str, permission_id: &str) -> AccountPermissionUpdateCall<'a, S> {
        AccountPermissionUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _permission_id: permission_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    pub fn get(&self, account_id: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
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
    /// * `accountId` - The GTM Account ID.
    pub fn update(&self, request: Account, account_id: &str) -> AccountUpdateCall<'a, S> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



