use super::*;
/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`BigtableAdmin`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigtableadmin2 as bigtableadmin2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigtableadmin2::{BigtableAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = BigtableAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)` and `projects_operations_list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a BigtableAdmin<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn projects_operations_list(&self, name: &str) -> OperationProjectOperationListCall<'a, S> {
        OperationProjectOperationListCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, name: &str) -> OperationCancelCall<'a, S> {
        OperationCancelCall {
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
    pub fn delete(&self, name: &str) -> OperationDeleteCall<'a, S> {
        OperationDeleteCall {
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
/// It is not used directly, but through the [`BigtableAdmin`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigtableadmin2 as bigtableadmin2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigtableadmin2::{BigtableAdmin, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = BigtableAdmin::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `instances_app_profiles_create(...)`, `instances_app_profiles_delete(...)`, `instances_app_profiles_get(...)`, `instances_app_profiles_list(...)`, `instances_app_profiles_patch(...)`, `instances_clusters_backups_copy(...)`, `instances_clusters_backups_create(...)`, `instances_clusters_backups_delete(...)`, `instances_clusters_backups_get(...)`, `instances_clusters_backups_get_iam_policy(...)`, `instances_clusters_backups_list(...)`, `instances_clusters_backups_patch(...)`, `instances_clusters_backups_set_iam_policy(...)`, `instances_clusters_backups_test_iam_permissions(...)`, `instances_clusters_create(...)`, `instances_clusters_delete(...)`, `instances_clusters_get(...)`, `instances_clusters_hot_tablets_list(...)`, `instances_clusters_list(...)`, `instances_clusters_partial_update_cluster(...)`, `instances_clusters_update(...)`, `instances_create(...)`, `instances_delete(...)`, `instances_get(...)`, `instances_get_iam_policy(...)`, `instances_list(...)`, `instances_partial_update_instance(...)`, `instances_set_iam_policy(...)`, `instances_tables_check_consistency(...)`, `instances_tables_create(...)`, `instances_tables_delete(...)`, `instances_tables_drop_row_range(...)`, `instances_tables_generate_consistency_token(...)`, `instances_tables_get(...)`, `instances_tables_get_iam_policy(...)`, `instances_tables_list(...)`, `instances_tables_modify_column_families(...)`, `instances_tables_patch(...)`, `instances_tables_restore(...)`, `instances_tables_set_iam_policy(...)`, `instances_tables_test_iam_permissions(...)`, `instances_tables_undelete(...)`, `instances_test_iam_permissions(...)`, `instances_update(...)`, `locations_get(...)` and `locations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a BigtableAdmin<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an app profile within an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The unique name of the instance in which to create the new app profile. Values are of the form `projects/{project}/instances/{instance}`.
    pub fn instances_app_profiles_create(&self, request: AppProfile, parent: &str) -> ProjectInstanceAppProfileCreateCall<'a, S> {
        ProjectInstanceAppProfileCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _ignore_warnings: Default::default(),
            _app_profile_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an app profile from an instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the app profile to be deleted. Values are of the form `projects/{project}/instances/{instance}/appProfiles/{app_profile}`.
    pub fn instances_app_profiles_delete(&self, name: &str) -> ProjectInstanceAppProfileDeleteCall<'a, S> {
        ProjectInstanceAppProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _ignore_warnings: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about an app profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the requested app profile. Values are of the form `projects/{project}/instances/{instance}/appProfiles/{app_profile}`.
    pub fn instances_app_profiles_get(&self, name: &str) -> ProjectInstanceAppProfileGetCall<'a, S> {
        ProjectInstanceAppProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about app profiles in an instance.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The unique name of the instance for which a list of app profiles is requested. Values are of the form `projects/{project}/instances/{instance}`. Use `{instance} = '-'` to list AppProfiles for all Instances in a project, e.g., `projects/myproject/instances/-`.
    pub fn instances_app_profiles_list(&self, parent: &str) -> ProjectInstanceAppProfileListCall<'a, S> {
        ProjectInstanceAppProfileListCall {
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
    /// Updates an app profile within an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique name of the app profile. Values are of the form `projects/{project}/instances/{instance}/appProfiles/_a-zA-Z0-9*`.
    pub fn instances_app_profiles_patch(&self, request: AppProfile, name: &str) -> ProjectInstanceAppProfilePatchCall<'a, S> {
        ProjectInstanceAppProfilePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _ignore_warnings: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copy a Cloud Bigtable backup to a new backup in the destination cluster located in the destination instance and project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the destination cluster that will contain the backup copy. The cluster must already exists. Values are of the form: `projects/{project}/instances/{instance}/clusters/{cluster}`.
    pub fn instances_clusters_backups_copy(&self, request: CopyBackupRequest, parent: &str) -> ProjectInstanceClusterBackupCopyCall<'a, S> {
        ProjectInstanceClusterBackupCopyCall {
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
    /// Starts creating a new Cloud Bigtable Backup. The returned backup long-running operation can be used to track creation of the backup. The metadata field type is CreateBackupMetadata. The response field type is Backup, if successful. Cancelling the returned operation will stop the creation and delete the backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. This must be one of the clusters in the instance in which this table is located. The backup will be stored in this cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    pub fn instances_clusters_backups_create(&self, request: Backup, parent: &str) -> ProjectInstanceClusterBackupCreateCall<'a, S> {
        ProjectInstanceClusterBackupCreateCall {
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
    /// Deletes a pending or completed Cloud Bigtable backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the backup to delete. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup}`.
    pub fn instances_clusters_backups_delete(&self, name: &str) -> ProjectInstanceClusterBackupDeleteCall<'a, S> {
        ProjectInstanceClusterBackupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets metadata on a pending or completed Cloud Bigtable Backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the backup. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup}`.
    pub fn instances_clusters_backups_get(&self, name: &str) -> ProjectInstanceClusterBackupGetCall<'a, S> {
        ProjectInstanceClusterBackupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a Table resource. Returns an empty policy if the resource exists but does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_clusters_backups_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectInstanceClusterBackupGetIamPolicyCall<'a, S> {
        ProjectInstanceClusterBackupGetIamPolicyCall {
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
    /// Lists Cloud Bigtable backups. Returns both completed and pending backups.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The cluster to list backups from. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}`. Use `{cluster} = '-'` to list backups for all clusters in an instance, e.g., `projects/{project}/instances/{instance}/clusters/-`.
    pub fn instances_clusters_backups_list(&self, parent: &str) -> ProjectInstanceClusterBackupListCall<'a, S> {
        ProjectInstanceClusterBackupListCall {
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
    /// Updates a pending or completed Cloud Bigtable Backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A globally unique identifier for the backup which cannot be changed. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/ backups/_a-zA-Z0-9*` The final segment of the name must be between 1 and 50 characters in length. The backup is stored in the cluster identified by the prefix of the backup name of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    pub fn instances_clusters_backups_patch(&self, request: Backup, name: &str) -> ProjectInstanceClusterBackupPatchCall<'a, S> {
        ProjectInstanceClusterBackupPatchCall {
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
    /// Sets the access control policy on a Table resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_clusters_backups_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectInstanceClusterBackupSetIamPolicyCall<'a, S> {
        ProjectInstanceClusterBackupSetIamPolicyCall {
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
    /// Returns permissions that the caller has on the specified table resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_clusters_backups_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectInstanceClusterBackupTestIamPermissionCall<'a, S> {
        ProjectInstanceClusterBackupTestIamPermissionCall {
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
    /// Lists hot tablets in a cluster, within the time range provided. Hot tablets are ordered based on CPU usage.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The cluster name to list hot tablets. Value is in the following form: `projects/{project}/instances/{instance}/clusters/{cluster}`.
    pub fn instances_clusters_hot_tablets_list(&self, parent: &str) -> ProjectInstanceClusterHotTabletListCall<'a, S> {
        ProjectInstanceClusterHotTabletListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _start_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _end_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a cluster within an instance. Note that exactly one of Cluster.serve_nodes and Cluster.cluster_config.cluster_autoscaling_config can be set. If serve_nodes is set to non-zero, then the cluster is manually scaled. If cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is enabled.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The unique name of the instance in which to create the new cluster. Values are of the form `projects/{project}/instances/{instance}`.
    pub fn instances_clusters_create(&self, request: Cluster, parent: &str) -> ProjectInstanceClusterCreateCall<'a, S> {
        ProjectInstanceClusterCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _cluster_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a cluster from an instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the cluster to be deleted. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    pub fn instances_clusters_delete(&self, name: &str) -> ProjectInstanceClusterDeleteCall<'a, S> {
        ProjectInstanceClusterDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a cluster.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the requested cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    pub fn instances_clusters_get(&self, name: &str) -> ProjectInstanceClusterGetCall<'a, S> {
        ProjectInstanceClusterGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about clusters in an instance.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The unique name of the instance for which a list of clusters is requested. Values are of the form `projects/{project}/instances/{instance}`. Use `{instance} = '-'` to list Clusters for all Instances in a project, e.g., `projects/myproject/instances/-`.
    pub fn instances_clusters_list(&self, parent: &str) -> ProjectInstanceClusterListCall<'a, S> {
        ProjectInstanceClusterListCall {
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
    /// Partially updates a cluster within a project. This method is the preferred way to update a Cluster. To enable and update autoscaling, set cluster_config.cluster_autoscaling_config. When autoscaling is enabled, serve_nodes is treated as an OUTPUT_ONLY field, meaning that updates to it are ignored. Note that an update cannot simultaneously set serve_nodes to non-zero and cluster_config.cluster_autoscaling_config to non-empty, and also specify both in the update_mask. To disable autoscaling, clear cluster_config.cluster_autoscaling_config, and explicitly set a serve_node count via the update_mask.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique name of the cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/a-z*`.
    pub fn instances_clusters_partial_update_cluster(&self, request: Cluster, name: &str) -> ProjectInstanceClusterPartialUpdateClusterCall<'a, S> {
        ProjectInstanceClusterPartialUpdateClusterCall {
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
    /// Updates a cluster within an instance. Note that UpdateCluster does not support updating cluster_config.cluster_autoscaling_config. In order to update it, you must use PartialUpdateCluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique name of the cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/a-z*`.
    pub fn instances_clusters_update(&self, request: Cluster, name: &str) -> ProjectInstanceClusterUpdateCall<'a, S> {
        ProjectInstanceClusterUpdateCall {
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
    /// Checks replication consistency based on a consistency token, that is, if replication has caught up based on the conditions specified in the token and the check request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique name of the Table for which to check replication consistency. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
    pub fn instances_tables_check_consistency(&self, request: CheckConsistencyRequest, name: &str) -> ProjectInstanceTableCheckConsistencyCall<'a, S> {
        ProjectInstanceTableCheckConsistencyCall {
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
    /// Creates a new table in the specified instance. The table can be created with a full set of initial column families, specified in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The unique name of the instance in which to create the table. Values are of the form `projects/{project}/instances/{instance}`.
    pub fn instances_tables_create(&self, request: CreateTableRequest, parent: &str) -> ProjectInstanceTableCreateCall<'a, S> {
        ProjectInstanceTableCreateCall {
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
    /// Permanently deletes a specified table and all of its data.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the table to be deleted. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
    pub fn instances_tables_delete(&self, name: &str) -> ProjectInstanceTableDeleteCall<'a, S> {
        ProjectInstanceTableDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently drop/delete a row range from a specified table. The request can specify whether to delete all rows in a table, or only those that match a particular prefix.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique name of the table on which to drop a range of rows. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
    pub fn instances_tables_drop_row_range(&self, request: DropRowRangeRequest, name: &str) -> ProjectInstanceTableDropRowRangeCall<'a, S> {
        ProjectInstanceTableDropRowRangeCall {
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
    /// Generates a consistency token for a Table, which can be used in CheckConsistency to check whether mutations to the table that finished before this call started have been replicated. The tokens will be available for 90 days.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique name of the Table for which to create a consistency token. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
    pub fn instances_tables_generate_consistency_token(&self, request: GenerateConsistencyTokenRequest, name: &str) -> ProjectInstanceTableGenerateConsistencyTokenCall<'a, S> {
        ProjectInstanceTableGenerateConsistencyTokenCall {
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
    /// Gets metadata information about the specified table.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the requested table. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
    pub fn instances_tables_get(&self, name: &str) -> ProjectInstanceTableGetCall<'a, S> {
        ProjectInstanceTableGetCall {
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
    /// Gets the access control policy for a Table resource. Returns an empty policy if the resource exists but does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_tables_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectInstanceTableGetIamPolicyCall<'a, S> {
        ProjectInstanceTableGetIamPolicyCall {
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
    /// Lists all tables served from a specified instance.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The unique name of the instance for which tables should be listed. Values are of the form `projects/{project}/instances/{instance}`.
    pub fn instances_tables_list(&self, parent: &str) -> ProjectInstanceTableListCall<'a, S> {
        ProjectInstanceTableListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a series of column family modifications on the specified table. Either all or none of the modifications will occur before this method returns, but data requests received prior to that point may see a table where only some modifications have taken effect.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique name of the table whose families should be modified. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
    pub fn instances_tables_modify_column_families(&self, request: ModifyColumnFamiliesRequest, name: &str) -> ProjectInstanceTableModifyColumnFamilyCall<'a, S> {
        ProjectInstanceTableModifyColumnFamilyCall {
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
    /// Updates a specified table.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique name of the table. Values are of the form `projects/{project}/instances/{instance}/tables/_a-zA-Z0-9*`. Views: `NAME_ONLY`, `SCHEMA_VIEW`, `REPLICATION_VIEW`, `STATS_VIEW`, `FULL`
    pub fn instances_tables_patch(&self, request: Table, name: &str) -> ProjectInstanceTablePatchCall<'a, S> {
        ProjectInstanceTablePatchCall {
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
    /// Create a new table by restoring from a completed backup. The returned table long-running operation can be used to track the progress of the operation, and to cancel it. The metadata field type is RestoreTableMetadata. The response type is Table, if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the instance in which to create the restored table. Values are of the form `projects//instances/`.
    pub fn instances_tables_restore(&self, request: RestoreTableRequest, parent: &str) -> ProjectInstanceTableRestoreCall<'a, S> {
        ProjectInstanceTableRestoreCall {
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
    /// Sets the access control policy on a Table resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_tables_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectInstanceTableSetIamPolicyCall<'a, S> {
        ProjectInstanceTableSetIamPolicyCall {
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
    /// Returns permissions that the caller has on the specified table resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_tables_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectInstanceTableTestIamPermissionCall<'a, S> {
        ProjectInstanceTableTestIamPermissionCall {
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
    /// Restores a specified table which was accidentally deleted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The unique name of the table to be restored. Values are of the form `projects/{project}/instances/{instance}/tables/{table}`.
    pub fn instances_tables_undelete(&self, request: UndeleteTableRequest, name: &str) -> ProjectInstanceTableUndeleteCall<'a, S> {
        ProjectInstanceTableUndeleteCall {
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
    /// Create an instance within a project. Note that exactly one of Cluster.serve_nodes and Cluster.cluster_config.cluster_autoscaling_config can be set. If serve_nodes is set to non-zero, then the cluster is manually scaled. If cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is enabled.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The unique name of the project in which to create the new instance. Values are of the form `projects/{project}`.
    pub fn instances_create(&self, request: CreateInstanceRequest, parent: &str) -> ProjectInstanceCreateCall<'a, S> {
        ProjectInstanceCreateCall {
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
    /// Delete an instance from a project.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the instance to be deleted. Values are of the form `projects/{project}/instances/{instance}`.
    pub fn instances_delete(&self, name: &str) -> ProjectInstanceDeleteCall<'a, S> {
        ProjectInstanceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about an instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the requested instance. Values are of the form `projects/{project}/instances/{instance}`.
    pub fn instances_get(&self, name: &str) -> ProjectInstanceGetCall<'a, S> {
        ProjectInstanceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for an instance resource. Returns an empty policy if an instance exists but does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectInstanceGetIamPolicyCall<'a, S> {
        ProjectInstanceGetIamPolicyCall {
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
    /// Lists information about instances in a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The unique name of the project for which a list of instances is requested. Values are of the form `projects/{project}`.
    pub fn instances_list(&self, parent: &str) -> ProjectInstanceListCall<'a, S> {
        ProjectInstanceListCall {
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
    /// Partially updates an instance within a project. This method can modify all fields of an Instance and is the preferred way to update an Instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique name of the instance. Values are of the form `projects/{project}/instances/a-z+[a-z0-9]`.
    pub fn instances_partial_update_instance(&self, request: Instance, name: &str) -> ProjectInstancePartialUpdateInstanceCall<'a, S> {
        ProjectInstancePartialUpdateInstanceCall {
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
    /// Sets the access control policy on an instance resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectInstanceSetIamPolicyCall<'a, S> {
        ProjectInstanceSetIamPolicyCall {
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
    /// Returns permissions that the caller has on the specified instance resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn instances_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectInstanceTestIamPermissionCall<'a, S> {
        ProjectInstanceTestIamPermissionCall {
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
    /// Updates an instance within a project. This method updates only the display name and type for an Instance. To update other Instance properties, such as labels, use PartialUpdateInstance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The unique name of the instance. Values are of the form `projects/{project}/instances/a-z+[a-z0-9]`.
    pub fn instances_update(&self, request: Instance, name: &str) -> ProjectInstanceUpdateCall<'a, S> {
        ProjectInstanceUpdateCall {
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



