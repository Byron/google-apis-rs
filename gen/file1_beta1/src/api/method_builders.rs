use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudFilestore`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_file1_beta1 as file1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use file1_beta1::{CloudFilestore, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudFilestore::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_backups_create(...)`, `locations_backups_delete(...)`, `locations_backups_get(...)`, `locations_backups_list(...)`, `locations_backups_patch(...)`, `locations_get(...)`, `locations_instances_create(...)`, `locations_instances_delete(...)`, `locations_instances_get(...)`, `locations_instances_list(...)`, `locations_instances_patch(...)`, `locations_instances_restore(...)`, `locations_instances_revert(...)`, `locations_instances_shares_create(...)`, `locations_instances_shares_delete(...)`, `locations_instances_shares_get(...)`, `locations_instances_shares_list(...)`, `locations_instances_shares_patch(...)`, `locations_instances_snapshots_create(...)`, `locations_instances_snapshots_delete(...)`, `locations_instances_snapshots_get(...)`, `locations_instances_snapshots_list(...)`, `locations_instances_snapshots_patch(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)` and `locations_operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudFilestore<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The backup's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**.
    pub fn locations_backups_create(&self, request: Backup, parent: &str) -> ProjectLocationBackupCreateCall<'a, S> {
        ProjectLocationBackupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _backup_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The backup resource name, in the format `projects/{project_id}/locations/{location}/backups/{backup_id}`
    pub fn locations_backups_delete(&self, name: &str) -> ProjectLocationBackupDeleteCall<'a, S> {
        ProjectLocationBackupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a specific backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The backup resource name, in the format `projects/{project_id}/locations/{location}/backups/{backup_id}`.
    pub fn locations_backups_get(&self, name: &str) -> ProjectLocationBackupGetCall<'a, S> {
        ProjectLocationBackupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all backups in a project for either a specified location or for all locations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location for which to retrieve backup information, in the format `projects/{project_id}/locations/{location}`. In Filestore, backup locations map to Google Cloud regions, for example **us-west1**. To retrieve backup information for all locations, use "-" for the `{location}` value.
    pub fn locations_backups_list(&self, parent: &str) -> ProjectLocationBackupListCall<'a, S> {
        ProjectLocationBackupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates the settings of a specific backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`.
    pub fn locations_backups_patch(&self, request: Backup, name: &str) -> ProjectLocationBackupPatchCall<'a, S> {
        ProjectLocationBackupPatchCall {
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
    /// Creates a share.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Filestore Instance to create the share for, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_shares_create(&self, request: Share, parent: &str) -> ProjectLocationInstanceShareCreateCall<'a, S> {
        ProjectLocationInstanceShareCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _share_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a share.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The share resource name, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}/share/{share_id}`
    pub fn locations_instances_shares_delete(&self, name: &str) -> ProjectLocationInstanceShareDeleteCall<'a, S> {
        ProjectLocationInstanceShareDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a specific share.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The share resource name, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}/shares/{share_id}`
    pub fn locations_instances_shares_get(&self, name: &str) -> ProjectLocationInstanceShareGetCall<'a, S> {
        ProjectLocationInstanceShareGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all shares for a specified instance.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The instance for which to retrieve share information, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`.
    pub fn locations_instances_shares_list(&self, parent: &str) -> ProjectLocationInstanceShareListCall<'a, S> {
        ProjectLocationInstanceShareListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates the settings of a specific share.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the share, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/shares/{share_id}`.
    pub fn locations_instances_shares_patch(&self, request: Share, name: &str) -> ProjectLocationInstanceSharePatchCall<'a, S> {
        ProjectLocationInstanceSharePatchCall {
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
    /// Creates a snapshot.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Filestore Instance to create the snapshots of, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_snapshots_create(&self, request: Snapshot, parent: &str) -> ProjectLocationInstanceSnapshotCreateCall<'a, S> {
        ProjectLocationInstanceSnapshotCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _snapshot_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a snapshot.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The snapshot resource name, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}/snapshots/{snapshot_id}`
    pub fn locations_instances_snapshots_delete(&self, name: &str) -> ProjectLocationInstanceSnapshotDeleteCall<'a, S> {
        ProjectLocationInstanceSnapshotDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a specific snapshot.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The snapshot resource name, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}/snapshots/{snapshot_id}`
    pub fn locations_instances_snapshots_get(&self, name: &str) -> ProjectLocationInstanceSnapshotGetCall<'a, S> {
        ProjectLocationInstanceSnapshotGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all snapshots in a project for either a specified location or for all locations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The instance for which to retrieve snapshot information, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`.
    pub fn locations_instances_snapshots_list(&self, parent: &str) -> ProjectLocationInstanceSnapshotListCall<'a, S> {
        ProjectLocationInstanceSnapshotListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates the settings of a specific snapshot.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`.
    pub fn locations_instances_snapshots_patch(&self, request: Snapshot, name: &str) -> ProjectLocationInstanceSnapshotPatchCall<'a, S> {
        ProjectLocationInstanceSnapshotPatchCall {
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
    /// Creates an instance. When creating from a backup, the capacity of the new instance needs to be equal to or larger than the capacity of the backup (and also equal to or larger than the minimum capacity of the tier).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The instance's project and location, in the format `projects/{project_id}/locations/{location}`. In Filestore, locations map to Google Cloud zones, for example **us-west1-b**.
    pub fn locations_instances_create(&self, request: Instance, parent: &str) -> ProjectLocationInstanceCreateCall<'a, S> {
        ProjectLocationInstanceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _instance_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The instance resource name, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`
    pub fn locations_instances_delete(&self, name: &str) -> ProjectLocationInstanceDeleteCall<'a, S> {
        ProjectLocationInstanceDeleteCall {
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
    /// Gets the details of a specific instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The instance resource name, in the format `projects/{project_id}/locations/{location}/instances/{instance_id}`.
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
    /// Lists all instances in a project for either a specified location or for all locations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location for which to retrieve instance information, in the format `projects/{project_id}/locations/{location}`. In Cloud Filestore, locations map to Google Cloud zones, for example **us-west1-b**. To retrieve instance information for all locations, use "-" for the `{location}` value.
    pub fn locations_instances_list(&self, parent: &str) -> ProjectLocationInstanceListCall<'a, S> {
        ProjectLocationInstanceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates the settings of a specific instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`.
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
    /// Restores an existing instance's file share from a backup. The capacity of the instance needs to be equal to or larger than the capacity of the backup (and also equal to or larger than the minimum capacity of the tier).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`.
    pub fn locations_instances_restore(&self, request: RestoreInstanceRequest, name: &str) -> ProjectLocationInstanceRestoreCall<'a, S> {
        ProjectLocationInstanceRestoreCall {
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
    /// Revert an existing instance's file system to a specified snapshot.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. projects/{project_id}/locations/{location_id}/instances/{instance_id}. The resource name of the instance, in the format
    pub fn locations_instances_revert(&self, request: RevertInstanceRequest, name: &str) -> ProjectLocationInstanceRevertCall<'a, S> {
        ProjectLocationInstanceRevertCall {
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
            _include_unrevealed_locations: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



