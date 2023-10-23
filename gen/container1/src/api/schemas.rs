use super::*;
/// AcceleratorConfig represents a Hardware Accelerator request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceleratorConfig {
    /// The number of the accelerator cards exposed to an instance.
    #[serde(rename="acceleratorCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub accelerator_count: Option<i64>,
    /// The accelerator type resource name. List of supported accelerators [here](https://cloud.google.com/compute/docs/gpus)
    #[serde(rename="acceleratorType")]
    
    pub accelerator_type: Option<String>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA [mig user guide](https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning).
    #[serde(rename="gpuPartitionSize")]
    
    pub gpu_partition_size: Option<String>,
    /// The configuration for GPU sharing options.
    #[serde(rename="gpuSharingConfig")]
    
    pub gpu_sharing_config: Option<GPUSharingConfig>,
}

impl client::Part for AcceleratorConfig {}


/// Configuration for the addons that can be automatically spun up in the cluster, enabling additional functionality.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddonsConfig {
    /// Configuration for the Cloud Run addon, which allows the user to use a managed Knative service.
    #[serde(rename="cloudRunConfig")]
    
    pub cloud_run_config: Option<CloudRunConfig>,
    /// Configuration for the ConfigConnector add-on, a Kubernetes extension to manage hosted GCP services through the Kubernetes API
    #[serde(rename="configConnectorConfig")]
    
    pub config_connector_config: Option<ConfigConnectorConfig>,
    /// Configuration for NodeLocalDNS, a dns cache running on cluster nodes
    #[serde(rename="dnsCacheConfig")]
    
    pub dns_cache_config: Option<DnsCacheConfig>,
    /// Configuration for the Compute Engine Persistent Disk CSI driver.
    #[serde(rename="gcePersistentDiskCsiDriverConfig")]
    
    pub gce_persistent_disk_csi_driver_config: Option<GcePersistentDiskCsiDriverConfig>,
    /// Configuration for the GCP Filestore CSI driver.
    #[serde(rename="gcpFilestoreCsiDriverConfig")]
    
    pub gcp_filestore_csi_driver_config: Option<GcpFilestoreCsiDriverConfig>,
    /// Configuration for the Backup for GKE agent addon.
    #[serde(rename="gkeBackupAgentConfig")]
    
    pub gke_backup_agent_config: Option<GkeBackupAgentConfig>,
    /// Configuration for the horizontal pod autoscaling feature, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods.
    #[serde(rename="horizontalPodAutoscaling")]
    
    pub horizontal_pod_autoscaling: Option<HorizontalPodAutoscaling>,
    /// Configuration for the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster.
    #[serde(rename="httpLoadBalancing")]
    
    pub http_load_balancing: Option<HttpLoadBalancing>,
    /// Configuration for the Kubernetes Dashboard. This addon is deprecated, and will be disabled in 1.15. It is recommended to use the Cloud Console to manage and monitor your Kubernetes clusters, workloads and applications. For more information, see: https://cloud.google.com/kubernetes-engine/docs/concepts/dashboards
    #[serde(rename="kubernetesDashboard")]
    
    pub kubernetes_dashboard: Option<KubernetesDashboard>,
    /// Configuration for NetworkPolicy. This only tracks whether the addon is enabled or not on the Master, it does not track whether network policy is enabled for the nodes.
    #[serde(rename="networkPolicyConfig")]
    
    pub network_policy_config: Option<NetworkPolicyConfig>,
}

impl client::Part for AddonsConfig {}


/// Specifies options for controlling advanced machine features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedMachineFeatures {
    /// The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed.
    #[serde(rename="threadsPerCore")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub threads_per_core: Option<i64>,
}

impl client::Part for AdvancedMachineFeatures {}


/// Configuration for returning group information from authenticators.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatorGroupsConfig {
    /// Whether this cluster should return group membership lookups during authentication using a group of security groups.
    
    pub enabled: Option<bool>,
    /// The name of the security group-of-groups to be used. Only relevant if enabled = true.
    #[serde(rename="securityGroup")]
    
    pub security_group: Option<String>,
}

impl client::Part for AuthenticatorGroupsConfig {}


/// AutoUpgradeOptions defines the set of options for the user to control how the Auto Upgrades will proceed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoUpgradeOptions {
    /// [Output only] This field is set when upgrades are about to commence with the approximate start time for the upgrades, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[serde(rename="autoUpgradeStartTime")]
    
    pub auto_upgrade_start_time: Option<String>,
    /// [Output only] This field is set when upgrades are about to commence with the description of the upgrade.
    
    pub description: Option<String>,
}

impl client::Part for AutoUpgradeOptions {}


/// Autopilot is the configuration for Autopilot settings on the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Autopilot {
    /// Enable Autopilot
    
    pub enabled: Option<bool>,
}

impl client::Part for Autopilot {}


/// AutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoprovisioningNodePoolDefaults {
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption
    #[serde(rename="bootDiskKmsKey")]
    
    pub boot_disk_kms_key: Option<String>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB.
    #[serde(rename="diskSizeGb")]
    
    pub disk_size_gb: Option<i32>,
    /// Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard'
    #[serde(rename="diskType")]
    
    pub disk_type: Option<String>,
    /// The image type to use for NAP created node.
    #[serde(rename="imageType")]
    
    pub image_type: Option<String>,
    /// Specifies the node management options for NAP created node-pools.
    
    pub management: Option<NodeManagement>,
    /// Deprecated. Minimum CPU platform to be used for NAP created node pools. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as minCpuPlatform: Intel Haswell or minCpuPlatform: Intel Sandy Bridge. For more information, read [how to specify min CPU platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform). This field is deprecated, min_cpu_platform should be specified using https://cloud.google.com/requested-min-cpu-platform label selector on the pod. To unset the min cpu platform field pass "automatic" as field value.
    #[serde(rename="minCpuPlatform")]
    
    pub min_cpu_platform: Option<String>,
    /// Scopes that are used by NAP when creating node pools.
    #[serde(rename="oauthScopes")]
    
    pub oauth_scopes: Option<Vec<String>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Shielded Instance options.
    #[serde(rename="shieldedInstanceConfig")]
    
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,
    /// Specifies the upgrade settings for NAP created node pools
    #[serde(rename="upgradeSettings")]
    
    pub upgrade_settings: Option<UpgradeSettings>,
}

impl client::Part for AutoprovisioningNodePoolDefaults {}


/// Parameters for using BigQuery as the destination of resource usage export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryDestination {
    /// The ID of a BigQuery Dataset.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
}

impl client::Part for BigQueryDestination {}


/// Configuration for Binary Authorization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BinaryAuthorization {
    /// This field is deprecated. Leave this unset and instead configure BinaryAuthorization using evaluation_mode. If evaluation_mode is set to anything other than EVALUATION_MODE_UNSPECIFIED, this field is ignored.
    
    pub enabled: Option<bool>,
    /// Mode of operation for binauthz policy evaluation. If unspecified, defaults to DISABLED.
    #[serde(rename="evaluationMode")]
    
    pub evaluation_mode: Option<BinaryAuthorizationEvaluationModeEnum>,
}

impl client::Part for BinaryAuthorization {}


/// Information relevant to blue-green upgrade.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlueGreenInfo {
    /// The resource URLs of the \[managed instance groups\] (/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with blue pool.
    #[serde(rename="blueInstanceGroupUrls")]
    
    pub blue_instance_group_urls: Option<Vec<String>>,
    /// Time to start deleting blue pool to complete blue-green upgrade, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[serde(rename="bluePoolDeletionStartTime")]
    
    pub blue_pool_deletion_start_time: Option<String>,
    /// The resource URLs of the \[managed instance groups\] (/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with green pool.
    #[serde(rename="greenInstanceGroupUrls")]
    
    pub green_instance_group_urls: Option<Vec<String>>,
    /// Version of green pool.
    #[serde(rename="greenPoolVersion")]
    
    pub green_pool_version: Option<String>,
    /// Current blue-green upgrade phase.
    
    pub phase: Option<BlueGreenInfoPhaseEnum>,
}

impl client::Part for BlueGreenInfo {}


/// Settings for blue-green upgrade.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlueGreenSettings {
    /// Time needed after draining entire blue pool. After this period, blue pool will be cleaned up.
    #[serde(rename="nodePoolSoakDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub node_pool_soak_duration: Option<client::chrono::Duration>,
    /// Standard policy for the blue-green upgrade.
    #[serde(rename="standardRolloutPolicy")]
    
    pub standard_rollout_policy: Option<StandardRolloutPolicy>,
}

impl client::Part for BlueGreenSettings {}


/// CancelOperationRequest cancels a single operation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
/// * [zones operations cancel projects](ProjectZoneOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest {
    /// The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`.
    
    pub name: Option<String>,
    /// Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for CancelOperationRequest {}


/// CidrBlock contains an optional name and one CIDR block.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CidrBlock {
    /// cidr_block must be specified in CIDR notation.
    #[serde(rename="cidrBlock")]
    
    pub cidr_block: Option<String>,
    /// display_name is an optional field for users to identify CIDR blocks.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for CidrBlock {}


/// Configuration for client certificates on the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientCertificateConfig {
    /// Issue a client certificate.
    #[serde(rename="issueClientCertificate")]
    
    pub issue_client_certificate: Option<bool>,
}

impl client::Part for ClientCertificateConfig {}


/// Configuration options for the Cloud Run feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunConfig {
    /// Whether Cloud Run addon is enabled for this cluster.
    
    pub disabled: Option<bool>,
    /// Which load balancer type is installed for Cloud Run.
    #[serde(rename="loadBalancerType")]
    
    pub load_balancer_type: Option<CloudRunConfigLoadBalancerTypeEnum>,
}

impl client::Part for CloudRunConfig {}


/// A Google Kubernetes Engine cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters get projects](ProjectLocationClusterGetCall) (response)
/// * [zones clusters get projects](ProjectZoneClusterGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    /// Configurations for the various addons available to run in the cluster.
    #[serde(rename="addonsConfig")]
    
    pub addons_config: Option<AddonsConfig>,
    /// Configuration controlling RBAC group membership information.
    #[serde(rename="authenticatorGroupsConfig")]
    
    pub authenticator_groups_config: Option<AuthenticatorGroupsConfig>,
    /// Autopilot configuration for the cluster.
    
    pub autopilot: Option<Autopilot>,
    /// Cluster-level autoscaling configuration.
    
    pub autoscaling: Option<ClusterAutoscaling>,
    /// Configuration for Binary Authorization.
    #[serde(rename="binaryAuthorization")]
    
    pub binary_authorization: Option<BinaryAuthorization>,
    /// The IP address range of the container pods in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`). Leave blank to have one automatically chosen or specify a `/14` block in `10.0.0.0/8`.
    #[serde(rename="clusterIpv4Cidr")]
    
    pub cluster_ipv4_cidr: Option<String>,
    /// Which conditions caused the current cluster state.
    
    pub conditions: Option<Vec<StatusCondition>>,
    /// Configuration of Confidential Nodes. All the nodes in the cluster will be Confidential VM once enabled.
    #[serde(rename="confidentialNodes")]
    
    pub confidential_nodes: Option<ConfidentialNodes>,
    /// Configuration for the fine-grained cost management feature.
    #[serde(rename="costManagementConfig")]
    
    pub cost_management_config: Option<CostManagementConfig>,
    /// [Output only] The time the cluster was created, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[serde(rename="createTime")]
    
    pub create_time: Option<String>,
    /// [Output only] The current software version of the master endpoint.
    #[serde(rename="currentMasterVersion")]
    
    pub current_master_version: Option<String>,
    /// [Output only] The number of nodes currently in the cluster. Deprecated. Call Kubernetes API directly to retrieve node information.
    #[serde(rename="currentNodeCount")]
    
    pub current_node_count: Option<i32>,
    /// [Output only] Deprecated, use [NodePools.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools) instead. The current version of the node software components. If they are currently at multiple versions because they're in the process of being upgraded, this reflects the minimum version of all nodes.
    #[serde(rename="currentNodeVersion")]
    
    pub current_node_version: Option<String>,
    /// Configuration of etcd encryption.
    #[serde(rename="databaseEncryption")]
    
    pub database_encryption: Option<DatabaseEncryption>,
    /// The default constraint on the maximum number of pods that can be run simultaneously on a node in the node pool of this cluster. Only honored if cluster created with IP Alias support.
    #[serde(rename="defaultMaxPodsConstraint")]
    
    pub default_max_pods_constraint: Option<MaxPodsConstraint>,
    /// An optional description of this cluster.
    
    pub description: Option<String>,
    /// Kubernetes alpha features are enabled on this cluster. This includes alpha API groups (e.g. v1alpha1) and features that may not be production ready in the kubernetes version of the master and nodes. The cluster has no SLA for uptime and master/node upgrades are disabled. Alpha enabled clusters are automatically deleted thirty days after creation.
    #[serde(rename="enableKubernetesAlpha")]
    
    pub enable_kubernetes_alpha: Option<bool>,
    /// Enable the ability to use Cloud TPUs in this cluster.
    #[serde(rename="enableTpu")]
    
    pub enable_tpu: Option<bool>,
    /// [Output only] The IP address of this cluster's master endpoint. The endpoint can be accessed from the internet at `https://username:password@endpoint/`. See the `masterAuth` property of this resource for username and password information.
    
    pub endpoint: Option<String>,
    /// This checksum is computed by the server based on the value of cluster fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// [Output only] The time the cluster will be automatically deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<String>,
    /// Output only. Unique id for the cluster.
    
    pub id: Option<String>,
    /// Configuration for Identity Service component.
    #[serde(rename="identityServiceConfig")]
    
    pub identity_service_config: Option<IdentityServiceConfig>,
    /// The initial Kubernetes version for this cluster. Valid versions are those found in validMasterVersions returned by getServerConfig. The version can be upgraded over time; such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "","-": picks the default Kubernetes version
    #[serde(rename="initialClusterVersion")]
    
    pub initial_cluster_version: Option<String>,
    /// The number of nodes to create in this cluster. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "node_config") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. This field is deprecated, use node_pool.initial_node_count instead.
    #[serde(rename="initialNodeCount")]
    
    pub initial_node_count: Option<i32>,
    /// Deprecated. Use node_pools.instance_group_urls.
    #[serde(rename="instanceGroupUrls")]
    
    pub instance_group_urls: Option<Vec<String>>,
    /// Configuration for cluster IP allocation.
    #[serde(rename="ipAllocationPolicy")]
    
    pub ip_allocation_policy: Option<IPAllocationPolicy>,
    /// The fingerprint of the set of labels for this cluster.
    #[serde(rename="labelFingerprint")]
    
    pub label_fingerprint: Option<String>,
    /// Configuration for the legacy ABAC authorization mode.
    #[serde(rename="legacyAbac")]
    
    pub legacy_abac: Option<LegacyAbac>,
    /// [Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides.
    
    pub location: Option<String>,
    /// The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This field provides a default value if [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) are not specified during node pool creation. Warning: changing cluster locations will update the [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) of all node pools and will result in nodes being added and/or removed.
    
    pub locations: Option<Vec<String>>,
    /// Logging configuration for the cluster.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<LoggingConfig>,
    /// The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions.
    #[serde(rename="loggingService")]
    
    pub logging_service: Option<String>,
    /// Configure the maintenance policy for this cluster.
    #[serde(rename="maintenancePolicy")]
    
    pub maintenance_policy: Option<MaintenancePolicy>,
    /// The authentication information for accessing the master endpoint. If unspecified, the defaults are used: For clusters before v1.12, if master_auth is unspecified, `username` will be set to "admin", a random password will be generated, and a client certificate will be issued.
    #[serde(rename="masterAuth")]
    
    pub master_auth: Option<MasterAuth>,
    /// The configuration options for master authorized networks feature.
    #[serde(rename="masterAuthorizedNetworksConfig")]
    
    pub master_authorized_networks_config: Option<MasterAuthorizedNetworksConfig>,
    /// Configuration for issuance of mTLS keys and certificates to Kubernetes pods.
    #[serde(rename="meshCertificates")]
    
    pub mesh_certificates: Option<MeshCertificates>,
    /// Monitoring configuration for the cluster.
    #[serde(rename="monitoringConfig")]
    
    pub monitoring_config: Option<MonitoringConfig>,
    /// The monitoring service the cluster should use to write metrics. Currently available options: * "monitoring.googleapis.com/kubernetes" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions.
    #[serde(rename="monitoringService")]
    
    pub monitoring_service: Option<String>,
    /// The name of this cluster. The name must be unique within this project and location (e.g. zone or region), and can be up to 40 characters with the following restrictions: * Lowercase letters, numbers, and hyphens only. * Must start with a letter. * Must end with a number or a letter.
    
    pub name: Option<String>,
    /// The name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. If left unspecified, the `default` network will be used.
    
    pub network: Option<String>,
    /// Configuration for cluster networking.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<NetworkConfig>,
    /// Configuration options for the NetworkPolicy feature.
    #[serde(rename="networkPolicy")]
    
    pub network_policy: Option<NetworkPolicy>,
    /// Parameters used in creating the cluster's nodes. For requests, this field should only be used in lieu of a "node_pool" object, since this configuration (along with the "initial_node_count") will be used to create a "NodePool" object with an auto-generated name. Do not use this and a node_pool at the same time. For responses, this field will be populated with the node configuration of the first node pool. (For configuration of each node pool, see `node_pool.config`) If unspecified, the defaults are used. This field is deprecated, use node_pool.config instead.
    #[serde(rename="nodeConfig")]
    
    pub node_config: Option<NodeConfig>,
    /// [Output only] The size of the address space on each node for hosting containers. This is provisioned from within the `container_ipv4_cidr` range. This field will only be set when cluster is in route-based network mode.
    #[serde(rename="nodeIpv4CidrSize")]
    
    pub node_ipv4_cidr_size: Option<i32>,
    /// Node pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters.
    #[serde(rename="nodePoolAutoConfig")]
    
    pub node_pool_auto_config: Option<NodePoolAutoConfig>,
    /// Default NodePool settings for the entire cluster. These settings are overridden if specified on the specific NodePool object.
    #[serde(rename="nodePoolDefaults")]
    
    pub node_pool_defaults: Option<NodePoolDefaults>,
    /// The node pools associated with this cluster. This field should not be set if "node_config" or "initial_node_count" are specified.
    #[serde(rename="nodePools")]
    
    pub node_pools: Option<Vec<NodePool>>,
    /// Notification configuration of the cluster.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<NotificationConfig>,
    /// Configuration for private cluster.
    #[serde(rename="privateClusterConfig")]
    
    pub private_cluster_config: Option<PrivateClusterConfig>,
    /// Release channel configuration.
    #[serde(rename="releaseChannel")]
    
    pub release_channel: Option<ReleaseChannel>,
    /// The resource labels for the cluster to use to annotate any related Google Compute Engine resources.
    #[serde(rename="resourceLabels")]
    
    pub resource_labels: Option<HashMap<String, String>>,
    /// Configuration for exporting resource usages. Resource usage export is disabled when this config is unspecified.
    #[serde(rename="resourceUsageExportConfig")]
    
    pub resource_usage_export_config: Option<ResourceUsageExportConfig>,
    /// [Output only] Server-defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output only] The IP address range of the Kubernetes services in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last `/16` from the container CIDR.
    #[serde(rename="servicesIpv4Cidr")]
    
    pub services_ipv4_cidr: Option<String>,
    /// Shielded Nodes configuration.
    #[serde(rename="shieldedNodes")]
    
    pub shielded_nodes: Option<ShieldedNodes>,
    /// [Output only] The current status of this cluster.
    
    pub status: Option<ClusterStatusEnum>,
    /// [Output only] Deprecated. Use conditions instead. Additional information about the current status of this cluster, if available.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// The name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is connected.
    
    pub subnetwork: Option<String>,
    /// [Output only] The IP address range of the Cloud TPUs in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`).
    #[serde(rename="tpuIpv4CidrBlock")]
    
    pub tpu_ipv4_cidr_block: Option<String>,
    /// Cluster-level Vertical Pod Autoscaling configuration.
    #[serde(rename="verticalPodAutoscaling")]
    
    pub vertical_pod_autoscaling: Option<VerticalPodAutoscaling>,
    /// Configuration for the use of Kubernetes Service Accounts in GCP IAM policies.
    #[serde(rename="workloadIdentityConfig")]
    
    pub workload_identity_config: Option<WorkloadIdentityConfig>,
    /// [Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field is deprecated, use location instead.
    
    pub zone: Option<String>,
}

impl client::ResponseResult for Cluster {}


/// ClusterAutoscaling contains global, per-cluster information required by Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterAutoscaling {
    /// The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes can be created by NAP.
    #[serde(rename="autoprovisioningLocations")]
    
    pub autoprovisioning_locations: Option<Vec<String>>,
    /// AutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP.
    #[serde(rename="autoprovisioningNodePoolDefaults")]
    
    pub autoprovisioning_node_pool_defaults: Option<AutoprovisioningNodePoolDefaults>,
    /// Defines autoscaling behaviour.
    #[serde(rename="autoscalingProfile")]
    
    pub autoscaling_profile: Option<ClusterAutoscalingAutoscalingProfileEnum>,
    /// Enables automatic node pool creation and deletion.
    #[serde(rename="enableNodeAutoprovisioning")]
    
    pub enable_node_autoprovisioning: Option<bool>,
    /// Contains global constraints regarding minimum and maximum amount of resources in the cluster.
    #[serde(rename="resourceLimits")]
    
    pub resource_limits: Option<Vec<ResourceLimit>>,
}

impl client::Part for ClusterAutoscaling {}


/// ClusterUpdate describes an update to the cluster. Exactly one update can be applied to a cluster with each request, so at most one field can be provided.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterUpdate {
    /// Configurations for the various addons available to run in the cluster.
    #[serde(rename="desiredAddonsConfig")]
    
    pub desired_addons_config: Option<AddonsConfig>,
    /// The desired authenticator groups config for the cluster.
    #[serde(rename="desiredAuthenticatorGroupsConfig")]
    
    pub desired_authenticator_groups_config: Option<AuthenticatorGroupsConfig>,
    /// The desired configuration options for the Binary Authorization feature.
    #[serde(rename="desiredBinaryAuthorization")]
    
    pub desired_binary_authorization: Option<BinaryAuthorization>,
    /// Cluster-level autoscaling configuration.
    #[serde(rename="desiredClusterAutoscaling")]
    
    pub desired_cluster_autoscaling: Option<ClusterAutoscaling>,
    /// The desired configuration for the fine-grained cost management feature.
    #[serde(rename="desiredCostManagementConfig")]
    
    pub desired_cost_management_config: Option<CostManagementConfig>,
    /// Configuration of etcd encryption.
    #[serde(rename="desiredDatabaseEncryption")]
    
    pub desired_database_encryption: Option<DatabaseEncryption>,
    /// The desired datapath provider for the cluster.
    #[serde(rename="desiredDatapathProvider")]
    
    pub desired_datapath_provider: Option<ClusterUpdateDesiredDatapathProviderEnum>,
    /// The desired status of whether to disable default sNAT for this cluster.
    #[serde(rename="desiredDefaultSnatStatus")]
    
    pub desired_default_snat_status: Option<DefaultSnatStatus>,
    /// DNSConfig contains clusterDNS config for this cluster.
    #[serde(rename="desiredDnsConfig")]
    
    pub desired_dns_config: Option<DNSConfig>,
    /// Enable/Disable private endpoint for the cluster's master.
    #[serde(rename="desiredEnablePrivateEndpoint")]
    
    pub desired_enable_private_endpoint: Option<bool>,
    /// The desired config of Gateway API on this cluster.
    #[serde(rename="desiredGatewayApiConfig")]
    
    pub desired_gateway_api_config: Option<GatewayAPIConfig>,
    /// The desired GCFS config for the cluster
    #[serde(rename="desiredGcfsConfig")]
    
    pub desired_gcfs_config: Option<GcfsConfig>,
    /// The desired Identity Service component configuration.
    #[serde(rename="desiredIdentityServiceConfig")]
    
    pub desired_identity_service_config: Option<IdentityServiceConfig>,
    /// The desired image type for the node pool. NOTE: Set the "desired_node_pool" field as well.
    #[serde(rename="desiredImageType")]
    
    pub desired_image_type: Option<String>,
    /// The desired config of Intra-node visibility.
    #[serde(rename="desiredIntraNodeVisibilityConfig")]
    
    pub desired_intra_node_visibility_config: Option<IntraNodeVisibilityConfig>,
    /// The desired L4 Internal Load Balancer Subsetting configuration.
    #[serde(rename="desiredL4ilbSubsettingConfig")]
    
    pub desired_l4ilb_subsetting_config: Option<ILBSubsettingConfig>,
    /// The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This list must always include the cluster's primary zone. Warning: changing cluster locations will update the locations of all node pools and will result in nodes being added and/or removed.
    #[serde(rename="desiredLocations")]
    
    pub desired_locations: Option<Vec<String>>,
    /// The desired logging configuration.
    #[serde(rename="desiredLoggingConfig")]
    
    pub desired_logging_config: Option<LoggingConfig>,
    /// The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions.
    #[serde(rename="desiredLoggingService")]
    
    pub desired_logging_service: Option<String>,
    /// The desired configuration options for master authorized networks feature.
    #[serde(rename="desiredMasterAuthorizedNetworksConfig")]
    
    pub desired_master_authorized_networks_config: Option<MasterAuthorizedNetworksConfig>,
    /// The Kubernetes version to change the master to. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "-": picks the default Kubernetes version
    #[serde(rename="desiredMasterVersion")]
    
    pub desired_master_version: Option<String>,
    /// Configuration for issuance of mTLS keys and certificates to Kubernetes pods.
    #[serde(rename="desiredMeshCertificates")]
    
    pub desired_mesh_certificates: Option<MeshCertificates>,
    /// The desired monitoring configuration.
    #[serde(rename="desiredMonitoringConfig")]
    
    pub desired_monitoring_config: Option<MonitoringConfig>,
    /// The monitoring service the cluster should use to write metrics. Currently available options: * "monitoring.googleapis.com/kubernetes" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions.
    #[serde(rename="desiredMonitoringService")]
    
    pub desired_monitoring_service: Option<String>,
    /// The desired network tags that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters.
    #[serde(rename="desiredNodePoolAutoConfigNetworkTags")]
    
    pub desired_node_pool_auto_config_network_tags: Option<NetworkTags>,
    /// Autoscaler configuration for the node pool specified in desired_node_pool_id. If there is only one pool in the cluster and desired_node_pool_id is not provided then the change applies to that single node pool.
    #[serde(rename="desiredNodePoolAutoscaling")]
    
    pub desired_node_pool_autoscaling: Option<NodePoolAutoscaling>,
    /// The node pool to be upgraded. This field is mandatory if "desired_node_version", "desired_image_family" or "desired_node_pool_autoscaling" is specified and there is more than one node pool on the cluster.
    #[serde(rename="desiredNodePoolId")]
    
    pub desired_node_pool_id: Option<String>,
    /// The desired node pool logging configuration defaults for the cluster.
    #[serde(rename="desiredNodePoolLoggingConfig")]
    
    pub desired_node_pool_logging_config: Option<NodePoolLoggingConfig>,
    /// The Kubernetes version to change the nodes to (typically an upgrade). Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "-": picks the Kubernetes master version
    #[serde(rename="desiredNodeVersion")]
    
    pub desired_node_version: Option<String>,
    /// The desired notification configuration.
    #[serde(rename="desiredNotificationConfig")]
    
    pub desired_notification_config: Option<NotificationConfig>,
    /// The desired private cluster configuration.
    #[serde(rename="desiredPrivateClusterConfig")]
    
    pub desired_private_cluster_config: Option<PrivateClusterConfig>,
    /// The desired state of IPv6 connectivity to Google Services.
    #[serde(rename="desiredPrivateIpv6GoogleAccess")]
    
    pub desired_private_ipv6_google_access: Option<ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum>,
    /// The desired release channel configuration.
    #[serde(rename="desiredReleaseChannel")]
    
    pub desired_release_channel: Option<ReleaseChannel>,
    /// The desired configuration for exporting resource usage.
    #[serde(rename="desiredResourceUsageExportConfig")]
    
    pub desired_resource_usage_export_config: Option<ResourceUsageExportConfig>,
    /// ServiceExternalIPsConfig specifies the config for the use of Services with ExternalIPs field.
    #[serde(rename="desiredServiceExternalIpsConfig")]
    
    pub desired_service_external_ips_config: Option<ServiceExternalIPsConfig>,
    /// Configuration for Shielded Nodes.
    #[serde(rename="desiredShieldedNodes")]
    
    pub desired_shielded_nodes: Option<ShieldedNodes>,
    /// The desired stack type of the cluster. If a stack type is provided and does not match the current stack type of the cluster, update will attempt to change the stack type to the new type.
    #[serde(rename="desiredStackType")]
    
    pub desired_stack_type: Option<ClusterUpdateDesiredStackTypeEnum>,
    /// Cluster-level Vertical Pod Autoscaling configuration.
    #[serde(rename="desiredVerticalPodAutoscaling")]
    
    pub desired_vertical_pod_autoscaling: Option<VerticalPodAutoscaling>,
    /// Configuration for Workload Identity.
    #[serde(rename="desiredWorkloadIdentityConfig")]
    
    pub desired_workload_identity_config: Option<WorkloadIdentityConfig>,
    /// The current etag of the cluster. If an etag is provided and does not match the current etag of the cluster, update will be blocked and an ABORTED error will be returned.
    
    pub etag: Option<String>,
}

impl client::Part for ClusterUpdate {}


/// CompleteIPRotationRequest moves the cluster master back into single-IP mode.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters complete ip rotation projects](ProjectLocationClusterCompleteIpRotationCall) (request)
/// * [zones clusters complete ip rotation projects](ProjectZoneClusterCompleteIpRotationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteIPRotationRequest {
    /// Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster name) of the cluster to complete IP rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for CompleteIPRotationRequest {}


/// CompleteNodePoolUpgradeRequest sets the name of target node pool to complete upgrade.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools complete upgrade projects](ProjectLocationClusterNodePoolCompleteUpgradeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteNodePoolUpgradeRequest { _never_set: Option<bool> }

impl client::RequestValue for CompleteNodePoolUpgradeRequest {}


/// ConfidentialNodes is configuration for the confidential nodes feature, which makes nodes run on confidential VMs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfidentialNodes {
    /// Whether Confidential Nodes feature is enabled.
    
    pub enabled: Option<bool>,
}

impl client::Part for ConfidentialNodes {}


/// Configuration options for the Config Connector add-on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigConnectorConfig {
    /// Whether Cloud Connector is enabled for this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for ConfigConnectorConfig {}


/// Parameters for controlling consumption metering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsumptionMeteringConfig {
    /// Whether to enable consumption metering for this cluster. If enabled, a second BigQuery table will be created to hold resource consumption records.
    
    pub enabled: Option<bool>,
}

impl client::Part for ConsumptionMeteringConfig {}


/// Configuration for fine-grained cost management feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CostManagementConfig {
    /// Whether the feature is enabled or not.
    
    pub enabled: Option<bool>,
}

impl client::Part for CostManagementConfig {}


/// CreateClusterRequest creates a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters create projects](ProjectLocationClusterCreateCall) (request)
/// * [zones clusters create projects](ProjectZoneClusterCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateClusterRequest {
    /// Required. A [cluster resource](https://cloud.google.com/container-engine/reference/rest/v1/projects.locations.clusters)
    
    pub cluster: Option<Cluster>,
    /// The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`.
    
    pub parent: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for CreateClusterRequest {}


/// CreateNodePoolRequest creates a node pool for a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools create projects](ProjectLocationClusterNodePoolCreateCall) (request)
/// * [zones clusters node pools create projects](ProjectZoneClusterNodePoolCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateNodePoolRequest {
    /// Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. The node pool to create.
    #[serde(rename="nodePool")]
    
    pub node_pool: Option<NodePool>,
    /// The parent (project, location, cluster name) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub parent: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for CreateNodePoolRequest {}


/// DNSConfig contains the desired set of options for configuring clusterDNS.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DNSConfig {
    /// cluster_dns indicates which in-cluster DNS provider should be used.
    #[serde(rename="clusterDns")]
    
    pub cluster_dns: Option<DNSConfigClusterDnsEnum>,
    /// cluster_dns_domain is the suffix used for all cluster service records.
    #[serde(rename="clusterDnsDomain")]
    
    pub cluster_dns_domain: Option<String>,
    /// cluster_dns_scope indicates the scope of access to cluster DNS records.
    #[serde(rename="clusterDnsScope")]
    
    pub cluster_dns_scope: Option<DNSConfigClusterDnsScopeEnum>,
}

impl client::Part for DNSConfig {}


/// Time window specified for daily maintenance operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DailyMaintenanceWindow {
    /// [Output only] Duration of the time window, automatically chosen to be smallest possible in the given scenario. Duration will be in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format "PTnHnMnS".
    
    pub duration: Option<String>,
    /// Time within the maintenance window to start the maintenance operations. Time format should be in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format "HH:MM", where HH : [00-23] and MM : [00-59] GMT.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
}

impl client::Part for DailyMaintenanceWindow {}


/// Configuration of etcd encryption.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseEncryption {
    /// Name of CloudKMS key to use for the encryption of secrets in etcd. Ex. projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key
    #[serde(rename="keyName")]
    
    pub key_name: Option<String>,
    /// Denotes the state of etcd encryption.
    
    pub state: Option<DatabaseEncryptionStateEnum>,
}

impl client::Part for DatabaseEncryption {}


/// DefaultSnatStatus contains the desired state of whether default sNAT should be disabled on the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DefaultSnatStatus {
    /// Disables cluster default sNAT rules.
    
    pub disabled: Option<bool>,
}

impl client::Part for DefaultSnatStatus {}


/// Configuration for NodeLocal DNSCache
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DnsCacheConfig {
    /// Whether NodeLocal DNSCache is enabled for this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for DnsCacheConfig {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools complete upgrade projects](ProjectLocationClusterNodePoolCompleteUpgradeCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [zones operations cancel projects](ProjectZoneOperationCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// EphemeralStorageLocalSsdConfig contains configuration for the node ephemeral storage using Local SSD.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EphemeralStorageLocalSsdConfig {
    /// Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD is 375 GB in size. If zero, it means to disable using local SSDs as ephemeral storage. The limit for this value is dependent upon the maximum number of disks available on a machine per zone. See: https://cloud.google.com/compute/docs/disks/local-ssd for more information.
    #[serde(rename="localSsdCount")]
    
    pub local_ssd_count: Option<i32>,
}

impl client::Part for EphemeralStorageLocalSsdConfig {}


/// Configuration of Fast Socket feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FastSocket {
    /// Whether Fast Socket features are enabled in the node pool.
    
    pub enabled: Option<bool>,
}

impl client::Part for FastSocket {}


/// Allows filtering to one or more specific event types. If event types are present, those and only those event types will be transmitted to the cluster. Other types will be skipped. If no filter is specified, or no event types are present, all event types will be sent
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// Event types to allowlist.
    #[serde(rename="eventType")]
    
    pub event_type: Option<Vec<FilterEventTypeEnum>>,
}

impl client::Part for Filter {}


/// GPUSharingConfig represents the GPU sharing configuration for Hardware Accelerators.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GPUSharingConfig {
    /// The type of GPU sharing strategy to enable on the GPU node.
    #[serde(rename="gpuSharingStrategy")]
    
    pub gpu_sharing_strategy: Option<GPUSharingConfigGpuSharingStrategyEnum>,
    /// The max number of containers that can share a physical GPU.
    #[serde(rename="maxSharedClientsPerGpu")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_shared_clients_per_gpu: Option<i64>,
}

impl client::Part for GPUSharingConfig {}


/// GatewayAPIConfig contains the desired config of Gateway API on this cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GatewayAPIConfig {
    /// The Gateway API release channel to use for Gateway API.
    
    pub channel: Option<GatewayAPIConfigChannelEnum>,
}

impl client::Part for GatewayAPIConfig {}


/// Configuration for the Compute Engine PD CSI driver.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcePersistentDiskCsiDriverConfig {
    /// Whether the Compute Engine PD CSI driver is enabled for this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for GcePersistentDiskCsiDriverConfig {}


/// GcfsConfig contains configurations of Google Container File System (image streaming).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcfsConfig {
    /// Whether to use GCFS.
    
    pub enabled: Option<bool>,
}

impl client::Part for GcfsConfig {}


/// Configuration for the GCP Filestore CSI driver.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcpFilestoreCsiDriverConfig {
    /// Whether the GCP Filestore CSI driver is enabled for this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for GcpFilestoreCsiDriverConfig {}


/// GetJSONWebKeysResponse is a valid JSON Web Key Set as specififed in rfc 7517
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters get jwks projects](ProjectLocationClusterGetJwkCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetJSONWebKeysResponse {
    /// OnePlatform automatically extracts this field and uses it to set the HTTP Cache-Control header.
    #[serde(rename="cacheHeader")]
    
    pub cache_header: Option<HttpCacheControlResponseHeader>,
    /// The public component of the keys used by the cluster to sign token requests.
    
    pub keys: Option<Vec<Jwk>>,
}

impl client::ResponseResult for GetJSONWebKeysResponse {}


/// GetOpenIDConfigResponse is an OIDC discovery document for the cluster. See the OpenID Connect Discovery 1.0 specification for details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters well-known get openid-configuration projects](ProjectLocationClusterWellKnownGetOpenidConfigurationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetOpenIDConfigResponse {
    /// OnePlatform automatically extracts this field and uses it to set the HTTP Cache-Control header.
    #[serde(rename="cacheHeader")]
    
    pub cache_header: Option<HttpCacheControlResponseHeader>,
    /// Supported claims.
    
    pub claims_supported: Option<Vec<String>>,
    /// Supported grant types.
    
    pub grant_types: Option<Vec<String>>,
    /// supported ID Token signing Algorithms.
    
    pub id_token_signing_alg_values_supported: Option<Vec<String>>,
    /// OIDC Issuer.
    
    pub issuer: Option<String>,
    /// JSON Web Key uri.
    
    pub jwks_uri: Option<String>,
    /// Supported response types.
    
    pub response_types_supported: Option<Vec<String>>,
    /// Supported subject types.
    
    pub subject_types_supported: Option<Vec<String>>,
}

impl client::ResponseResult for GetOpenIDConfigResponse {}


/// Configuration for the Backup for GKE Agent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeBackupAgentConfig {
    /// Whether the Backup for GKE agent is enabled for this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for GkeBackupAgentConfig {}


/// Configuration options for the horizontal pod autoscaling feature, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HorizontalPodAutoscaling {
    /// Whether the Horizontal Pod Autoscaling feature is enabled in the cluster. When enabled, it ensures that metrics are collected into Stackdriver Monitoring.
    
    pub disabled: Option<bool>,
}

impl client::Part for HorizontalPodAutoscaling {}


/// RFC-2616: cache control support
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpCacheControlResponseHeader {
    /// 14.6 response cache age, in seconds since the response is generated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub age: Option<i64>,
    /// 14.9 request and response directives
    
    pub directive: Option<String>,
    /// 14.21 response cache expires, in RFC 1123 date format
    
    pub expires: Option<String>,
}

impl client::Part for HttpCacheControlResponseHeader {}


/// Configuration options for the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpLoadBalancing {
    /// Whether the HTTP Load Balancing controller is enabled in the cluster. When enabled, it runs a small pod in the cluster that manages the load balancers.
    
    pub disabled: Option<bool>,
}

impl client::Part for HttpLoadBalancing {}


/// ILBSubsettingConfig contains the desired config of L4 Internal LoadBalancer subsetting on this cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ILBSubsettingConfig {
    /// Enables l4 ILB subsetting for this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for ILBSubsettingConfig {}


/// Configuration for controlling how IPs are allocated in the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IPAllocationPolicy {
    /// This field is deprecated, use cluster_ipv4_cidr_block.
    #[serde(rename="clusterIpv4Cidr")]
    
    pub cluster_ipv4_cidr: Option<String>,
    /// The IP address range for the cluster pod IPs. If this field is set, then `cluster.cluster_ipv4_cidr` must be left blank. This field is only applicable when `use_ip_aliases` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use.
    #[serde(rename="clusterIpv4CidrBlock")]
    
    pub cluster_ipv4_cidr_block: Option<String>,
    /// The name of the secondary range to be used for the cluster CIDR block. The secondary range will be used for pod IP addresses. This must be an existing secondary range associated with the cluster subnetwork. This field is only applicable with use_ip_aliases is true and create_subnetwork is false.
    #[serde(rename="clusterSecondaryRangeName")]
    
    pub cluster_secondary_range_name: Option<String>,
    /// Whether a new subnetwork will be created automatically for the cluster. This field is only applicable when `use_ip_aliases` is true.
    #[serde(rename="createSubnetwork")]
    
    pub create_subnetwork: Option<bool>,
    /// The ipv6 access type (internal or external) when create_subnetwork is true
    #[serde(rename="ipv6AccessType")]
    
    pub ipv6_access_type: Option<IPAllocationPolicyIpv6AccessTypeEnum>,
    /// This field is deprecated, use node_ipv4_cidr_block.
    #[serde(rename="nodeIpv4Cidr")]
    
    pub node_ipv4_cidr: Option<String>,
    /// The IP address range of the instance IPs in this cluster. This is applicable only if `create_subnetwork` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use.
    #[serde(rename="nodeIpv4CidrBlock")]
    
    pub node_ipv4_cidr_block: Option<String>,
    /// This field is deprecated, use services_ipv4_cidr_block.
    #[serde(rename="servicesIpv4Cidr")]
    
    pub services_ipv4_cidr: Option<String>,
    /// The IP address range of the services IPs in this cluster. If blank, a range will be automatically chosen with the default size. This field is only applicable when `use_ip_aliases` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use.
    #[serde(rename="servicesIpv4CidrBlock")]
    
    pub services_ipv4_cidr_block: Option<String>,
    /// The name of the secondary range to be used as for the services CIDR block. The secondary range will be used for service ClusterIPs. This must be an existing secondary range associated with the cluster subnetwork. This field is only applicable with use_ip_aliases is true and create_subnetwork is false.
    #[serde(rename="servicesSecondaryRangeName")]
    
    pub services_secondary_range_name: Option<String>,
    /// The IP stack type of the cluster
    #[serde(rename="stackType")]
    
    pub stack_type: Option<IPAllocationPolicyStackTypeEnum>,
    /// A custom subnetwork name to be used if `create_subnetwork` is true. If this field is empty, then an automatic name will be chosen for the new subnetwork.
    #[serde(rename="subnetworkName")]
    
    pub subnetwork_name: Option<String>,
    /// The IP address range of the Cloud TPUs in this cluster. If unspecified, a range will be automatically chosen with the default size. This field is only applicable when `use_ip_aliases` is true. If unspecified, the range will use the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use.
    #[serde(rename="tpuIpv4CidrBlock")]
    
    pub tpu_ipv4_cidr_block: Option<String>,
    /// Whether alias IPs will be used for pod IPs in the cluster. This is used in conjunction with use_routes. It cannot be true if use_routes is true. If both use_ip_aliases and use_routes are false, then the server picks the default IP allocation mode
    #[serde(rename="useIpAliases")]
    
    pub use_ip_aliases: Option<bool>,
    /// Whether routes will be used for pod IPs in the cluster. This is used in conjunction with use_ip_aliases. It cannot be true if use_ip_aliases is true. If both use_ip_aliases and use_routes are false, then the server picks the default IP allocation mode
    #[serde(rename="useRoutes")]
    
    pub use_routes: Option<bool>,
}

impl client::Part for IPAllocationPolicy {}


/// IdentityServiceConfig is configuration for Identity Service which allows customers to use external identity providers with the K8S API
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityServiceConfig {
    /// Whether to enable the Identity Service component
    
    pub enabled: Option<bool>,
}

impl client::Part for IdentityServiceConfig {}


/// IntraNodeVisibilityConfig contains the desired config of the intra-node visibility on this cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntraNodeVisibilityConfig {
    /// Enables intra node visibility for this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for IntraNodeVisibilityConfig {}


/// Jwk is a JSON Web Key as specified in RFC 7517
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Jwk {
    /// Algorithm.
    
    pub alg: Option<String>,
    /// Used for ECDSA keys.
    
    pub crv: Option<String>,
    /// Used for RSA keys.
    
    pub e: Option<String>,
    /// Key ID.
    
    pub kid: Option<String>,
    /// Key Type.
    
    pub kty: Option<String>,
    /// Used for RSA keys.
    
    pub n: Option<String>,
    /// Permitted uses for the public keys.
    #[serde(rename="use")]
    
    pub use_: Option<String>,
    /// Used for ECDSA keys.
    
    pub x: Option<String>,
    /// Used for ECDSA keys.
    
    pub y: Option<String>,
}

impl client::Part for Jwk {}


/// Configuration for the Kubernetes Dashboard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KubernetesDashboard {
    /// Whether the Kubernetes Dashboard is enabled for this cluster.
    
    pub disabled: Option<bool>,
}

impl client::Part for KubernetesDashboard {}


/// Configuration for the legacy Attribute Based Access Control authorization mode.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LegacyAbac {
    /// Whether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM.
    
    pub enabled: Option<bool>,
}

impl client::Part for LegacyAbac {}


/// Parameters that can be configured on Linux nodes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinuxNodeConfig {
    /// cgroup_mode specifies the cgroup mode to be used on the node.
    #[serde(rename="cgroupMode")]
    
    pub cgroup_mode: Option<LinuxNodeConfigCgroupModeEnum>,
    /// The Linux kernel parameters to be applied to the nodes and all pods running on the nodes. The following parameters are supported. net.core.busy_poll net.core.busy_read net.core.netdev_max_backlog net.core.rmem_max net.core.wmem_default net.core.wmem_max net.core.optmem_max net.core.somaxconn net.ipv4.tcp_rmem net.ipv4.tcp_wmem net.ipv4.tcp_tw_reuse
    
    pub sysctls: Option<HashMap<String, String>>,
}

impl client::Part for LinuxNodeConfig {}


/// ListClustersResponse is the result of ListClustersRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters list projects](ProjectLocationClusterListCall) (response)
/// * [zones clusters list projects](ProjectZoneClusterListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClustersResponse {
    /// A list of clusters in the project in the specified zone, or across all ones.
    
    pub clusters: Option<Vec<Cluster>>,
    /// If any zones are listed here, the list of clusters returned may be missing those zones.
    #[serde(rename="missingZones")]
    
    pub missing_zones: Option<Vec<String>>,
}

impl client::ResponseResult for ListClustersResponse {}


/// ListNodePoolsResponse is the result of ListNodePoolsRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools list projects](ProjectLocationClusterNodePoolListCall) (response)
/// * [zones clusters node pools list projects](ProjectZoneClusterNodePoolListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNodePoolsResponse {
    /// A list of node pools for a cluster.
    #[serde(rename="nodePools")]
    
    pub node_pools: Option<Vec<NodePool>>,
}

impl client::ResponseResult for ListNodePoolsResponse {}


/// ListOperationsResponse is the result of ListOperationsRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
/// * [zones operations list projects](ProjectZoneOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// If any zones are listed here, the list of operations returned may be missing the operations from those zones.
    #[serde(rename="missingZones")]
    
    pub missing_zones: Option<Vec<String>>,
    /// A list of operations in the project in the specified zone.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// ListUsableSubnetworksResponse is the response of ListUsableSubnetworksRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [aggregated usable subnetworks list projects](ProjectAggregatedUsableSubnetworkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUsableSubnetworksResponse {
    /// This token allows you to get the next page of results for list requests. If the number of results is larger than `page_size`, use the `next_page_token` as a value for the query parameter `page_token` in the next request. The value will become empty when there are no more pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of usable subnetworks in the specified network project.
    
    pub subnetworks: Option<Vec<UsableSubnetwork>>,
}

impl client::ResponseResult for ListUsableSubnetworksResponse {}


/// LocalNvmeSsdBlockConfig contains configuration for using raw-block local NVMe SSD.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalNvmeSsdBlockConfig {
    /// The number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size. If zero, it means no raw-block local NVMe SSD disks to be attached to the node. The limit for this value is dependent upon the maximum number of disks available on a machine per zone. See: https://cloud.google.com/compute/docs/disks/local-ssd for more information.
    #[serde(rename="localSsdCount")]
    
    pub local_ssd_count: Option<i32>,
}

impl client::Part for LocalNvmeSsdBlockConfig {}


/// LoggingComponentConfig is cluster logging component configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoggingComponentConfig {
    /// Select components to collect logs. An empty set would disable all logging.
    #[serde(rename="enableComponents")]
    
    pub enable_components: Option<Vec<LoggingComponentConfigEnableComponentsEnum>>,
}

impl client::Part for LoggingComponentConfig {}


/// LoggingConfig is cluster logging configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Logging components configuration
    #[serde(rename="componentConfig")]
    
    pub component_config: Option<LoggingComponentConfig>,
}

impl client::Part for LoggingConfig {}


/// LoggingVariantConfig specifies the behaviour of the logging component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoggingVariantConfig {
    /// Logging variant deployed on nodes.
    
    pub variant: Option<LoggingVariantConfigVariantEnum>,
}

impl client::Part for LoggingVariantConfig {}


/// Represents the Maintenance exclusion option.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceExclusionOptions {
    /// Scope specifies the upgrade scope which upgrades are blocked by the exclusion.
    
    pub scope: Option<MaintenanceExclusionOptionScopeEnum>,
}

impl client::Part for MaintenanceExclusionOptions {}


/// MaintenancePolicy defines the maintenance policy to be used for the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenancePolicy {
    /// A hash identifying the version of this policy, so that updates to fields of the policy won't accidentally undo intermediate changes (and so that users of the API unaware of some fields won't accidentally remove other fields). Make a `get()` request to the cluster to get the current resource version and include it with requests to set the policy.
    #[serde(rename="resourceVersion")]
    
    pub resource_version: Option<String>,
    /// Specifies the maintenance window in which maintenance may be performed.
    
    pub window: Option<MaintenanceWindow>,
}

impl client::Part for MaintenancePolicy {}


/// MaintenanceWindow defines the maintenance window to be used for the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// DailyMaintenanceWindow specifies a daily maintenance operation window.
    #[serde(rename="dailyMaintenanceWindow")]
    
    pub daily_maintenance_window: Option<DailyMaintenanceWindow>,
    /// Exceptions to maintenance window. Non-emergency maintenance should not occur in these windows.
    #[serde(rename="maintenanceExclusions")]
    
    pub maintenance_exclusions: Option<HashMap<String, TimeWindow>>,
    /// RecurringWindow specifies some number of recurring time periods for maintenance to occur. The time windows may be overlapping. If no maintenance windows are set, maintenance can occur at any time.
    #[serde(rename="recurringWindow")]
    
    pub recurring_window: Option<RecurringTimeWindow>,
}

impl client::Part for MaintenanceWindow {}


/// ManagedPrometheusConfig defines the configuration for Google Cloud Managed Service for Prometheus.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManagedPrometheusConfig {
    /// Enable Managed Collection.
    
    pub enabled: Option<bool>,
}

impl client::Part for ManagedPrometheusConfig {}


/// The authentication information for accessing the master endpoint. Authentication can be done using HTTP basic auth or using client certificates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MasterAuth {
    /// [Output only] Base64-encoded public certificate used by clients to authenticate to the cluster endpoint.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// Configuration for client certificate authentication on the cluster. For clusters before v1.12, if no configuration is specified, a client certificate is issued.
    #[serde(rename="clientCertificateConfig")]
    
    pub client_certificate_config: Option<ClientCertificateConfig>,
    /// [Output only] Base64-encoded private key used by clients to authenticate to the cluster endpoint.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// [Output only] Base64-encoded public certificate that is the root of trust for the cluster.
    #[serde(rename="clusterCaCertificate")]
    
    pub cluster_ca_certificate: Option<String>,
    /// The password to use for HTTP basic authentication to the master endpoint. Because the master endpoint is open to the Internet, you should create a strong password. If a password is provided for cluster creation, username must be non-empty. Warning: basic authentication is deprecated, and will be removed in GKE control plane versions 1.19 and newer. For a list of recommended authentication methods, see: https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication
    
    pub password: Option<String>,
    /// The username to use for HTTP basic authentication to the master endpoint. For clusters v1.6.0 and later, basic authentication can be disabled by leaving username unspecified (or setting it to the empty string). Warning: basic authentication is deprecated, and will be removed in GKE control plane versions 1.19 and newer. For a list of recommended authentication methods, see: https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication
    
    pub username: Option<String>,
}

impl client::Part for MasterAuth {}


/// Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MasterAuthorizedNetworksConfig {
    /// cidr_blocks define up to 50 external networks that could access Kubernetes master through HTTPS.
    #[serde(rename="cidrBlocks")]
    
    pub cidr_blocks: Option<Vec<CidrBlock>>,
    /// Whether or not master authorized networks is enabled.
    
    pub enabled: Option<bool>,
    /// Whether master is accessbile via Google Compute Engine Public IP addresses.
    #[serde(rename="gcpPublicCidrsAccessEnabled")]
    
    pub gcp_public_cidrs_access_enabled: Option<bool>,
}

impl client::Part for MasterAuthorizedNetworksConfig {}


/// Constraints applied to pods.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaxPodsConstraint {
    /// Constraint enforced on the max num of pods per node.
    #[serde(rename="maxPodsPerNode")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_pods_per_node: Option<i64>,
}

impl client::Part for MaxPodsConstraint {}


/// Configuration for issuance of mTLS keys and certificates to Kubernetes pods.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MeshCertificates {
    /// enable_certificates controls issuance of workload mTLS certificates. If set, the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster, which can then be configured by creating a WorkloadCertificateConfig Custom Resource. Requires Workload Identity (workload_pool must be non-empty).
    #[serde(rename="enableCertificates")]
    
    pub enable_certificates: Option<bool>,
}

impl client::Part for MeshCertificates {}


/// Progress metric is (string, int|float|string) pair.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// For metrics with floating point value.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// For metrics with integer value.
    #[serde(rename="intValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int_value: Option<i64>,
    /// Required. Metric name, e.g., "nodes total", "percent done".
    
    pub name: Option<String>,
    /// For metrics with custom values (ratios, visual progress, etc.).
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for Metric {}


/// MonitoringComponentConfig is cluster monitoring component configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringComponentConfig {
    /// Select components to collect metrics. An empty set would disable all monitoring.
    #[serde(rename="enableComponents")]
    
    pub enable_components: Option<Vec<MonitoringComponentConfigEnableComponentsEnum>>,
}

impl client::Part for MonitoringComponentConfig {}


/// MonitoringConfig is cluster monitoring configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Monitoring components configuration
    #[serde(rename="componentConfig")]
    
    pub component_config: Option<MonitoringComponentConfig>,
    /// Enable Google Cloud Managed Service for Prometheus in the cluster.
    #[serde(rename="managedPrometheusConfig")]
    
    pub managed_prometheus_config: Option<ManagedPrometheusConfig>,
}

impl client::Part for MonitoringConfig {}


/// NetworkConfig reports the relative names of network & subnetwork.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// The desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation.
    #[serde(rename="datapathProvider")]
    
    pub datapath_provider: Option<NetworkConfigDatapathProviderEnum>,
    /// Whether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when default_snat_status is disabled. When disabled is set to false, default IP masquerade rules will be applied to the nodes to prevent sNAT on cluster internal traffic.
    #[serde(rename="defaultSnatStatus")]
    
    pub default_snat_status: Option<DefaultSnatStatus>,
    /// DNSConfig contains clusterDNS config for this cluster.
    #[serde(rename="dnsConfig")]
    
    pub dns_config: Option<DNSConfig>,
    /// Whether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network.
    #[serde(rename="enableIntraNodeVisibility")]
    
    pub enable_intra_node_visibility: Option<bool>,
    /// Whether L4ILB Subsetting is enabled for this cluster.
    #[serde(rename="enableL4ilbSubsetting")]
    
    pub enable_l4ilb_subsetting: Option<bool>,
    /// GatewayAPIConfig contains the desired config of Gateway API on this cluster.
    #[serde(rename="gatewayApiConfig")]
    
    pub gateway_api_config: Option<GatewayAPIConfig>,
    /// Output only. The relative name of the Google Compute Engine network(https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. Example: projects/my-project/global/networks/my-network
    
    pub network: Option<String>,
    /// The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)
    #[serde(rename="privateIpv6GoogleAccess")]
    
    pub private_ipv6_google_access: Option<NetworkConfigPrivateIpv6GoogleAccessEnum>,
    /// ServiceExternalIPsConfig specifies if services with externalIPs field are blocked or not.
    #[serde(rename="serviceExternalIpsConfig")]
    
    pub service_external_ips_config: Option<ServiceExternalIPsConfig>,
    /// Output only. The relative name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/vpc) to which the cluster is connected. Example: projects/my-project/regions/us-central1/subnetworks/my-subnet
    
    pub subnetwork: Option<String>,
}

impl client::Part for NetworkConfig {}


/// Configuration of all network bandwidth tiers
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkPerformanceConfig {
    /// Specifies the total network bandwidth tier for the NodePool.
    #[serde(rename="totalEgressBandwidthTier")]
    
    pub total_egress_bandwidth_tier: Option<NetworkPerformanceConfigTotalEgressBandwidthTierEnum>,
}

impl client::Part for NetworkPerformanceConfig {}


/// Configuration options for the NetworkPolicy feature. https://kubernetes.io/docs/concepts/services-networking/networkpolicies/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Whether network policy is enabled on the cluster.
    
    pub enabled: Option<bool>,
    /// The selected network policy provider.
    
    pub provider: Option<NetworkPolicyProviderEnum>,
}

impl client::Part for NetworkPolicy {}


/// Configuration for NetworkPolicy. This only tracks whether the addon is enabled or not on the Master, it does not track whether network policy is enabled for the nodes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkPolicyConfig {
    /// Whether NetworkPolicy is enabled for this cluster.
    
    pub disabled: Option<bool>,
}

impl client::Part for NetworkPolicyConfig {}


/// Collection of Compute Engine network tags that can be applied to a node's underlying VM instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkTags {
    /// List of network tags.
    
    pub tags: Option<Vec<String>>,
}

impl client::Part for NetworkTags {}


/// Parameters that describe the nodes in a cluster. GKE Autopilot clusters do not recognize parameters in `NodeConfig`. Use AutoprovisioningNodePoolDefaults instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    /// A list of hardware accelerators to be attached to each node. See https://cloud.google.com/compute/docs/gpus for more information about support for GPUs.
    
    pub accelerators: Option<Vec<AcceleratorConfig>>,
    /// Advanced features for the Compute Engine VM.
    #[serde(rename="advancedMachineFeatures")]
    
    pub advanced_machine_features: Option<AdvancedMachineFeatures>,
    ///  The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption
    #[serde(rename="bootDiskKmsKey")]
    
    pub boot_disk_kms_key: Option<String>,
    /// Confidential nodes config. All the nodes in the node pool will be Confidential VM once enabled.
    #[serde(rename="confidentialNodes")]
    
    pub confidential_nodes: Option<ConfidentialNodes>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB.
    #[serde(rename="diskSizeGb")]
    
    pub disk_size_gb: Option<i32>,
    /// Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard'
    #[serde(rename="diskType")]
    
    pub disk_type: Option<String>,
    /// Parameters for the node ephemeral storage using Local SSDs. If unspecified, ephemeral storage is backed by the boot disk.
    #[serde(rename="ephemeralStorageLocalSsdConfig")]
    
    pub ephemeral_storage_local_ssd_config: Option<EphemeralStorageLocalSsdConfig>,
    /// Enable or disable NCCL fast socket for the node pool.
    #[serde(rename="fastSocket")]
    
    pub fast_socket: Option<FastSocket>,
    /// Google Container File System (image streaming) configs.
    #[serde(rename="gcfsConfig")]
    
    pub gcfs_config: Option<GcfsConfig>,
    /// Enable or disable gvnic in the node pool.
    
    pub gvnic: Option<VirtualNIC>,
    /// The image type to use for this node. Note that for a given image type, the latest version of it will be used.
    #[serde(rename="imageType")]
    
    pub image_type: Option<String>,
    /// Node kubelet configs.
    #[serde(rename="kubeletConfig")]
    
    pub kubelet_config: Option<NodeKubeletConfig>,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node. In case of conflict in label keys, the applied set may differ depending on the Kubernetes version -- it's best to assume the behavior is undefined and conflicts should be avoided. For more information, including usage and the valid values, see: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    
    pub labels: Option<HashMap<String, String>>,
    /// Parameters that can be configured on Linux nodes.
    #[serde(rename="linuxNodeConfig")]
    
    pub linux_node_config: Option<LinuxNodeConfig>,
    /// Parameters for using raw-block Local NVMe SSDs.
    #[serde(rename="localNvmeSsdBlockConfig")]
    
    pub local_nvme_ssd_block_config: Option<LocalNvmeSsdBlockConfig>,
    /// The number of local SSD disks to be attached to the node. The limit for this value is dependent upon the maximum number of disks available on a machine per zone. See: https://cloud.google.com/compute/docs/disks/local-ssd for more information.
    #[serde(rename="localSsdCount")]
    
    pub local_ssd_count: Option<i32>,
    /// Logging configuration.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<NodePoolLoggingConfig>,
    /// The name of a Google Compute Engine [machine type](https://cloud.google.com/compute/docs/machine-types) If unspecified, the default machine type is `e2-medium`.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// The metadata key/value pairs assigned to instances in the cluster. Keys must conform to the regexp `[a-zA-Z0-9-_]+` and be less than 128 bytes in length. These are reflected as part of a URL in the metadata server. Additionally, to avoid ambiguity, keys must not conflict with any other metadata keys for the project or be one of the reserved keys: - "cluster-location" - "cluster-name" - "cluster-uid" - "configure-sh" - "containerd-configure-sh" - "enable-os-login" - "gci-ensure-gke-docker" - "gci-metrics-enabled" - "gci-update-strategy" - "instance-template" - "kube-env" - "startup-script" - "user-data" - "disable-address-manager" - "windows-startup-script-ps1" - "common-psm1" - "k8s-node-setup-psm1" - "install-ssh-psm1" - "user-profile-psm1" Values are free-form strings, and only have meaning as interpreted by the image running in the instance. The only restriction placed on them is that each value's size must be less than or equal to 32 KB. The total size of all keys and values must be less than 512 KB.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as `minCpuPlatform: "Intel Haswell"` or `minCpuPlatform: "Intel Sandy Bridge"`. For more information, read [how to specify min CPU platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
    #[serde(rename="minCpuPlatform")]
    
    pub min_cpu_platform: Option<String>,
    /// Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on [sole tenant nodes](https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes).
    #[serde(rename="nodeGroup")]
    
    pub node_group: Option<String>,
    /// The set of Google API scopes to be made available on all of the node VMs under the "default" service account. The following scopes are recommended, but not required, and by default are not included: * `https://www.googleapis.com/auth/compute` is required for mounting persistent storage on your nodes. * `https://www.googleapis.com/auth/devstorage.read_only` is required for communicating with **gcr.io** (the [Google Container Registry](https://cloud.google.com/container-registry/)). If unspecified, no scopes are added, unless Cloud Logging or Cloud Monitoring are enabled, in which case their required scopes will be added.
    #[serde(rename="oauthScopes")]
    
    pub oauth_scopes: Option<Vec<String>>,
    /// Whether the nodes are created as preemptible VM instances. See: https://cloud.google.com/compute/docs/instances/preemptible for more information about preemptible VM instances.
    
    pub preemptible: Option<bool>,
    /// The optional reservation affinity. Setting this field will apply the specified [Zonal Compute Reservation](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) to this node pool.
    #[serde(rename="reservationAffinity")]
    
    pub reservation_affinity: Option<ReservationAffinity>,
    /// The resource labels for the node pool to use to annotate any related Google Compute Engine resources.
    #[serde(rename="resourceLabels")]
    
    pub resource_labels: Option<HashMap<String, String>>,
    /// Sandbox configuration for this node.
    #[serde(rename="sandboxConfig")]
    
    pub sandbox_config: Option<SandboxConfig>,
    /// The Google Cloud Platform Service Account to be used by the node VMs. Specify the email address of the Service Account; otherwise, if no Service Account is specified, the "default" service account is used.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Shielded Instance options.
    #[serde(rename="shieldedInstanceConfig")]
    
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,
    /// Spot flag for enabling Spot VM, which is a rebrand of the existing preemptible flag.
    
    pub spot: Option<bool>,
    /// The list of instance tags applied to all nodes. Tags are used to identify valid sources or targets for network firewalls and are specified by the client during cluster or node pool creation. Each tag within the list must comply with RFC1035.
    
    pub tags: Option<Vec<String>>,
    /// List of kubernetes taints to be applied to each node. For more information, including usage and the valid values, see: https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/
    
    pub taints: Option<Vec<NodeTaint>>,
    /// Parameters that can be configured on Windows nodes.
    #[serde(rename="windowsNodeConfig")]
    
    pub windows_node_config: Option<WindowsNodeConfig>,
    /// The workload metadata configuration for this node.
    #[serde(rename="workloadMetadataConfig")]
    
    pub workload_metadata_config: Option<WorkloadMetadataConfig>,
}

impl client::Part for NodeConfig {}


/// Subset of NodeConfig message that has defaults.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfigDefaults {
    /// GCFS (Google Container File System, also known as Riptide) options.
    #[serde(rename="gcfsConfig")]
    
    pub gcfs_config: Option<GcfsConfig>,
    /// Logging configuration for node pools.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<NodePoolLoggingConfig>,
}

impl client::Part for NodeConfigDefaults {}


/// Node kubelet configs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeKubeletConfig {
    /// Enable CPU CFS quota enforcement for containers that specify CPU limits. This option is enabled by default which makes kubelet use CFS quota (https://www.kernel.org/doc/Documentation/scheduler/sched-bwc.txt) to enforce container CPU limits. Otherwise, CPU limits will not be enforced at all. Disable this option to mitigate CPU throttling problems while still having your pods to be in Guaranteed QoS class by specifying the CPU limits. The default value is 'true' if unspecified.
    #[serde(rename="cpuCfsQuota")]
    
    pub cpu_cfs_quota: Option<bool>,
    /// Set the CPU CFS quota period value 'cpu.cfs_period_us'. The string must be a sequence of decimal numbers, each with optional fraction and a unit suffix, such as "300ms". Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h". The value must be a positive duration.
    #[serde(rename="cpuCfsQuotaPeriod")]
    
    pub cpu_cfs_quota_period: Option<String>,
    /// Control the CPU management policy on the node. See https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/ The following values are allowed. * "none": the default, which represents the existing scheduling behavior. * "static": allows pods with certain resource characteristics to be granted increased CPU affinity and exclusivity on the node. The default value is 'none' if unspecified.
    #[serde(rename="cpuManagerPolicy")]
    
    pub cpu_manager_policy: Option<String>,
    /// Set the Pod PID limits. See https://kubernetes.io/docs/concepts/policy/pid-limiting/#pod-pid-limits Controls the maximum number of processes allowed to run in a pod. The value must be greater than or equal to 1024 and less than 4194304.
    #[serde(rename="podPidsLimit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pod_pids_limit: Option<i64>,
}

impl client::Part for NodeKubeletConfig {}


/// Collection of node-level [Kubernetes labels](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeLabels {
    /// Map of node label keys and node label values.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for NodeLabels {}


/// NodeManagement defines the set of node management services turned on for the node pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeManagement {
    /// A flag that specifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered.
    #[serde(rename="autoRepair")]
    
    pub auto_repair: Option<bool>,
    /// A flag that specifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes.
    #[serde(rename="autoUpgrade")]
    
    pub auto_upgrade: Option<bool>,
    /// Specifies the Auto Upgrade knobs for the node pool.
    #[serde(rename="upgradeOptions")]
    
    pub upgrade_options: Option<AutoUpgradeOptions>,
}

impl client::Part for NodeManagement {}


/// Parameters for node pool-level network config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeNetworkConfig {
    /// Input only. Whether to create a new range for pod IPs in this node pool. Defaults are provided for `pod_range` and `pod_ipv4_cidr_block` if they are not specified. If neither `create_pod_range` or `pod_range` are specified, the cluster-level default (`ip_allocation_policy.cluster_ipv4_cidr_block`) is used. Only applicable if `ip_allocation_policy.use_ip_aliases` is true. This field cannot be changed after the node pool has been created.
    #[serde(rename="createPodRange")]
    
    pub create_pod_range: Option<bool>,
    /// Whether nodes have internal IP addresses only. If enable_private_nodes is not specified, then the value is derived from cluster.privateClusterConfig.enablePrivateNodes
    #[serde(rename="enablePrivateNodes")]
    
    pub enable_private_nodes: Option<bool>,
    /// Network bandwidth tier configuration.
    #[serde(rename="networkPerformanceConfig")]
    
    pub network_performance_config: Option<NetworkPerformanceConfig>,
    /// The IP address range for pod IPs in this node pool. Only applicable if `create_pod_range` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) to pick a specific range to use. Only applicable if `ip_allocation_policy.use_ip_aliases` is true. This field cannot be changed after the node pool has been created.
    #[serde(rename="podIpv4CidrBlock")]
    
    pub pod_ipv4_cidr_block: Option<String>,
    /// The ID of the secondary range for pod IPs. If `create_pod_range` is true, this ID is used for the new range. If `create_pod_range` is false, uses an existing secondary range with this ID. Only applicable if `ip_allocation_policy.use_ip_aliases` is true. This field cannot be changed after the node pool has been created.
    #[serde(rename="podRange")]
    
    pub pod_range: Option<String>,
}

impl client::Part for NodeNetworkConfig {}


/// NodePool contains the name and configuration for a cluster’s node pool. Node pools are a set of nodes (i.e. VM’s), with a common configuration and specification, under the control of the cluster master. They may have a set of Kubernetes labels applied to them, which may be used to reference them during pod scheduling. They may also be resized up or down, to accommodate the workload.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools get projects](ProjectLocationClusterNodePoolGetCall) (response)
/// * [zones clusters node pools get projects](ProjectZoneClusterNodePoolGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodePool {
    /// Autoscaler configuration for this NodePool. Autoscaler is enabled only if a valid configuration is present.
    
    pub autoscaling: Option<NodePoolAutoscaling>,
    /// Which conditions caused the current node pool state.
    
    pub conditions: Option<Vec<StatusCondition>>,
    /// The node configuration of the pool.
    
    pub config: Option<NodeConfig>,
    /// This checksum is computed by the server based on the value of node pool fields, and may be sent on update requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// The initial node count for the pool. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota.
    #[serde(rename="initialNodeCount")]
    
    pub initial_node_count: Option<i32>,
    /// [Output only] The resource URLs of the [managed instance groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with this node pool. During the node pool blue-green upgrade operation, the URLs contain both blue and green resources.
    #[serde(rename="instanceGroupUrls")]
    
    pub instance_group_urls: Option<Vec<String>>,
    /// The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the [Cluster.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed.
    
    pub locations: Option<Vec<String>>,
    /// NodeManagement configuration for this NodePool.
    
    pub management: Option<NodeManagement>,
    /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
    #[serde(rename="maxPodsConstraint")]
    
    pub max_pods_constraint: Option<MaxPodsConstraint>,
    /// The name of the node pool.
    
    pub name: Option<String>,
    /// Networking configuration for this NodePool. If specified, it overrides the cluster-level defaults.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<NodeNetworkConfig>,
    /// Specifies the node placement policy.
    #[serde(rename="placementPolicy")]
    
    pub placement_policy: Option<PlacementPolicy>,
    /// [Output only] The pod CIDR block size per node in this node pool.
    #[serde(rename="podIpv4CidrSize")]
    
    pub pod_ipv4_cidr_size: Option<i32>,
    /// [Output only] Server-defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output only] The status of the nodes in this pool instance.
    
    pub status: Option<NodePoolStatusEnum>,
    /// [Output only] Deprecated. Use conditions instead. Additional information about the current status of this node pool instance, if available.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Output only. [Output only] Update info contains relevant information during a node pool update.
    #[serde(rename="updateInfo")]
    
    pub update_info: Option<UpdateInfo>,
    /// Upgrade settings control disruption and speed of the upgrade.
    #[serde(rename="upgradeSettings")]
    
    pub upgrade_settings: Option<UpgradeSettings>,
    /// The version of the Kubernetes of this node.
    
    pub version: Option<String>,
}

impl client::ResponseResult for NodePool {}


/// Node pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodePoolAutoConfig {
    /// The list of instance tags applied to all nodes. Tags are used to identify valid sources or targets for network firewalls and are specified by the client during cluster creation. Each tag within the list must comply with RFC1035.
    #[serde(rename="networkTags")]
    
    pub network_tags: Option<NetworkTags>,
}

impl client::Part for NodePoolAutoConfig {}


/// NodePoolAutoscaling contains information required by cluster autoscaler to adjust the size of the node pool to the current cluster usage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodePoolAutoscaling {
    /// Can this node pool be deleted automatically.
    
    pub autoprovisioned: Option<bool>,
    /// Is autoscaling enabled for this node pool.
    
    pub enabled: Option<bool>,
    /// Location policy used when scaling up a nodepool.
    #[serde(rename="locationPolicy")]
    
    pub location_policy: Option<NodePoolAutoscalingLocationPolicyEnum>,
    /// Maximum number of nodes for one location in the NodePool. Must be >= min_node_count. There has to be enough quota to scale up the cluster.
    #[serde(rename="maxNodeCount")]
    
    pub max_node_count: Option<i32>,
    /// Minimum number of nodes for one location in the NodePool. Must be >= 1 and <= max_node_count.
    #[serde(rename="minNodeCount")]
    
    pub min_node_count: Option<i32>,
    /// Maximum number of nodes in the node pool. Must be greater than total_min_node_count. There has to be enough quota to scale up the cluster. The total_*_node_count fields are mutually exclusive with the *_node_count fields.
    #[serde(rename="totalMaxNodeCount")]
    
    pub total_max_node_count: Option<i32>,
    /// Minimum number of nodes in the node pool. Must be greater than 1 less than total_max_node_count. The total_*_node_count fields are mutually exclusive with the *_node_count fields.
    #[serde(rename="totalMinNodeCount")]
    
    pub total_min_node_count: Option<i32>,
}

impl client::Part for NodePoolAutoscaling {}


/// Subset of Nodepool message that has defaults.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodePoolDefaults {
    /// Subset of NodeConfig message that has defaults.
    #[serde(rename="nodeConfigDefaults")]
    
    pub node_config_defaults: Option<NodeConfigDefaults>,
}

impl client::Part for NodePoolDefaults {}


/// NodePoolLoggingConfig specifies logging configuration for nodepools.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodePoolLoggingConfig {
    /// Logging variant configuration.
    #[serde(rename="variantConfig")]
    
    pub variant_config: Option<LoggingVariantConfig>,
}

impl client::Part for NodePoolLoggingConfig {}


/// Kubernetes taint is comprised of three fields: key, value, and effect. Effect can only be one of three types: NoSchedule, PreferNoSchedule or NoExecute. See [here](https://kubernetes.io/docs/concepts/configuration/taint-and-toleration) for more information, including usage and the valid values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeTaint {
    /// Effect for taint.
    
    pub effect: Option<NodeTaintEffectEnum>,
    /// Key for taint.
    
    pub key: Option<String>,
    /// Value for taint.
    
    pub value: Option<String>,
}

impl client::Part for NodeTaint {}


/// Collection of Kubernetes [node taints](https://kubernetes.io/docs/concepts/configuration/taint-and-toleration).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeTaints {
    /// List of node taints.
    
    pub taints: Option<Vec<NodeTaint>>,
}

impl client::Part for NodeTaints {}


/// NotificationConfig is the configuration of notifications.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Notification config for Pub/Sub.
    
    pub pubsub: Option<PubSub>,
}

impl client::Part for NotificationConfig {}


/// This operation resource represents operations that may have happened or are happening on the cluster. All fields are output only.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools create projects](ProjectLocationClusterNodePoolCreateCall) (response)
/// * [locations clusters node pools delete projects](ProjectLocationClusterNodePoolDeleteCall) (response)
/// * [locations clusters node pools rollback projects](ProjectLocationClusterNodePoolRollbackCall) (response)
/// * [locations clusters node pools set autoscaling projects](ProjectLocationClusterNodePoolSetAutoscalingCall) (response)
/// * [locations clusters node pools set management projects](ProjectLocationClusterNodePoolSetManagementCall) (response)
/// * [locations clusters node pools set size projects](ProjectLocationClusterNodePoolSetSizeCall) (response)
/// * [locations clusters node pools update projects](ProjectLocationClusterNodePoolUpdateCall) (response)
/// * [locations clusters complete ip rotation projects](ProjectLocationClusterCompleteIpRotationCall) (response)
/// * [locations clusters create projects](ProjectLocationClusterCreateCall) (response)
/// * [locations clusters delete projects](ProjectLocationClusterDeleteCall) (response)
/// * [locations clusters set addons projects](ProjectLocationClusterSetAddonCall) (response)
/// * [locations clusters set legacy abac projects](ProjectLocationClusterSetLegacyAbacCall) (response)
/// * [locations clusters set locations projects](ProjectLocationClusterSetLocationCall) (response)
/// * [locations clusters set logging projects](ProjectLocationClusterSetLoggingCall) (response)
/// * [locations clusters set maintenance policy projects](ProjectLocationClusterSetMaintenancePolicyCall) (response)
/// * [locations clusters set master auth projects](ProjectLocationClusterSetMasterAuthCall) (response)
/// * [locations clusters set monitoring projects](ProjectLocationClusterSetMonitoringCall) (response)
/// * [locations clusters set network policy projects](ProjectLocationClusterSetNetworkPolicyCall) (response)
/// * [locations clusters set resource labels projects](ProjectLocationClusterSetResourceLabelCall) (response)
/// * [locations clusters start ip rotation projects](ProjectLocationClusterStartIpRotationCall) (response)
/// * [locations clusters update projects](ProjectLocationClusterUpdateCall) (response)
/// * [locations clusters update master projects](ProjectLocationClusterUpdateMasterCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [zones clusters node pools autoscaling projects](ProjectZoneClusterNodePoolAutoscalingCall) (response)
/// * [zones clusters node pools create projects](ProjectZoneClusterNodePoolCreateCall) (response)
/// * [zones clusters node pools delete projects](ProjectZoneClusterNodePoolDeleteCall) (response)
/// * [zones clusters node pools rollback projects](ProjectZoneClusterNodePoolRollbackCall) (response)
/// * [zones clusters node pools set management projects](ProjectZoneClusterNodePoolSetManagementCall) (response)
/// * [zones clusters node pools set size projects](ProjectZoneClusterNodePoolSetSizeCall) (response)
/// * [zones clusters node pools update projects](ProjectZoneClusterNodePoolUpdateCall) (response)
/// * [zones clusters addons projects](ProjectZoneClusterAddonCall) (response)
/// * [zones clusters complete ip rotation projects](ProjectZoneClusterCompleteIpRotationCall) (response)
/// * [zones clusters create projects](ProjectZoneClusterCreateCall) (response)
/// * [zones clusters delete projects](ProjectZoneClusterDeleteCall) (response)
/// * [zones clusters legacy abac projects](ProjectZoneClusterLegacyAbacCall) (response)
/// * [zones clusters locations projects](ProjectZoneClusterLocationCall) (response)
/// * [zones clusters logging projects](ProjectZoneClusterLoggingCall) (response)
/// * [zones clusters master projects](ProjectZoneClusterMasterCall) (response)
/// * [zones clusters monitoring projects](ProjectZoneClusterMonitoringCall) (response)
/// * [zones clusters resource labels projects](ProjectZoneClusterResourceLabelCall) (response)
/// * [zones clusters set maintenance policy projects](ProjectZoneClusterSetMaintenancePolicyCall) (response)
/// * [zones clusters set master auth projects](ProjectZoneClusterSetMasterAuthCall) (response)
/// * [zones clusters set network policy projects](ProjectZoneClusterSetNetworkPolicyCall) (response)
/// * [zones clusters start ip rotation projects](ProjectZoneClusterStartIpRotationCall) (response)
/// * [zones clusters update projects](ProjectZoneClusterUpdateCall) (response)
/// * [zones operations get projects](ProjectZoneOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// Which conditions caused the current cluster state. Deprecated. Use field error instead.
    #[serde(rename="clusterConditions")]
    
    pub cluster_conditions: Option<Vec<StatusCondition>>,
    /// Detailed operation progress, if available.
    
    pub detail: Option<String>,
    /// [Output only] The time the operation completed, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// The error result of the operation in case of failure.
    
    pub error: Option<Status>,
    /// [Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides.
    
    pub location: Option<String>,
    /// The server-assigned ID for the operation.
    
    pub name: Option<String>,
    /// Which conditions caused the current node pool state. Deprecated. Use field error instead.
    #[serde(rename="nodepoolConditions")]
    
    pub nodepool_conditions: Option<Vec<StatusCondition>>,
    /// The operation type.
    #[serde(rename="operationType")]
    
    pub operation_type: Option<OperationOperationTypeEnum>,
    /// Output only. [Output only] Progress information for an operation.
    
    pub progress: Option<OperationProgress>,
    /// Server-defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output only] The time the operation started, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// The current status of the operation.
    
    pub status: Option<OperationStatusEnum>,
    /// Output only. If an error has occurred, a textual description of the error. Deprecated. Use the field error instead.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Server-defined URL for the target of the operation.
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation is taking place. This field is deprecated, use location instead.
    
    pub zone: Option<String>,
}

impl client::ResponseResult for Operation {}


/// Information about operation (or operation stage) progress.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationProgress {
    /// Progress metric bundle, for example: metrics: [{name: "nodes done", int_value: 15}, {name: "nodes total", int_value: 32}] or metrics: [{name: "progress", double_value: 0.56}, {name: "progress scale", double_value: 1.0}]
    
    pub metrics: Option<Vec<Metric>>,
    /// A non-parameterized string describing an operation stage. Unset for single-stage operations.
    
    pub name: Option<String>,
    /// Substages of an operation or a stage.
    
    pub stages: Option<Vec<OperationProgress>>,
    /// Status of an operation stage. Unset for single-stage operations.
    
    pub status: Option<OperationProgresStatusEnum>,
}

impl client::Part for OperationProgress {}


/// PlacementPolicy defines the placement policy used by the node pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacementPolicy {
    /// The type of placement.
    #[serde(rename="type")]
    
    pub type_: Option<PlacementPolicyTypeEnum>,
}

impl client::Part for PlacementPolicy {}


/// Configuration options for private clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateClusterConfig {
    /// Whether the master's internal IP address is used as the cluster endpoint.
    #[serde(rename="enablePrivateEndpoint")]
    
    pub enable_private_endpoint: Option<bool>,
    /// Whether nodes have internal IP addresses only. If enabled, all nodes are given only RFC 1918 private addresses and communicate with the master via private networking.
    #[serde(rename="enablePrivateNodes")]
    
    pub enable_private_nodes: Option<bool>,
    /// Controls master global access settings.
    #[serde(rename="masterGlobalAccessConfig")]
    
    pub master_global_access_config: Option<PrivateClusterMasterGlobalAccessConfig>,
    /// The IP range in CIDR notation to use for the hosted master network. This range will be used for assigning internal IP addresses to the master or set of masters, as well as the ILB VIP. This range must not overlap with any other ranges in use within the cluster's network.
    #[serde(rename="masterIpv4CidrBlock")]
    
    pub master_ipv4_cidr_block: Option<String>,
    /// Output only. The peering name in the customer VPC used by this cluster.
    #[serde(rename="peeringName")]
    
    pub peering_name: Option<String>,
    /// Output only. The internal IP address of this cluster's master endpoint.
    #[serde(rename="privateEndpoint")]
    
    pub private_endpoint: Option<String>,
    /// Subnet to provision the master's private endpoint during cluster creation. Specified in projects/*/regions/*/subnetworks/* format.
    #[serde(rename="privateEndpointSubnetwork")]
    
    pub private_endpoint_subnetwork: Option<String>,
    /// Output only. The external IP address of this cluster's master endpoint.
    #[serde(rename="publicEndpoint")]
    
    pub public_endpoint: Option<String>,
}

impl client::Part for PrivateClusterConfig {}


/// Configuration for controlling master global access settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateClusterMasterGlobalAccessConfig {
    /// Whenever master is accessible globally or not.
    
    pub enabled: Option<bool>,
}

impl client::Part for PrivateClusterMasterGlobalAccessConfig {}


/// Pub/Sub specific notification config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PubSub {
    /// Enable notifications for Pub/Sub.
    
    pub enabled: Option<bool>,
    /// Allows filtering to one or more specific event types. If no filter is specified, or if a filter is specified with no event types, all event types will be sent
    
    pub filter: Option<Filter>,
    /// The desired Pub/Sub topic to which notifications will be sent by GKE. Format is `projects/{project}/topics/{topic}`.
    
    pub topic: Option<String>,
}

impl client::Part for PubSub {}


/// Represents an arbitrary window of time that recurs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecurringTimeWindow {
    /// An RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how this window reccurs. They go on for the span of time between the start and end time. For example, to have something repeat every weekday, you'd use: `FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR` To repeat some window daily (equivalent to the DailyMaintenanceWindow): `FREQ=DAILY` For the first weekend of every month: `FREQ=MONTHLY;BYSETPOS=1;BYDAY=SA,SU` This specifies how frequently the window starts. Eg, if you wanted to have a 9-5 UTC-4 window every weekday, you'd use something like: ``` start time = 2019-01-01T09:00:00-0400 end time = 2019-01-01T17:00:00-0400 recurrence = FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR ``` Windows can span multiple days. Eg, to make the window encompass every weekend from midnight Saturday till the last minute of Sunday UTC: ``` start time = 2019-01-05T00:00:00Z end time = 2019-01-07T23:59:00Z recurrence = FREQ=WEEKLY;BYDAY=SA ``` Note the start and end time's specific dates are largely arbitrary except to specify duration of the window and when it first starts. The FREQ values of HOURLY, MINUTELY, and SECONDLY are not supported.
    
    pub recurrence: Option<String>,
    /// The window of the first recurrence.
    
    pub window: Option<TimeWindow>,
}

impl client::Part for RecurringTimeWindow {}


/// ReleaseChannel indicates which release channel a cluster is subscribed to. Release channels are arranged in order of risk. When a cluster is subscribed to a release channel, Google maintains both the master version and the node version. Node auto-upgrade defaults to true and cannot be disabled.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseChannel {
    /// channel specifies which release channel the cluster is subscribed to.
    
    pub channel: Option<ReleaseChannelChannelEnum>,
}

impl client::Part for ReleaseChannel {}


/// ReleaseChannelConfig exposes configuration for a release channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseChannelConfig {
    /// The release channel this configuration applies to.
    
    pub channel: Option<ReleaseChannelConfigChannelEnum>,
    /// The default version for newly created clusters on the channel.
    #[serde(rename="defaultVersion")]
    
    pub default_version: Option<String>,
    /// List of valid versions for the channel.
    #[serde(rename="validVersions")]
    
    pub valid_versions: Option<Vec<String>>,
}

impl client::Part for ReleaseChannelConfig {}


/// [ReservationAffinity](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) is the configuration of desired reservation which instances could take capacity from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReservationAffinity {
    /// Corresponds to the type of reservation consumption.
    #[serde(rename="consumeReservationType")]
    
    pub consume_reservation_type: Option<ReservationAffinityConsumeReservationTypeEnum>,
    /// Corresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify "compute.googleapis.com/reservation-name" as the key and specify the name of your reservation as its value.
    
    pub key: Option<String>,
    /// Corresponds to the label value(s) of reservation resource(s).
    
    pub values: Option<Vec<String>>,
}

impl client::Part for ReservationAffinity {}


/// Collection of [GCP labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceLabels {
    /// Map of node label keys and node label values.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for ResourceLabels {}


/// Contains information about amount of some resource in the cluster. For memory, value should be in GB.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceLimit {
    /// Maximum amount of the resource in the cluster.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum: Option<i64>,
    /// Minimum amount of the resource in the cluster.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub minimum: Option<i64>,
    /// Resource name "cpu", "memory" or gpu-specific string.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
}

impl client::Part for ResourceLimit {}


/// Configuration for exporting cluster resource usages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUsageExportConfig {
    /// Configuration to use BigQuery as usage export destination.
    #[serde(rename="bigqueryDestination")]
    
    pub bigquery_destination: Option<BigQueryDestination>,
    /// Configuration to enable resource consumption metering.
    #[serde(rename="consumptionMeteringConfig")]
    
    pub consumption_metering_config: Option<ConsumptionMeteringConfig>,
    /// Whether to enable network egress metering for this cluster. If enabled, a daemonset will be created in the cluster to meter network egress traffic.
    #[serde(rename="enableNetworkEgressMetering")]
    
    pub enable_network_egress_metering: Option<bool>,
}

impl client::Part for ResourceUsageExportConfig {}


/// RollbackNodePoolUpgradeRequest rollbacks the previously Aborted or Failed NodePool upgrade. This will be an no-op if the last upgrade successfully completed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools rollback projects](ProjectLocationClusterNodePoolRollbackCall) (request)
/// * [zones clusters node pools rollback projects](ProjectZoneClusterNodePoolRollbackCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackNodePoolUpgradeRequest {
    /// Deprecated. The name of the cluster to rollback. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster, node pool id) of the node poll to rollback upgrade. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    
    pub name: Option<String>,
    /// Deprecated. The name of the node pool to rollback. This field has been deprecated and replaced by the name field.
    #[serde(rename="nodePoolId")]
    
    pub node_pool_id: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Option for rollback to ignore the PodDisruptionBudget. Default value is false.
    #[serde(rename="respectPdb")]
    
    pub respect_pdb: Option<bool>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for RollbackNodePoolUpgradeRequest {}


/// SandboxConfig contains configurations of the sandbox to use for the node.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Type of the sandbox to use for the node.
    #[serde(rename="type")]
    
    pub type_: Option<SandboxConfigTypeEnum>,
}

impl client::Part for SandboxConfig {}


/// Kubernetes Engine service configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get server config projects](ProjectLocationGetServerConfigCall) (response)
/// * [zones get serverconfig projects](ProjectZoneGetServerconfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// List of release channel configurations.
    
    pub channels: Option<Vec<ReleaseChannelConfig>>,
    /// Version of Kubernetes the service deploys by default.
    #[serde(rename="defaultClusterVersion")]
    
    pub default_cluster_version: Option<String>,
    /// Default image type.
    #[serde(rename="defaultImageType")]
    
    pub default_image_type: Option<String>,
    /// List of valid image types.
    #[serde(rename="validImageTypes")]
    
    pub valid_image_types: Option<Vec<String>>,
    /// List of valid master versions, in descending order.
    #[serde(rename="validMasterVersions")]
    
    pub valid_master_versions: Option<Vec<String>>,
    /// List of valid node upgrade target versions, in descending order.
    #[serde(rename="validNodeVersions")]
    
    pub valid_node_versions: Option<Vec<String>>,
}

impl client::ResponseResult for ServerConfig {}


/// Config to block services with externalIPs field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceExternalIPsConfig {
    /// Whether Services with ExternalIPs field are allowed or not.
    
    pub enabled: Option<bool>,
}

impl client::Part for ServiceExternalIPsConfig {}


/// SetAddonsConfigRequest sets the addons associated with the cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set addons projects](ProjectLocationClusterSetAddonCall) (request)
/// * [zones clusters addons projects](ProjectZoneClusterAddonCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetAddonsConfigRequest {
    /// Required. The desired configurations for the various addons available to run in the cluster.
    #[serde(rename="addonsConfig")]
    
    pub addons_config: Option<AddonsConfig>,
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster) of the cluster to set addons. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetAddonsConfigRequest {}


/// SetLabelsRequest sets the Google Cloud Platform labels on a Google Container Engine cluster, which will in turn set them for Google Compute Engine resources used by that cluster
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set resource labels projects](ProjectLocationClusterSetResourceLabelCall) (request)
/// * [zones clusters resource labels projects](ProjectZoneClusterResourceLabelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetLabelsRequest {
    /// Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. The fingerprint of the previous set of labels for this resource, used to detect conflicts. The fingerprint is initially generated by Kubernetes Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash when updating or changing labels. Make a `get()` request to the resource to get the latest fingerprint.
    #[serde(rename="labelFingerprint")]
    
    pub label_fingerprint: Option<String>,
    /// The name (project, location, cluster name) of the cluster to set labels. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. The labels to set for that cluster.
    #[serde(rename="resourceLabels")]
    
    pub resource_labels: Option<HashMap<String, String>>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetLabelsRequest {}


/// SetLegacyAbacRequest enables or disables the ABAC authorization mechanism for a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set legacy abac projects](ProjectLocationClusterSetLegacyAbacCall) (request)
/// * [zones clusters legacy abac projects](ProjectZoneClusterLegacyAbacCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetLegacyAbacRequest {
    /// Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. Whether ABAC authorization will be enabled in the cluster.
    
    pub enabled: Option<bool>,
    /// The name (project, location, cluster name) of the cluster to set legacy abac. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetLegacyAbacRequest {}


/// SetLocationsRequest sets the locations of the cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set locations projects](ProjectLocationClusterSetLocationCall) (request)
/// * [zones clusters locations projects](ProjectZoneClusterLocationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetLocationsRequest {
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. Changing the locations a cluster is in will result in nodes being either created or removed from the cluster, depending on whether locations are being added or removed. This list must always include the cluster's primary zone.
    
    pub locations: Option<Vec<String>>,
    /// The name (project, location, cluster) of the cluster to set locations. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetLocationsRequest {}


/// SetLoggingServiceRequest sets the logging service of a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set logging projects](ProjectLocationClusterSetLoggingCall) (request)
/// * [zones clusters logging projects](ProjectZoneClusterLoggingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetLoggingServiceRequest {
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions.
    #[serde(rename="loggingService")]
    
    pub logging_service: Option<String>,
    /// The name (project, location, cluster) of the cluster to set logging. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetLoggingServiceRequest {}


/// SetMaintenancePolicyRequest sets the maintenance policy for a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set maintenance policy projects](ProjectLocationClusterSetMaintenancePolicyCall) (request)
/// * [zones clusters set maintenance policy projects](ProjectZoneClusterSetMaintenancePolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetMaintenancePolicyRequest {
    /// Required. The name of the cluster to update.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. The maintenance policy to be set for the cluster. An empty field clears the existing maintenance policy.
    #[serde(rename="maintenancePolicy")]
    
    pub maintenance_policy: Option<MaintenancePolicy>,
    /// The name (project, location, cluster name) of the cluster to set maintenance policy. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Required. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects).
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetMaintenancePolicyRequest {}


/// SetMasterAuthRequest updates the admin password of a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set master auth projects](ProjectLocationClusterSetMasterAuthCall) (request)
/// * [zones clusters set master auth projects](ProjectZoneClusterSetMasterAuthCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetMasterAuthRequest {
    /// Required. The exact form of action to be taken on the master auth.
    
    pub action: Option<SetMasterAuthRequestActionEnum>,
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster) of the cluster to set auth. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. A description of the update.
    
    pub update: Option<MasterAuth>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetMasterAuthRequest {}


/// SetMonitoringServiceRequest sets the monitoring service of a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set monitoring projects](ProjectLocationClusterSetMonitoringCall) (request)
/// * [zones clusters monitoring projects](ProjectZoneClusterMonitoringCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetMonitoringServiceRequest {
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. The monitoring service the cluster should use to write metrics. Currently available options: * "monitoring.googleapis.com/kubernetes" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions.
    #[serde(rename="monitoringService")]
    
    pub monitoring_service: Option<String>,
    /// The name (project, location, cluster) of the cluster to set monitoring. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetMonitoringServiceRequest {}


/// SetNetworkPolicyRequest enables/disables network policy for a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters set network policy projects](ProjectLocationClusterSetNetworkPolicyCall) (request)
/// * [zones clusters set network policy projects](ProjectZoneClusterSetNetworkPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetNetworkPolicyRequest {
    /// Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster name) of the cluster to set networking policy. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Required. Configuration options for the NetworkPolicy feature.
    #[serde(rename="networkPolicy")]
    
    pub network_policy: Option<NetworkPolicy>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetNetworkPolicyRequest {}


/// SetNodePoolAutoscalingRequest sets the autoscaler settings of a node pool.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools set autoscaling projects](ProjectLocationClusterNodePoolSetAutoscalingCall) (request)
/// * [zones clusters node pools autoscaling projects](ProjectZoneClusterNodePoolAutoscalingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetNodePoolAutoscalingRequest {
    /// Required. Autoscaling configuration for the node pool.
    
    pub autoscaling: Option<NodePoolAutoscaling>,
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster, node pool) of the node pool to set autoscaler settings. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    
    pub name: Option<String>,
    /// Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="nodePoolId")]
    
    pub node_pool_id: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetNodePoolAutoscalingRequest {}


/// SetNodePoolManagementRequest sets the node management properties of a node pool.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools set management projects](ProjectLocationClusterNodePoolSetManagementCall) (request)
/// * [zones clusters node pools set management projects](ProjectZoneClusterNodePoolSetManagementCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetNodePoolManagementRequest {
    /// Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. NodeManagement configuration for the node pool.
    
    pub management: Option<NodeManagement>,
    /// The name (project, location, cluster, node pool id) of the node pool to set management properties. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    
    pub name: Option<String>,
    /// Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field.
    #[serde(rename="nodePoolId")]
    
    pub node_pool_id: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetNodePoolManagementRequest {}


/// SetNodePoolSizeRequest sets the size of a node pool.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools set size projects](ProjectLocationClusterNodePoolSetSizeCall) (request)
/// * [zones clusters node pools set size projects](ProjectZoneClusterNodePoolSetSizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetNodePoolSizeRequest {
    /// Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster, node pool id) of the node pool to set size. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    
    pub name: Option<String>,
    /// Required. The desired node count for the pool.
    #[serde(rename="nodeCount")]
    
    pub node_count: Option<i32>,
    /// Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field.
    #[serde(rename="nodePoolId")]
    
    pub node_pool_id: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for SetNodePoolSizeRequest {}


/// A set of Shielded Instance options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShieldedInstanceConfig {
    /// Defines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created.
    #[serde(rename="enableIntegrityMonitoring")]
    
    pub enable_integrity_monitoring: Option<bool>,
    /// Defines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails.
    #[serde(rename="enableSecureBoot")]
    
    pub enable_secure_boot: Option<bool>,
}

impl client::Part for ShieldedInstanceConfig {}


/// Configuration of Shielded Nodes feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShieldedNodes {
    /// Whether Shielded Nodes features are enabled on all nodes in this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for ShieldedNodes {}


/// Standard rollout policy is the default policy for blue-green.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StandardRolloutPolicy {
    /// Number of blue nodes to drain in a batch.
    #[serde(rename="batchNodeCount")]
    
    pub batch_node_count: Option<i32>,
    /// Percentage of the blue pool nodes to drain in a batch. The range of this field should be (0.0, 1.0].
    #[serde(rename="batchPercentage")]
    
    pub batch_percentage: Option<f32>,
    /// Soak time after each batch gets drained. Default to zero.
    #[serde(rename="batchSoakDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub batch_soak_duration: Option<client::chrono::Duration>,
}

impl client::Part for StandardRolloutPolicy {}


/// StartIPRotationRequest creates a new IP for the cluster and then performs a node upgrade on each node pool to point to the new IP.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters start ip rotation projects](ProjectLocationClusterStartIpRotationCall) (request)
/// * [zones clusters start ip rotation projects](ProjectZoneClusterStartIpRotationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartIPRotationRequest {
    /// Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster name) of the cluster to start IP rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Whether to rotate credentials during IP rotation.
    #[serde(rename="rotateCredentials")]
    
    pub rotate_credentials: Option<bool>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for StartIPRotationRequest {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// StatusCondition describes why a cluster or a node pool has a certain status (e.g., ERROR or DEGRADED).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatusCondition {
    /// Canonical code of the condition.
    #[serde(rename="canonicalCode")]
    
    pub canonical_code: Option<StatusConditionCanonicalCodeEnum>,
    /// Machine-friendly representation of the condition Deprecated. Use canonical_code instead.
    
    pub code: Option<StatusConditionCodeEnum>,
    /// Human-friendly representation of the condition
    
    pub message: Option<String>,
}

impl client::Part for StatusCondition {}


/// Represents an arbitrary window of time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeWindow {
    /// The time that the window ends. The end time should take place after the start time.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// MaintenanceExclusionOptions provides maintenance exclusion related options.
    #[serde(rename="maintenanceExclusionOptions")]
    
    pub maintenance_exclusion_options: Option<MaintenanceExclusionOptions>,
    /// The time that the window first starts.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeWindow {}


/// UpdateClusterRequest updates the settings of a cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters update projects](ProjectLocationClusterUpdateCall) (request)
/// * [zones clusters update projects](ProjectZoneClusterUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateClusterRequest {
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. A description of the update.
    
    pub update: Option<ClusterUpdate>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for UpdateClusterRequest {}


/// UpdateInfo contains resource (instance groups, etc), status and other intermediate information relevant to a node pool upgrade.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// Information of a blue-green upgrade.
    #[serde(rename="blueGreenInfo")]
    
    pub blue_green_info: Option<BlueGreenInfo>,
}

impl client::Part for UpdateInfo {}


/// UpdateMasterRequest updates the master of the cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters update master projects](ProjectLocationClusterUpdateMasterCall) (request)
/// * [zones clusters master projects](ProjectZoneClusterMasterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMasterRequest {
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Required. The Kubernetes version to change the master to. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "-": picks the default Kubernetes version
    #[serde(rename="masterVersion")]
    
    pub master_version: Option<String>,
    /// The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`.
    
    pub name: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for UpdateMasterRequest {}


/// UpdateNodePoolRequests update a node pool’s image and/or version.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations clusters node pools update projects](ProjectLocationClusterNodePoolUpdateCall) (request)
/// * [zones clusters node pools update projects](ProjectZoneClusterNodePoolUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateNodePoolRequest {
    /// Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Confidential nodes config. All the nodes in the node pool will be Confidential VM once enabled.
    #[serde(rename="confidentialNodes")]
    
    pub confidential_nodes: Option<ConfidentialNodes>,
    /// The current etag of the node pool. If an etag is provided and does not match the current etag of the node pool, update will be blocked and an ABORTED error will be returned.
    
    pub etag: Option<String>,
    /// Enable or disable NCCL fast socket for the node pool.
    #[serde(rename="fastSocket")]
    
    pub fast_socket: Option<FastSocket>,
    /// GCFS config.
    #[serde(rename="gcfsConfig")]
    
    pub gcfs_config: Option<GcfsConfig>,
    /// Enable or disable gvnic on the node pool.
    
    pub gvnic: Option<VirtualNIC>,
    /// Required. The desired image type for the node pool.
    #[serde(rename="imageType")]
    
    pub image_type: Option<String>,
    /// Node kubelet configs.
    #[serde(rename="kubeletConfig")]
    
    pub kubelet_config: Option<NodeKubeletConfig>,
    /// The desired node labels to be applied to all nodes in the node pool. If this field is not present, the labels will not be changed. Otherwise, the existing node labels will be *replaced* with the provided labels.
    
    pub labels: Option<NodeLabels>,
    /// Parameters that can be configured on Linux nodes.
    #[serde(rename="linuxNodeConfig")]
    
    pub linux_node_config: Option<LinuxNodeConfig>,
    /// The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the node pool's nodes should be located. Changing the locations for a node pool will result in nodes being either created or removed from the node pool, depending on whether locations are being added or removed.
    
    pub locations: Option<Vec<String>>,
    /// Logging configuration.
    #[serde(rename="loggingConfig")]
    
    pub logging_config: Option<NodePoolLoggingConfig>,
    /// The name (project, location, cluster, node pool) of the node pool to update. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    
    pub name: Option<String>,
    /// Node network config.
    #[serde(rename="nodeNetworkConfig")]
    
    pub node_network_config: Option<NodeNetworkConfig>,
    /// Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field.
    #[serde(rename="nodePoolId")]
    
    pub node_pool_id: Option<String>,
    /// Required. The Kubernetes version to change the nodes to (typically an upgrade). Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - "latest": picks the highest valid Kubernetes version - "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit Kubernetes version - "-": picks the Kubernetes master version
    #[serde(rename="nodeVersion")]
    
    pub node_version: Option<String>,
    /// Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The resource labels for the node pool to use to annotate any related Google Compute Engine resources.
    #[serde(rename="resourceLabels")]
    
    pub resource_labels: Option<ResourceLabels>,
    /// The desired network tags to be applied to all nodes in the node pool. If this field is not present, the tags will not be changed. Otherwise, the existing network tags will be *replaced* with the provided tags.
    
    pub tags: Option<NetworkTags>,
    /// The desired node taints to be applied to all nodes in the node pool. If this field is not present, the taints will not be changed. Otherwise, the existing node taints will be *replaced* with the provided taints.
    
    pub taints: Option<NodeTaints>,
    /// Upgrade settings control disruption and speed of the upgrade.
    #[serde(rename="upgradeSettings")]
    
    pub upgrade_settings: Option<UpgradeSettings>,
    /// Parameters that can be configured on Windows nodes.
    #[serde(rename="windowsNodeConfig")]
    
    pub windows_node_config: Option<WindowsNodeConfig>,
    /// The desired workload metadata config for the node pool.
    #[serde(rename="workloadMetadataConfig")]
    
    pub workload_metadata_config: Option<WorkloadMetadataConfig>,
    /// Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field.
    
    pub zone: Option<String>,
}

impl client::RequestValue for UpdateNodePoolRequest {}


/// These upgrade settings control the level of parallelism and the level of disruption caused by an upgrade. maxUnavailable controls the number of nodes that can be simultaneously unavailable. maxSurge controls the number of additional nodes that can be added to the node pool temporarily for the time of the upgrade to increase the number of available nodes. (maxUnavailable + maxSurge) determines the level of parallelism (how many nodes are being upgraded at the same time). Note: upgrades inevitably introduce some disruption since workloads need to be moved from old nodes to new, upgraded ones. Even if maxUnavailable=0, this holds true. (Disruption stays within the limits of PodDisruptionBudget, if it is configured.) Consider a hypothetical node pool with 5 nodes having maxSurge=2, maxUnavailable=1. This means the upgrade process upgrades 3 nodes simultaneously. It creates 2 additional (upgraded) nodes, then it brings down 3 old (not yet upgraded) nodes at the same time. This ensures that there are always at least 4 nodes available. These upgrade settings configure the upgrade strategy for the node pool. Use strategy to switch between the strategies applied to the node pool. If the strategy is ROLLING, use max_surge and max_unavailable to control the level of parallelism and the level of disruption caused by upgrade. 1. maxSurge controls the number of additional nodes that can be added to the node pool temporarily for the time of the upgrade to increase the number of available nodes. 2. maxUnavailable controls the number of nodes that can be simultaneously unavailable. 3. (maxUnavailable + maxSurge) determines the level of parallelism (how many nodes are being upgraded at the same time). If the strategy is BLUE_GREEN, use blue_green_settings to configure the blue-green upgrade related settings. 1. standard_rollout_policy is the default policy. The policy is used to control the way blue pool gets drained. The draining is executed in the batch mode. The batch size could be specified as either percentage of the node pool size or the number of nodes. batch_soak_duration is the soak time after each batch gets drained. 2. node_pool_soak_duration is the soak time after all blue nodes are drained. After this period, the blue pool nodes will be deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeSettings {
    /// Settings for blue-green upgrade strategy.
    #[serde(rename="blueGreenSettings")]
    
    pub blue_green_settings: Option<BlueGreenSettings>,
    /// The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process.
    #[serde(rename="maxSurge")]
    
    pub max_surge: Option<i32>,
    /// The maximum number of nodes that can be simultaneously unavailable during the upgrade process. A node is considered available if its status is Ready.
    #[serde(rename="maxUnavailable")]
    
    pub max_unavailable: Option<i32>,
    /// Update strategy of the node pool.
    
    pub strategy: Option<UpgradeSettingStrategyEnum>,
}

impl client::Part for UpgradeSettings {}


/// UsableSubnetwork resource returns the subnetwork name, its associated network and the primary CIDR range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsableSubnetwork {
    /// The range of internal addresses that are owned by this subnetwork.
    #[serde(rename="ipCidrRange")]
    
    pub ip_cidr_range: Option<String>,
    /// Network Name. Example: projects/my-project/global/networks/my-network
    
    pub network: Option<String>,
    /// Secondary IP ranges.
    #[serde(rename="secondaryIpRanges")]
    
    pub secondary_ip_ranges: Option<Vec<UsableSubnetworkSecondaryRange>>,
    /// A human readable status message representing the reasons for cases where the caller cannot use the secondary ranges under the subnet. For example if the secondary_ip_ranges is empty due to a permission issue, an insufficient permission message will be given by status_message.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Subnetwork Name. Example: projects/my-project/regions/us-central1/subnetworks/my-subnet
    
    pub subnetwork: Option<String>,
}

impl client::Part for UsableSubnetwork {}


/// Secondary IP range of a usable subnetwork.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsableSubnetworkSecondaryRange {
    /// The range of IP addresses belonging to this subnetwork secondary range.
    #[serde(rename="ipCidrRange")]
    
    pub ip_cidr_range: Option<String>,
    /// The name associated with this subnetwork secondary range, used when adding an alias IP range to a VM instance.
    #[serde(rename="rangeName")]
    
    pub range_name: Option<String>,
    /// This field is to determine the status of the secondary range programmably.
    
    pub status: Option<UsableSubnetworkSecondaryRangeStatusEnum>,
}

impl client::Part for UsableSubnetworkSecondaryRange {}


/// VerticalPodAutoscaling contains global, per-cluster information required by Vertical Pod Autoscaler to automatically adjust the resources of pods controlled by it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerticalPodAutoscaling {
    /// Enables vertical pod autoscaling.
    
    pub enabled: Option<bool>,
}

impl client::Part for VerticalPodAutoscaling {}


/// Configuration of gVNIC feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VirtualNIC {
    /// Whether gVNIC features are enabled in the node pool.
    
    pub enabled: Option<bool>,
}

impl client::Part for VirtualNIC {}


/// Parameters that can be configured on Windows nodes. Windows Node Config that define the parameters that will be used to configure the Windows node pool settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsNodeConfig {
    /// OSVersion specifies the Windows node config to be used on the node
    #[serde(rename="osVersion")]
    
    pub os_version: Option<WindowsNodeConfigOsVersionEnum>,
}

impl client::Part for WindowsNodeConfig {}


/// Configuration for the use of Kubernetes Service Accounts in GCP IAM policies.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadIdentityConfig {
    /// The workload pool to attach all Kubernetes service accounts to.
    #[serde(rename="workloadPool")]
    
    pub workload_pool: Option<String>,
}

impl client::Part for WorkloadIdentityConfig {}


/// WorkloadMetadataConfig defines the metadata configuration to expose to workloads on the node pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadMetadataConfig {
    /// Mode is the configuration for how to expose metadata to workloads running on the node pool.
    
    pub mode: Option<WorkloadMetadataConfigModeEnum>,
}

impl client::Part for WorkloadMetadataConfig {}


