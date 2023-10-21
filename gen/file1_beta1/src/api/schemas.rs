use super::*;
/// A Filestore backup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations backups create projects](ProjectLocationBackupCreateCall) (request)
/// * [locations backups get projects](ProjectLocationBackupGetCall) (response)
/// * [locations backups patch projects](ProjectLocationBackupPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Backup {
    /// Output only. Capacity of the source file share when the backup was created.
    #[serde(rename="capacityGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub capacity_gb: Option<i64>,
    /// Output only. The time when the backup was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected.
    
    pub description: Option<String>,
    /// Output only. Amount of bytes that will be downloaded if the backup is restored
    #[serde(rename="downloadBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub download_bytes: Option<i64>,
    /// Immutable. KMS key name used for data encryption.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Resource labels to represent user provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`.
    
    pub name: Option<String>,
    /// Output only. Reserved for future use.
    #[serde(rename="satisfiesPzs")]
    
    pub satisfies_pzs: Option<bool>,
    /// Name of the file share in the source Filestore instance that the backup is created from.
    #[serde(rename="sourceFileShare")]
    
    pub source_file_share: Option<String>,
    /// The resource name of the source Filestore instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`, used to create this backup.
    #[serde(rename="sourceInstance")]
    
    pub source_instance: Option<String>,
    /// Output only. The service tier of the source Filestore instance that this backup is created from.
    #[serde(rename="sourceInstanceTier")]
    
    pub source_instance_tier: Option<String>,
    /// Output only. The backup state.
    
    pub state: Option<String>,
    /// Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion.
    #[serde(rename="storageBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_bytes: Option<i64>,
}

impl client::RequestValue for Backup {}
impl client::ResponseResult for Backup {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// File share configuration for the instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileShareConfig {
    /// File share capacity in gigabytes (GB). Filestore defines 1 GB as 1024^3 bytes.
    #[serde(rename="capacityGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub capacity_gb: Option<i64>,
    /// The name of the file share (must be 32 characters or less for Enterprise and High Scale SSD tiers and 16 characters or less for all other tiers).
    
    pub name: Option<String>,
    /// Nfs Export Options. There is a limit of 10 export options per file share.
    #[serde(rename="nfsExportOptions")]
    
    pub nfs_export_options: Option<Vec<NfsExportOptions>>,
    /// The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`, that this file share has been restored from.
    #[serde(rename="sourceBackup")]
    
    pub source_backup: Option<String>,
}

impl client::Part for FileShareConfig {}


/// A Filestore instance.
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
    /// The storage capacity of the instance in gigabytes (GB = 1024^3 bytes). This capacity can be increased up to `max_capacity_gb` GB in multipliers of `capacity_step_size_gb` GB.
    #[serde(rename="capacityGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub capacity_gb: Option<i64>,
    /// Output only. The increase/decrease capacity step size.
    #[serde(rename="capacityStepSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub capacity_step_size_gb: Option<i64>,
    /// Output only. The time when the instance was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The description of the instance (2048 characters or less).
    
    pub description: Option<String>,
    /// Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other.
    
    pub etag: Option<String>,
    /// File system shares on the instance. For this version, only a single file share is supported.
    #[serde(rename="fileShares")]
    
    pub file_shares: Option<Vec<FileShareConfig>>,
    /// KMS key name used for data encryption.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Resource labels to represent user provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The max capacity of the instance.
    #[serde(rename="maxCapacityGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_capacity_gb: Option<i64>,
    /// Output only. The max number of shares allowed.
    #[serde(rename="maxShareCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_share_count: Option<i64>,
    /// Indicates whether this instance uses a multi-share configuration with which it can have more than one file-share or none at all. File-shares are added, updated and removed through the separate file-share APIs.
    #[serde(rename="multiShareEnabled")]
    
    pub multi_share_enabled: Option<bool>,
    /// Output only. The resource name of the instance, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}`.
    
    pub name: Option<String>,
    /// VPC networks to which the instance is connected. For this version, only a single network is supported.
    
    pub networks: Option<Vec<NetworkConfig>>,
    /// Immutable. The protocol indicates the access protocol for all shares in the instance. This field is immutable and it cannot be changed after the instance has been created. Default value: `NFS_V3`.
    
    pub protocol: Option<String>,
    /// Output only. Reserved for future use.
    #[serde(rename="satisfiesPzs")]
    
    pub satisfies_pzs: Option<bool>,
    /// Output only. The instance state.
    
    pub state: Option<String>,
    /// Output only. Additional information about the instance state, if available.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Output only. Field indicates all the reasons the instance is in "SUSPENDED" state.
    #[serde(rename="suspensionReasons")]
    
    pub suspension_reasons: Option<Vec<String>>,
    /// The service tier of the instance.
    
    pub tier: Option<String>,
}

impl client::RequestValue for Instance {}
impl client::ResponseResult for Instance {}


/// ListBackupsResponse is the result of ListBackupsRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations backups list projects](ProjectLocationBackupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBackupsResponse {
    /// A list of backups in the project for the specified location. If the `{location}` value in the request is "-", the response contains a list of backups from all locations. If any location is unreachable, the response will only return backups in reachable locations and the "unreachable" field will be populated with a list of unreachable locations.
    
    pub backups: Option<Vec<Backup>>,
    /// The token you can use to retrieve the next page of results. Not returned if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListBackupsResponse {}


/// ListInstancesResponse is the result of ListInstancesRequest.
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
    /// A list of instances in the project for the specified location. If the `{location}` value in the request is "-", the response contains a list of instances from all locations. If any location is unreachable, the response will only return instances in reachable locations and the "unreachable" field will be populated with a list of unreachable locations.
    
    pub instances: Option<Vec<Instance>>,
    /// The token you can use to retrieve the next page of results. Not returned if there are no more results in the list.
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


/// ListSharesResponse is the result of ListSharesRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances shares list projects](ProjectLocationInstanceShareListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSharesResponse {
    /// The token you can use to retrieve the next page of results. Not returned if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of shares in the project for the specified instance.
    
    pub shares: Option<Vec<Share>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListSharesResponse {}


/// ListSnapshotsResponse is the result of ListSnapshotsRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances snapshots list projects](ProjectLocationInstanceSnapshotListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSnapshotsResponse {
    /// The token you can use to retrieve the next page of results. Not returned if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of snapshots in the project for the specified instance.
    
    pub snapshots: Option<Vec<Snapshot>>,
}

impl client::ResponseResult for ListSnapshotsResponse {}


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


/// Network configuration for the instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// The network connect mode of the Filestore instance. If not provided, the connect mode defaults to DIRECT_PEERING.
    #[serde(rename="connectMode")]
    
    pub connect_mode: Option<String>,
    /// Output only. IPv4 addresses in the format `{octet1}.{octet2}.{octet3}.{octet4}` or IPv6 addresses in the format `{block1}:{block2}:{block3}:{block4}:{block5}:{block6}:{block7}:{block8}`.
    #[serde(rename="ipAddresses")]
    
    pub ip_addresses: Option<Vec<String>>,
    /// Internet protocol versions for which the instance has IP addresses assigned. For this version, only MODE_IPV4 is supported.
    
    pub modes: Option<Vec<String>>,
    /// The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected.
    
    pub network: Option<String>,
    /// Optional, reserved_ip_range can have one of the following two types of values. * CIDR range value when using DIRECT_PEERING connect mode. * [Allocated IP address range](https://cloud.google.com/compute/docs/ip-addresses/reserve-static-internal-ip-address) when using PRIVATE_SERVICE_ACCESS connect mode. When the name of an allocated IP address range is specified, it must be one of the ranges associated with the private service access connection. When specified as a direct CIDR value, it must be a /29 CIDR block for Basic tier, a /24 CIDR block for High Scale tier, or a /26 CIDR block for Enterprise tier in one of the [internal IP address ranges](https://www.arin.net/reference/research/statistics/address_filters/) that identifies the range of IP addresses reserved for this instance. For example, 10.0.0.0/29, 192.168.0.0/24, or 192.168.0.0/26, respectively. The range you specify can't overlap with either existing subnets or assigned IP address ranges for other Filestore instances in the selected VPC network.
    #[serde(rename="reservedIpRange")]
    
    pub reserved_ip_range: Option<String>,
}

impl client::Part for NetworkConfig {}


/// NFS export options specifications.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NfsExportOptions {
    /// Either READ_ONLY, for allowing only read requests on the exported directory, or READ_WRITE, for allowing both read and write requests. The default is READ_WRITE.
    #[serde(rename="accessMode")]
    
    pub access_mode: Option<String>,
    /// An integer representing the anonymous group id with a default value of 65534. Anon_gid may only be set with squash_mode of ROOT_SQUASH. An error will be returned if this field is specified for other squash_mode settings.
    #[serde(rename="anonGid")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub anon_gid: Option<i64>,
    /// An integer representing the anonymous user id with a default value of 65534. Anon_uid may only be set with squash_mode of ROOT_SQUASH. An error will be returned if this field is specified for other squash_mode settings.
    #[serde(rename="anonUid")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub anon_uid: Option<i64>,
    /// List of either an IPv4 addresses in the format `{octet1}.{octet2}.{octet3}.{octet4}` or CIDR ranges in the format `{octet1}.{octet2}.{octet3}.{octet4}/{mask size}` which may mount the file share. Overlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned. The limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions.
    #[serde(rename="ipRanges")]
    
    pub ip_ranges: Option<Vec<String>>,
    /// Either NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH, for not allowing root access. The default is NO_ROOT_SQUASH.
    #[serde(rename="squashMode")]
    
    pub squash_mode: Option<String>,
}

impl client::Part for NfsExportOptions {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations backups create projects](ProjectLocationBackupCreateCall) (response)
/// * [locations backups delete projects](ProjectLocationBackupDeleteCall) (response)
/// * [locations backups patch projects](ProjectLocationBackupPatchCall) (response)
/// * [locations instances shares create projects](ProjectLocationInstanceShareCreateCall) (response)
/// * [locations instances shares delete projects](ProjectLocationInstanceShareDeleteCall) (response)
/// * [locations instances shares patch projects](ProjectLocationInstanceSharePatchCall) (response)
/// * [locations instances snapshots create projects](ProjectLocationInstanceSnapshotCreateCall) (response)
/// * [locations instances snapshots delete projects](ProjectLocationInstanceSnapshotDeleteCall) (response)
/// * [locations instances snapshots patch projects](ProjectLocationInstanceSnapshotPatchCall) (response)
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (response)
/// * [locations instances delete projects](ProjectLocationInstanceDeleteCall) (response)
/// * [locations instances patch projects](ProjectLocationInstancePatchCall) (response)
/// * [locations instances restore projects](ProjectLocationInstanceRestoreCall) (response)
/// * [locations instances revert projects](ProjectLocationInstanceRevertCall) (response)
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


/// RestoreInstanceRequest restores an existing instance’s file share from a backup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances restore projects](ProjectLocationInstanceRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreInstanceRequest {
    /// Required. Name of the file share in the Filestore instance that the backup is being restored to.
    #[serde(rename="fileShare")]
    
    pub file_share: Option<String>,
    /// The resource name of the backup, in the format `projects/{project_id}/locations/{location_id}/backups/{backup_id}`.
    #[serde(rename="sourceBackup")]
    
    pub source_backup: Option<String>,
    /// The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/snapshots/{snapshot_id}`.
    #[serde(rename="sourceSnapshot")]
    
    pub source_snapshot: Option<String>,
}

impl client::RequestValue for RestoreInstanceRequest {}


/// RevertInstanceRequest reverts the given instance’s file share to the specified snapshot.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances revert projects](ProjectLocationInstanceRevertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevertInstanceRequest {
    /// Required. The snapshot resource ID, in the format 'my-snapshot', where the specified ID is the {snapshot_id} of the fully qualified name like projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}
    #[serde(rename="targetSnapshotId")]
    
    pub target_snapshot_id: Option<String>,
}

impl client::RequestValue for RevertInstanceRequest {}


/// A Filestore share.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances shares create projects](ProjectLocationInstanceShareCreateCall) (request)
/// * [locations instances shares get projects](ProjectLocationInstanceShareGetCall) (response)
/// * [locations instances shares patch projects](ProjectLocationInstanceSharePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Share {
    /// File share capacity in gigabytes (GB). Filestore defines 1 GB as 1024^3 bytes. Must be greater than 0.
    #[serde(rename="capacityGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub capacity_gb: Option<i64>,
    /// Output only. The time when the share was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of the share with 2048 characters or less. Requests with longer descriptions will be rejected.
    
    pub description: Option<String>,
    /// Resource labels to represent user provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// The mount name of the share. Must be 63 characters or less and consist of uppercase or lowercase letters, numbers, and underscores.
    #[serde(rename="mountName")]
    
    pub mount_name: Option<String>,
    /// Output only. The resource name of the share, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/shares/{share_id}`.
    
    pub name: Option<String>,
    /// Nfs Export Options. There is a limit of 10 export options per file share.
    #[serde(rename="nfsExportOptions")]
    
    pub nfs_export_options: Option<Vec<NfsExportOptions>>,
    /// Output only. The share state.
    
    pub state: Option<String>,
}

impl client::RequestValue for Share {}
impl client::ResponseResult for Share {}


/// A Filestore snapshot.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances snapshots create projects](ProjectLocationInstanceSnapshotCreateCall) (request)
/// * [locations instances snapshots get projects](ProjectLocationInstanceSnapshotGetCall) (response)
/// * [locations instances snapshots patch projects](ProjectLocationInstanceSnapshotPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Snapshot {
    /// Output only. The time when the snapshot was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected.
    
    pub description: Option<String>,
    /// Output only. The amount of bytes needed to allocate a full copy of the snapshot content
    #[serde(rename="filesystemUsedBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub filesystem_used_bytes: Option<i64>,
    /// Resource labels to represent user provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The resource name of the snapshot, in the format `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`.
    
    pub name: Option<String>,
    /// Output only. The snapshot state.
    
    pub state: Option<String>,
}

impl client::RequestValue for Snapshot {}
impl client::ResponseResult for Snapshot {}


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


