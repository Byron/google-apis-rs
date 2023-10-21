use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Baremetalsolution`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_baremetalsolution2 as baremetalsolution2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use baremetalsolution2::{Baremetalsolution, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Baremetalsolution::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_instance_provisioning_settings_fetch(...)`, `locations_instances_create(...)`, `locations_instances_detach_lun(...)`, `locations_instances_disable_interactive_serial_console(...)`, `locations_instances_enable_interactive_serial_console(...)`, `locations_instances_get(...)`, `locations_instances_list(...)`, `locations_instances_patch(...)`, `locations_instances_reset(...)`, `locations_instances_start(...)`, `locations_instances_stop(...)`, `locations_list(...)`, `locations_networks_get(...)`, `locations_networks_list(...)`, `locations_networks_list_network_usage(...)`, `locations_networks_patch(...)`, `locations_nfs_shares_create(...)`, `locations_nfs_shares_delete(...)`, `locations_nfs_shares_get(...)`, `locations_nfs_shares_list(...)`, `locations_nfs_shares_patch(...)`, `locations_operations_get(...)`, `locations_provisioning_configs_create(...)`, `locations_provisioning_configs_get(...)`, `locations_provisioning_configs_patch(...)`, `locations_provisioning_configs_submit(...)`, `locations_provisioning_quotas_list(...)`, `locations_ssh_keys_create(...)`, `locations_ssh_keys_delete(...)`, `locations_ssh_keys_list(...)`, `locations_volumes_get(...)`, `locations_volumes_list(...)`, `locations_volumes_luns_get(...)`, `locations_volumes_luns_list(...)`, `locations_volumes_patch(...)`, `locations_volumes_resize(...)`, `locations_volumes_snapshots_create(...)`, `locations_volumes_snapshots_delete(...)`, `locations_volumes_snapshots_get(...)`, `locations_volumes_snapshots_list(...)` and `locations_volumes_snapshots_restore_volume_snapshot(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Baremetalsolution<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get instance provisioning settings for a given project. This is hidden method used by UI only.
    /// 
    /// # Arguments
    ///
    /// * `location` - Required. The parent project and location containing the ProvisioningSettings.
    pub fn locations_instance_provisioning_settings_fetch(&self, location: &str) -> ProjectLocationInstanceProvisioningSettingFetchCall<'a, S> {
        ProjectLocationInstanceProvisioningSettingFetchCall {
            hub: self.hub,
            _location: location.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create an Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent project and location.
    pub fn locations_instances_create(&self, request: Instance, parent: &str) -> ProjectLocationInstanceCreateCall<'a, S> {
        ProjectLocationInstanceCreateCall {
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
    /// Detach LUN from Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instance` - Required. Name of the instance.
    pub fn locations_instances_detach_lun(&self, request: DetachLunRequest, instance: &str) -> ProjectLocationInstanceDetachLunCall<'a, S> {
        ProjectLocationInstanceDetachLunCall {
            hub: self.hub,
            _request: request,
            _instance: instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Disable the interactive serial console feature on an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource.
    pub fn locations_instances_disable_interactive_serial_console(&self, request: DisableInteractiveSerialConsoleRequest, name: &str) -> ProjectLocationInstanceDisableInteractiveSerialConsoleCall<'a, S> {
        ProjectLocationInstanceDisableInteractiveSerialConsoleCall {
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
    /// Enable the interactive serial console feature on an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource.
    pub fn locations_instances_enable_interactive_serial_console(&self, request: EnableInteractiveSerialConsoleRequest, name: &str) -> ProjectLocationInstanceEnableInteractiveSerialConsoleCall<'a, S> {
        ProjectLocationInstanceEnableInteractiveSerialConsoleCall {
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
    /// Get details about a single server.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource.
    pub fn locations_instances_get(&self, name: &str) -> ProjectLocationInstanceGetCall<'a, S> {
        ProjectLocationInstanceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List servers in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListInstancesRequest.
    pub fn locations_instances_list(&self, parent: &str) -> ProjectLocationInstanceListCall<'a, S> {
        ProjectLocationInstanceListCall {
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
    /// Update details of a single server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of this `Instance`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. Format: `projects/{project}/locations/{location}/instances/{instance}`
    pub fn locations_instances_patch(&self, request: Instance, name: &str) -> ProjectLocationInstancePatchCall<'a, S> {
        ProjectLocationInstancePatchCall {
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
    /// Perform an ungraceful, hard reset on a server. Equivalent to shutting the power off and then turning it back on.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource.
    pub fn locations_instances_reset(&self, request: ResetInstanceRequest, name: &str) -> ProjectLocationInstanceResetCall<'a, S> {
        ProjectLocationInstanceResetCall {
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
    /// Starts a server that was shutdown.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource.
    pub fn locations_instances_start(&self, request: StartInstanceRequest, name: &str) -> ProjectLocationInstanceStartCall<'a, S> {
        ProjectLocationInstanceStartCall {
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
    /// Stop a running server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource.
    pub fn locations_instances_stop(&self, request: StopInstanceRequest, name: &str) -> ProjectLocationInstanceStopCall<'a, S> {
        ProjectLocationInstanceStopCall {
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
    /// Get details of a single network.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource.
    pub fn locations_networks_get(&self, name: &str) -> ProjectLocationNetworkGetCall<'a, S> {
        ProjectLocationNetworkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List network in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListNetworksRequest.
    pub fn locations_networks_list(&self, parent: &str) -> ProjectLocationNetworkListCall<'a, S> {
        ProjectLocationNetworkListCall {
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
    /// List all Networks (and used IPs for each Network) in the vendor account associated with the specified project.
    /// 
    /// # Arguments
    ///
    /// * `location` - Required. Parent value (project and location).
    pub fn locations_networks_list_network_usage(&self, location: &str) -> ProjectLocationNetworkListNetworkUsageCall<'a, S> {
        ProjectLocationNetworkListNetworkUsageCall {
            hub: self.hub,
            _location: location.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update details of a single network.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of this `Network`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. Format: `projects/{project}/locations/{location}/networks/{network}`
    pub fn locations_networks_patch(&self, request: Network, name: &str) -> ProjectLocationNetworkPatchCall<'a, S> {
        ProjectLocationNetworkPatchCall {
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
    /// Create an NFS share.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent project and location.
    pub fn locations_nfs_shares_create(&self, request: NfsShare, parent: &str) -> ProjectLocationNfsShareCreateCall<'a, S> {
        ProjectLocationNfsShareCreateCall {
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
    /// Delete an NFS share. The underlying volume is automatically deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the NFS share to delete.
    pub fn locations_nfs_shares_delete(&self, name: &str) -> ProjectLocationNfsShareDeleteCall<'a, S> {
        ProjectLocationNfsShareDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get details of a single NFS share.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource.
    pub fn locations_nfs_shares_get(&self, name: &str) -> ProjectLocationNfsShareGetCall<'a, S> {
        ProjectLocationNfsShareGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List NFS shares.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListNfsSharesRequest.
    pub fn locations_nfs_shares_list(&self, parent: &str) -> ProjectLocationNfsShareListCall<'a, S> {
        ProjectLocationNfsShareListCall {
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
    /// Update details of a single NFS share.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of the NFS share.
    pub fn locations_nfs_shares_patch(&self, request: NfsShare, name: &str) -> ProjectLocationNfsSharePatchCall<'a, S> {
        ProjectLocationNfsSharePatchCall {
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
    /// Get details about an operation. This method used only to work around CCFE lack of passthrough LRO support (b/221498758).
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
    /// Create new ProvisioningConfig.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent project and location containing the ProvisioningConfig.
    pub fn locations_provisioning_configs_create(&self, request: ProvisioningConfig, parent: &str) -> ProjectLocationProvisioningConfigCreateCall<'a, S> {
        ProjectLocationProvisioningConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get ProvisioningConfig by name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the ProvisioningConfig.
    pub fn locations_provisioning_configs_get(&self, name: &str) -> ProjectLocationProvisioningConfigGetCall<'a, S> {
        ProjectLocationProvisioningConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update existing ProvisioningConfig.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The system-generated name of the provisioning config. This follows the UUID format.
    pub fn locations_provisioning_configs_patch(&self, request: ProvisioningConfig, name: &str) -> ProjectLocationProvisioningConfigPatchCall<'a, S> {
        ProjectLocationProvisioningConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit a provisiong configuration for a given project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent project and location containing the ProvisioningConfig.
    pub fn locations_provisioning_configs_submit(&self, request: SubmitProvisioningConfigRequest, parent: &str) -> ProjectLocationProvisioningConfigSubmitCall<'a, S> {
        ProjectLocationProvisioningConfigSubmitCall {
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
    /// List the budget details to provision resources on a given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListProvisioningQuotasRequest.
    pub fn locations_provisioning_quotas_list(&self, parent: &str) -> ProjectLocationProvisioningQuotaListCall<'a, S> {
        ProjectLocationProvisioningQuotaListCall {
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
    /// Register a public SSH key in the specified project for use with the interactive serial console feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent containing the SSH keys.
    pub fn locations_ssh_keys_create(&self, request: SSHKey, parent: &str) -> ProjectLocationSshKeyCreateCall<'a, S> {
        ProjectLocationSshKeyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _ssh_key_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a public SSH key registered in the specified project.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the SSH key to delete. Currently, the only valid value for the location is "global".
    pub fn locations_ssh_keys_delete(&self, name: &str) -> ProjectLocationSshKeyDeleteCall<'a, S> {
        ProjectLocationSshKeyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the public SSH keys registered for the specified project. These SSH keys are used only for the interactive serial console feature.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent containing the SSH keys. Currently, the only valid value for the location is "global".
    pub fn locations_ssh_keys_list(&self, parent: &str) -> ProjectLocationSshKeyListCall<'a, S> {
        ProjectLocationSshKeyListCall {
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
    /// Get details of a single storage logical unit number(LUN).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource.
    pub fn locations_volumes_luns_get(&self, name: &str) -> ProjectLocationVolumeLunGetCall<'a, S> {
        ProjectLocationVolumeLunGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List storage volume luns for given storage volume.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListLunsRequest.
    pub fn locations_volumes_luns_list(&self, parent: &str) -> ProjectLocationVolumeLunListCall<'a, S> {
        ProjectLocationVolumeLunListCall {
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
    /// Takes a snapshot of a boot volume. Returns INVALID_ARGUMENT if called for a non-boot volume.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The volume to snapshot.
    pub fn locations_volumes_snapshots_create(&self, request: VolumeSnapshot, parent: &str) -> ProjectLocationVolumeSnapshotCreateCall<'a, S> {
        ProjectLocationVolumeSnapshotCreateCall {
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
    /// Deletes a volume snapshot. Returns INVALID_ARGUMENT if called for a non-boot volume.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the snapshot to delete.
    pub fn locations_volumes_snapshots_delete(&self, name: &str) -> ProjectLocationVolumeSnapshotDeleteCall<'a, S> {
        ProjectLocationVolumeSnapshotDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified snapshot resource. Returns INVALID_ARGUMENT if called for a non-boot volume.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the snapshot.
    pub fn locations_volumes_snapshots_get(&self, name: &str) -> ProjectLocationVolumeSnapshotGetCall<'a, S> {
        ProjectLocationVolumeSnapshotGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of snapshots for the specified volume. Returns a response with an empty list of snapshots if called for a non-boot volume.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListVolumesRequest.
    pub fn locations_volumes_snapshots_list(&self, parent: &str) -> ProjectLocationVolumeSnapshotListCall<'a, S> {
        ProjectLocationVolumeSnapshotListCall {
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
    /// Uses the specified snapshot to restore its parent volume. Returns INVALID_ARGUMENT if called for a non-boot volume.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `volumeSnapshot` - Required. Name of the snapshot which will be used to restore its parent volume.
    pub fn locations_volumes_snapshots_restore_volume_snapshot(&self, request: RestoreVolumeSnapshotRequest, volume_snapshot: &str) -> ProjectLocationVolumeSnapshotRestoreVolumeSnapshotCall<'a, S> {
        ProjectLocationVolumeSnapshotRestoreVolumeSnapshotCall {
            hub: self.hub,
            _request: request,
            _volume_snapshot: volume_snapshot.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get details of a single storage volume.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource.
    pub fn locations_volumes_get(&self, name: &str) -> ProjectLocationVolumeGetCall<'a, S> {
        ProjectLocationVolumeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List storage volumes in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListVolumesRequest.
    pub fn locations_volumes_list(&self, parent: &str) -> ProjectLocationVolumeListCall<'a, S> {
        ProjectLocationVolumeListCall {
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
    /// Update details of a single storage volume.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of this `Volume`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. Format: `projects/{project}/locations/{location}/volumes/{volume}`
    pub fn locations_volumes_patch(&self, request: Volume, name: &str) -> ProjectLocationVolumePatchCall<'a, S> {
        ProjectLocationVolumePatchCall {
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
    /// Emergency Volume resize.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `volume` - Required. Volume to resize.
    pub fn locations_volumes_resize(&self, request: ResizeVolumeRequest, volume: &str) -> ProjectLocationVolumeResizeCall<'a, S> {
        ProjectLocationVolumeResizeCall {
            hub: self.hub,
            _request: request,
            _volume: volume.to_string(),
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



