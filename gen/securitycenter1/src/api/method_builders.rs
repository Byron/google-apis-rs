use super::*;
/// A builder providing access to all methods supported on *folder* resources.
/// It is not used directly, but through the [`SecurityCommandCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_securitycenter1 as securitycenter1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_group(...)`, `assets_list(...)`, `assets_update_security_marks(...)`, `big_query_exports_create(...)`, `big_query_exports_delete(...)`, `big_query_exports_get(...)`, `big_query_exports_list(...)`, `big_query_exports_patch(...)`, `findings_bulk_mute(...)`, `mute_configs_create(...)`, `mute_configs_delete(...)`, `mute_configs_get(...)`, `mute_configs_list(...)`, `mute_configs_patch(...)`, `notification_configs_create(...)`, `notification_configs_delete(...)`, `notification_configs_get(...)`, `notification_configs_list(...)`, `notification_configs_patch(...)`, `sources_findings_external_systems_patch(...)`, `sources_findings_group(...)`, `sources_findings_list(...)`, `sources_findings_patch(...)`, `sources_findings_set_mute(...)`, `sources_findings_set_state(...)`, `sources_findings_update_security_marks(...)` and `sources_list(...)`
/// // to build up your call.
/// let rb = hub.folders();
/// # }
/// ```
pub struct FolderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SecurityCommandCenter<S>,
}

impl<'a, S> client::MethodsBuilder for FolderMethods<'a, S> {}

impl<'a, S> FolderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization's assets and groups them by their specified properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent to group the assets by. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_group(&self, request: GroupAssetsRequest, parent: &str) -> FolderAssetGroupCall<'a, S> {
        FolderAssetGroupCall {
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
    /// Lists an organization's assets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource that contains the assets. The value that you can specify on parent depends on the method in which you specify parent. You can specify one of the following values: "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_list(&self, parent: &str) -> FolderAssetListCall<'a, S> {
        FolderAssetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn assets_update_security_marks(&self, request: SecurityMarks, name: &str) -> FolderAssetUpdateSecurityMarkCall<'a, S> {
        FolderAssetUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource of the new BigQuery export. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn big_query_exports_create(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, parent: &str) -> FolderBigQueryExportCreateCall<'a, S> {
        FolderBigQueryExportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _big_query_export_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the BigQuery export to delete. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_delete(&self, name: &str) -> FolderBigQueryExportDeleteCall<'a, S> {
        FolderBigQueryExportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the BigQuery export to retrieve. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_get(&self, name: &str) -> FolderBigQueryExportGetCall<'a, S> {
        FolderBigQueryExportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists BigQuery exports. Note that when requesting BigQuery exports at a given level all exports under that level are also returned e.g. if requesting BigQuery exports under a folder, then all BigQuery exports immediately under the folder plus the ones created under the projects within the folder are returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of BigQuery exports. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn big_query_exports_list(&self, parent: &str) -> FolderBigQueryExportListCall<'a, S> {
        FolderBigQueryExportListCall {
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
    /// Updates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests.
    pub fn big_query_exports_patch(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, name: &str) -> FolderBigQueryExportPatchCall<'a, S> {
        FolderBigQueryExportPatchCall {
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
    /// Kicks off an LRO to bulk mute findings for a parent based on a filter. The parent can be either an organization, folder or project. The findings matched by the filter will be muted after the LRO is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, at which bulk action needs to be applied. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn findings_bulk_mute(&self, request: BulkMuteFindingsRequest, parent: &str) -> FolderFindingBulkMuteCall<'a, S> {
        FolderFindingBulkMuteCall {
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
    /// Creates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new mute configs's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn mute_configs_create(&self, request: GoogleCloudSecuritycenterV1MuteConfig, parent: &str) -> FolderMuteConfigCreateCall<'a, S> {
        FolderMuteConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _mute_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, or projects/{project}/muteConfigs/{config_id}
    pub fn mute_configs_delete(&self, name: &str) -> FolderMuteConfigDeleteCall<'a, S> {
        FolderMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, or projects/{project}/muteConfigs/{config_id}
    pub fn mute_configs_get(&self, name: &str) -> FolderMuteConfigGetCall<'a, S> {
        FolderMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists mute configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of mute configs. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn mute_configs_list(&self, parent: &str) -> FolderMuteConfigListCall<'a, S> {
        FolderMuteConfigListCall {
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
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}"
    pub fn mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> FolderMuteConfigPatchCall<'a, S> {
        FolderMuteConfigPatchCall {
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
    /// Creates a notification config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_create(&self, request: NotificationConfig, parent: &str) -> FolderNotificationConfigCreateCall<'a, S> {
        FolderNotificationConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to delete. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_delete(&self, name: &str) -> FolderNotificationConfigDeleteCall<'a, S> {
        FolderNotificationConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to get. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_get(&self, name: &str) -> FolderNotificationConfigGetCall<'a, S> {
        FolderNotificationConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists notification configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent in which to list the notification configurations. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_list(&self, parent: &str) -> FolderNotificationConfigListCall<'a, S> {
        FolderNotificationConfigListCall {
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
    ///  Updates a notification config. The following update fields are allowed: description, pubsub_topic, streaming_config.filter
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket".
    pub fn notification_configs_patch(&self, request: NotificationConfig, name: &str) -> FolderNotificationConfigPatchCall<'a, S> {
        FolderNotificationConfigPatchCall {
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
    /// Updates external system. This is for a given finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira"
    pub fn sources_findings_external_systems_patch(&self, request: GoogleCloudSecuritycenterV1ExternalSystem, name: &str) -> FolderSourceFindingExternalSystemPatchCall<'a, S> {
        FolderSourceFindingExternalSystemPatchCall {
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
    /// Filters an organization or source's findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the source to groupBy. Its format is "organizations/[organization_id]/sources/[source_id]", folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]. To groupBy across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/-, or projects/{project_id}/sources/-
    pub fn sources_findings_group(&self, request: GroupFindingsRequest, parent: &str) -> FolderSourceFindingGroupCall<'a, S> {
        FolderSourceFindingGroupCall {
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
    /// Lists an organization or source's findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the source the findings belong to. Its format is "organizations/[organization_id]/sources/[source_id], folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]". To list across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/- or projects/{projects_id}/sources/-
    pub fn sources_findings_list(&self, parent: &str) -> FolderSourceFindingListCall<'a, S> {
        FolderSourceFindingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}"
    pub fn sources_findings_patch(&self, request: Finding, name: &str) -> FolderSourceFindingPatchCall<'a, S> {
        FolderSourceFindingPatchCall {
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
    /// Updates the mute state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_mute(&self, request: SetMuteRequest, name: &str) -> FolderSourceFindingSetMuteCall<'a, S> {
        FolderSourceFindingSetMuteCall {
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
    /// Updates the state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_state(&self, request: SetFindingStateRequest, name: &str) -> FolderSourceFindingSetStateCall<'a, S> {
        FolderSourceFindingSetStateCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn sources_findings_update_security_marks(&self, request: SecurityMarks, name: &str) -> FolderSourceFindingUpdateSecurityMarkCall<'a, S> {
        FolderSourceFindingUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all sources belonging to an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent of sources to list. Its format should be "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn sources_list(&self, parent: &str) -> FolderSourceListCall<'a, S> {
        FolderSourceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`SecurityCommandCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_securitycenter1 as securitycenter1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_group(...)`, `assets_list(...)`, `assets_run_discovery(...)`, `assets_update_security_marks(...)`, `big_query_exports_create(...)`, `big_query_exports_delete(...)`, `big_query_exports_get(...)`, `big_query_exports_list(...)`, `big_query_exports_patch(...)`, `findings_bulk_mute(...)`, `get_organization_settings(...)`, `mute_configs_create(...)`, `mute_configs_delete(...)`, `mute_configs_get(...)`, `mute_configs_list(...)`, `mute_configs_patch(...)`, `notification_configs_create(...)`, `notification_configs_delete(...)`, `notification_configs_get(...)`, `notification_configs_list(...)`, `notification_configs_patch(...)`, `operations_cancel(...)`, `operations_delete(...)`, `operations_get(...)`, `operations_list(...)`, `sources_create(...)`, `sources_findings_create(...)`, `sources_findings_external_systems_patch(...)`, `sources_findings_group(...)`, `sources_findings_list(...)`, `sources_findings_patch(...)`, `sources_findings_set_mute(...)`, `sources_findings_set_state(...)`, `sources_findings_update_security_marks(...)`, `sources_get(...)`, `sources_get_iam_policy(...)`, `sources_list(...)`, `sources_patch(...)`, `sources_set_iam_policy(...)`, `sources_test_iam_permissions(...)` and `update_organization_settings(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SecurityCommandCenter<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization's assets and groups them by their specified properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent to group the assets by. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_group(&self, request: GroupAssetsRequest, parent: &str) -> OrganizationAssetGroupCall<'a, S> {
        OrganizationAssetGroupCall {
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
    /// Lists an organization's assets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource that contains the assets. The value that you can specify on parent depends on the method in which you specify parent. You can specify one of the following values: "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_list(&self, parent: &str) -> OrganizationAssetListCall<'a, S> {
        OrganizationAssetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs asset discovery. The discovery is tracked with a long-running operation. This API can only be called with limited frequency for an organization. If it is called too frequently the caller will receive a TOO_MANY_REQUESTS error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization to run asset discovery for. Its format is "organizations/[organization_id]".
    pub fn assets_run_discovery(&self, request: RunAssetDiscoveryRequest, parent: &str) -> OrganizationAssetRunDiscoveryCall<'a, S> {
        OrganizationAssetRunDiscoveryCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn assets_update_security_marks(&self, request: SecurityMarks, name: &str) -> OrganizationAssetUpdateSecurityMarkCall<'a, S> {
        OrganizationAssetUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource of the new BigQuery export. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn big_query_exports_create(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, parent: &str) -> OrganizationBigQueryExportCreateCall<'a, S> {
        OrganizationBigQueryExportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _big_query_export_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the BigQuery export to delete. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_delete(&self, name: &str) -> OrganizationBigQueryExportDeleteCall<'a, S> {
        OrganizationBigQueryExportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the BigQuery export to retrieve. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_get(&self, name: &str) -> OrganizationBigQueryExportGetCall<'a, S> {
        OrganizationBigQueryExportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists BigQuery exports. Note that when requesting BigQuery exports at a given level all exports under that level are also returned e.g. if requesting BigQuery exports under a folder, then all BigQuery exports immediately under the folder plus the ones created under the projects within the folder are returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of BigQuery exports. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn big_query_exports_list(&self, parent: &str) -> OrganizationBigQueryExportListCall<'a, S> {
        OrganizationBigQueryExportListCall {
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
    /// Updates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests.
    pub fn big_query_exports_patch(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, name: &str) -> OrganizationBigQueryExportPatchCall<'a, S> {
        OrganizationBigQueryExportPatchCall {
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
    /// Kicks off an LRO to bulk mute findings for a parent based on a filter. The parent can be either an organization, folder or project. The findings matched by the filter will be muted after the LRO is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, at which bulk action needs to be applied. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn findings_bulk_mute(&self, request: BulkMuteFindingsRequest, parent: &str) -> OrganizationFindingBulkMuteCall<'a, S> {
        OrganizationFindingBulkMuteCall {
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
    /// Creates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new mute configs's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn mute_configs_create(&self, request: GoogleCloudSecuritycenterV1MuteConfig, parent: &str) -> OrganizationMuteConfigCreateCall<'a, S> {
        OrganizationMuteConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _mute_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, or projects/{project}/muteConfigs/{config_id}
    pub fn mute_configs_delete(&self, name: &str) -> OrganizationMuteConfigDeleteCall<'a, S> {
        OrganizationMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, or projects/{project}/muteConfigs/{config_id}
    pub fn mute_configs_get(&self, name: &str) -> OrganizationMuteConfigGetCall<'a, S> {
        OrganizationMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists mute configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of mute configs. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn mute_configs_list(&self, parent: &str) -> OrganizationMuteConfigListCall<'a, S> {
        OrganizationMuteConfigListCall {
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
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}"
    pub fn mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> OrganizationMuteConfigPatchCall<'a, S> {
        OrganizationMuteConfigPatchCall {
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
    /// Creates a notification config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_create(&self, request: NotificationConfig, parent: &str) -> OrganizationNotificationConfigCreateCall<'a, S> {
        OrganizationNotificationConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to delete. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_delete(&self, name: &str) -> OrganizationNotificationConfigDeleteCall<'a, S> {
        OrganizationNotificationConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to get. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_get(&self, name: &str) -> OrganizationNotificationConfigGetCall<'a, S> {
        OrganizationNotificationConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists notification configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent in which to list the notification configurations. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_list(&self, parent: &str) -> OrganizationNotificationConfigListCall<'a, S> {
        OrganizationNotificationConfigListCall {
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
    ///  Updates a notification config. The following update fields are allowed: description, pubsub_topic, streaming_config.filter
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket".
    pub fn notification_configs_patch(&self, request: NotificationConfig, name: &str) -> OrganizationNotificationConfigPatchCall<'a, S> {
        OrganizationNotificationConfigPatchCall {
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
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, name: &str) -> OrganizationOperationCancelCall<'a, S> {
        OrganizationOperationCancelCall {
            hub: self.hub,
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
    pub fn operations_delete(&self, name: &str) -> OrganizationOperationDeleteCall<'a, S> {
        OrganizationOperationDeleteCall {
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
    pub fn operations_get(&self, name: &str) -> OrganizationOperationGetCall<'a, S> {
        OrganizationOperationGetCall {
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
    pub fn operations_list(&self, name: &str) -> OrganizationOperationListCall<'a, S> {
        OrganizationOperationListCall {
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
    /// Updates external system. This is for a given finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira"
    pub fn sources_findings_external_systems_patch(&self, request: GoogleCloudSecuritycenterV1ExternalSystem, name: &str) -> OrganizationSourceFindingExternalSystemPatchCall<'a, S> {
        OrganizationSourceFindingExternalSystemPatchCall {
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
    /// Creates a finding. The corresponding source must exist for finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new finding's parent. Its format should be "organizations/[organization_id]/sources/[source_id]".
    pub fn sources_findings_create(&self, request: Finding, parent: &str) -> OrganizationSourceFindingCreateCall<'a, S> {
        OrganizationSourceFindingCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _finding_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization or source's findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the source to groupBy. Its format is "organizations/[organization_id]/sources/[source_id]", folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]. To groupBy across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/-, or projects/{project_id}/sources/-
    pub fn sources_findings_group(&self, request: GroupFindingsRequest, parent: &str) -> OrganizationSourceFindingGroupCall<'a, S> {
        OrganizationSourceFindingGroupCall {
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
    /// Lists an organization or source's findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the source the findings belong to. Its format is "organizations/[organization_id]/sources/[source_id], folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]". To list across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/- or projects/{projects_id}/sources/-
    pub fn sources_findings_list(&self, parent: &str) -> OrganizationSourceFindingListCall<'a, S> {
        OrganizationSourceFindingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}"
    pub fn sources_findings_patch(&self, request: Finding, name: &str) -> OrganizationSourceFindingPatchCall<'a, S> {
        OrganizationSourceFindingPatchCall {
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
    /// Updates the mute state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_mute(&self, request: SetMuteRequest, name: &str) -> OrganizationSourceFindingSetMuteCall<'a, S> {
        OrganizationSourceFindingSetMuteCall {
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
    /// Updates the state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_state(&self, request: SetFindingStateRequest, name: &str) -> OrganizationSourceFindingSetStateCall<'a, S> {
        OrganizationSourceFindingSetStateCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn sources_findings_update_security_marks(&self, request: SecurityMarks, name: &str) -> OrganizationSourceFindingUpdateSecurityMarkCall<'a, S> {
        OrganizationSourceFindingUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new source's parent. Its format should be "organizations/[organization_id]".
    pub fn sources_create(&self, request: Source, parent: &str) -> OrganizationSourceCreateCall<'a, S> {
        OrganizationSourceCreateCall {
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
    /// Gets a source.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Relative resource name of the source. Its format is "organizations/[organization_id]/source/[source_id]".
    pub fn sources_get(&self, name: &str) -> OrganizationSourceGetCall<'a, S> {
        OrganizationSourceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy on the specified Source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn sources_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> OrganizationSourceGetIamPolicyCall<'a, S> {
        OrganizationSourceGetIamPolicyCall {
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
    /// Lists all sources belonging to an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent of sources to list. Its format should be "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn sources_list(&self, parent: &str) -> OrganizationSourceListCall<'a, S> {
        OrganizationSourceListCall {
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
    /// Updates a source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}"
    pub fn sources_patch(&self, request: Source, name: &str) -> OrganizationSourcePatchCall<'a, S> {
        OrganizationSourcePatchCall {
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
    /// Sets the access control policy on the specified Source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn sources_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> OrganizationSourceSetIamPolicyCall<'a, S> {
        OrganizationSourceSetIamPolicyCall {
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
    /// Returns the permissions that a caller has on the specified source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn sources_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> OrganizationSourceTestIamPermissionCall<'a, S> {
        OrganizationSourceTestIamPermissionCall {
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
    /// Gets the settings for an organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the organization to get organization settings for. Its format is "organizations/[organization_id]/organizationSettings".
    pub fn get_organization_settings(&self, name: &str) -> OrganizationGetOrganizationSettingCall<'a, S> {
        OrganizationGetOrganizationSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an organization's settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings".
    pub fn update_organization_settings(&self, request: OrganizationSettings, name: &str) -> OrganizationUpdateOrganizationSettingCall<'a, S> {
        OrganizationUpdateOrganizationSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`SecurityCommandCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_securitycenter1 as securitycenter1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_group(...)`, `assets_list(...)`, `assets_update_security_marks(...)`, `big_query_exports_create(...)`, `big_query_exports_delete(...)`, `big_query_exports_get(...)`, `big_query_exports_list(...)`, `big_query_exports_patch(...)`, `findings_bulk_mute(...)`, `mute_configs_create(...)`, `mute_configs_delete(...)`, `mute_configs_get(...)`, `mute_configs_list(...)`, `mute_configs_patch(...)`, `notification_configs_create(...)`, `notification_configs_delete(...)`, `notification_configs_get(...)`, `notification_configs_list(...)`, `notification_configs_patch(...)`, `sources_findings_external_systems_patch(...)`, `sources_findings_group(...)`, `sources_findings_list(...)`, `sources_findings_patch(...)`, `sources_findings_set_mute(...)`, `sources_findings_set_state(...)`, `sources_findings_update_security_marks(...)` and `sources_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SecurityCommandCenter<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization's assets and groups them by their specified properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent to group the assets by. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_group(&self, request: GroupAssetsRequest, parent: &str) -> ProjectAssetGroupCall<'a, S> {
        ProjectAssetGroupCall {
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
    /// Lists an organization's assets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource that contains the assets. The value that you can specify on parent depends on the method in which you specify parent. You can specify one of the following values: "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_list(&self, parent: &str) -> ProjectAssetListCall<'a, S> {
        ProjectAssetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn assets_update_security_marks(&self, request: SecurityMarks, name: &str) -> ProjectAssetUpdateSecurityMarkCall<'a, S> {
        ProjectAssetUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource of the new BigQuery export. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn big_query_exports_create(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, parent: &str) -> ProjectBigQueryExportCreateCall<'a, S> {
        ProjectBigQueryExportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _big_query_export_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the BigQuery export to delete. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_delete(&self, name: &str) -> ProjectBigQueryExportDeleteCall<'a, S> {
        ProjectBigQueryExportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the BigQuery export to retrieve. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_get(&self, name: &str) -> ProjectBigQueryExportGetCall<'a, S> {
        ProjectBigQueryExportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists BigQuery exports. Note that when requesting BigQuery exports at a given level all exports under that level are also returned e.g. if requesting BigQuery exports under a folder, then all BigQuery exports immediately under the folder plus the ones created under the projects within the folder are returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of BigQuery exports. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn big_query_exports_list(&self, parent: &str) -> ProjectBigQueryExportListCall<'a, S> {
        ProjectBigQueryExportListCall {
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
    /// Updates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests.
    pub fn big_query_exports_patch(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, name: &str) -> ProjectBigQueryExportPatchCall<'a, S> {
        ProjectBigQueryExportPatchCall {
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
    /// Kicks off an LRO to bulk mute findings for a parent based on a filter. The parent can be either an organization, folder or project. The findings matched by the filter will be muted after the LRO is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, at which bulk action needs to be applied. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn findings_bulk_mute(&self, request: BulkMuteFindingsRequest, parent: &str) -> ProjectFindingBulkMuteCall<'a, S> {
        ProjectFindingBulkMuteCall {
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
    /// Creates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new mute configs's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn mute_configs_create(&self, request: GoogleCloudSecuritycenterV1MuteConfig, parent: &str) -> ProjectMuteConfigCreateCall<'a, S> {
        ProjectMuteConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _mute_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, or projects/{project}/muteConfigs/{config_id}
    pub fn mute_configs_delete(&self, name: &str) -> ProjectMuteConfigDeleteCall<'a, S> {
        ProjectMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, or projects/{project}/muteConfigs/{config_id}
    pub fn mute_configs_get(&self, name: &str) -> ProjectMuteConfigGetCall<'a, S> {
        ProjectMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists mute configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of mute configs. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn mute_configs_list(&self, parent: &str) -> ProjectMuteConfigListCall<'a, S> {
        ProjectMuteConfigListCall {
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
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}"
    pub fn mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> ProjectMuteConfigPatchCall<'a, S> {
        ProjectMuteConfigPatchCall {
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
    /// Creates a notification config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_create(&self, request: NotificationConfig, parent: &str) -> ProjectNotificationConfigCreateCall<'a, S> {
        ProjectNotificationConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to delete. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_delete(&self, name: &str) -> ProjectNotificationConfigDeleteCall<'a, S> {
        ProjectNotificationConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to get. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_get(&self, name: &str) -> ProjectNotificationConfigGetCall<'a, S> {
        ProjectNotificationConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists notification configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent in which to list the notification configurations. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_list(&self, parent: &str) -> ProjectNotificationConfigListCall<'a, S> {
        ProjectNotificationConfigListCall {
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
    ///  Updates a notification config. The following update fields are allowed: description, pubsub_topic, streaming_config.filter
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket".
    pub fn notification_configs_patch(&self, request: NotificationConfig, name: &str) -> ProjectNotificationConfigPatchCall<'a, S> {
        ProjectNotificationConfigPatchCall {
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
    /// Updates external system. This is for a given finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira"
    pub fn sources_findings_external_systems_patch(&self, request: GoogleCloudSecuritycenterV1ExternalSystem, name: &str) -> ProjectSourceFindingExternalSystemPatchCall<'a, S> {
        ProjectSourceFindingExternalSystemPatchCall {
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
    /// Filters an organization or source's findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the source to groupBy. Its format is "organizations/[organization_id]/sources/[source_id]", folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]. To groupBy across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/-, or projects/{project_id}/sources/-
    pub fn sources_findings_group(&self, request: GroupFindingsRequest, parent: &str) -> ProjectSourceFindingGroupCall<'a, S> {
        ProjectSourceFindingGroupCall {
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
    /// Lists an organization or source's findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the source the findings belong to. Its format is "organizations/[organization_id]/sources/[source_id], folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]". To list across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/- or projects/{projects_id}/sources/-
    pub fn sources_findings_list(&self, parent: &str) -> ProjectSourceFindingListCall<'a, S> {
        ProjectSourceFindingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}"
    pub fn sources_findings_patch(&self, request: Finding, name: &str) -> ProjectSourceFindingPatchCall<'a, S> {
        ProjectSourceFindingPatchCall {
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
    /// Updates the mute state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_mute(&self, request: SetMuteRequest, name: &str) -> ProjectSourceFindingSetMuteCall<'a, S> {
        ProjectSourceFindingSetMuteCall {
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
    /// Updates the state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_state(&self, request: SetFindingStateRequest, name: &str) -> ProjectSourceFindingSetStateCall<'a, S> {
        ProjectSourceFindingSetStateCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn sources_findings_update_security_marks(&self, request: SecurityMarks, name: &str) -> ProjectSourceFindingUpdateSecurityMarkCall<'a, S> {
        ProjectSourceFindingUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all sources belonging to an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent of sources to list. Its format should be "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn sources_list(&self, parent: &str) -> ProjectSourceListCall<'a, S> {
        ProjectSourceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



