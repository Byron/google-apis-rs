use super::*;
/// A Compute Engine network accessConfig. Identical to the accessConfig on corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessConfig {
    /// Name of this access configuration.
    
    pub name: Option<String>,
    /// An external IP address associated with this instance.
    #[serde(rename="natIp")]
    
    pub nat_ip: Option<String>,
    /// Type of this access configuration file. (Currently only ONE_TO_ONE_NAT is legal.)
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for AccessConfig {}


/// An Action encapsulates a set of commands as a single runnable module with additional information needed during run-time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    /// A list of commands to run sequentially for this action.
    
    pub commands: Option<Vec<String>>,
    /// The timeout in milliseconds for this action to run.
    #[serde(rename="timeoutMs")]
    
    pub timeout_ms: Option<i32>,
}

impl client::Part for Action {}


/// An allowed port resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllowedRule {
    /// ?tcp?, ?udp? or ?icmp?
    #[serde(rename="IPProtocol")]
    
    pub ip_protocol: Option<String>,
    /// List of ports or port ranges (Example inputs include: ["22"], [?33?, "12345-12349"].
    
    pub ports: Option<Vec<String>>,
}

impl client::Part for AllowedRule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingModule {
    /// no description provided
    #[serde(rename="coolDownPeriodSec")]
    
    pub cool_down_period_sec: Option<i32>,
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="maxNumReplicas")]
    
    pub max_num_replicas: Option<i32>,
    /// no description provided
    #[serde(rename="minNumReplicas")]
    
    pub min_num_replicas: Option<i32>,
    /// no description provided
    #[serde(rename="signalType")]
    
    pub signal_type: Option<String>,
    /// no description provided
    #[serde(rename="targetModule")]
    
    pub target_module: Option<String>,
    /// target_utilization should be in range [0,1].
    #[serde(rename="targetUtilization")]
    
    pub target_utilization: Option<f64>,
}

impl client::Part for AutoscalingModule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingModuleStatus {
    /// [Output Only] The URL of the corresponding Autoscaling configuration.
    #[serde(rename="autoscalingConfigUrl")]
    
    pub autoscaling_config_url: Option<String>,
}

impl client::Part for AutoscalingModuleStatus {}


/// [Output Only] The current state of a replica or module.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeployState {
    /// [Output Only] Human readable details about the current state.
    
    pub details: Option<String>,
    /// [Output Only] The status of the deployment. Possible values include: 
    /// - UNKNOWN
    /// - DEPLOYING
    /// - DEPLOYED
    /// - DEPLOYMENT_FAILED
    /// - DELETING
    /// - DELETED
    /// - DELETE_FAILED
    
    pub status: Option<String>,
}

impl client::Part for DeployState {}


/// A deployment represents a physical instantiation of a Template.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete deployments](DeploymentDeleteCall) (none)
/// * [get deployments](DeploymentGetCall) (response)
/// * [insert deployments](DeploymentInsertCall) (request|response)
/// * [list deployments](DeploymentListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    /// [Output Only] The time when this deployment was created.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// A user-supplied description of this Deployment.
    
    pub description: Option<String>,
    /// [Output Only] List of status for the modules in this deployment.
    
    pub modules: Option<HashMap<String, ModuleStatus>>,
    /// Name of this deployment. The name must conform to the following regular expression: [a-zA-Z0-9-_]{1,64}
    
    pub name: Option<String>,
    /// The set of parameter overrides to apply to the corresponding Template before deploying.
    
    pub overrides: Option<Vec<ParamOverride>>,
    /// [Output Only] Current status of this deployment.
    
    pub state: Option<DeployState>,
    /// The name of the Template on which this deployment is based.
    #[serde(rename="templateName")]
    
    pub template_name: Option<String>,
}

impl client::RequestValue for Deployment {}
impl client::Resource for Deployment {}
impl client::ResponseResult for Deployment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list deployments](DeploymentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentsListResponse {
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<Deployment>>,
}

impl client::ResponseResult for DeploymentsListResponse {}


/// How to attach a disk to a Replica.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskAttachment {
    /// The device name of this disk.
    #[serde(rename="deviceName")]
    
    pub device_name: Option<String>,
    /// A zero-based index to assign to this disk, where 0 is reserved for the boot disk. If not specified, this is assigned by the server.
    
    pub index: Option<u32>,
}

impl client::Part for DiskAttachment {}


/// An environment variable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvVariable {
    /// Whether this variable is hidden or visible.
    
    pub hidden: Option<bool>,
    /// Value of the environment variable.
    
    pub value: Option<String>,
}

impl client::Part for EnvVariable {}


/// A pre-existing persistent disk that will be attached to every Replica in the Pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExistingDisk {
    /// Optional. How the disk will be attached to the Replica.
    
    pub attachment: Option<DiskAttachment>,
    /// The fully-qualified URL of the Persistent Disk resource. It must be in the same zone as the Pool.
    
    pub source: Option<String>,
}

impl client::Part for ExistingDisk {}


/// A Firewall resource
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallModule {
    /// The allowed ports or port ranges.
    
    pub allowed: Option<Vec<AllowedRule>>,
    /// The description of the firewall (optional)
    
    pub description: Option<String>,
    /// The NetworkModule to which this firewall should apply. If not specified, or if specified as 'default', this firewall will be applied to the 'default' network.
    
    pub network: Option<String>,
    /// Source IP ranges to apply this firewall to, see the GCE Spec for details on syntax
    #[serde(rename="sourceRanges")]
    
    pub source_ranges: Option<Vec<String>>,
    /// Source Tags to apply this firewall to, see the GCE Spec for details on syntax
    #[serde(rename="sourceTags")]
    
    pub source_tags: Option<Vec<String>>,
    /// Target Tags to apply this firewall to, see the GCE Spec for details on syntax
    #[serde(rename="targetTags")]
    
    pub target_tags: Option<Vec<String>>,
}

impl client::Part for FirewallModule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallModuleStatus {
    /// [Output Only] The URL of the corresponding Firewall resource.
    #[serde(rename="firewallUrl")]
    
    pub firewall_url: Option<String>,
}

impl client::Part for FirewallModuleStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HealthCheckModule {
    /// no description provided
    #[serde(rename="checkIntervalSec")]
    
    pub check_interval_sec: Option<i32>,
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="healthyThreshold")]
    
    pub healthy_threshold: Option<i32>,
    /// no description provided
    
    pub host: Option<String>,
    /// no description provided
    
    pub path: Option<String>,
    /// no description provided
    
    pub port: Option<i32>,
    /// no description provided
    #[serde(rename="timeoutSec")]
    
    pub timeout_sec: Option<i32>,
    /// no description provided
    #[serde(rename="unhealthyThreshold")]
    
    pub unhealthy_threshold: Option<i32>,
}

impl client::Part for HealthCheckModule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HealthCheckModuleStatus {
    /// [Output Only] The HealthCheck URL.
    #[serde(rename="healthCheckUrl")]
    
    pub health_check_url: Option<String>,
}

impl client::Part for HealthCheckModuleStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LbModule {
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="healthChecks")]
    
    pub health_checks: Option<Vec<String>>,
    /// no description provided
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// no description provided
    #[serde(rename="ipProtocol")]
    
    pub ip_protocol: Option<String>,
    /// no description provided
    #[serde(rename="portRange")]
    
    pub port_range: Option<String>,
    /// no description provided
    #[serde(rename="sessionAffinity")]
    
    pub session_affinity: Option<String>,
    /// no description provided
    #[serde(rename="targetModules")]
    
    pub target_modules: Option<Vec<String>>,
}

impl client::Part for LbModule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LbModuleStatus {
    /// [Output Only] The URL of the corresponding ForwardingRule in GCE.
    #[serde(rename="forwardingRuleUrl")]
    
    pub forwarding_rule_url: Option<String>,
    /// [Output Only] The URL of the corresponding TargetPool resource in GCE.
    #[serde(rename="targetPoolUrl")]
    
    pub target_pool_url: Option<String>,
}

impl client::Part for LbModuleStatus {}


/// A Compute Engine metadata entry. Identical to the metadata on the corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The fingerprint of the metadata.
    #[serde(rename="fingerPrint")]
    
    pub finger_print: Option<String>,
    /// A list of metadata items.
    
    pub items: Option<Vec<MetadataItem>>,
}

impl client::Part for Metadata {}


/// A Compute Engine metadata item, defined as a key:value pair. Identical to the metadata on the corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataItem {
    /// A metadata key.
    
    pub key: Option<String>,
    /// A metadata value.
    
    pub value: Option<String>,
}

impl client::Part for MetadataItem {}


/// A module in a configuration. A module represents a single homogeneous, possibly replicated task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    /// no description provided
    #[serde(rename="autoscalingModule")]
    
    pub autoscaling_module: Option<AutoscalingModule>,
    /// no description provided
    #[serde(rename="firewallModule")]
    
    pub firewall_module: Option<FirewallModule>,
    /// no description provided
    #[serde(rename="healthCheckModule")]
    
    pub health_check_module: Option<HealthCheckModule>,
    /// no description provided
    #[serde(rename="lbModule")]
    
    pub lb_module: Option<LbModule>,
    /// no description provided
    #[serde(rename="networkModule")]
    
    pub network_module: Option<NetworkModule>,
    /// no description provided
    #[serde(rename="replicaPoolModule")]
    
    pub replica_pool_module: Option<ReplicaPoolModule>,
    /// The type of this module. Valid values ("AUTOSCALING", "FIREWALL", "HEALTH_CHECK", "LOAD_BALANCING", "NETWORK", "REPLICA_POOL")
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Module {}


/// [Output Only] Aggregate status for a module.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModuleStatus {
    /// [Output Only] The status of the AutoscalingModule, set for type AUTOSCALING.
    #[serde(rename="autoscalingModuleStatus")]
    
    pub autoscaling_module_status: Option<AutoscalingModuleStatus>,
    /// [Output Only] The status of the FirewallModule, set for type FIREWALL.
    #[serde(rename="firewallModuleStatus")]
    
    pub firewall_module_status: Option<FirewallModuleStatus>,
    /// [Output Only] The status of the HealthCheckModule, set for type HEALTH_CHECK.
    #[serde(rename="healthCheckModuleStatus")]
    
    pub health_check_module_status: Option<HealthCheckModuleStatus>,
    /// [Output Only] The status of the LbModule, set for type LOAD_BALANCING.
    #[serde(rename="lbModuleStatus")]
    
    pub lb_module_status: Option<LbModuleStatus>,
    /// [Output Only] The status of the NetworkModule, set for type NETWORK.
    #[serde(rename="networkModuleStatus")]
    
    pub network_module_status: Option<NetworkModuleStatus>,
    /// [Output Only] The status of the ReplicaPoolModule, set for type VM.
    #[serde(rename="replicaPoolModuleStatus")]
    
    pub replica_pool_module_status: Option<ReplicaPoolModuleStatus>,
    /// [Output Only] The current state of the module.
    
    pub state: Option<DeployState>,
    /// [Output Only] The type of the module.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ModuleStatus {}


/// A Compute Engine NetworkInterface resource. Identical to the NetworkInterface on the corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// An array of configurations for this interface. This specifies how this interface is configured to interact with other network services
    #[serde(rename="accessConfigs")]
    
    pub access_configs: Option<Vec<AccessConfig>>,
    /// Name of the interface.
    
    pub name: Option<String>,
    /// The name of the NetworkModule to which this interface applies. If not specified, or specified as 'default', this will use the 'default' network.
    
    pub network: Option<String>,
    /// An optional IPV4 internal network address to assign to the instance for this network interface.
    #[serde(rename="networkIp")]
    
    pub network_ip: Option<String>,
}

impl client::Part for NetworkInterface {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkModule {
    /// Required; The range of internal addresses that are legal on this network. This range is a CIDR specification, for example: 192.168.0.0/16.
    #[serde(rename="IPv4Range")]
    
    pub i_pv4_range: Option<String>,
    /// The description of the network.
    
    pub description: Option<String>,
    /// An optional address that is used for default routing to other networks. This must be within the range specified by IPv4Range, and is typicall the first usable address in that range. If not specified, the default value is the first usable address in IPv4Range.
    #[serde(rename="gatewayIPv4")]
    
    pub gateway_i_pv4: Option<String>,
}

impl client::Part for NetworkModule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkModuleStatus {
    /// [Output Only] The URL of the corresponding Network resource.
    #[serde(rename="networkUrl")]
    
    pub network_url: Option<String>,
}

impl client::Part for NetworkModuleStatus {}


/// A Persistent Disk resource that will be created and attached to each Replica in the Pool. Each Replica will have a unique persistent disk that is created and attached to that Replica.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewDisk {
    /// How the disk will be attached to the Replica.
    
    pub attachment: Option<DiskAttachment>,
    /// If true, then this disk will be deleted when the instance is deleted.
    #[serde(rename="autoDelete")]
    
    pub auto_delete: Option<bool>,
    /// If true, indicates that this is the root persistent disk.
    
    pub boot: Option<bool>,
    /// Create the new disk using these parameters. The name of the disk will be <instance_name>-<five_random_charactersgt;.
    #[serde(rename="initializeParams")]
    
    pub initialize_params: Option<NewDiskInitializeParams>,
}

impl client::Part for NewDisk {}


/// Initialization parameters for creating a new disk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewDiskInitializeParams {
    /// The size of the created disk in gigabytes.
    #[serde(rename="diskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disk_size_gb: Option<i64>,
    /// Name of the disk type resource describing which disk type to use to create the disk. For example 'pd-ssd' or 'pd-standard'. Default is 'pd-standard'
    #[serde(rename="diskType")]
    
    pub disk_type: Option<String>,
    /// The fully-qualified URL of a source image to use to create this disk.
    #[serde(rename="sourceImage")]
    
    pub source_image: Option<String>,
}

impl client::Part for NewDiskInitializeParams {}


/// A specification for overriding parameters in a Template that corresponds to the Deployment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParamOverride {
    /// A JSON Path expression that specifies which parameter should be overridden.
    
    pub path: Option<String>,
    /// The new value to assign to the overridden parameter.
    
    pub value: Option<String>,
}

impl client::Part for ParamOverride {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolModule {
    /// A list of environment variables.
    #[serde(rename="envVariables")]
    
    pub env_variables: Option<HashMap<String, EnvVariable>>,
    /// The Health Checks to configure for the ReplicaPoolModule
    #[serde(rename="healthChecks")]
    
    pub health_checks: Option<Vec<String>>,
    /// Number of replicas in this module.
    #[serde(rename="numReplicas")]
    
    pub num_replicas: Option<i32>,
    /// Information for a ReplicaPoolModule.
    #[serde(rename="replicaPoolParams")]
    
    pub replica_pool_params: Option<ReplicaPoolParams>,
    /// [Output Only] The name of the Resource View associated with a ReplicaPoolModule. This field will be generated by the service.
    #[serde(rename="resourceView")]
    
    pub resource_view: Option<String>,
}

impl client::Part for ReplicaPoolModule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolModuleStatus {
    /// [Output Only] The URL of the associated ReplicaPool resource.
    #[serde(rename="replicaPoolUrl")]
    
    pub replica_pool_url: Option<String>,
    /// [Output Only] The URL of the Resource Group associated with this ReplicaPool.
    #[serde(rename="resourceViewUrl")]
    
    pub resource_view_url: Option<String>,
}

impl client::Part for ReplicaPoolModuleStatus {}


/// Configuration information for a ReplicaPools resource. Specifying an item within will determine the ReplicaPools API version used for a ReplicaPoolModule. Only one may be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolParams {
    /// ReplicaPoolParams specifications for use with ReplicaPools v1beta1.
    
    pub v1beta1: Option<ReplicaPoolParamsV1Beta1>,
}

impl client::Part for ReplicaPoolParams {}


/// Configuration information for a ReplicaPools v1beta1 API resource. Directly maps to ReplicaPool InitTemplate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolParamsV1Beta1 {
    /// Whether these replicas should be restarted if they experience a failure. The default value is true.
    #[serde(rename="autoRestart")]
    
    pub auto_restart: Option<bool>,
    /// The base name for instances within this ReplicaPool.
    #[serde(rename="baseInstanceName")]
    
    pub base_instance_name: Option<String>,
    /// Enables IP Forwarding
    #[serde(rename="canIpForward")]
    
    pub can_ip_forward: Option<bool>,
    /// An optional textual description of the resource.
    
    pub description: Option<String>,
    /// A list of existing Persistent Disk resources to attach to each replica in the pool. Each disk will be attached in read-only mode to every replica.
    #[serde(rename="disksToAttach")]
    
    pub disks_to_attach: Option<Vec<ExistingDisk>>,
    /// A list of Disk resources to create and attach to each Replica in the Pool. Currently, you can only define one disk and it must be a root persistent disk. Note that Replica Pool will create a root persistent disk for each replica.
    #[serde(rename="disksToCreate")]
    
    pub disks_to_create: Option<Vec<NewDisk>>,
    /// Name of the Action to be run during initialization of a ReplicaPoolModule.
    #[serde(rename="initAction")]
    
    pub init_action: Option<String>,
    /// The machine type for this instance. Either a complete URL, or the resource name (e.g. n1-standard-1).
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// The metadata key/value pairs assigned to this instance.
    
    pub metadata: Option<Metadata>,
    /// A list of network interfaces for the instance. Currently only one interface is supported by Google Compute Engine.
    #[serde(rename="networkInterfaces")]
    
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// no description provided
    #[serde(rename="onHostMaintenance")]
    
    pub on_host_maintenance: Option<String>,
    /// A list of Service Accounts to enable for this instance.
    #[serde(rename="serviceAccounts")]
    
    pub service_accounts: Option<Vec<ServiceAccount>>,
    /// A list of tags to apply to the Google Compute Engine instance to identify resources.
    
    pub tags: Option<Tag>,
    /// The zone for this ReplicaPool.
    
    pub zone: Option<String>,
}

impl client::Part for ReplicaPoolParamsV1Beta1 {}


/// A Compute Engine service account, identical to the Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// Service account email address.
    
    pub email: Option<String>,
    /// List of OAuth2 scopes to obtain for the service account.
    
    pub scopes: Option<Vec<String>>,
}

impl client::Part for ServiceAccount {}


/// A Compute Engine Instance tag, identical to the tags on the corresponding Compute Engine Instance resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    /// The fingerprint of the tag.
    #[serde(rename="fingerPrint")]
    
    pub finger_print: Option<String>,
    /// Items contained in this tag.
    
    pub items: Option<Vec<String>>,
}

impl client::Part for Tag {}


/// A Template represents a complete configuration for a Deployment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete templates](TemplateDeleteCall) (none)
/// * [get templates](TemplateGetCall) (response)
/// * [insert templates](TemplateInsertCall) (request|response)
/// * [list templates](TemplateListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    /// Action definitions for use in Module intents in this Template.
    
    pub actions: Option<HashMap<String, Action>>,
    /// A user-supplied description of this Template.
    
    pub description: Option<String>,
    /// A list of modules for this Template.
    
    pub modules: Option<HashMap<String, Module>>,
    /// Name of this Template. The name must conform to the expression: [a-zA-Z0-9-_]{1,64}
    
    pub name: Option<String>,
}

impl client::RequestValue for Template {}
impl client::Resource for Template {}
impl client::ResponseResult for Template {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list templates](TemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TemplatesListResponse {
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<Template>>,
}

impl client::ResponseResult for TemplatesListResponse {}


