use super::*;
/// Request for ApplyParameters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances apply parameters projects](ProjectLocationInstanceApplyParameterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplyParametersRequest {
    /// Whether to apply instance-level parameter group to all nodes. If set to true, users are restricted from specifying individual nodes, and `ApplyParameters` updates all nodes within the instance.
    #[serde(rename="applyAll")]
    
    pub apply_all: Option<bool>,
    /// Nodes to which the instance-level parameter group is applied.
    #[serde(rename="nodeIds")]
    
    pub node_ids: Option<Vec<String>>,
}

impl client::RequestValue for ApplyParametersRequest {}


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


/// Maintenance policy per instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMemcacheV1MaintenancePolicy {
    /// Output only. The time when the policy was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Description of what this policy is for. Create/Update methods return INVALID_ARGUMENT if the length is greater than 512.
    
    pub description: Option<String>,
    /// Output only. The time when the policy was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Maintenance window that is applied to resources covered by this policy. Minimum 1. For the current version, the maximum number of weekly_maintenance_windows is expected to be one.
    #[serde(rename="weeklyMaintenanceWindow")]
    
    pub weekly_maintenance_window: Option<Vec<WeeklyMaintenanceWindow>>,
}

impl client::Part for GoogleCloudMemcacheV1MaintenancePolicy {}


/// A Memorystore for Memcached instance
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
    /// The full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the instance is connected. If left unspecified, the `default` network will be used.
    #[serde(rename="authorizedNetwork")]
    
    pub authorized_network: Option<String>,
    /// Output only. The time the instance was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Endpoint for the Discovery API.
    #[serde(rename="discoveryEndpoint")]
    
    pub discovery_endpoint: Option<String>,
    /// User provided name for the instance, which is only used for display purposes. Cannot be more than 80 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// List of messages that describe the current state of the Memcached instance.
    #[serde(rename="instanceMessages")]
    
    pub instance_messages: Option<Vec<InstanceMessage>>,
    /// Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources
    
    pub labels: Option<HashMap<String, String>>,
    /// The maintenance policy for the instance. If not provided, the maintenance event will be performed based on Memorystore internal rollout schedule.
    #[serde(rename="maintenancePolicy")]
    
    pub maintenance_policy: Option<GoogleCloudMemcacheV1MaintenancePolicy>,
    /// Output only. Published maintenance schedule.
    #[serde(rename="maintenanceSchedule")]
    
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    /// Output only. The full version of memcached server running on this instance. System automatically determines the full memcached version for an instance based on the input MemcacheVersion. The full version format will be "memcached-1.5.16".
    #[serde(rename="memcacheFullVersion")]
    
    pub memcache_full_version: Option<String>,
    /// Output only. List of Memcached nodes. Refer to Node message for more details.
    #[serde(rename="memcacheNodes")]
    
    pub memcache_nodes: Option<Vec<Node>>,
    /// The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest supported major version is `MEMCACHE_1_5`. The minor version will be automatically determined by our system based on the latest supported minor version.
    #[serde(rename="memcacheVersion")]
    
    pub memcache_version: Option<InstanceMemcacheVersionEnum>,
    /// Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Memcached instances are managed and addressed at the regional level so `location_id` here refers to a Google Cloud region; however, users may choose which zones Memcached nodes should be provisioned in within an instance. Refer to zones field for more details.
    
    pub name: Option<String>,
    /// Required. Configuration for Memcached nodes.
    #[serde(rename="nodeConfig")]
    
    pub node_config: Option<NodeConfig>,
    /// Required. Number of nodes in the Memcached instance.
    #[serde(rename="nodeCount")]
    
    pub node_count: Option<i32>,
    /// User defined parameters to apply to the memcached process on each node.
    
    pub parameters: Option<MemcacheParameters>,
    /// Output only. The state of this Memcached instance.
    
    pub state: Option<InstanceStateEnum>,
    /// Output only. The time the instance was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Zones in which Memcached nodes should be provisioned. Memcached nodes will be equally distributed across these zones. If not provided, the service will by default create nodes in all zones in the region for the instance.
    
    pub zones: Option<Vec<String>>,
}

impl client::RequestValue for Instance {}
impl client::ResponseResult for Instance {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceMessage {
    /// A code that correspond to one type of user-facing message.
    
    pub code: Option<InstanceMessageCodeEnum>,
    /// Message on memcached instance which will be exposed to users.
    
    pub message: Option<String>,
}

impl client::Part for InstanceMessage {}


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
    /// A list of Memcached instances in the project in the specified location, or across all locations. If the `location_id` in the parent field of the request is "-", all regions available to the project are queried, and the results aggregated.
    
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
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Upcoming maintenance schedule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemcacheParameters {
    /// Output only. The unique ID associated with this set of parameters. Users can use this id to determine if the parameters associated with the instance differ from the parameters associated with the nodes. A discrepancy between parameter ids can inform users that they may need to take action to apply parameters on nodes.
    
    pub id: Option<String>,
    /// User defined set of parameters to use in the memcached process.
    
    pub params: Option<HashMap<String, String>>,
}

impl client::Part for MemcacheParameters {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// Output only. Hostname or IP address of the Memcached node used by the clients to connect to the Memcached server on this node.
    
    pub host: Option<String>,
    /// Output only. Identifier of the Memcached node. The node id does not include project or location like the Memcached instance name.
    #[serde(rename="nodeId")]
    
    pub node_id: Option<String>,
    /// User defined parameters currently applied to the node.
    
    pub parameters: Option<MemcacheParameters>,
    /// Output only. The port number of the Memcached server on this node.
    
    pub port: Option<i32>,
    /// Output only. Current state of the Memcached node.
    
    pub state: Option<NodeStateEnum>,
    /// Output only. Location (GCP Zone) for the Memcached node.
    
    pub zone: Option<String>,
}

impl client::Part for Node {}


/// Configuration for a Memcached Node.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Required. Number of cpus per Memcached node.
    #[serde(rename="cpuCount")]
    
    pub cpu_count: Option<i32>,
    /// Required. Memory size in MiB for each Memcached node.
    #[serde(rename="memorySizeMb")]
    
    pub memory_size_mb: Option<i32>,
}

impl client::Part for NodeConfig {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances apply parameters projects](ProjectLocationInstanceApplyParameterCall) (response)
/// * [locations instances create projects](ProjectLocationInstanceCreateCall) (response)
/// * [locations instances delete projects](ProjectLocationInstanceDeleteCall) (response)
/// * [locations instances patch projects](ProjectLocationInstancePatchCall) (response)
/// * [locations instances reschedule maintenance projects](ProjectLocationInstanceRescheduleMaintenanceCall) (response)
/// * [locations instances update parameters projects](ProjectLocationInstanceUpdateParameterCall) (response)
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
    
    pub reschedule_type: Option<RescheduleMaintenanceRequestRescheduleTypeEnum>,
    /// Timestamp when the maintenance shall be rescheduled to if reschedule_type=SPECIFIC_TIME, in RFC 3339 format, for example `2012-11-15T16:19:00.094Z`.
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


/// Request for UpdateParameters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations instances update parameters projects](ProjectLocationInstanceUpdateParameterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateParametersRequest {
    /// The parameters to apply to the instance.
    
    pub parameters: Option<MemcacheParameters>,
    /// Required. Mask of fields to update.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for UpdateParametersRequest {}


/// Time window specified for weekly operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeeklyMaintenanceWindow {
    /// Required. Allows to define schedule that runs specified day of the week.
    
    pub day: Option<WeeklyMaintenanceWindowDayEnum>,
    /// Required. Duration of the time window.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Required. Start time of the window in UTC.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for WeeklyMaintenanceWindow {}


