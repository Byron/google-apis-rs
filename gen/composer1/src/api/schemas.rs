use super::*;
/// Allowed IP range with user-provided description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllowedIpRange {
    /// Optional. User-provided description. It must contain at most 300 characters.
    
    pub description: Option<String>,
    /// IP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: `192.168.1.1` or `192.168.0.0/16` or `2001:db8::/32` or `2001:0db8:0000:0042:0000:8a2e:0370:7334`. IP range prefixes should be properly truncated. For example, `1.2.3.4/24` should be truncated to `1.2.3.0/24`. Similarly, for IPv6, `2001:db8::1/32` should be truncated to `2001:db8::/32`.
    
    pub value: Option<String>,
}

impl client::Part for AllowedIpRange {}


/// CIDR block with an optional name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CidrBlock {
    /// CIDR block that must be specified in CIDR notation.
    #[serde(rename="cidrBlock")]
    
    pub cidr_block: Option<String>,
    /// User-defined name that identifies the CIDR block.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for CidrBlock {}


/// The configuration of Cloud SQL instance that is used by the Apache Airflow software.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Optional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used. Supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
}

impl client::Part for DatabaseConfig {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The encryption options for the Cloud Composer environment and its dependencies.Supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Optional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated. If not specified, Google-managed key will be used.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// An environment for running orchestration tasks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments create projects](ProjectLocationEnvironmentCreateCall) (request)
/// * [locations environments get projects](ProjectLocationEnvironmentGetCall) (response)
/// * [locations environments patch projects](ProjectLocationEnvironmentPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// Configuration parameters for this environment.
    
    pub config: Option<EnvironmentConfig>,
    /// Output only. The time at which this environment was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62} * Values must conform to regexp: [\p{Ll}\p{Lo}\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size.
    
    pub labels: Option<HashMap<String, String>>,
    /// The resource name of the environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}" EnvironmentId must start with a lowercase letter followed by up to 63 lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    
    pub name: Option<String>,
    /// The current state of the environment.
    
    pub state: Option<EnvironmentStateEnum>,
    /// Output only. The time at which this environment was last modified.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The UUID (Universally Unique IDentifier) associated with this environment. This value is generated when the environment is created.
    
    pub uuid: Option<String>,
}

impl client::RequestValue for Environment {}
impl client::ResponseResult for Environment {}


/// Configuration information for an environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Output only. The URI of the Apache Airflow Web UI hosted within this environment (see [Airflow web interface](https://cloud.google.com/composer/docs/how-to/accessing/airflow-web-interface)).
    #[serde(rename="airflowUri")]
    
    pub airflow_uri: Option<String>,
    /// Output only. The Cloud Storage prefix of the DAGs for this environment. Although Cloud Storage objects reside in a flat namespace, a hierarchical file tree can be simulated using "/"-delimited object name prefixes. DAG objects for this environment reside in a simulated directory with the given prefix.
    #[serde(rename="dagGcsPrefix")]
    
    pub dag_gcs_prefix: Option<String>,
    /// Optional. The configuration settings for Cloud SQL instance used internally by Apache Airflow software.
    #[serde(rename="databaseConfig")]
    
    pub database_config: Option<DatabaseConfig>,
    /// Optional. The encryption options for the Cloud Composer environment and its dependencies. Cannot be updated.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Optional. The size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[serde(rename="environmentSize")]
    
    pub environment_size: Option<EnvironmentConfigEnvironmentSizeEnum>,
    /// Output only. The Kubernetes Engine cluster used to run this environment.
    #[serde(rename="gkeCluster")]
    
    pub gke_cluster: Option<String>,
    /// Optional. The maintenance window is the period when Cloud Composer components may undergo maintenance. It is defined so that maintenance is not executed during peak hours or critical time periods. The system will not be under maintenance for every occurrence of this window, but when maintenance is planned, it will be scheduled during the window. The maintenance window period must encompass at least 12 hours per week. This may be split into multiple chunks, each with a size of at least 4 hours. If this value is omitted, the default value for maintenance window will be applied. The default value is Saturday and Sunday 00-06 GMT.
    #[serde(rename="maintenanceWindow")]
    
    pub maintenance_window: Option<MaintenanceWindow>,
    /// Optional. The configuration options for GKE cluster master authorized networks. By default master authorized networks feature is: - in case of private environment: enabled with no external networks allowlisted. - in case of public environment: disabled.
    #[serde(rename="masterAuthorizedNetworksConfig")]
    
    pub master_authorized_networks_config: Option<MasterAuthorizedNetworksConfig>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[serde(rename="nodeConfig")]
    
    pub node_config: Option<NodeConfig>,
    /// The number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[serde(rename="nodeCount")]
    
    pub node_count: Option<i32>,
    /// The configuration used for the Private IP Cloud Composer environment.
    #[serde(rename="privateEnvironmentConfig")]
    
    pub private_environment_config: Option<PrivateEnvironmentConfig>,
    /// Optional. The Recovery settings configuration of an environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[serde(rename="recoveryConfig")]
    
    pub recovery_config: Option<RecoveryConfig>,
    /// The configuration settings for software inside the environment.
    #[serde(rename="softwareConfig")]
    
    pub software_config: Option<SoftwareConfig>,
    /// Optional. The configuration settings for the Airflow web server App Engine instance.
    #[serde(rename="webServerConfig")]
    
    pub web_server_config: Option<WebServerConfig>,
    /// Optional. The network-level access control policy for the Airflow web server. If unspecified, no network-level access restrictions will be applied.
    #[serde(rename="webServerNetworkAccessControl")]
    
    pub web_server_network_access_control: Option<WebServerNetworkAccessControl>,
    /// Optional. The workloads configuration settings for the GKE cluster associated with the Cloud Composer environment. The GKE cluster runs Airflow scheduler, web server and workers workloads. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[serde(rename="workloadsConfig")]
    
    pub workloads_config: Option<WorkloadsConfig>,
}

impl client::Part for EnvironmentConfig {}


/// Configuration for controlling how IPs are allocated in the GKE cluster running the Apache Airflow software.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IPAllocationPolicy {
    /// Optional. The IP address range used to allocate IP addresses to pods in the GKE cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when `use_ip_aliases` is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific netmask. Set to a [CIDR](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use.
    #[serde(rename="clusterIpv4CidrBlock")]
    
    pub cluster_ipv4_cidr_block: Option<String>,
    /// Optional. The name of the GKE cluster's secondary range used to allocate IP addresses to pods. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when `use_ip_aliases` is true.
    #[serde(rename="clusterSecondaryRangeName")]
    
    pub cluster_secondary_range_name: Option<String>,
    /// Optional. The IP address range of the services IP addresses in this GKE cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when `use_ip_aliases` is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific netmask. Set to a [CIDR](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use.
    #[serde(rename="servicesIpv4CidrBlock")]
    
    pub services_ipv4_cidr_block: Option<String>,
    /// Optional. The name of the services' secondary range used to allocate IP addresses to the GKE cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when `use_ip_aliases` is true.
    #[serde(rename="servicesSecondaryRangeName")]
    
    pub services_secondary_range_name: Option<String>,
    /// Optional. Whether or not to enable Alias IPs in the GKE cluster. If `true`, a VPC-native cluster is created. This field is only supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use VPC-native GKE clusters.
    #[serde(rename="useIpAliases")]
    
    pub use_ip_aliases: Option<bool>,
}

impl client::Part for IPAllocationPolicy {}


/// ImageVersion information
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageVersion {
    /// Whether it is impossible to create an environment with the image version.
    #[serde(rename="creationDisabled")]
    
    pub creation_disabled: Option<bool>,
    /// The string identifier of the ImageVersion, in the form: "composer-x.y.z-airflow-a.b.c"
    #[serde(rename="imageVersionId")]
    
    pub image_version_id: Option<String>,
    /// Whether this is the default ImageVersion used by Composer during environment creation if no input ImageVersion is specified.
    #[serde(rename="isDefault")]
    
    pub is_default: Option<bool>,
    /// The date of the version release.
    #[serde(rename="releaseDate")]
    
    pub release_date: Option<Date>,
    /// supported python versions
    #[serde(rename="supportedPythonVersions")]
    
    pub supported_python_versions: Option<Vec<String>>,
    /// Whether it is impossible to upgrade an environment running with the image version.
    #[serde(rename="upgradeDisabled")]
    
    pub upgrade_disabled: Option<bool>,
}

impl client::Part for ImageVersion {}


/// The environments in a project and location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments list projects](ProjectLocationEnvironmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEnvironmentsResponse {
    /// The list of environments returned by a ListEnvironmentsRequest.
    
    pub environments: Option<Vec<Environment>>,
    /// The page token used to query for the next page if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEnvironmentsResponse {}


/// The ImageVersions in a project and location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations image versions list projects](ProjectLocationImageVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListImageVersionsResponse {
    /// The list of supported ImageVersions in a location.
    #[serde(rename="imageVersions")]
    
    pub image_versions: Option<Vec<ImageVersion>>,
    /// The page token used to query for the next page if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListImageVersionsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Request to load a snapshot into a Cloud Composer environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments load snapshot projects](ProjectLocationEnvironmentLoadSnapshotCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoadSnapshotRequest {
    /// Whether or not to skip setting Airflow overrides when loading the environment's state.
    #[serde(rename="skipAirflowOverridesSetting")]
    
    pub skip_airflow_overrides_setting: Option<bool>,
    /// Whether or not to skip setting environment variables when loading the environment's state.
    #[serde(rename="skipEnvironmentVariablesSetting")]
    
    pub skip_environment_variables_setting: Option<bool>,
    /// Whether or not to skip copying Cloud Storage data when loading the environment's state.
    #[serde(rename="skipGcsDataCopying")]
    
    pub skip_gcs_data_copying: Option<bool>,
    /// Whether or not to skip installing Pypi packages when loading the environment's state.
    #[serde(rename="skipPypiPackagesInstallation")]
    
    pub skip_pypi_packages_installation: Option<bool>,
    /// A Cloud Storage path to a snapshot to load, e.g.: "gs://my-bucket/snapshots/project_location_environment_timestamp".
    #[serde(rename="snapshotPath")]
    
    pub snapshot_path: Option<String>,
}

impl client::RequestValue for LoadSnapshotRequest {}


/// The configuration settings for Cloud Composer maintenance window. The following example: ``` { "startTime":"2019-08-01T01:00:00Z" "endTime":"2019-08-01T07:00:00Z" "recurrence":"FREQ=WEEKLY;BYDAY=TU,WE" } ``` would define a maintenance window between 01 and 07 hours UTC during each Tuesday and Wednesday.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// Required. Maintenance window end time. It is used only to calculate the duration of the maintenance window. The value for end-time must be in the future, relative to `start_time`.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Maintenance window recurrence. Format is a subset of [RFC-5545](https://tools.ietf.org/html/rfc5545) `RRULE`. The only allowed values for `FREQ` field are `FREQ=DAILY` and `FREQ=WEEKLY;BYDAY=...` Example values: `FREQ=WEEKLY;BYDAY=TU,WE`, `FREQ=DAILY`.
    
    pub recurrence: Option<String>,
    /// Required. Start time of the first recurrence of the maintenance window.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for MaintenanceWindow {}


/// Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MasterAuthorizedNetworksConfig {
    /// Up to 50 external networks that could access Kubernetes master through HTTPS.
    #[serde(rename="cidrBlocks")]
    
    pub cidr_blocks: Option<Vec<CidrBlock>>,
    /// Whether or not master authorized networks feature is enabled.
    
    pub enabled: Option<bool>,
}

impl client::Part for MasterAuthorizedNetworksConfig {}


/// Configuration options for networking connections in the Composer 2 environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkingConfig {
    /// Optional. Indicates the user requested specifc connection type between Tenant and Customer projects. You cannot set networking connection type in public IP environment.
    #[serde(rename="connectionType")]
    
    pub connection_type: Option<NetworkingConfigConnectionTypeEnum>,
}

impl client::Part for NetworkingConfig {}


/// The configuration information for the Kubernetes Engine nodes running the Apache Airflow software.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Optional. The disk size in GB used for node VMs. Minimum size is 30GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[serde(rename="diskSizeGb")]
    
    pub disk_size_gb: Option<i32>,
    /// Optional. Deploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent
    #[serde(rename="enableIpMasqAgent")]
    
    pub enable_ip_masq_agent: Option<bool>,
    /// Optional. The configuration for controlling how IPs are allocated in the GKE cluster.
    #[serde(rename="ipAllocationPolicy")]
    
    pub ip_allocation_policy: Option<IPAllocationPolicy>,
    /// Optional. The Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones) in which to deploy the VMs used to run the Apache Airflow software, specified as a [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name). For example: “projects/{projectId}/zones/{zoneId}”. This `location` must belong to the enclosing environment’s project and location. If both this field and `nodeConfig.machineType` are specified, `nodeConfig.machineType` must belong to this `location`; if both are unspecified, the service will pick a zone in the Compute Engine region corresponding to the Cloud Composer location, and propagate that choice to both fields. If only one field (`location` or `nodeConfig.machineType`) is specified, the location information from the specified field will be propagated to the unspecified field. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.\*.
    
    pub location: Option<String>,
    /// Optional. The Compute Engine [machine type](https://cloud.google.com/compute/docs/machine-types) used for cluster instances, specified as a [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name). For example: “projects/{projectId}/zones/{zoneId}/machineTypes/{machineTypeId}”. The `machineType` must belong to the enclosing environment’s project and location. If both this field and `nodeConfig.location` are specified, this `machineType` must belong to the `nodeConfig.location`; if both are unspecified, the service will pick a zone in the Compute Engine region corresponding to the Cloud Composer location, and propagate that choice to both fields. If exactly one of this field and `nodeConfig.location` is specified, the location information from the specified field will be propagated to the unspecified field. The `machineTypeId` must not be a [shared-core machine type](https://cloud.google.com/compute/docs/machine-types#sharedcore). If this field is unspecified, the `machineTypeId` defaults to “n1-standard-1”. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.\*.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// Optional. The Compute Engine network to be used for machine communications, specified as a [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name). For example: “projects/{projectId}/global/networks/{networkId}”. If unspecified, the “default” network ID in the environment’s project is used. If a [Custom Subnet Network](https://cloud.google.com/vpc/docs/vpc#vpc_networks_and_subnets) is provided, `nodeConfig.subnetwork` must also be provided. For [Shared VPC](https://cloud.google.com/vpc/docs/shared-vpc) subnetwork requirements, see `nodeConfig.subnetwork`.
    
    pub network: Option<String>,
    /// Optional. The set of Google API scopes to be made available on all node VMs. If `oauth_scopes` is empty, defaults to ["https://www.googleapis.com/auth/cloud-platform"]. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[serde(rename="oauthScopes")]
    
    pub oauth_scopes: Option<Vec<String>>,
    /// Optional. The Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the "default" Compute Engine service account is used. Cannot be updated.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Optional. The Compute Engine subnetwork to be used for machine communications, specified as a [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name). For example: “projects/{projectId}/regions/{regionId}/subnetworks/{subnetworkId}” If a subnetwork is provided, `nodeConfig.network` must also be provided, and the subnetwork must belong to the enclosing environment’s project and location.
    
    pub subnetwork: Option<String>,
    /// Optional. The list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    
    pub tags: Option<Vec<String>>,
}

impl client::Part for NodeConfig {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments create projects](ProjectLocationEnvironmentCreateCall) (response)
/// * [locations environments delete projects](ProjectLocationEnvironmentDeleteCall) (response)
/// * [locations environments load snapshot projects](ProjectLocationEnvironmentLoadSnapshotCall) (response)
/// * [locations environments patch projects](ProjectLocationEnvironmentPatchCall) (response)
/// * [locations environments save snapshot projects](ProjectLocationEnvironmentSaveSnapshotCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Configuration options for the private GKE cluster in a Cloud Composer environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateClusterConfig {
    /// Optional. If `true`, access to the public endpoint of the GKE cluster is denied.
    #[serde(rename="enablePrivateEndpoint")]
    
    pub enable_private_endpoint: Option<bool>,
    /// Optional. The CIDR block from which IPv4 range for GKE master will be reserved. If left blank, the default value of '172.16.0.0/23' is used.
    #[serde(rename="masterIpv4CidrBlock")]
    
    pub master_ipv4_cidr_block: Option<String>,
    /// Output only. The IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the GKE cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network.
    #[serde(rename="masterIpv4ReservedRange")]
    
    pub master_ipv4_reserved_range: Option<String>,
}

impl client::Part for PrivateClusterConfig {}


/// The configuration information for configuring a Private IP Cloud Composer environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateEnvironmentConfig {
    /// Optional. When specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork.
    #[serde(rename="cloudComposerConnectionSubnetwork")]
    
    pub cloud_composer_connection_subnetwork: Option<String>,
    /// Optional. The CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[serde(rename="cloudComposerNetworkIpv4CidrBlock")]
    
    pub cloud_composer_network_ipv4_cidr_block: Option<String>,
    /// Output only. The IP range reserved for the tenant project's Cloud Composer network. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[serde(rename="cloudComposerNetworkIpv4ReservedRange")]
    
    pub cloud_composer_network_ipv4_reserved_range: Option<String>,
    /// Optional. The CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from `web_server_ipv4_cidr_block`.
    #[serde(rename="cloudSqlIpv4CidrBlock")]
    
    pub cloud_sql_ipv4_cidr_block: Option<String>,
    /// Optional. If `true`, a Private IP Cloud Composer environment is created. If this field is set to true, `IPAllocationPolicy.use_ip_aliases` must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[serde(rename="enablePrivateEnvironment")]
    
    pub enable_private_environment: Option<bool>,
    /// Optional. When enabled, IPs from public (non-RFC1918) ranges can be used for `IPAllocationPolicy.cluster_ipv4_cidr_block` and `IPAllocationPolicy.service_ipv4_cidr_block`.
    #[serde(rename="enablePrivatelyUsedPublicIps")]
    
    pub enable_privately_used_public_ips: Option<bool>,
    /// Optional. Configuration for the network connections configuration in the environment.
    #[serde(rename="networkingConfig")]
    
    pub networking_config: Option<NetworkingConfig>,
    /// Optional. Configuration for the private GKE cluster for a Private IP Cloud Composer environment.
    #[serde(rename="privateClusterConfig")]
    
    pub private_cluster_config: Option<PrivateClusterConfig>,
    /// Optional. The CIDR block from which IP range for web server will be reserved. Needs to be disjoint from `private_cluster_config.master_ipv4_cidr_block` and `cloud_sql_ipv4_cidr_block`. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[serde(rename="webServerIpv4CidrBlock")]
    
    pub web_server_ipv4_cidr_block: Option<String>,
    /// Output only. The IP range reserved for the tenant project's App Engine VMs. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[serde(rename="webServerIpv4ReservedRange")]
    
    pub web_server_ipv4_reserved_range: Option<String>,
}

impl client::Part for PrivateEnvironmentConfig {}


/// The Recovery settings of an environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryConfig {
    /// Optional. The configuration for scheduled snapshot creation mechanism.
    #[serde(rename="scheduledSnapshotsConfig")]
    
    pub scheduled_snapshots_config: Option<ScheduledSnapshotsConfig>,
}

impl client::Part for RecoveryConfig {}


/// Request to create a snapshot of a Cloud Composer environment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments save snapshot projects](ProjectLocationEnvironmentSaveSnapshotCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SaveSnapshotRequest {
    /// Location in a Cloud Storage where the snapshot is going to be stored, e.g.: "gs://my-bucket/snapshots".
    #[serde(rename="snapshotLocation")]
    
    pub snapshot_location: Option<String>,
}

impl client::RequestValue for SaveSnapshotRequest {}


/// The configuration for scheduled snapshot creation mechanism.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScheduledSnapshotsConfig {
    /// Optional. Whether scheduled snapshots creation is enabled.
    
    pub enabled: Option<bool>,
    /// Optional. The cron expression representing the time when snapshots creation mechanism runs. This field is subject to additional validation around frequency of execution.
    #[serde(rename="snapshotCreationSchedule")]
    
    pub snapshot_creation_schedule: Option<String>,
    /// Optional. The Cloud Storage location for storing automatically created snapshots.
    #[serde(rename="snapshotLocation")]
    
    pub snapshot_location: Option<String>,
    /// Optional. Time zone that sets the context to interpret snapshot_creation_schedule.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for ScheduledSnapshotsConfig {}


/// Configuration for resources used by Airflow schedulers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchedulerResource {
    /// Optional. The number of schedulers.
    
    pub count: Option<i32>,
    /// Optional. CPU request and limit for a single Airflow scheduler replica.
    
    pub cpu: Option<f32>,
    /// Optional. Memory (GB) request and limit for a single Airflow scheduler replica.
    #[serde(rename="memoryGb")]
    
    pub memory_gb: Option<f32>,
    /// Optional. Storage (GB) request and limit for a single Airflow scheduler replica.
    #[serde(rename="storageGb")]
    
    pub storage_gb: Option<f32>,
}

impl client::Part for SchedulerResource {}


/// Specifies the selection and configuration of software inside the environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareConfig {
    /// Optional. Apache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example “core-dags_are_paused_at_creation”. Section names must not contain hyphens (“-”), opening square brackets (“\[”), or closing square brackets (“\]”). The property name must not be empty and must not contain an equals sign (“=”) or semicolon (“;”). Section and property names must not contain a period (“.”). Apache Airflow configuration property names must be written in [snake_case](https://en.wikipedia.org/wiki/Snake_case). Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are [blocked](https://cloud.google.com/composer/docs/concepts/airflow-configurations), and cannot be overridden.
    #[serde(rename="airflowConfigOverrides")]
    
    pub airflow_config_overrides: Option<HashMap<String, String>>,
    /// Optional. Additional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression `a-zA-Z_*`. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression `AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+`), and they cannot match any of the following reserved names: * `AIRFLOW_HOME` * `C_FORCE_ROOT` * `CONTAINER_NAME` * `DAGS_FOLDER` * `GCP_PROJECT` * `GCS_BUCKET` * `GKE_CLUSTER_NAME` * `SQL_DATABASE` * `SQL_INSTANCE` * `SQL_PASSWORD` * `SQL_PROJECT` * `SQL_REGION` * `SQL_USER`
    #[serde(rename="envVariables")]
    
    pub env_variables: Option<HashMap<String, String>>,
    /// The version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression `composer-([0-9]+(\.[0-9]+\.[0-9]+(-preview\.[0-9]+)?)?|latest)-airflow-([0-9]+(\.[0-9]+(\.[0-9]+)?)?)`. When used as input, the server also checks if the provided version is supported and denies the request for an unsupported version. The Cloud Composer portion of the image version is a full [semantic version](https://semver.org), or an alias in the form of major version number or `latest`. When an alias is provided, the server replaces it with the current Cloud Composer version that satisfies the alias. The Apache Airflow portion of the image version is a full semantic version that points to one of the supported Apache Airflow versions, or an alias in the form of only major or major.minor versions specified. When an alias is provided, the server replaces it with the latest Apache Airflow version that satisfies the alias and is supported in the given Cloud Composer version. In all cases, the resolved image version is stored in the same field. See also [version list](https://cloud.google.com/composer/docs/concepts/versioning/composer-versions) and [versioning overview](https://cloud.google.com/composer/docs/concepts/versioning/composer-versioning-overview).
    #[serde(rename="imageVersion")]
    
    pub image_version: Option<String>,
    /// Optional. Custom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name such as "numpy" and values are the lowercase extras and version specifier such as "==1.12.0", "[devel,gcp_api]", or "[devel]>=1.8.2, <1.9.2". To specify a package without pinning it to a version specifier, use the empty string as the value.
    #[serde(rename="pypiPackages")]
    
    pub pypi_packages: Option<HashMap<String, String>>,
    /// Optional. The major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '3'. Cannot be updated. This field is only supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use Python major version 3.
    #[serde(rename="pythonVersion")]
    
    pub python_version: Option<String>,
    /// Optional. The number of schedulers for Airflow. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-2.*.*.
    #[serde(rename="schedulerCount")]
    
    pub scheduler_count: Option<i32>,
}

impl client::Part for SoftwareConfig {}


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


/// The configuration settings for the Airflow web server App Engine instance. Supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebServerConfig {
    /// Optional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
}

impl client::Part for WebServerConfig {}


/// Network-level access control policy for the Airflow web server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebServerNetworkAccessControl {
    /// A collection of allowed IP ranges with descriptions.
    #[serde(rename="allowedIpRanges")]
    
    pub allowed_ip_ranges: Option<Vec<AllowedIpRange>>,
}

impl client::Part for WebServerNetworkAccessControl {}


/// Configuration for resources used by Airflow web server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebServerResource {
    /// Optional. CPU request and limit for Airflow web server.
    
    pub cpu: Option<f32>,
    /// Optional. Memory (GB) request and limit for Airflow web server.
    #[serde(rename="memoryGb")]
    
    pub memory_gb: Option<f32>,
    /// Optional. Storage (GB) request and limit for Airflow web server.
    #[serde(rename="storageGb")]
    
    pub storage_gb: Option<f32>,
}

impl client::Part for WebServerResource {}


/// Configuration for resources used by Airflow workers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkerResource {
    /// Optional. CPU request and limit for a single Airflow worker replica.
    
    pub cpu: Option<f32>,
    /// Optional. Maximum number of workers for autoscaling.
    #[serde(rename="maxCount")]
    
    pub max_count: Option<i32>,
    /// Optional. Memory (GB) request and limit for a single Airflow worker replica.
    #[serde(rename="memoryGb")]
    
    pub memory_gb: Option<f32>,
    /// Optional. Minimum number of workers for autoscaling.
    #[serde(rename="minCount")]
    
    pub min_count: Option<i32>,
    /// Optional. Storage (GB) request and limit for a single Airflow worker replica.
    #[serde(rename="storageGb")]
    
    pub storage_gb: Option<f32>,
}

impl client::Part for WorkerResource {}


/// The Kubernetes workloads configuration for GKE cluster associated with the Cloud Composer environment. Supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadsConfig {
    /// Optional. Resources used by Airflow schedulers.
    
    pub scheduler: Option<SchedulerResource>,
    /// Optional. Resources used by Airflow web server.
    #[serde(rename="webServer")]
    
    pub web_server: Option<WebServerResource>,
    /// Optional. Resources used by Airflow workers.
    
    pub worker: Option<WorkerResource>,
}

impl client::Part for WorkloadsConfig {}


