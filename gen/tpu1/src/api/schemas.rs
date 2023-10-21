use super::*;
/// A accelerator type that a Node can be configured with.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations accelerator types get projects](ProjectLocationAcceleratorTypeGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceleratorType {
    /// The resource name.
    
    pub name: Option<String>,
    /// the accelerator type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for AcceleratorType {}


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


/// Response for ListAcceleratorTypes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations accelerator types list projects](ProjectLocationAcceleratorTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAcceleratorTypesResponse {
    /// The listed nodes.
    #[serde(rename="acceleratorTypes")]
    
    pub accelerator_types: Option<Vec<AcceleratorType>>,
    /// The next page token or empty if none.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListAcceleratorTypesResponse {}


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


/// Response for ListNodes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nodes list projects](ProjectLocationNodeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNodesResponse {
    /// The next page token or empty if none.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The listed nodes.
    
    pub nodes: Option<Vec<Node>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListNodesResponse {}


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


/// Response for ListTensorFlowVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tensorflow versions list projects](ProjectLocationTensorflowVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTensorFlowVersionsResponse {
    /// The next page token or empty if none.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The listed nodes.
    #[serde(rename="tensorflowVersions")]
    
    pub tensorflow_versions: Option<Vec<TensorFlowVersion>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListTensorFlowVersionsResponse {}


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


/// A network endpoint over which a TPU worker can be reached.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkEndpoint {
    /// The IP address of this network endpoint.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// The port of this network endpoint.
    
    pub port: Option<i32>,
}

impl client::Part for NetworkEndpoint {}


/// A TPU instance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nodes create projects](ProjectLocationNodeCreateCall) (request)
/// * [locations nodes get projects](ProjectLocationNodeGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// Required. The type of hardware accelerators associated with this node.
    #[serde(rename="acceleratorType")]
    
    pub accelerator_type: Option<String>,
    /// Output only. The API version that created this Node.
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block.
    #[serde(rename="cidrBlock")]
    
    pub cidr_block: Option<String>,
    /// Output only. The time when the node was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The user-supplied description of the TPU. Maximum of 512 characters.
    
    pub description: Option<String>,
    /// The health status of the TPU node.
    
    pub health: Option<String>,
    /// Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy.
    #[serde(rename="healthDescription")]
    
    pub health_description: Option<String>,
    /// Output only. DEPRECATED! Use network_endpoints instead. The network address for the TPU Node as visible to Compute Engine instances.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Resource labels to represent user-provided metadata.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Immutable. The name of the TPU
    
    pub name: Option<String>,
    /// The name of a network they wish to peer the TPU node to. It must be a preexisting Compute Engine network inside of the project on which this API has been activated. If none is provided, "default" will be used.
    
    pub network: Option<String>,
    /// Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that Tensorflow clients of the node reach out to the 0th entry in this map first.
    #[serde(rename="networkEndpoints")]
    
    pub network_endpoints: Option<Vec<NetworkEndpoint>>,
    /// Output only. DEPRECATED! Use network_endpoints instead. The network port for the TPU Node as visible to Compute Engine instances.
    
    pub port: Option<String>,
    /// The scheduling options for this node.
    #[serde(rename="schedulingConfig")]
    
    pub scheduling_config: Option<SchedulingConfig>,
    /// Output only. The service account used to run the tensor flow services within the node. To share resources, including Google Cloud Storage data, with the Tensorflow job running in the Node, this account must have permissions to that data.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Output only. The current state for the TPU Node.
    
    pub state: Option<String>,
    /// Output only. The Symptoms that have occurred to the TPU Node.
    
    pub symptoms: Option<Vec<Symptom>>,
    /// Required. The version of Tensorflow running in the Node.
    #[serde(rename="tensorflowVersion")]
    
    pub tensorflow_version: Option<String>,
    /// Whether the VPC peering for the node is set up through Service Networking API. The VPC Peering should be set up before provisioning the node. If this field is set, cidr_block field should not be specified. If the network, that you want to peer the TPU Node to, is Shared VPC networks, the node must be created with this this field enabled.
    #[serde(rename="useServiceNetworking")]
    
    pub use_service_networking: Option<bool>,
}

impl client::RequestValue for Node {}
impl client::ResponseResult for Node {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nodes create projects](ProjectLocationNodeCreateCall) (response)
/// * [locations nodes delete projects](ProjectLocationNodeDeleteCall) (response)
/// * [locations nodes reimage projects](ProjectLocationNodeReimageCall) (response)
/// * [locations nodes start projects](ProjectLocationNodeStartCall) (response)
/// * [locations nodes stop projects](ProjectLocationNodeStopCall) (response)
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


/// Request for ReimageNode.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nodes reimage projects](ProjectLocationNodeReimageCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReimageNodeRequest {
    /// The version for reimage to create.
    #[serde(rename="tensorflowVersion")]
    
    pub tensorflow_version: Option<String>,
}

impl client::RequestValue for ReimageNodeRequest {}


/// Sets the scheduling options for this node.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchedulingConfig {
    /// Defines whether the node is preemptible.
    
    pub preemptible: Option<bool>,
    /// Whether the node is created under a reservation.
    
    pub reserved: Option<bool>,
}

impl client::Part for SchedulingConfig {}


/// Request for StartNode.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nodes start projects](ProjectLocationNodeStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartNodeRequest { _never_set: Option<bool> }

impl client::RequestValue for StartNodeRequest {}


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


/// Request for StopNode.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations nodes stop projects](ProjectLocationNodeStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopNodeRequest { _never_set: Option<bool> }

impl client::RequestValue for StopNodeRequest {}


/// A Symptom instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Symptom {
    /// Timestamp when the Symptom is created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Detailed information of the current Symptom.
    
    pub details: Option<String>,
    /// Type of the Symptom.
    #[serde(rename="symptomType")]
    
    pub symptom_type: Option<String>,
    /// A string used to uniquely distinguish a worker within a TPU node.
    #[serde(rename="workerId")]
    
    pub worker_id: Option<String>,
}

impl client::Part for Symptom {}


/// A tensorflow version that a Node can be configured with.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tensorflow versions get projects](ProjectLocationTensorflowVersionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TensorFlowVersion {
    /// The resource name.
    
    pub name: Option<String>,
    /// the tensorflow version.
    
    pub version: Option<String>,
}

impl client::ResponseResult for TensorFlowVersion {}


