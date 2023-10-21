use super::*;
/// An Instance Group Manager resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [abandon instances instance group managers](InstanceGroupManagerAbandonInstanceCall) (none)
/// * [delete instance group managers](InstanceGroupManagerDeleteCall) (none)
/// * [delete instances instance group managers](InstanceGroupManagerDeleteInstanceCall) (none)
/// * [get instance group managers](InstanceGroupManagerGetCall) (response)
/// * [insert instance group managers](InstanceGroupManagerInsertCall) (request)
/// * [list instance group managers](InstanceGroupManagerListCall) (none)
/// * [recreate instances instance group managers](InstanceGroupManagerRecreateInstanceCall) (none)
/// * [resize instance group managers](InstanceGroupManagerResizeCall) (none)
/// * [set instance template instance group managers](InstanceGroupManagerSetInstanceTemplateCall) (none)
/// * [set target pools instance group managers](InstanceGroupManagerSetTargetPoolCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManager {
    /// The autohealing policy for this managed instance group. You can specify only one value.
    #[serde(rename="autoHealingPolicies")]
    
    pub auto_healing_policies: Option<Vec<ReplicaPoolAutoHealingPolicy>>,
    /// The base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name.
    #[serde(rename="baseInstanceName")]
    
    pub base_instance_name: Option<String>,
    /// [Output only] The time the instance group manager was created, in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// [Output only] The number of instances that currently exist and are a part of this group. This includes instances that are starting but are not yet RUNNING, and instances that are in the process of being deleted or abandoned.
    #[serde(rename="currentSize")]
    
    pub current_size: Option<i32>,
    /// An optional textual description of the instance group manager.
    
    pub description: Option<String>,
    /// [Output only] Fingerprint of the instance group manager. This field is used for optimistic locking. An up-to-date fingerprint must be provided in order to modify the Instance Group Manager resource.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub fingerprint: Option<Vec<u8>>,
    /// [Output only] The full URL of the instance group created by the manager. This group contains all of the instances being managed, and cannot contain non-managed instances.
    
    pub group: Option<String>,
    /// [Output only] A server-assigned unique identifier for the resource.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// The full URL to an instance template from which all new instances will be created.
    #[serde(rename="instanceTemplate")]
    
    pub instance_template: Option<String>,
    /// [Output only] The resource type. Always replicapool#instanceGroupManager.
    
    pub kind: Option<String>,
    /// The name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens.
    
    pub name: Option<String>,
    /// [Output only] The fully qualified URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The full URL of all target pools to which new instances in the group are added. Updating the target pool values does not affect existing instances.
    #[serde(rename="targetPools")]
    
    pub target_pools: Option<Vec<String>>,
    /// [Output only] The number of instances that the manager is attempting to maintain. Deleting or abandoning instances affects this number, as does resizing the group.
    #[serde(rename="targetSize")]
    
    pub target_size: Option<i32>,
}

impl client::RequestValue for InstanceGroupManager {}
impl client::Resource for InstanceGroupManager {}
impl client::ResponseResult for InstanceGroupManager {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list instance group managers](InstanceGroupManagerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagerList {
    /// Unique identifier for the resource; defined by the server (output only).
    
    pub id: Option<String>,
    /// A list of instance resources.
    
    pub items: Option<Vec<InstanceGroupManager>>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// A token used to continue a truncated list request (output only).
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Server defined URL for this resource (output only).
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for InstanceGroupManagerList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [abandon instances instance group managers](InstanceGroupManagerAbandonInstanceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersAbandonInstancesRequest {
    /// The names of one or more instances to abandon. For example:
    /// { 'instances': [ 'instance-c3po', 'instance-r2d2' ] }
    
    pub instances: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersAbandonInstancesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete instances instance group managers](InstanceGroupManagerDeleteInstanceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersDeleteInstancesRequest {
    /// Names of instances to delete.
    /// 
    /// Example: 'instance-foo', 'instance-bar'
    
    pub instances: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersDeleteInstancesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recreate instances instance group managers](InstanceGroupManagerRecreateInstanceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersRecreateInstancesRequest {
    /// The names of one or more instances to recreate. For example:
    /// { 'instances': [ 'instance-c3po', 'instance-r2d2' ] }
    
    pub instances: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersRecreateInstancesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set instance template instance group managers](InstanceGroupManagerSetInstanceTemplateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersSetInstanceTemplateRequest {
    /// The full URL to an Instance Template from which all new instances will be created.
    #[serde(rename="instanceTemplate")]
    
    pub instance_template: Option<String>,
}

impl client::RequestValue for InstanceGroupManagersSetInstanceTemplateRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set target pools instance group managers](InstanceGroupManagerSetTargetPoolCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersSetTargetPoolsRequest {
    /// The current fingerprint of the Instance Group Manager resource. If this does not match the server-side fingerprint of the resource, then the request will be rejected.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub fingerprint: Option<Vec<u8>>,
    /// A list of fully-qualified URLs to existing Target Pool resources. New instances in the Instance Group Manager will be added to the specified target pools; existing instances are not affected.
    #[serde(rename="targetPools")]
    
    pub target_pools: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersSetTargetPoolsRequest {}


/// An operation resource, used to manage asynchronous API requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [abandon instances instance group managers](InstanceGroupManagerAbandonInstanceCall) (response)
/// * [delete instance group managers](InstanceGroupManagerDeleteCall) (response)
/// * [delete instances instance group managers](InstanceGroupManagerDeleteInstanceCall) (response)
/// * [insert instance group managers](InstanceGroupManagerInsertCall) (response)
/// * [recreate instances instance group managers](InstanceGroupManagerRecreateInstanceCall) (response)
/// * [resize instance group managers](InstanceGroupManagerResizeCall) (response)
/// * [set instance template instance group managers](InstanceGroupManagerSetInstanceTemplateCall) (response)
/// * [set target pools instance group managers](InstanceGroupManagerSetTargetPoolCall) (response)
/// * [get zone operations](ZoneOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// [Output only] An optional identifier specified by the client when the mutation was initiated. Must be unique for all operation resources in the project.
    #[serde(rename="clientOperationId")]
    
    pub client_operation_id: Option<String>,
    /// [Output Only] The time that this operation was requested, in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// [Output Only] The time that this operation was completed, in RFC3339 text format.
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// [Output Only] If errors occurred during processing of this operation, this field will be populated.
    
    pub error: Option<OperationError>,
    /// [Output only] If operation fails, the HTTP error message returned.
    #[serde(rename="httpErrorMessage")]
    
    pub http_error_message: Option<String>,
    /// [Output only] If operation fails, the HTTP error status code returned.
    #[serde(rename="httpErrorStatusCode")]
    
    pub http_error_status_code: Option<i32>,
    /// [Output Only] Unique identifier for the resource, generated by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] The time that this operation was requested, in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// [Output only] Type of the resource.
    
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    
    pub name: Option<String>,
    /// [Output only] Type of the operation. Operations include insert, update, and delete.
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// [Output only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the operation will be complete. This number should be monotonically increasing as the operation progresses.
    
    pub progress: Option<i32>,
    /// [Output Only] URL of the region where the operation resides. Only available when performing regional operations.
    
    pub region: Option<String>,
    /// [Output Only] Server-defined fully-qualified URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] The time that this operation was started by the server, in RFC3339 text format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// [Output Only] Status of the operation.
    
    pub status: Option<String>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// [Output Only] Unique target ID which identifies a particular incarnation of the target.
    #[serde(rename="targetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_id: Option<u64>,
    /// [Output only] URL of the resource the operation is mutating.
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// [Output Only] User who requested the operation, for example: user@example.com.
    
    pub user: Option<String>,
    /// [Output Only] If there are issues with this operation, a warning is returned.
    
    pub warnings: Option<Vec<OperationWarnings>>,
    /// [Output Only] URL of the zone where the operation resides. Only available when performing per-zone operations.
    
    pub zone: Option<String>,
}

impl client::ResponseResult for Operation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list zone operations](ZoneOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationList {
    /// Unique identifier for the resource; defined by the server (output only).
    
    pub id: Option<String>,
    /// The operation resources.
    
    pub items: Option<Vec<Operation>>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// A token used to continue a truncated list request (output only).
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Server defined URL for this resource (output only).
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for OperationList {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolAutoHealingPolicy {
    /// The action to perform when an instance becomes unhealthy. Possible values are RECREATE or REBOOT. RECREATE replaces an unhealthy instance with a new instance that is based on the instance template for this managed instance group. REBOOT performs a soft reboot on an instance. If the instance cannot reboot, the instance performs a hard restart.
    #[serde(rename="actionType")]
    
    pub action_type: Option<String>,
    /// The URL for the HealthCheck that signals autohealing.
    #[serde(rename="healthCheck")]
    
    pub health_check: Option<String>,
}

impl client::Part for ReplicaPoolAutoHealingPolicy {}


/// [Output Only] If errors occurred during processing of this operation, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationError {
    /// [Output Only] The array of errors encountered while processing this operation.
    
    pub errors: Option<Vec<OperationErrorErrors>>,
}

impl client::NestedType for OperationError {}
impl client::Part for OperationError {}


/// [Output Only] The array of errors encountered while processing this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationErrorErrors {
    /// [Output Only] The error type identifier for this error.
    
    pub code: Option<String>,
    /// [Output Only] Indicates the field in the request which caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationErrorErrors {}
impl client::Part for OperationErrorErrors {}


/// [Output Only] If there are issues with this operation, a warning is returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarnings {
    /// [Output only] The warning type identifier for this warning.
    
    pub code: Option<String>,
    /// [Output only] Metadata for this warning in key:value format.
    
    pub data: Option<Vec<OperationWarningsData>>,
    /// [Output only] Optional human-readable details for this warning.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationWarnings {}
impl client::Part for OperationWarnings {}


/// [Output only] Metadata for this warning in key:value format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarningsData {
    /// [Output Only] Metadata key for this warning.
    
    pub key: Option<String>,
    /// [Output Only] Metadata value for this warning.
    
    pub value: Option<String>,
}

impl client::NestedType for OperationWarningsData {}
impl client::Part for OperationWarningsData {}


