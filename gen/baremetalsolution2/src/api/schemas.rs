use super::*;
/// Represents an 'access point' for the share.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllowedClient {
    /// Allow dev flag. Which controls whether to allow creation of devices.
    #[serde(rename="allowDev")]
    
    pub allow_dev: Option<bool>,
    /// Allow the setuid flag.
    #[serde(rename="allowSuid")]
    
    pub allow_suid: Option<bool>,
    /// The subnet of IP addresses permitted to access the share.
    #[serde(rename="allowedClientsCidr")]
    
    pub allowed_clients_cidr: Option<String>,
    /// Mount permissions.
    #[serde(rename="mountPermissions")]
    
    pub mount_permissions: Option<String>,
    /// The network the access point sits on.
    
    pub network: Option<String>,
    /// Output only. The path to access NFS, in format shareIP:/InstanceID InstanceID is the generated ID instead of customer provided name. example like "10.0.0.0:/g123456789-nfs001"
    #[serde(rename="nfsPath")]
    
    pub nfs_path: Option<String>,
    /// Disable root squashing, which is a feature of NFS. Root squash is a special mapping of the remote superuser (root) identity when using identity authentication.
    #[serde(rename="noRootSquash")]
    
    pub no_root_squash: Option<bool>,
    /// Output only. The IP address of the share on this network. Assigned automatically during provisioning based on the network's services_cidr.
    #[serde(rename="shareIp")]
    
    pub share_ip: Option<String>,
}

impl client::Part for AllowedClient {}


/// Message for detach specific LUN from an Instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances detach lun projects](ProjectLocationInstanceDetachLunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetachLunRequest {
    /// Required. Name of the Lun to detach.
    
    pub lun: Option<String>,
    /// If true, performs lun unmapping without instance reboot.
    #[serde(rename="skipReboot")]
    
    pub skip_reboot: Option<bool>,
}

impl client::RequestValue for DetachLunRequest {}


/// Message for disabling the interactive serial console on an instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances disable interactive serial console projects](ProjectLocationInstanceDisableInteractiveSerialConsoleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DisableInteractiveSerialConsoleRequest { _never_set: Option<bool> }

impl client::RequestValue for DisableInteractiveSerialConsoleRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ssh keys delete projects](ProjectLocationSshKeyDeleteCall) (response)
/// * [locations volumes snapshots delete projects](ProjectLocationVolumeSnapshotDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Message for enabling the interactive serial console on an instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances enable interactive serial console projects](ProjectLocationInstanceEnableInteractiveSerialConsoleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnableInteractiveSerialConsoleRequest { _never_set: Option<bool> }

impl client::RequestValue for EnableInteractiveSerialConsoleRequest {}


/// Response with all provisioning settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instance provisioning settings fetch projects](ProjectLocationInstanceProvisioningSettingFetchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchInstanceProvisioningSettingsResponse {
    /// The OS images available.
    
    pub images: Option<Vec<OSImage>>,
}

impl client::ResponseResult for FetchInstanceProvisioningSettingsResponse {}


/// Each logical interface represents a logical abstraction of the underlying physical interface (for eg. bond, nic) of the instance. Each logical interface can effectively map to multiple network-IP pairs and still be mapped to one underlying physical interface.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBaremetalsolutionV2LogicalInterface {
    /// The index of the logical interface mapping to the index of the hardware bond or nic on the chosen network template. This field is deprecated.
    #[serde(rename="interfaceIndex")]
    
    pub interface_index: Option<i32>,
    /// List of logical network interfaces within a logical interface.
    #[serde(rename="logicalNetworkInterfaces")]
    
    pub logical_network_interfaces: Option<Vec<LogicalNetworkInterface>>,
    /// Interface name. This is of syntax or and forms part of the network template name.
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudBaremetalsolutionV2LogicalInterface {}


/// Logical interface.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBaremetalsolutionV2ServerNetworkTemplateLogicalInterface {
    /// Interface name. This is not a globally unique identifier. Name is unique only inside the ServerNetworkTemplate. This is of syntax or and forms part of the network template name.
    
    pub name: Option<String>,
    /// If true, interface must have network connected.
    
    pub required: Option<bool>,
    /// Interface type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudBaremetalsolutionV2ServerNetworkTemplateLogicalInterface {}


/// A server.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (request)
/// * [locations instances get projects](ProjectLocationInstanceGetCall) (response)
/// * [locations instances patch projects](ProjectLocationInstancePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// Output only. Create a time stamp.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// True if you enable hyperthreading for the server, otherwise false. The default value is false.
    #[serde(rename="hyperthreadingEnabled")]
    
    pub hyperthreading_enabled: Option<bool>,
    /// Output only. An identifier for the `Instance`, generated by the backend.
    
    pub id: Option<String>,
    /// Output only. True if the interactive serial console feature is enabled for the instance, false otherwise. The default value is false.
    #[serde(rename="interactiveSerialConsoleEnabled")]
    
    pub interactive_serial_console_enabled: Option<bool>,
    /// Labels as key value pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// List of logical interfaces for the instance. The number of logical interfaces will be the same as number of hardware bond/nic on the chosen network template. For the non-multivlan configurations (for eg, existing servers) that use existing default network template (bondaa-bondaa), both the Instance.networks field and the Instance.logical_interfaces fields will be filled to ensure backward compatibility. For the others, only Instance.logical_interfaces will be filled.
    #[serde(rename="logicalInterfaces")]
    
    pub logical_interfaces: Option<Vec<GoogleCloudBaremetalsolutionV2LogicalInterface>>,
    /// Output only. Text field about info for logging in.
    #[serde(rename="loginInfo")]
    
    pub login_info: Option<String>,
    /// Immutable. List of LUNs associated with this server.
    
    pub luns: Option<Vec<Lun>>,
    /// Immutable. The server type. [Available server types](https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations)
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// Immutable. The resource name of this `Instance`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. Format: `projects/{project}/locations/{location}/instances/{instance}`
    
    pub name: Option<String>,
    /// Instance network template name. For eg, bondaa-bondaa, bondab-nic, etc. Generally, the template name follows the syntax of "bond" or "nic".
    #[serde(rename="networkTemplate")]
    
    pub network_template: Option<String>,
    /// Output only. List of networks associated with this server.
    
    pub networks: Option<Vec<Network>>,
    /// The OS image currently installed on the server.
    #[serde(rename="osImage")]
    
    pub os_image: Option<String>,
    /// Immutable. Pod name. Pod is an independent part of infrastructure. Instance can be connected to the assets (networks, volumes) allocated in the same pod only.
    
    pub pod: Option<String>,
    /// Output only. The state of the server.
    
    pub state: Option<String>,
    /// Output only. Update a time stamp.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Input only. List of Volumes to attach to this Instance on creation. This field won't be populated in Get/List responses.
    
    pub volumes: Option<Vec<Volume>>,
    /// The workload profile for the instance.
    #[serde(rename="workloadProfile")]
    
    pub workload_profile: Option<String>,
}

impl client::RequestValue for Instance {}
impl client::ResponseResult for Instance {}


/// Configuration parameters for a new instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceConfig {
    /// If true networks can be from different projects of the same vendor account.
    #[serde(rename="accountNetworksEnabled")]
    
    pub account_networks_enabled: Option<bool>,
    /// Client network address. Filled if InstanceConfig.multivlan_config is false.
    #[serde(rename="clientNetwork")]
    
    pub client_network: Option<NetworkAddress>,
    /// Whether the instance should be provisioned with Hyperthreading enabled.
    
    pub hyperthreading: Option<bool>,
    /// A transient unique identifier to idenfity an instance within an ProvisioningConfig request.
    
    pub id: Option<String>,
    /// Instance type. [Available types](https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations)
    #[serde(rename="instanceType")]
    
    pub instance_type: Option<String>,
    /// List of logical interfaces for the instance. The number of logical interfaces will be the same as number of hardware bond/nic on the chosen network template. Filled if InstanceConfig.multivlan_config is true.
    #[serde(rename="logicalInterfaces")]
    
    pub logical_interfaces: Option<Vec<GoogleCloudBaremetalsolutionV2LogicalInterface>>,
    /// Output only. The name of the instance config.
    
    pub name: Option<String>,
    /// The type of network configuration on the instance.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<String>,
    /// Server network template name. Filled if InstanceConfig.multivlan_config is true.
    #[serde(rename="networkTemplate")]
    
    pub network_template: Option<String>,
    /// OS image to initialize the instance. [Available images](https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations)
    #[serde(rename="osImage")]
    
    pub os_image: Option<String>,
    /// Private network address, if any. Filled if InstanceConfig.multivlan_config is false.
    #[serde(rename="privateNetwork")]
    
    pub private_network: Option<NetworkAddress>,
    /// User note field, it can be used by customers to add additional information for the BMS Ops team .
    #[serde(rename="userNote")]
    
    pub user_note: Option<String>,
}

impl client::Part for InstanceConfig {}


/// A resource budget.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceQuota {
    /// Number of machines than can be created for the given location and instance_type.
    #[serde(rename="availableMachineCount")]
    
    pub available_machine_count: Option<i32>,
    /// The gcp service of the provisioning quota.
    #[serde(rename="gcpService")]
    
    pub gcp_service: Option<String>,
    /// Instance type. Deprecated: use gcp_service.
    #[serde(rename="instanceType")]
    
    pub instance_type: Option<String>,
    /// Location where the quota applies.
    
    pub location: Option<String>,
    /// Output only. The name of the instance quota.
    
    pub name: Option<String>,
}

impl client::Part for InstanceQuota {}


/// A GCP vlan attachment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntakeVlanAttachment {
    /// Identifier of the VLAN attachment.
    
    pub id: Option<String>,
    /// Attachment pairing key.
    #[serde(rename="pairingKey")]
    
    pub pairing_key: Option<String>,
}

impl client::Part for IntakeVlanAttachment {}


/// Response message for the list of servers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances list projects](ProjectLocationInstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstancesResponse {
    /// The list of servers.
    
    pub instances: Option<Vec<Instance>>,
    /// A token identifying a page of results from the server.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListInstancesResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// Response message containing the list of storage volume luns.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes luns list projects](ProjectLocationVolumeLunListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLunsResponse {
    /// The list of luns.
    
    pub luns: Option<Vec<Lun>>,
    /// A token identifying a page of results from the server.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListLunsResponse {}


/// Response with Networks with IPs
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations networks list network usage projects](ProjectLocationNetworkListNetworkUsageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNetworkUsageResponse {
    /// Networks with IPs.
    
    pub networks: Option<Vec<NetworkUsage>>,
}

impl client::ResponseResult for ListNetworkUsageResponse {}


/// Response message containing the list of networks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations networks list projects](ProjectLocationNetworkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNetworksResponse {
    /// The list of networks.
    
    pub networks: Option<Vec<Network>>,
    /// A token identifying a page of results from the server.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListNetworksResponse {}


/// Response message containing the list of NFS shares.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nfs shares list projects](ProjectLocationNfsShareListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNfsSharesResponse {
    /// A token identifying a page of results from the server.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of NFS shares.
    #[serde(rename="nfsShares")]
    
    pub nfs_shares: Option<Vec<NfsShare>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListNfsSharesResponse {}


/// Response message for the list of provisioning quotas.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations provisioning quotas list projects](ProjectLocationProvisioningQuotaListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProvisioningQuotasResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The provisioning quotas registered in this project.
    #[serde(rename="provisioningQuotas")]
    
    pub provisioning_quotas: Option<Vec<ProvisioningQuota>>,
}

impl client::ResponseResult for ListProvisioningQuotasResponse {}


/// Message for response of ListSSHKeys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ssh keys list projects](ProjectLocationSshKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSSHKeysResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The SSH keys registered in the project.
    #[serde(rename="sshKeys")]
    
    pub ssh_keys: Option<Vec<SSHKey>>,
}

impl client::ResponseResult for ListSSHKeysResponse {}


/// Response message containing the list of volume snapshots.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes snapshots list projects](ProjectLocationVolumeSnapshotListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVolumeSnapshotsResponse {
    /// A token identifying a page of results from the server.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
    /// The list of snapshots.
    #[serde(rename="volumeSnapshots")]
    
    pub volume_snapshots: Option<Vec<VolumeSnapshot>>,
}

impl client::ResponseResult for ListVolumeSnapshotsResponse {}


/// Response message containing the list of storage volumes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes list projects](ProjectLocationVolumeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVolumesResponse {
    /// A token identifying a page of results from the server.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
    /// The list of storage volumes.
    
    pub volumes: Option<Vec<Volume>>,
}

impl client::ResponseResult for ListVolumesResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Each logical network interface is effectively a network and IP pair.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogicalNetworkInterface {
    /// Whether this interface is the default gateway for the instance. Only one interface can be the default gateway for the instance.
    #[serde(rename="defaultGateway")]
    
    pub default_gateway: Option<bool>,
    /// An identifier for the `Network`, generated by the backend.
    
    pub id: Option<String>,
    /// IP address in the network
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Name of the network
    
    pub network: Option<String>,
    /// Type of network.
    #[serde(rename="networkType")]
    
    pub network_type: Option<String>,
}

impl client::Part for LogicalNetworkInterface {}


/// A storage volume logical unit number (LUN).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes luns get projects](ProjectLocationVolumeLunGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lun {
    /// Display if this LUN is a boot LUN.
    #[serde(rename="bootLun")]
    
    pub boot_lun: Option<bool>,
    /// An identifier for the LUN, generated by the backend.
    
    pub id: Option<String>,
    /// The LUN multiprotocol type ensures the characteristics of the LUN are optimized for each operating system.
    #[serde(rename="multiprotocolType")]
    
    pub multiprotocol_type: Option<String>,
    /// Output only. The name of the LUN.
    
    pub name: Option<String>,
    /// Display if this LUN can be shared between multiple physical servers.
    
    pub shareable: Option<bool>,
    /// The size of this LUN, in gigabytes.
    #[serde(rename="sizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_gb: Option<i64>,
    /// The state of this storage volume.
    
    pub state: Option<String>,
    /// The storage type for this LUN.
    #[serde(rename="storageType")]
    
    pub storage_type: Option<String>,
    /// Display the storage volume for this LUN.
    #[serde(rename="storageVolume")]
    
    pub storage_volume: Option<String>,
    /// The WWID for this LUN.
    
    pub wwid: Option<String>,
}

impl client::ResponseResult for Lun {}


/// A LUN(Logical Unit Number) range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LunRange {
    /// Number of LUNs to create.
    
    pub quantity: Option<i32>,
    /// The requested size of each LUN, in GB.
    #[serde(rename="sizeGb")]
    
    pub size_gb: Option<i32>,
}

impl client::Part for LunRange {}


/// A Network.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations networks get projects](ProjectLocationNetworkGetCall) (response)
/// * [locations networks patch projects](ProjectLocationNetworkPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    /// The cidr of the Network.
    
    pub cidr: Option<String>,
    /// Output only. Gateway ip address.
    #[serde(rename="gatewayIp")]
    
    pub gateway_ip: Option<String>,
    /// An identifier for the `Network`, generated by the backend.
    
    pub id: Option<String>,
    /// IP address configured.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Whether network uses standard frames or jumbo ones.
    #[serde(rename="jumboFramesEnabled")]
    
    pub jumbo_frames_enabled: Option<bool>,
    /// Labels as key value pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// List of physical interfaces.
    #[serde(rename="macAddress")]
    
    pub mac_address: Option<Vec<String>>,
    /// Input only. List of mount points to attach the network to.
    #[serde(rename="mountPoints")]
    
    pub mount_points: Option<Vec<NetworkMountPoint>>,
    /// Output only. The resource name of this `Network`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. Format: `projects/{project}/locations/{location}/networks/{network}`
    
    pub name: Option<String>,
    /// Output only. Pod name.
    
    pub pod: Option<String>,
    /// List of IP address reservations in this network. When updating this field, an error will be generated if a reservation conflicts with an IP address already allocated to a physical server.
    
    pub reservations: Option<Vec<NetworkAddressReservation>>,
    /// IP range for reserved for services (e.g. NFS).
    #[serde(rename="servicesCidr")]
    
    pub services_cidr: Option<String>,
    /// The Network state.
    
    pub state: Option<String>,
    /// The type of this network.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The vlan id of the Network.
    #[serde(rename="vlanId")]
    
    pub vlan_id: Option<String>,
    /// The vrf for the Network.
    
    pub vrf: Option<VRF>,
}

impl client::RequestValue for Network {}
impl client::ResponseResult for Network {}


/// A network.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkAddress {
    /// IPv4 address to be assigned to the server.
    
    pub address: Option<String>,
    /// Name of the existing network to use.
    #[serde(rename="existingNetworkId")]
    
    pub existing_network_id: Option<String>,
    /// Id of the network to use, within the same ProvisioningConfig request.
    #[serde(rename="networkId")]
    
    pub network_id: Option<String>,
}

impl client::Part for NetworkAddress {}


/// A reservation of one or more addresses in a network.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkAddressReservation {
    /// The last address of this reservation block, inclusive. I.e., for cases when reservations are only single addresses, end_address and start_address will be the same. Must be specified as a single IPv4 address, e.g. 10.1.2.2.
    #[serde(rename="endAddress")]
    
    pub end_address: Option<String>,
    /// A note about this reservation, intended for human consumption.
    
    pub note: Option<String>,
    /// The first address of this reservation block. Must be specified as a single IPv4 address, e.g. 10.1.2.2.
    #[serde(rename="startAddress")]
    
    pub start_address: Option<String>,
}

impl client::Part for NetworkAddressReservation {}


/// Configuration parameters for a new network.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Interconnect bandwidth. Set only when type is CLIENT.
    
    pub bandwidth: Option<String>,
    /// CIDR range of the network.
    
    pub cidr: Option<String>,
    /// The GCP service of the network. Available gcp_service are in https://cloud.google.com/bare-metal/docs/bms-planning.
    #[serde(rename="gcpService")]
    
    pub gcp_service: Option<String>,
    /// A transient unique identifier to identify a volume within an ProvisioningConfig request.
    
    pub id: Option<String>,
    /// The JumboFramesEnabled option for customer to set.
    #[serde(rename="jumboFramesEnabled")]
    
    pub jumbo_frames_enabled: Option<bool>,
    /// Output only. The name of the network config.
    
    pub name: Option<String>,
    /// Service CIDR, if any.
    #[serde(rename="serviceCidr")]
    
    pub service_cidr: Option<String>,
    /// The type of this network, either Client or Private.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// User note field, it can be used by customers to add additional information for the BMS Ops team .
    #[serde(rename="userNote")]
    
    pub user_note: Option<String>,
    /// List of VLAN attachments. As of now there are always 2 attachments, but it is going to change in the future (multi vlan).
    #[serde(rename="vlanAttachments")]
    
    pub vlan_attachments: Option<Vec<IntakeVlanAttachment>>,
    /// Whether the VLAN attachment pair is located in the same project.
    #[serde(rename="vlanSameProject")]
    
    pub vlan_same_project: Option<bool>,
}

impl client::Part for NetworkConfig {}


/// Mount point for a network.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkMountPoint {
    /// Network should be a default gateway.
    #[serde(rename="defaultGateway")]
    
    pub default_gateway: Option<bool>,
    /// Instance to attach network to.
    
    pub instance: Option<String>,
    /// Ip address of the server.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Logical interface to detach from.
    #[serde(rename="logicalInterface")]
    
    pub logical_interface: Option<String>,
}

impl client::Part for NetworkMountPoint {}


/// Network with all used IP addresses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkUsage {
    /// Network.
    
    pub network: Option<Network>,
    /// All used IP addresses in this network.
    #[serde(rename="usedIps")]
    
    pub used_ips: Option<Vec<String>>,
}

impl client::Part for NetworkUsage {}


/// A NFS export entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NfsExport {
    /// Allow dev flag in NfsShare AllowedClientsRequest.
    #[serde(rename="allowDev")]
    
    pub allow_dev: Option<bool>,
    /// Allow the setuid flag.
    #[serde(rename="allowSuid")]
    
    pub allow_suid: Option<bool>,
    /// A CIDR range.
    
    pub cidr: Option<String>,
    /// Either a single machine, identified by an ID, or a comma-separated list of machine IDs.
    #[serde(rename="machineId")]
    
    pub machine_id: Option<String>,
    /// Network to use to publish the export.
    #[serde(rename="networkId")]
    
    pub network_id: Option<String>,
    /// Disable root squashing, which is a feature of NFS. Root squash is a special mapping of the remote superuser (root) identity when using identity authentication.
    #[serde(rename="noRootSquash")]
    
    pub no_root_squash: Option<bool>,
    /// Export permissions.
    
    pub permissions: Option<String>,
}

impl client::Part for NfsExport {}


/// An NFS share.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nfs shares create projects](ProjectLocationNfsShareCreateCall) (request)
/// * [locations nfs shares get projects](ProjectLocationNfsShareGetCall) (response)
/// * [locations nfs shares patch projects](ProjectLocationNfsSharePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NfsShare {
    /// List of allowed access points.
    #[serde(rename="allowedClients")]
    
    pub allowed_clients: Option<Vec<AllowedClient>>,
    /// Output only. An identifier for the NFS share, generated by the backend. This is the same value as nfs_share_id and will replace it in the future.
    
    pub id: Option<String>,
    /// Labels as key value pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of the NFS share.
    
    pub name: Option<String>,
    /// Output only. An identifier for the NFS share, generated by the backend. This field will be deprecated in the future, use `id` instead.
    #[serde(rename="nfsShareId")]
    
    pub nfs_share_id: Option<String>,
    /// The requested size, in GiB.
    #[serde(rename="requestedSizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub requested_size_gib: Option<i64>,
    /// Output only. The state of the NFS share.
    
    pub state: Option<String>,
    /// Immutable. The storage type of the underlying volume.
    #[serde(rename="storageType")]
    
    pub storage_type: Option<String>,
    /// Output only. The underlying volume of the share. Created automatically during provisioning.
    
    pub volume: Option<String>,
}

impl client::RequestValue for NfsShare {}
impl client::ResponseResult for NfsShare {}


/// Operation System image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OSImage {
    /// Instance types this image is applicable to. [Available types](https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations)
    #[serde(rename="applicableInstanceTypes")]
    
    pub applicable_instance_types: Option<Vec<String>>,
    /// OS Image code.
    
    pub code: Option<String>,
    /// OS Image description.
    
    pub description: Option<String>,
    /// Output only. OS Image's unique name.
    
    pub name: Option<String>,
    /// Network templates that can be used with this OS Image.
    #[serde(rename="supportedNetworkTemplates")]
    
    pub supported_network_templates: Option<Vec<ServerNetworkTemplate>>,
}

impl client::Part for OSImage {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (response)
/// * [locations instances detach lun projects](ProjectLocationInstanceDetachLunCall) (response)
/// * [locations instances disable interactive serial console projects](ProjectLocationInstanceDisableInteractiveSerialConsoleCall) (response)
/// * [locations instances enable interactive serial console projects](ProjectLocationInstanceEnableInteractiveSerialConsoleCall) (response)
/// * [locations instances patch projects](ProjectLocationInstancePatchCall) (response)
/// * [locations instances reset projects](ProjectLocationInstanceResetCall) (response)
/// * [locations instances start projects](ProjectLocationInstanceStartCall) (response)
/// * [locations instances stop projects](ProjectLocationInstanceStopCall) (response)
/// * [locations networks patch projects](ProjectLocationNetworkPatchCall) (response)
/// * [locations nfs shares create projects](ProjectLocationNfsShareCreateCall) (response)
/// * [locations nfs shares delete projects](ProjectLocationNfsShareDeleteCall) (response)
/// * [locations nfs shares patch projects](ProjectLocationNfsSharePatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations volumes snapshots restore volume snapshot projects](ProjectLocationVolumeSnapshotRestoreVolumeSnapshotCall) (response)
/// * [locations volumes patch projects](ProjectLocationVolumePatchCall) (response)
/// * [locations volumes resize projects](ProjectLocationVolumeResizeCall) (response)
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


/// A provisioning configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations provisioning configs create projects](ProjectLocationProvisioningConfigCreateCall) (request|response)
/// * [locations provisioning configs get projects](ProjectLocationProvisioningConfigGetCall) (response)
/// * [locations provisioning configs patch projects](ProjectLocationProvisioningConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProvisioningConfig {
    /// Output only. URI to Cloud Console UI view of this provisioning config.
    #[serde(rename="cloudConsoleUri")]
    
    pub cloud_console_uri: Option<String>,
    /// Optional. The user-defined identifier of the provisioning config.
    #[serde(rename="customId")]
    
    pub custom_id: Option<String>,
    /// Email provided to send a confirmation with provisioning config to. Deprecated in favour of email field in request messages.
    
    pub email: Option<String>,
    /// A service account to enable customers to access instance credentials upon handover.
    #[serde(rename="handoverServiceAccount")]
    
    pub handover_service_account: Option<String>,
    /// Instances to be created.
    
    pub instances: Option<Vec<InstanceConfig>>,
    /// Optional. Location name of this ProvisioningConfig. It is optional only for Intake UI transition period.
    
    pub location: Option<String>,
    /// Output only. The system-generated name of the provisioning config. This follows the UUID format.
    
    pub name: Option<String>,
    /// Networks to be created.
    
    pub networks: Option<Vec<NetworkConfig>>,
    /// Output only. State of ProvisioningConfig.
    
    pub state: Option<String>,
    /// Optional status messages associated with the FAILED state.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// A generated ticket id to track provisioning request.
    #[serde(rename="ticketId")]
    
    pub ticket_id: Option<String>,
    /// Output only. Last update timestamp.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Volumes to be created.
    
    pub volumes: Option<Vec<VolumeConfig>>,
    /// If true, VPC SC is enabled for the cluster.
    #[serde(rename="vpcScEnabled")]
    
    pub vpc_sc_enabled: Option<bool>,
}

impl client::RequestValue for ProvisioningConfig {}
impl client::ResponseResult for ProvisioningConfig {}


/// A provisioning quota for a given project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProvisioningQuota {
    /// The asset type of this provisioning quota.
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// The available count of the provisioning quota.
    #[serde(rename="availableCount")]
    
    pub available_count: Option<i32>,
    /// The gcp service of the provisioning quota.
    #[serde(rename="gcpService")]
    
    pub gcp_service: Option<String>,
    /// Instance quota.
    #[serde(rename="instanceQuota")]
    
    pub instance_quota: Option<InstanceQuota>,
    /// The specific location of the provisioining quota.
    
    pub location: Option<String>,
    /// Output only. The name of the provisioning quota.
    
    pub name: Option<String>,
    /// Network bandwidth, Gbps
    #[serde(rename="networkBandwidth")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub network_bandwidth: Option<i64>,
    /// Server count.
    #[serde(rename="serverCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub server_count: Option<i64>,
    /// Storage size (GB).
    #[serde(rename="storageGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_gib: Option<i64>,
}

impl client::Part for ProvisioningQuota {}


/// QOS policy parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QosPolicy {
    /// The bandwidth permitted by the QOS policy, in gbps.
    #[serde(rename="bandwidthGbps")]
    
    pub bandwidth_gbps: Option<f64>,
}

impl client::Part for QosPolicy {}


/// Message requesting to reset a server.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances reset projects](ProjectLocationInstanceResetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResetInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for ResetInstanceRequest {}


/// Request for emergency resize Volume.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes resize projects](ProjectLocationVolumeResizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResizeVolumeRequest {
    /// New Volume size, in GiB.
    #[serde(rename="sizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_gib: Option<i64>,
}

impl client::RequestValue for ResizeVolumeRequest {}


/// Message for restoring a volume snapshot.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes snapshots restore volume snapshot projects](ProjectLocationVolumeSnapshotRestoreVolumeSnapshotCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreVolumeSnapshotRequest { _never_set: Option<bool> }

impl client::RequestValue for RestoreVolumeSnapshotRequest {}


/// An SSH key, used for authorizing with the interactive serial console feature.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations ssh keys create projects](ProjectLocationSshKeyCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SSHKey {
    /// Output only. The name of this SSH key. Currently, the only valid value for the location is "global".
    
    pub name: Option<String>,
    /// The public SSH key. This must be in OpenSSH .authorized_keys format.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<String>,
}

impl client::RequestValue for SSHKey {}
impl client::ResponseResult for SSHKey {}


/// Network template.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServerNetworkTemplate {
    /// Instance types this template is applicable to.
    #[serde(rename="applicableInstanceTypes")]
    
    pub applicable_instance_types: Option<Vec<String>>,
    /// Logical interfaces.
    #[serde(rename="logicalInterfaces")]
    
    pub logical_interfaces: Option<Vec<GoogleCloudBaremetalsolutionV2ServerNetworkTemplateLogicalInterface>>,
    /// Output only. Template's unique name. The full resource name follows the pattern: `projects/{project}/locations/{location}/serverNetworkTemplate/{server_network_template}` Generally, the {server_network_template} follows the syntax of "bond" or "nic".
    
    pub name: Option<String>,
}

impl client::Part for ServerNetworkTemplate {}


/// Details about snapshot space reservation and usage on the storage volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotReservationDetail {
    /// The space on this storage volume reserved for snapshots, shown in GiB.
    #[serde(rename="reservedSpaceGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub reserved_space_gib: Option<i64>,
    /// Percent of the total Volume size reserved for snapshot copies. Enabling snapshots requires reserving 20% or more of the storage volume space for snapshots. Maximum reserved space for snapshots is 40%. Setting this field will effectively set snapshot_enabled to true.
    #[serde(rename="reservedSpacePercent")]
    
    pub reserved_space_percent: Option<i32>,
    /// The amount, in GiB, of available space in this storage volume's reserved snapshot space.
    #[serde(rename="reservedSpaceRemainingGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub reserved_space_remaining_gib: Option<i64>,
    /// The percent of snapshot space on this storage volume actually being used by the snapshot copies. This value might be higher than 100% if the snapshot copies have overflowed into the data portion of the storage volume.
    #[serde(rename="reservedSpaceUsedPercent")]
    
    pub reserved_space_used_percent: Option<i32>,
}

impl client::Part for SnapshotReservationDetail {}


/// Message requesting to start a server.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances start projects](ProjectLocationInstanceStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for StartInstanceRequest {}


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


/// Message requesting to stop a server.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances stop projects](ProjectLocationInstanceStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopInstanceRequest { _never_set: Option<bool> }

impl client::RequestValue for StopInstanceRequest {}


/// Request for SubmitProvisioningConfig.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations provisioning configs submit projects](ProjectLocationProvisioningConfigSubmitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubmitProvisioningConfigRequest {
    /// Optional. Email provided to send a confirmation with provisioning config to.
    
    pub email: Option<String>,
    /// Required. The ProvisioningConfig to create.
    #[serde(rename="provisioningConfig")]
    
    pub provisioning_config: Option<ProvisioningConfig>,
}

impl client::RequestValue for SubmitProvisioningConfigRequest {}


/// Response for SubmitProvisioningConfig.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations provisioning configs submit projects](ProjectLocationProvisioningConfigSubmitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubmitProvisioningConfigResponse {
    /// The submitted provisioning config.
    #[serde(rename="provisioningConfig")]
    
    pub provisioning_config: Option<ProvisioningConfig>,
}

impl client::ResponseResult for SubmitProvisioningConfigResponse {}


/// A network VRF.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VRF {
    /// The name of the VRF.
    
    pub name: Option<String>,
    /// The QOS policy applied to this VRF. The value is only meaningful when all the vlan attachments have the same QoS. This field should not be used for new integrations, use vlan attachment level qos instead. The field is left for backward-compatibility.
    #[serde(rename="qosPolicy")]
    
    pub qos_policy: Option<QosPolicy>,
    /// The possible state of VRF.
    
    pub state: Option<String>,
    /// The list of VLAN attachments for the VRF.
    #[serde(rename="vlanAttachments")]
    
    pub vlan_attachments: Option<Vec<VlanAttachment>>,
}

impl client::Part for VRF {}


/// VLAN attachment details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VlanAttachment {
    /// Immutable. The identifier of the attachment within vrf.
    
    pub id: Option<String>,
    /// Input only. Pairing key.
    #[serde(rename="pairingKey")]
    
    pub pairing_key: Option<String>,
    /// The peer IP of the attachment.
    #[serde(rename="peerIp")]
    
    pub peer_ip: Option<String>,
    /// The peer vlan ID of the attachment.
    #[serde(rename="peerVlanId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub peer_vlan_id: Option<i64>,
    /// The QOS policy applied to this VLAN attachment. This value should be preferred to using qos at vrf level.
    #[serde(rename="qosPolicy")]
    
    pub qos_policy: Option<QosPolicy>,
    /// The router IP of the attachment.
    #[serde(rename="routerIp")]
    
    pub router_ip: Option<String>,
}

impl client::Part for VlanAttachment {}


/// A storage volume.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes get projects](ProjectLocationVolumeGetCall) (response)
/// * [locations volumes patch projects](ProjectLocationVolumePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    /// The size, in GiB, that this storage volume has expanded as a result of an auto grow policy. In the absence of auto-grow, the value is 0.
    #[serde(rename="autoGrownSizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub auto_grown_size_gib: Option<i64>,
    /// Output only. Whether this volume is a boot volume. A boot volume is one which contains a boot LUN.
    #[serde(rename="bootVolume")]
    
    pub boot_volume: Option<bool>,
    /// The current size of this storage volume, in GiB, including space reserved for snapshots. This size might be different than the requested size if the storage volume has been configured with auto grow or auto shrink.
    #[serde(rename="currentSizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_size_gib: Option<i64>,
    /// Additional emergency size that was requested for this Volume, in GiB. current_size_gib includes this value.
    #[serde(rename="emergencySizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub emergency_size_gib: Option<i64>,
    /// An identifier for the `Volume`, generated by the backend.
    
    pub id: Option<String>,
    /// Labels as key value pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// Maximum size volume can be expanded to in case of evergency, in GiB.
    #[serde(rename="maxSizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_size_gib: Option<i64>,
    /// Output only. The resource name of this `Volume`. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. Format: `projects/{project}/locations/{location}/volumes/{volume}`
    
    pub name: Option<String>,
    /// Input only. User-specified notes for new Volume. Used to provision Volumes that require manual intervention.
    
    pub notes: Option<String>,
    /// Originally requested size, in GiB.
    #[serde(rename="originallyRequestedSizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub originally_requested_size_gib: Option<i64>,
    /// Immutable. Performance tier of the Volume. Default is SHARED.
    #[serde(rename="performanceTier")]
    
    pub performance_tier: Option<String>,
    /// Immutable. Pod name.
    
    pub pod: Option<String>,
    /// Output only. Storage protocol for the Volume.
    
    pub protocol: Option<String>,
    /// The space remaining in the storage volume for new LUNs, in GiB, excluding space reserved for snapshots.
    #[serde(rename="remainingSpaceGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub remaining_space_gib: Option<i64>,
    /// The requested size of this storage volume, in GiB.
    #[serde(rename="requestedSizeGib")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub requested_size_gib: Option<i64>,
    /// The behavior to use when snapshot reserved space is full.
    #[serde(rename="snapshotAutoDeleteBehavior")]
    
    pub snapshot_auto_delete_behavior: Option<String>,
    /// Whether snapshots are enabled.
    #[serde(rename="snapshotEnabled")]
    
    pub snapshot_enabled: Option<bool>,
    /// Details about snapshot space reservation and usage on the storage volume.
    #[serde(rename="snapshotReservationDetail")]
    
    pub snapshot_reservation_detail: Option<SnapshotReservationDetail>,
    /// The name of the snapshot schedule policy in use for this volume, if any.
    #[serde(rename="snapshotSchedulePolicy")]
    
    pub snapshot_schedule_policy: Option<String>,
    /// The state of this storage volume.
    
    pub state: Option<String>,
    /// Input only. Name of the storage aggregate pool to allocate the volume in. Can be used only for VOLUME_PERFORMANCE_TIER_ASSIGNED volumes.
    #[serde(rename="storageAggregatePool")]
    
    pub storage_aggregate_pool: Option<String>,
    /// The storage type for this volume.
    #[serde(rename="storageType")]
    
    pub storage_type: Option<String>,
    /// The workload profile for the volume.
    #[serde(rename="workloadProfile")]
    
    pub workload_profile: Option<String>,
}

impl client::RequestValue for Volume {}
impl client::ResponseResult for Volume {}


/// Configuration parameters for a new volume.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeConfig {
    /// The GCP service of the storage volume. Available gcp_service are in https://cloud.google.com/bare-metal/docs/bms-planning.
    #[serde(rename="gcpService")]
    
    pub gcp_service: Option<String>,
    /// A transient unique identifier to identify a volume within an ProvisioningConfig request.
    
    pub id: Option<String>,
    /// LUN ranges to be configured. Set only when protocol is PROTOCOL_FC.
    #[serde(rename="lunRanges")]
    
    pub lun_ranges: Option<Vec<LunRange>>,
    /// Machine ids connected to this volume. Set only when protocol is PROTOCOL_FC.
    #[serde(rename="machineIds")]
    
    pub machine_ids: Option<Vec<String>>,
    /// Output only. The name of the volume config.
    
    pub name: Option<String>,
    /// NFS exports. Set only when protocol is PROTOCOL_NFS.
    #[serde(rename="nfsExports")]
    
    pub nfs_exports: Option<Vec<NfsExport>>,
    /// Performance tier of the Volume. Default is SHARED.
    #[serde(rename="performanceTier")]
    
    pub performance_tier: Option<String>,
    /// Volume protocol.
    
    pub protocol: Option<String>,
    /// The requested size of this volume, in GB.
    #[serde(rename="sizeGb")]
    
    pub size_gb: Option<i32>,
    /// Whether snapshots should be enabled.
    #[serde(rename="snapshotsEnabled")]
    
    pub snapshots_enabled: Option<bool>,
    /// Input only. Name of the storage aggregate pool to allocate the volume in. Can be used only for VOLUME_PERFORMANCE_TIER_ASSIGNED volumes.
    #[serde(rename="storageAggregatePool")]
    
    pub storage_aggregate_pool: Option<String>,
    /// The type of this Volume.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// User note field, it can be used by customers to add additional information for the BMS Ops team .
    #[serde(rename="userNote")]
    
    pub user_note: Option<String>,
}

impl client::Part for VolumeConfig {}


/// A snapshot of a volume. Only boot volumes can have snapshots.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations volumes snapshots create projects](ProjectLocationVolumeSnapshotCreateCall) (request|response)
/// * [locations volumes snapshots get projects](ProjectLocationVolumeSnapshotGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VolumeSnapshot {
    /// Output only. The creation time of the snapshot.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The description of the snapshot.
    
    pub description: Option<String>,
    /// Output only. An identifier for the snapshot, generated by the backend.
    
    pub id: Option<String>,
    /// The name of the snapshot.
    
    pub name: Option<String>,
    /// Output only. The name of the volume which this snapshot belongs to.
    #[serde(rename="storageVolume")]
    
    pub storage_volume: Option<String>,
    /// Output only. The type of the snapshot which indicates whether it was scheduled or manual/ad-hoc.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for VolumeSnapshot {}
impl client::ResponseResult for VolumeSnapshot {}


