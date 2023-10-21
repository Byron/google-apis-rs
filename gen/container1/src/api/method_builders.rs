use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Container`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_container1 as container1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use container1::{Container, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Container::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_usable_subnetworks_list(...)`, `locations_clusters_complete_ip_rotation(...)`, `locations_clusters_create(...)`, `locations_clusters_delete(...)`, `locations_clusters_get(...)`, `locations_clusters_get_jwks(...)`, `locations_clusters_list(...)`, `locations_clusters_node_pools_complete_upgrade(...)`, `locations_clusters_node_pools_create(...)`, `locations_clusters_node_pools_delete(...)`, `locations_clusters_node_pools_get(...)`, `locations_clusters_node_pools_list(...)`, `locations_clusters_node_pools_rollback(...)`, `locations_clusters_node_pools_set_autoscaling(...)`, `locations_clusters_node_pools_set_management(...)`, `locations_clusters_node_pools_set_size(...)`, `locations_clusters_node_pools_update(...)`, `locations_clusters_set_addons(...)`, `locations_clusters_set_legacy_abac(...)`, `locations_clusters_set_locations(...)`, `locations_clusters_set_logging(...)`, `locations_clusters_set_maintenance_policy(...)`, `locations_clusters_set_master_auth(...)`, `locations_clusters_set_monitoring(...)`, `locations_clusters_set_network_policy(...)`, `locations_clusters_set_resource_labels(...)`, `locations_clusters_start_ip_rotation(...)`, `locations_clusters_update(...)`, `locations_clusters_update_master(...)`, `locations_clusters_well_known_get_openid_configuration(...)`, `locations_get_server_config(...)`, `locations_operations_cancel(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `zones_clusters_addons(...)`, `zones_clusters_complete_ip_rotation(...)`, `zones_clusters_create(...)`, `zones_clusters_delete(...)`, `zones_clusters_get(...)`, `zones_clusters_legacy_abac(...)`, `zones_clusters_list(...)`, `zones_clusters_locations(...)`, `zones_clusters_logging(...)`, `zones_clusters_master(...)`, `zones_clusters_monitoring(...)`, `zones_clusters_node_pools_autoscaling(...)`, `zones_clusters_node_pools_create(...)`, `zones_clusters_node_pools_delete(...)`, `zones_clusters_node_pools_get(...)`, `zones_clusters_node_pools_list(...)`, `zones_clusters_node_pools_rollback(...)`, `zones_clusters_node_pools_set_management(...)`, `zones_clusters_node_pools_set_size(...)`, `zones_clusters_node_pools_update(...)`, `zones_clusters_resource_labels(...)`, `zones_clusters_set_maintenance_policy(...)`, `zones_clusters_set_master_auth(...)`, `zones_clusters_set_network_policy(...)`, `zones_clusters_start_ip_rotation(...)`, `zones_clusters_update(...)`, `zones_get_serverconfig(...)`, `zones_operations_cancel(...)`, `zones_operations_get(...)` and `zones_operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Container<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists subnetworks that are usable for creating clusters in a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent project where subnetworks are usable. Specified in the format `projects/*`.
    pub fn aggregated_usable_subnetworks_list(&self, parent: &str) -> ProjectAggregatedUsableSubnetworkListCall<'a, S> {
        ProjectAggregatedUsableSubnetworkListCall {
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
    /// CompleteNodePoolUpgrade will signal an on-going node pool upgrade to complete.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster, node pool id) of the node pool to complete upgrade. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_complete_upgrade(&self, request: CompleteNodePoolUpgradeRequest, name: &str) -> ProjectLocationClusterNodePoolCompleteUpgradeCall<'a, S> {
        ProjectLocationClusterNodePoolCompleteUpgradeCall {
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
    /// Creates a node pool for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The parent (project, location, cluster name) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_node_pools_create(&self, request: CreateNodePoolRequest, parent: &str) -> ProjectLocationClusterNodePoolCreateCall<'a, S> {
        ProjectLocationClusterNodePoolCreateCall {
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
    /// Deletes a node pool from a cluster.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name (project, location, cluster, node pool id) of the node pool to delete. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_delete(&self, name: &str) -> ProjectLocationClusterNodePoolDeleteCall<'a, S> {
        ProjectLocationClusterNodePoolDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _node_pool_id: Default::default(),
            _cluster_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the requested node pool.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name (project, location, cluster, node pool id) of the node pool to get. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_get(&self, name: &str) -> ProjectLocationClusterNodePoolGetCall<'a, S> {
        ProjectLocationClusterNodePoolGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _node_pool_id: Default::default(),
            _cluster_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the node pools for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent (project, location, cluster name) where the node pools will be listed. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_node_pools_list(&self, parent: &str) -> ProjectLocationClusterNodePoolListCall<'a, S> {
        ProjectLocationClusterNodePoolListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _cluster_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rolls back a previously Aborted or Failed NodePool upgrade. This makes no changes if the last upgrade successfully completed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster, node pool id) of the node poll to rollback upgrade. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_rollback(&self, request: RollbackNodePoolUpgradeRequest, name: &str) -> ProjectLocationClusterNodePoolRollbackCall<'a, S> {
        ProjectLocationClusterNodePoolRollbackCall {
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
    /// Sets the autoscaling settings for the specified node pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster, node pool) of the node pool to set autoscaler settings. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_set_autoscaling(&self, request: SetNodePoolAutoscalingRequest, name: &str) -> ProjectLocationClusterNodePoolSetAutoscalingCall<'a, S> {
        ProjectLocationClusterNodePoolSetAutoscalingCall {
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
    /// Sets the NodeManagement options for a node pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster, node pool id) of the node pool to set management properties. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_set_management(&self, request: SetNodePoolManagementRequest, name: &str) -> ProjectLocationClusterNodePoolSetManagementCall<'a, S> {
        ProjectLocationClusterNodePoolSetManagementCall {
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
    /// Sets the size for a specific node pool. The new size will be used for all replicas, including future replicas created by modifying NodePool.locations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster, node pool id) of the node pool to set size. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_set_size(&self, request: SetNodePoolSizeRequest, name: &str) -> ProjectLocationClusterNodePoolSetSizeCall<'a, S> {
        ProjectLocationClusterNodePoolSetSizeCall {
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
    /// Updates the version and/or image type for the specified node pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster, node pool) of the node pool to update. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    pub fn locations_clusters_node_pools_update(&self, request: UpdateNodePoolRequest, name: &str) -> ProjectLocationClusterNodePoolUpdateCall<'a, S> {
        ProjectLocationClusterNodePoolUpdateCall {
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
    /// Gets the OIDC discovery document for the cluster. See the [OpenID Connect Discovery 1.0 specification](https://openid.net/specs/openid-connect-discovery-1_0.html) for details. This API is not yet intended for general use, and is not available for all clusters.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The cluster (project, location, cluster name) to get the discovery document for. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_well_known_get_openid_configuration(&self, parent: &str) -> ProjectLocationClusterWellKnownGetOpenidConfigurationCall<'a, S> {
        ProjectLocationClusterWellKnownGetOpenidConfigurationCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes master IP rotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster name) of the cluster to complete IP rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_complete_ip_rotation(&self, request: CompleteIPRotationRequest, name: &str) -> ProjectLocationClusterCompleteIpRotationCall<'a, S> {
        ProjectLocationClusterCompleteIpRotationCall {
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
    /// Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the Kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`.
    pub fn locations_clusters_create(&self, request: CreateClusterRequest, parent: &str) -> ProjectLocationClusterCreateCall<'a, S> {
        ProjectLocationClusterCreateCall {
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
    /// Deletes the cluster, including the Kubernetes endpoint and all worker nodes. Firewalls and routes that were configured during cluster creation are also deleted. Other Google Compute Engine resources that might be in use by the cluster, such as load balancer resources, are not deleted if they weren't present when the cluster was initially created.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name (project, location, cluster) of the cluster to delete. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_delete(&self, name: &str) -> ProjectLocationClusterDeleteCall<'a, S> {
        ProjectLocationClusterDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _cluster_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name (project, location, cluster) of the cluster to retrieve. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_get(&self, name: &str) -> ProjectLocationClusterGetCall<'a, S> {
        ProjectLocationClusterGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _cluster_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the public component of the cluster signing keys in JSON Web Key format. This API is not yet intended for general use, and is not available for all clusters.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The cluster (project, location, cluster name) to get keys for. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_get_jwks(&self, parent: &str) -> ProjectLocationClusterGetJwkCall<'a, S> {
        ProjectLocationClusterGetJwkCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all clusters owned by a project in either the specified zone or all zones.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent (project and location) where the clusters will be listed. Specified in the format `projects/*/locations/*`. Location "-" matches all zones and all regions.
    pub fn locations_clusters_list(&self, parent: &str) -> ProjectLocationClusterListCall<'a, S> {
        ProjectLocationClusterListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the addons for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster) of the cluster to set addons. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_addons(&self, request: SetAddonsConfigRequest, name: &str) -> ProjectLocationClusterSetAddonCall<'a, S> {
        ProjectLocationClusterSetAddonCall {
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
    /// Enables or disables the ABAC authorization mechanism on a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster name) of the cluster to set legacy abac. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_legacy_abac(&self, request: SetLegacyAbacRequest, name: &str) -> ProjectLocationClusterSetLegacyAbacCall<'a, S> {
        ProjectLocationClusterSetLegacyAbacCall {
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
    /// Sets the locations for a specific cluster. Deprecated. Use [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/update) instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster) of the cluster to set locations. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_locations(&self, request: SetLocationsRequest, name: &str) -> ProjectLocationClusterSetLocationCall<'a, S> {
        ProjectLocationClusterSetLocationCall {
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
    /// Sets the logging service for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster) of the cluster to set logging. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_logging(&self, request: SetLoggingServiceRequest, name: &str) -> ProjectLocationClusterSetLoggingCall<'a, S> {
        ProjectLocationClusterSetLoggingCall {
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
    /// Sets the maintenance policy for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster name) of the cluster to set maintenance policy. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_maintenance_policy(&self, request: SetMaintenancePolicyRequest, name: &str) -> ProjectLocationClusterSetMaintenancePolicyCall<'a, S> {
        ProjectLocationClusterSetMaintenancePolicyCall {
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
    /// Sets master auth materials. Currently supports changing the admin password or a specific cluster, either via password generation or explicitly setting the password.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster) of the cluster to set auth. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_master_auth(&self, request: SetMasterAuthRequest, name: &str) -> ProjectLocationClusterSetMasterAuthCall<'a, S> {
        ProjectLocationClusterSetMasterAuthCall {
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
    /// Sets the monitoring service for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster) of the cluster to set monitoring. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_monitoring(&self, request: SetMonitoringServiceRequest, name: &str) -> ProjectLocationClusterSetMonitoringCall<'a, S> {
        ProjectLocationClusterSetMonitoringCall {
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
    /// Enables or disables Network Policy for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster name) of the cluster to set networking policy. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_network_policy(&self, request: SetNetworkPolicyRequest, name: &str) -> ProjectLocationClusterSetNetworkPolicyCall<'a, S> {
        ProjectLocationClusterSetNetworkPolicyCall {
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
    /// Sets labels on a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster name) of the cluster to set labels. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_set_resource_labels(&self, request: SetLabelsRequest, name: &str) -> ProjectLocationClusterSetResourceLabelCall<'a, S> {
        ProjectLocationClusterSetResourceLabelCall {
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
    /// Starts master IP rotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster name) of the cluster to start IP rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_start_ip_rotation(&self, request: StartIPRotationRequest, name: &str) -> ProjectLocationClusterStartIpRotationCall<'a, S> {
        ProjectLocationClusterStartIpRotationCall {
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
    /// Updates the settings of a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_update(&self, request: UpdateClusterRequest, name: &str) -> ProjectLocationClusterUpdateCall<'a, S> {
        ProjectLocationClusterUpdateCall {
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
    /// Updates the master for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`.
    pub fn locations_clusters_update_master(&self, request: UpdateMasterRequest, name: &str) -> ProjectLocationClusterUpdateMasterCall<'a, S> {
        ProjectLocationClusterUpdateMasterCall {
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
    /// Cancels the specified operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`.
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
    /// Gets the specified operation.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name (project, location, operation id) of the operation to get. Specified in the format `projects/*/locations/*/operations/*`.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _operation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all operations in a project in a specific zone or all zones.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent (project and location) where the operations will be listed. Specified in the format `projects/*/locations/*`. Location "-" matches all zones and all regions.
    pub fn locations_operations_list(&self, parent: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns configuration info about the Google Kubernetes Engine service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name (project and location) of the server config to get, specified in the format `projects/*/locations/*`.
    pub fn locations_get_server_config(&self, name: &str) -> ProjectLocationGetServerConfigCall<'a, S> {
        ProjectLocationGetServerConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _zone: Default::default(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the autoscaling settings for the specified node pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    /// * `nodePoolId` - Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_node_pools_autoscaling(&self, request: SetNodePoolAutoscalingRequest, project_id: &str, zone: &str, cluster_id: &str, node_pool_id: &str) -> ProjectZoneClusterNodePoolAutoscalingCall<'a, S> {
        ProjectZoneClusterNodePoolAutoscalingCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _node_pool_id: node_pool_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a node pool for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field.
    pub fn zones_clusters_node_pools_create(&self, request: CreateNodePoolRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterNodePoolCreateCall<'a, S> {
        ProjectZoneClusterNodePoolCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a node pool from a cluster.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    /// * `nodePoolId` - Deprecated. The name of the node pool to delete. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_node_pools_delete(&self, project_id: &str, zone: &str, cluster_id: &str, node_pool_id: &str) -> ProjectZoneClusterNodePoolDeleteCall<'a, S> {
        ProjectZoneClusterNodePoolDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _node_pool_id: node_pool_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the requested node pool.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    /// * `nodePoolId` - Deprecated. The name of the node pool. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_node_pools_get(&self, project_id: &str, zone: &str, cluster_id: &str, node_pool_id: &str) -> ProjectZoneClusterNodePoolGetCall<'a, S> {
        ProjectZoneClusterNodePoolGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _node_pool_id: node_pool_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the node pools for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field.
    pub fn zones_clusters_node_pools_list(&self, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterNodePoolListCall<'a, S> {
        ProjectZoneClusterNodePoolListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rolls back a previously Aborted or Failed NodePool upgrade. This makes no changes if the last upgrade successfully completed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to rollback. This field has been deprecated and replaced by the name field.
    /// * `nodePoolId` - Deprecated. The name of the node pool to rollback. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_node_pools_rollback(&self, request: RollbackNodePoolUpgradeRequest, project_id: &str, zone: &str, cluster_id: &str, node_pool_id: &str) -> ProjectZoneClusterNodePoolRollbackCall<'a, S> {
        ProjectZoneClusterNodePoolRollbackCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _node_pool_id: node_pool_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the NodeManagement options for a node pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field.
    /// * `nodePoolId` - Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_node_pools_set_management(&self, request: SetNodePoolManagementRequest, project_id: &str, zone: &str, cluster_id: &str, node_pool_id: &str) -> ProjectZoneClusterNodePoolSetManagementCall<'a, S> {
        ProjectZoneClusterNodePoolSetManagementCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _node_pool_id: node_pool_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the size for a specific node pool. The new size will be used for all replicas, including future replicas created by modifying NodePool.locations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field.
    /// * `nodePoolId` - Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_node_pools_set_size(&self, request: SetNodePoolSizeRequest, project_id: &str, zone: &str, cluster_id: &str, node_pool_id: &str) -> ProjectZoneClusterNodePoolSetSizeCall<'a, S> {
        ProjectZoneClusterNodePoolSetSizeCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _node_pool_id: node_pool_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the version and/or image type for the specified node pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    /// * `nodePoolId` - Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_node_pools_update(&self, request: UpdateNodePoolRequest, project_id: &str, zone: &str, cluster_id: &str, node_pool_id: &str) -> ProjectZoneClusterNodePoolUpdateCall<'a, S> {
        ProjectZoneClusterNodePoolUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _node_pool_id: node_pool_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the addons for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_addons(&self, request: SetAddonsConfigRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterAddonCall<'a, S> {
        ProjectZoneClusterAddonCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes master IP rotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_complete_ip_rotation(&self, request: CompleteIPRotationRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterCompleteIpRotationCall<'a, S> {
        ProjectZoneClusterCompleteIpRotationCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the Kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
    pub fn zones_clusters_create(&self, request: CreateClusterRequest, project_id: &str, zone: &str) -> ProjectZoneClusterCreateCall<'a, S> {
        ProjectZoneClusterCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the cluster, including the Kubernetes endpoint and all worker nodes. Firewalls and routes that were configured during cluster creation are also deleted. Other Google Compute Engine resources that might be in use by the cluster, such as load balancer resources, are not deleted if they weren't present when the cluster was initially created.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to delete. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_delete(&self, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterDeleteCall<'a, S> {
        ProjectZoneClusterDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to retrieve. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_get(&self, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterGetCall<'a, S> {
        ProjectZoneClusterGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enables or disables the ABAC authorization mechanism on a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_legacy_abac(&self, request: SetLegacyAbacRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterLegacyAbacCall<'a, S> {
        ProjectZoneClusterLegacyAbacCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all clusters owned by a project in either the specified zone or all zones.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides, or "-" for all zones. This field has been deprecated and replaced by the parent field.
    pub fn zones_clusters_list(&self, project_id: &str, zone: &str) -> ProjectZoneClusterListCall<'a, S> {
        ProjectZoneClusterListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the locations for a specific cluster. Deprecated. Use [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/update) instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_locations(&self, request: SetLocationsRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterLocationCall<'a, S> {
        ProjectZoneClusterLocationCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the logging service for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_logging(&self, request: SetLoggingServiceRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterLoggingCall<'a, S> {
        ProjectZoneClusterLoggingCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the master for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_master(&self, request: UpdateMasterRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterMasterCall<'a, S> {
        ProjectZoneClusterMasterCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the monitoring service for a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_monitoring(&self, request: SetMonitoringServiceRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterMonitoringCall<'a, S> {
        ProjectZoneClusterMonitoringCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets labels on a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_resource_labels(&self, request: SetLabelsRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterResourceLabelCall<'a, S> {
        ProjectZoneClusterResourceLabelCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the maintenance policy for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects).
    /// * `zone` - Required. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides.
    /// * `clusterId` - Required. The name of the cluster to update.
    pub fn zones_clusters_set_maintenance_policy(&self, request: SetMaintenancePolicyRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterSetMaintenancePolicyCall<'a, S> {
        ProjectZoneClusterSetMaintenancePolicyCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets master auth materials. Currently supports changing the admin password or a specific cluster, either via password generation or explicitly setting the password.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_set_master_auth(&self, request: SetMasterAuthRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterSetMasterAuthCall<'a, S> {
        ProjectZoneClusterSetMasterAuthCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enables or disables Network Policy for a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_set_network_policy(&self, request: SetNetworkPolicyRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterSetNetworkPolicyCall<'a, S> {
        ProjectZoneClusterSetNetworkPolicyCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts master IP rotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_start_ip_rotation(&self, request: StartIPRotationRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterStartIpRotationCall<'a, S> {
        ProjectZoneClusterStartIpRotationCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the settings of a specific cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `clusterId` - Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    pub fn zones_clusters_update(&self, request: UpdateClusterRequest, project_id: &str, zone: &str, cluster_id: &str) -> ProjectZoneClusterUpdateCall<'a, S> {
        ProjectZoneClusterUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels the specified operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field.
    /// * `operationId` - Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field.
    pub fn zones_operations_cancel(&self, request: CancelOperationRequest, project_id: &str, zone: &str, operation_id: &str) -> ProjectZoneOperationCancelCall<'a, S> {
        ProjectZoneOperationCancelCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _operation_id: operation_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified operation.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    /// * `operationId` - Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field.
    pub fn zones_operations_get(&self, project_id: &str, zone: &str, operation_id: &str) -> ProjectZoneOperationGetCall<'a, S> {
        ProjectZoneOperationGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _operation_id: operation_id.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all operations in a project in a specific zone or all zones.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for, or `-` for all zones. This field has been deprecated and replaced by the parent field.
    pub fn zones_operations_list(&self, project_id: &str, zone: &str) -> ProjectZoneOperationListCall<'a, S> {
        ProjectZoneOperationListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns configuration info about the Google Kubernetes Engine service.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    /// * `zone` - Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for. This field has been deprecated and replaced by the name field.
    pub fn zones_get_serverconfig(&self, project_id: &str, zone: &str) -> ProjectZoneGetServerconfigCall<'a, S> {
        ProjectZoneGetServerconfigCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _zone: zone.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



