use super::*;
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


/// Request for Export.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances export projects](ProjectLocationInstanceExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportInstanceRequest {
    /// Required. Specify data to be exported.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<OutputConfig>,
}

impl client::RequestValue for ExportInstanceRequest {}


/// Request for Failover.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances failover projects](ProjectLocationInstanceFailoverCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FailoverInstanceRequest {
    /// Optional. Available data protection modes that the user can choose. If it's unspecified, data protection mode will be LIMITED_DATA_LOSS by default.
    #[serde(rename="dataProtectionMode")]
    
    pub data_protection_mode: Option<String>,
}

impl client::RequestValue for FailoverInstanceRequest {}


/// The Cloud Storage location for the output content
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsDestination {
    /// Required. Data destination URI (e.g. 'gs://my_bucket/my_object'). Existing files will be overwritten.
    
    pub uri: Option<String>,
}

impl client::Part for GcsDestination {}


/// The Cloud Storage location for the input content
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsSource {
    /// Required. Source data URI. (e.g. 'gs://my_bucket/my_object').
    
    pub uri: Option<String>,
}

impl client::Part for GcsSource {}


/// Request for Import.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances import projects](ProjectLocationInstanceImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportInstanceRequest {
    /// Required. Specify data to be imported.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<InputConfig>,
}

impl client::RequestValue for ImportInstanceRequest {}


/// The input content
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InputConfig {
    /// Google Cloud Storage location where input content is located.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsSource>,
}

impl client::Part for InputConfig {}


/// A Memorystore for Redis instance.
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
    /// Optional. If specified, at least one node will be provisioned in this zone in addition to the zone specified in location_id. Only applicable to standard tier. If provided, it must be a different zone from the one provided in [location_id]. Additional nodes beyond the first 2 will be placed in zones selected by the service.
    #[serde(rename="alternativeLocationId")]
    
    pub alternative_location_id: Option<String>,
    /// Optional. Indicates whether OSS Redis AUTH is enabled for the instance. If set to "true" AUTH is enabled on the instance. Default value is "false" meaning AUTH is disabled.
    #[serde(rename="authEnabled")]
    
    pub auth_enabled: Option<bool>,
    /// Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used.
    #[serde(rename="authorizedNetwork")]
    
    pub authorized_network: Option<String>,
    /// Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING.
    #[serde(rename="connectMode")]
    
    pub connect_mode: Option<String>,
    /// Output only. The time the instance was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current zone where the Redis primary node is located. In basic tier, this will always be the same as [location_id]. In standard tier, this can be the zone of any node in the instance.
    #[serde(rename="currentLocationId")]
    
    pub current_location_id: Option<String>,
    /// Optional. The KMS key reference that the customer provides when trying to create the instance.
    #[serde(rename="customerManagedKey")]
    
    pub customer_managed_key: Option<String>,
    /// An arbitrary and optional user-provided name for the instance.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Hostname or IP address of the exposed Redis endpoint used by clients to connect to the service.
    
    pub host: Option<String>,
    /// Resource labels to represent user provided metadata
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. The zone where the instance will be provisioned. If not provided, the service will choose a zone from the specified region for the instance. For standard tier, additional nodes will be added across multiple zones for protection against zonal failures. If specified, at least one node will be provisioned in this zone.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Optional. The maintenance policy for the instance. If not provided, maintenance events can be performed at any time.
    #[serde(rename="maintenancePolicy")]
    
    pub maintenance_policy: Option<MaintenancePolicy>,
    /// Output only. Date and time of upcoming maintenance events which have been scheduled.
    #[serde(rename="maintenanceSchedule")]
    
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    /// Required. Redis memory size in GiB.
    #[serde(rename="memorySizeGb")]
    
    pub memory_size_gb: Option<i32>,
    /// Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Redis instances are managed and addressed at regional level so location_id here refers to a GCP region; however, users may choose which specific zone (or collection of zones for cross-zone instances) an instance should be provisioned in. Refer to location_id and alternative_location_id fields for more details.
    
    pub name: Option<String>,
    /// Output only. Info per node.
    
    pub nodes: Option<Vec<NodeInfo>>,
    /// Optional. Persistence configuration parameters
    #[serde(rename="persistenceConfig")]
    
    pub persistence_config: Option<PersistenceConfig>,
    /// Output only. Cloud IAM identity used by import / export operations to transfer data to/from Cloud Storage. Format is "serviceAccount:". The value may change over time for a given instance so should be checked before each import/export operation.
    #[serde(rename="persistenceIamIdentity")]
    
    pub persistence_iam_identity: Option<String>,
    /// Output only. The port number of the exposed Redis endpoint.
    
    pub port: Option<i32>,
    /// Output only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only. Targets all healthy replica nodes in instance. Replication is asynchronous and replica nodes will exhibit some lag behind the primary. Write requests must target 'host'.
    #[serde(rename="readEndpoint")]
    
    pub read_endpoint: Option<String>,
    /// Output only. The port number of the exposed readonly redis endpoint. Standard tier only. Write requests should target 'port'.
    #[serde(rename="readEndpointPort")]
    
    pub read_endpoint_port: Option<i32>,
    /// Optional. Read replicas mode for the instance. Defaults to READ_REPLICAS_DISABLED.
    #[serde(rename="readReplicasMode")]
    
    pub read_replicas_mode: Option<String>,
    /// Optional. Redis configuration parameters, according to http://redis.io/topics/config. Currently, the only supported parameters are: Redis version 3.2 and newer: * maxmemory-policy * notify-keyspace-events Redis version 4.0 and newer: * activedefrag * lfu-decay-time * lfu-log-factor * maxmemory-gb Redis version 5.0 and newer: * stream-node-max-bytes * stream-node-max-entries
    #[serde(rename="redisConfigs")]
    
    pub redis_configs: Option<HashMap<String, String>>,
    /// Optional. The version of Redis software. If not provided, latest supported version will be used. Currently, the supported values are: * `REDIS_3_2` for Redis 3.2 compatibility * `REDIS_4_0` for Redis 4.0 compatibility (default) * `REDIS_5_0` for Redis 5.0 compatibility * `REDIS_6_X` for Redis 6.x compatibility
    #[serde(rename="redisVersion")]
    
    pub redis_version: Option<String>,
    /// Optional. The number of replica nodes. The valid range for the Standard Tier with read replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled for a Standard Tier instance, the only valid value is 1 and the default is 1. The valid value for basic tier is 0 and the default is also 0.
    #[serde(rename="replicaCount")]
    
    pub replica_count: Option<i32>,
    /// Optional. For DIRECT_PEERING mode, the CIDR range of internal addresses that are reserved for this instance. Range must be unique and non-overlapping with existing subnets in an authorized network. For PRIVATE_SERVICE_ACCESS mode, the name of one allocated IP address ranges associated with this private service access connection. If not provided, the service will choose an unused /29 block, for example, 10.0.0.0/29 or 192.168.0.0/29. For READ_REPLICAS_ENABLED the default block size is /28.
    #[serde(rename="reservedIpRange")]
    
    pub reserved_ip_range: Option<String>,
    /// Optional. Additional IP range for node placement. Required when enabling read replicas on an existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or "auto". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address range associated with the private service access connection, or "auto".
    #[serde(rename="secondaryIpRange")]
    
    pub secondary_ip_range: Option<String>,
    /// Output only. List of server CA certificates for the instance.
    #[serde(rename="serverCaCerts")]
    
    pub server_ca_certs: Option<Vec<TlsCertificate>>,
    /// Output only. The current state of this instance.
    
    pub state: Option<String>,
    /// Output only. Additional information about the current status of this instance, if available.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// Optional. reasons that causes instance in "SUSPENDED" state.
    #[serde(rename="suspensionReasons")]
    
    pub suspension_reasons: Option<Vec<String>>,
    /// Required. The service tier of the instance.
    
    pub tier: Option<String>,
    /// Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance.
    #[serde(rename="transitEncryptionMode")]
    
    pub transit_encryption_mode: Option<String>,
}

impl client::RequestValue for Instance {}
impl client::ResponseResult for Instance {}


/// Instance AUTH string details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances get auth string projects](ProjectLocationInstanceGetAuthStringCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceAuthString {
    /// AUTH string set on the instance.
    #[serde(rename="authString")]
    
    pub auth_string: Option<String>,
}

impl client::ResponseResult for InstanceAuthString {}


/// Response for ListInstances.
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
    /// A list of Redis instances in the project in the specified location, or across all locations. If the `location_id` in the parent field of the request is "-", all regions available to the project are queried, and the results aggregated. If in such an aggregated query a location is unavailable, a placeholder Redis entry is included in the response with the `name` field set to a value of the form `projects/{project_id}/locations/{location_id}/instances/`- and the `status` field set to ERROR and `status_message` field set to "location not available for ListInstances".
    
    pub instances: Option<Vec<Instance>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
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
    /// Resource ID for the region. For example: "us-east1".
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Output only. The set of available zones in the location. The map is keyed by the lowercase ID of each zone, as defined by Compute Engine. These keys can be specified in `location_id` or `alternative_location_id` fields when creating a Redis instance.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Full resource name for the region. For example: "projects/example-project/locations/us-east1".
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Maintenance policy for an instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenancePolicy {
    /// Output only. The time when the policy was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of what this policy is for. Create/Update methods return INVALID_ARGUMENT if the length is greater than 512.
    
    pub description: Option<String>,
    /// Output only. The time when the policy was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Maintenance window that is applied to resources covered by this policy. Minimum 1. For the current version, the maximum number of weekly_window is expected to be one.
    #[serde(rename="weeklyMaintenanceWindow")]
    
    pub weekly_maintenance_window: Option<Vec<WeeklyMaintenanceWindow>>,
}

impl client::Part for MaintenancePolicy {}


/// Upcoming maintenance schedule. If no maintenance is scheduled, fields are not populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    /// If the scheduled maintenance can be rescheduled, default is true.
    #[serde(rename="canReschedule")]
    
    pub can_reschedule: Option<bool>,
    /// Output only. The end time of any upcoming scheduled maintenance for this instance.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The deadline that the maintenance schedule start time can not go beyond, including reschedule.
    #[serde(rename="scheduleDeadlineTime")]
    
    pub schedule_deadline_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The start time of any upcoming scheduled maintenance for this instance.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for MaintenanceSchedule {}


/// Node specific properties.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Output only. Node identifying string. e.g. 'node-0', 'node-1'
    
    pub id: Option<String>,
    /// Output only. Location of the node.
    
    pub zone: Option<String>,
}

impl client::Part for NodeInfo {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (response)
/// * [locations instances delete projects](ProjectLocationInstanceDeleteCall) (response)
/// * [locations instances export projects](ProjectLocationInstanceExportCall) (response)
/// * [locations instances failover projects](ProjectLocationInstanceFailoverCall) (response)
/// * [locations instances import projects](ProjectLocationInstanceImportCall) (response)
/// * [locations instances patch projects](ProjectLocationInstancePatchCall) (response)
/// * [locations instances reschedule maintenance projects](ProjectLocationInstanceRescheduleMaintenanceCall) (response)
/// * [locations instances upgrade projects](ProjectLocationInstanceUpgradeCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// { `createTime`: The time the operation was created. `endTime`: The time the operation finished running. `target`: Server-defined resource path for the target of the operation. `verb`: Name of the verb executed by the operation. `statusDetail`: Human-readable status of the operation, if any. `cancelRequested`: Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`. `apiVersion`: API version used to start the operation. }
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// The output content
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Google Cloud Storage destination for output content.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsDestination>,
}

impl client::Part for OutputConfig {}


/// Configuration of the persistence functionality.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Optional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.
    #[serde(rename="persistenceMode")]
    
    pub persistence_mode: Option<String>,
    /// Output only. The next time that a snapshot attempt is scheduled to occur.
    #[serde(rename="rdbNextSnapshotTime")]
    
    pub rdb_next_snapshot_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Period between RDB snapshots. Snapshots will be attempted every period starting from the provided snapshot start time. For example, a start time of 01/01/2033 06:45 and SIX_HOURS snapshot period will do nothing until 01/01/2033, and then trigger snapshots every day at 06:45, 12:45, 18:45, and 00:45 the next day, and so on. If not provided, TWENTY_FOUR_HOURS will be used as default.
    #[serde(rename="rdbSnapshotPeriod")]
    
    pub rdb_snapshot_period: Option<String>,
    /// Optional. Date and time that the first snapshot was/will be attempted, and to which future snapshots will be aligned. If not provided, the current time will be used.
    #[serde(rename="rdbSnapshotStartTime")]
    
    pub rdb_snapshot_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PersistenceConfig {}


/// Request for RescheduleMaintenance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances reschedule maintenance projects](ProjectLocationInstanceRescheduleMaintenanceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RescheduleMaintenanceRequest {
    /// Required. If reschedule type is SPECIFIC_TIME, must set up schedule_time as well.
    #[serde(rename="rescheduleType")]
    
    pub reschedule_type: Option<String>,
    /// Optional. Timestamp when the maintenance shall be rescheduled to if reschedule_type=SPECIFIC_TIME, in RFC 3339 format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename="scheduleTime")]
    
    pub schedule_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for RescheduleMaintenanceRequest {}


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


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// TlsCertificate Resource
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TlsCertificate {
    /// PEM representation.
    
    pub cert: Option<String>,
    /// Output only. The time when the certificate was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2020-05-18T00:00:00.094Z`.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when the certificate expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2020-05-18T00:00:00.094Z`.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Serial number, as extracted from the certificate.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Sha1 Fingerprint of the certificate.
    #[serde(rename="sha1Fingerprint")]
    
    pub sha1_fingerprint: Option<String>,
}

impl client::Part for TlsCertificate {}


/// Request for UpgradeInstance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances upgrade projects](ProjectLocationInstanceUpgradeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeInstanceRequest {
    /// Required. Specifies the target version of Redis software to upgrade to.
    #[serde(rename="redisVersion")]
    
    pub redis_version: Option<String>,
}

impl client::RequestValue for UpgradeInstanceRequest {}


/// Time window in which disruptive maintenance updates occur. Non-disruptive updates can occur inside or outside this window.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeeklyMaintenanceWindow {
    /// Required. The day of week that maintenance updates occur.
    
    pub day: Option<String>,
    /// Output only. Duration of the maintenance window. The current window is fixed at 1 hour.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Required. Start time of the window in UTC time.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for WeeklyMaintenanceWindow {}


