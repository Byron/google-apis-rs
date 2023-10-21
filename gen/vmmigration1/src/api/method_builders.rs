use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`VMMigrationService`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_vmmigration1 as vmmigration1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use vmmigration1::{VMMigrationService, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = VMMigrationService::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_groups_add_group_migration(...)`, `locations_groups_create(...)`, `locations_groups_delete(...)`, `locations_groups_get(...)`, `locations_groups_list(...)`, `locations_groups_patch(...)`, `locations_groups_remove_group_migration(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_sources_create(...)`, `locations_sources_datacenter_connectors_create(...)`, `locations_sources_datacenter_connectors_delete(...)`, `locations_sources_datacenter_connectors_get(...)`, `locations_sources_datacenter_connectors_list(...)`, `locations_sources_datacenter_connectors_upgrade_appliance(...)`, `locations_sources_delete(...)`, `locations_sources_fetch_inventory(...)`, `locations_sources_get(...)`, `locations_sources_list(...)`, `locations_sources_migrating_vms_clone_jobs_cancel(...)`, `locations_sources_migrating_vms_clone_jobs_create(...)`, `locations_sources_migrating_vms_clone_jobs_get(...)`, `locations_sources_migrating_vms_clone_jobs_list(...)`, `locations_sources_migrating_vms_create(...)`, `locations_sources_migrating_vms_cutover_jobs_cancel(...)`, `locations_sources_migrating_vms_cutover_jobs_create(...)`, `locations_sources_migrating_vms_cutover_jobs_get(...)`, `locations_sources_migrating_vms_cutover_jobs_list(...)`, `locations_sources_migrating_vms_delete(...)`, `locations_sources_migrating_vms_finalize_migration(...)`, `locations_sources_migrating_vms_get(...)`, `locations_sources_migrating_vms_list(...)`, `locations_sources_migrating_vms_patch(...)`, `locations_sources_migrating_vms_pause_migration(...)`, `locations_sources_migrating_vms_replication_cycles_get(...)`, `locations_sources_migrating_vms_replication_cycles_list(...)`, `locations_sources_migrating_vms_resume_migration(...)`, `locations_sources_migrating_vms_start_migration(...)`, `locations_sources_patch(...)`, `locations_sources_utilization_reports_create(...)`, `locations_sources_utilization_reports_delete(...)`, `locations_sources_utilization_reports_get(...)`, `locations_sources_utilization_reports_list(...)`, `locations_target_projects_create(...)`, `locations_target_projects_delete(...)`, `locations_target_projects_get(...)`, `locations_target_projects_list(...)` and `locations_target_projects_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a VMMigrationService<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a MigratingVm to a Group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `group` - Required. The full path name of the Group to add to.
    pub fn locations_groups_add_group_migration(&self, request: AddGroupMigrationRequest, group: &str) -> ProjectLocationGroupAddGroupMigrationCall<'a, S> {
        ProjectLocationGroupAddGroupMigrationCall {
            hub: self.hub,
            _request: request,
            _group: group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Group in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Group's parent.
    pub fn locations_groups_create(&self, request: Group, parent: &str) -> ProjectLocationGroupCreateCall<'a, S> {
        ProjectLocationGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Group name.
    pub fn locations_groups_delete(&self, name: &str) -> ProjectLocationGroupDeleteCall<'a, S> {
        ProjectLocationGroupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The group name.
    pub fn locations_groups_get(&self, name: &str) -> ProjectLocationGroupGetCall<'a, S> {
        ProjectLocationGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Groups in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of groups.
    pub fn locations_groups_list(&self, parent: &str) -> ProjectLocationGroupListCall<'a, S> {
        ProjectLocationGroupListCall {
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
    /// Updates the parameters of a single Group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The Group name.
    pub fn locations_groups_patch(&self, request: Group, name: &str) -> ProjectLocationGroupPatchCall<'a, S> {
        ProjectLocationGroupPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a MigratingVm from a Group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `group` - Required. The name of the Group.
    pub fn locations_groups_remove_group_migration(&self, request: RemoveGroupMigrationRequest, group: &str) -> ProjectLocationGroupRemoveGroupMigrationCall<'a, S> {
        ProjectLocationGroupRemoveGroupMigrationCall {
            hub: self.hub,
            _request: request,
            _group: group.to_string(),
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
    /// Creates a new DatacenterConnector in a given Source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The DatacenterConnector's parent. Required. The Source in where the new DatacenterConnector will be created. For example: `projects/my-project/locations/us-central1/sources/my-source`
    pub fn locations_sources_datacenter_connectors_create(&self, request: DatacenterConnector, parent: &str) -> ProjectLocationSourceDatacenterConnectorCreateCall<'a, S> {
        ProjectLocationSourceDatacenterConnectorCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _datacenter_connector_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single DatacenterConnector.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The DatacenterConnector name.
    pub fn locations_sources_datacenter_connectors_delete(&self, name: &str) -> ProjectLocationSourceDatacenterConnectorDeleteCall<'a, S> {
        ProjectLocationSourceDatacenterConnectorDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single DatacenterConnector.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DatacenterConnector.
    pub fn locations_sources_datacenter_connectors_get(&self, name: &str) -> ProjectLocationSourceDatacenterConnectorGetCall<'a, S> {
        ProjectLocationSourceDatacenterConnectorGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DatacenterConnectors in a given Source.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of connectors.
    pub fn locations_sources_datacenter_connectors_list(&self, parent: &str) -> ProjectLocationSourceDatacenterConnectorListCall<'a, S> {
        ProjectLocationSourceDatacenterConnectorListCall {
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
    /// Upgrades the appliance relate to this DatacenterConnector to the in-place updateable version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `datacenterConnector` - Required. The DatacenterConnector name.
    pub fn locations_sources_datacenter_connectors_upgrade_appliance(&self, request: UpgradeApplianceRequest, datacenter_connector: &str) -> ProjectLocationSourceDatacenterConnectorUpgradeApplianceCall<'a, S> {
        ProjectLocationSourceDatacenterConnectorUpgradeApplianceCall {
            hub: self.hub,
            _request: request,
            _datacenter_connector: datacenter_connector.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates the cancellation of a running clone job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The clone job id
    pub fn locations_sources_migrating_vms_clone_jobs_cancel(&self, request: CancelCloneJobRequest, name: &str) -> ProjectLocationSourceMigratingVmCloneJobCancelCall<'a, S> {
        ProjectLocationSourceMigratingVmCloneJobCancelCall {
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
    /// Initiates a Clone of a specific migrating VM.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Clone's parent.
    pub fn locations_sources_migrating_vms_clone_jobs_create(&self, request: CloneJob, parent: &str) -> ProjectLocationSourceMigratingVmCloneJobCreateCall<'a, S> {
        ProjectLocationSourceMigratingVmCloneJobCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _clone_job_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single CloneJob.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CloneJob.
    pub fn locations_sources_migrating_vms_clone_jobs_get(&self, name: &str) -> ProjectLocationSourceMigratingVmCloneJobGetCall<'a, S> {
        ProjectLocationSourceMigratingVmCloneJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists CloneJobs of a given migrating VM.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of source VMs.
    pub fn locations_sources_migrating_vms_clone_jobs_list(&self, parent: &str) -> ProjectLocationSourceMigratingVmCloneJobListCall<'a, S> {
        ProjectLocationSourceMigratingVmCloneJobListCall {
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
    /// Initiates the cancellation of a running cutover job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The cutover job id
    pub fn locations_sources_migrating_vms_cutover_jobs_cancel(&self, request: CancelCutoverJobRequest, name: &str) -> ProjectLocationSourceMigratingVmCutoverJobCancelCall<'a, S> {
        ProjectLocationSourceMigratingVmCutoverJobCancelCall {
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
    /// Initiates a Cutover of a specific migrating VM. The returned LRO is completed when the cutover job resource is created and the job is initiated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Cutover's parent.
    pub fn locations_sources_migrating_vms_cutover_jobs_create(&self, request: CutoverJob, parent: &str) -> ProjectLocationSourceMigratingVmCutoverJobCreateCall<'a, S> {
        ProjectLocationSourceMigratingVmCutoverJobCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _cutover_job_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single CutoverJob.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the CutoverJob.
    pub fn locations_sources_migrating_vms_cutover_jobs_get(&self, name: &str) -> ProjectLocationSourceMigratingVmCutoverJobGetCall<'a, S> {
        ProjectLocationSourceMigratingVmCutoverJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists CutoverJobs of a given migrating VM.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of migrating VMs.
    pub fn locations_sources_migrating_vms_cutover_jobs_list(&self, parent: &str) -> ProjectLocationSourceMigratingVmCutoverJobListCall<'a, S> {
        ProjectLocationSourceMigratingVmCutoverJobListCall {
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
    /// Gets details of a single ReplicationCycle.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ReplicationCycle.
    pub fn locations_sources_migrating_vms_replication_cycles_get(&self, name: &str) -> ProjectLocationSourceMigratingVmReplicationCycleGetCall<'a, S> {
        ProjectLocationSourceMigratingVmReplicationCycleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists ReplicationCycles in a given MigratingVM.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of ReplicationCycles.
    pub fn locations_sources_migrating_vms_replication_cycles_list(&self, parent: &str) -> ProjectLocationSourceMigratingVmReplicationCycleListCall<'a, S> {
        ProjectLocationSourceMigratingVmReplicationCycleListCall {
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
    /// Creates a new MigratingVm in a given Source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The MigratingVm's parent.
    pub fn locations_sources_migrating_vms_create(&self, request: MigratingVm, parent: &str) -> ProjectLocationSourceMigratingVmCreateCall<'a, S> {
        ProjectLocationSourceMigratingVmCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _migrating_vm_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single MigratingVm.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the MigratingVm.
    pub fn locations_sources_migrating_vms_delete(&self, name: &str) -> ProjectLocationSourceMigratingVmDeleteCall<'a, S> {
        ProjectLocationSourceMigratingVmDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks a migration as completed, deleting migration resources that are no longer being used. Only applicable after cutover is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `migratingVm` - Required. The name of the MigratingVm.
    pub fn locations_sources_migrating_vms_finalize_migration(&self, request: FinalizeMigrationRequest, migrating_vm: &str) -> ProjectLocationSourceMigratingVmFinalizeMigrationCall<'a, S> {
        ProjectLocationSourceMigratingVmFinalizeMigrationCall {
            hub: self.hub,
            _request: request,
            _migrating_vm: migrating_vm.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single MigratingVm.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the MigratingVm.
    pub fn locations_sources_migrating_vms_get(&self, name: &str) -> ProjectLocationSourceMigratingVmGetCall<'a, S> {
        ProjectLocationSourceMigratingVmGetCall {
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
    /// Lists MigratingVms in a given Source.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of MigratingVms.
    pub fn locations_sources_migrating_vms_list(&self, parent: &str) -> ProjectLocationSourceMigratingVmListCall<'a, S> {
        ProjectLocationSourceMigratingVmListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Updates the parameters of a single MigratingVm.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The identifier of the MigratingVm.
    pub fn locations_sources_migrating_vms_patch(&self, request: MigratingVm, name: &str) -> ProjectLocationSourceMigratingVmPatchCall<'a, S> {
        ProjectLocationSourceMigratingVmPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Pauses a migration for a VM. If cycle tasks are running they will be cancelled, preserving source task data. Further replication cycles will not be triggered while the VM is paused.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `migratingVm` - Required. The name of the MigratingVm.
    pub fn locations_sources_migrating_vms_pause_migration(&self, request: PauseMigrationRequest, migrating_vm: &str) -> ProjectLocationSourceMigratingVmPauseMigrationCall<'a, S> {
        ProjectLocationSourceMigratingVmPauseMigrationCall {
            hub: self.hub,
            _request: request,
            _migrating_vm: migrating_vm.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resumes a migration for a VM. When called on a paused migration, will start the process of uploading data and creating snapshots; when called on a completed cut-over migration, will update the migration to active state and start the process of uploading data and creating snapshots.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `migratingVm` - Required. The name of the MigratingVm.
    pub fn locations_sources_migrating_vms_resume_migration(&self, request: ResumeMigrationRequest, migrating_vm: &str) -> ProjectLocationSourceMigratingVmResumeMigrationCall<'a, S> {
        ProjectLocationSourceMigratingVmResumeMigrationCall {
            hub: self.hub,
            _request: request,
            _migrating_vm: migrating_vm.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts migration for a VM. Starts the process of uploading data and creating snapshots, in replication cycles scheduled by the policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `migratingVm` - Required. The name of the MigratingVm.
    pub fn locations_sources_migrating_vms_start_migration(&self, request: StartMigrationRequest, migrating_vm: &str) -> ProjectLocationSourceMigratingVmStartMigrationCall<'a, S> {
        ProjectLocationSourceMigratingVmStartMigrationCall {
            hub: self.hub,
            _request: request,
            _migrating_vm: migrating_vm.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new UtilizationReport.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Utilization Report's parent.
    pub fn locations_sources_utilization_reports_create(&self, request: UtilizationReport, parent: &str) -> ProjectLocationSourceUtilizationReportCreateCall<'a, S> {
        ProjectLocationSourceUtilizationReportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _utilization_report_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Utilization Report.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Utilization Report name.
    pub fn locations_sources_utilization_reports_delete(&self, name: &str) -> ProjectLocationSourceUtilizationReportDeleteCall<'a, S> {
        ProjectLocationSourceUtilizationReportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single Utilization Report.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Utilization Report name.
    pub fn locations_sources_utilization_reports_get(&self, name: &str) -> ProjectLocationSourceUtilizationReportGetCall<'a, S> {
        ProjectLocationSourceUtilizationReportGetCall {
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
    /// Lists Utilization Reports of the given Source.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The Utilization Reports parent.
    pub fn locations_sources_utilization_reports_list(&self, parent: &str) -> ProjectLocationSourceUtilizationReportListCall<'a, S> {
        ProjectLocationSourceUtilizationReportListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Creates a new Source in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Source's parent.
    pub fn locations_sources_create(&self, request: Source, parent: &str) -> ProjectLocationSourceCreateCall<'a, S> {
        ProjectLocationSourceCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _source_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Source.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Source name.
    pub fn locations_sources_delete(&self, name: &str) -> ProjectLocationSourceDeleteCall<'a, S> {
        ProjectLocationSourceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List remote source's inventory of VMs. The remote source is the onprem vCenter (remote in the sense it's not in Compute Engine). The inventory describes the list of existing VMs in that source. Note that this operation lists the VMs on the remote source, as opposed to listing the MigratingVms resources in the vmmigration service.
    /// 
    /// # Arguments
    ///
    /// * `source` - Required. The name of the Source.
    pub fn locations_sources_fetch_inventory(&self, source: &str) -> ProjectLocationSourceFetchInventoryCall<'a, S> {
        ProjectLocationSourceFetchInventoryCall {
            hub: self.hub,
            _source: source.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _force_refresh: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Source.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Source name.
    pub fn locations_sources_get(&self, name: &str) -> ProjectLocationSourceGetCall<'a, S> {
        ProjectLocationSourceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Sources in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of sources.
    pub fn locations_sources_list(&self, parent: &str) -> ProjectLocationSourceListCall<'a, S> {
        ProjectLocationSourceListCall {
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
    /// Updates the parameters of a single Source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The Source name.
    pub fn locations_sources_patch(&self, request: Source, name: &str) -> ProjectLocationSourcePatchCall<'a, S> {
        ProjectLocationSourcePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new TargetProject in a given project. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The TargetProject's parent.
    pub fn locations_target_projects_create(&self, request: TargetProject, parent: &str) -> ProjectLocationTargetProjectCreateCall<'a, S> {
        ProjectLocationTargetProjectCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _target_project_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single TargetProject. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The TargetProject name.
    pub fn locations_target_projects_delete(&self, name: &str) -> ProjectLocationTargetProjectDeleteCall<'a, S> {
        ProjectLocationTargetProjectDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single TargetProject. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The TargetProject name.
    pub fn locations_target_projects_get(&self, name: &str) -> ProjectLocationTargetProjectGetCall<'a, S> {
        ProjectLocationTargetProjectGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists TargetProjects in a given project. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of targets.
    pub fn locations_target_projects_list(&self, parent: &str) -> ProjectLocationTargetProjectListCall<'a, S> {
        ProjectLocationTargetProjectListCall {
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
    /// Updates the parameters of a single TargetProject. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The name of the target project.
    pub fn locations_target_projects_patch(&self, request: TargetProject, name: &str) -> ProjectLocationTargetProjectPatchCall<'a, S> {
        ProjectLocationTargetProjectPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
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



