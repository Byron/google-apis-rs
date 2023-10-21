use super::*;
/// Update of a single instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceUpdate {
    /// Errors that occurred during the instance update.
    
    pub error: Option<InstanceUpdateError>,
    /// Fully-qualified URL of the instance being updated.
    
    pub instance: Option<String>,
    /// Status of the instance update. Possible values are:  
    /// - "PENDING": The instance update is pending execution. 
    /// - "ROLLING_FORWARD": The instance update is going forward. 
    /// - "ROLLING_BACK": The instance update is being rolled back. 
    /// - "PAUSED": The instance update is temporarily paused (inactive). 
    /// - "ROLLED_OUT": The instance update is finished, the instance is running the new template. 
    /// - "ROLLED_BACK": The instance update is finished, the instance has been reverted to the previous template. 
    /// - "CANCELLED": The instance update is paused and no longer can be resumed, undefined in which template the instance is running.
    
    pub status: Option<String>,
}

impl client::Part for InstanceUpdate {}


/// Response returned by ListInstanceUpdates method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list instance updates rolling updates](RollingUpdateListInstanceUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceUpdateList {
    /// Collection of requested instance updates.
    
    pub items: Option<Vec<InstanceUpdate>>,
    /// [Output Only] Type of the resource.
    
    pub kind: Option<String>,
    /// A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// [Output Only] The fully qualified URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for InstanceUpdateList {}


/// An operation resource, used to manage asynchronous API requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel rolling updates](RollingUpdateCancelCall) (response)
/// * [insert rolling updates](RollingUpdateInsertCall) (response)
/// * [pause rolling updates](RollingUpdatePauseCall) (response)
/// * [resume rolling updates](RollingUpdateResumeCall) (response)
/// * [rollback rolling updates](RollingUpdateRollbackCall) (response)
/// * [get zone operations](ZoneOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// no description provided
    #[serde(rename="clientOperationId")]
    
    pub client_operation_id: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// no description provided
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// [Output Only] If errors occurred during processing of this operation, this field will be populated.
    
    pub error: Option<OperationError>,
    /// no description provided
    #[serde(rename="httpErrorMessage")]
    
    pub http_error_message: Option<String>,
    /// no description provided
    #[serde(rename="httpErrorStatusCode")]
    
    pub http_error_status_code: Option<i32>,
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] The time that this operation was requested. This is in RFC 3339 format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// [Output Only] Type of the resource. Always replicapoolupdater#operation for Operation resources.
    
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// no description provided
    
    pub progress: Option<i32>,
    /// [Output Only] URL of the region where the operation resides.
    
    pub region: Option<String>,
    /// [Output Only] The fully qualified URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] The time that this operation was started by the server. This is in RFC 3339 format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// [Output Only] Status of the operation. Can be one of the following: "PENDING", "RUNNING", or "DONE".
    
    pub status: Option<String>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// [Output Only] Unique target id which identifies a particular incarnation of the target.
    #[serde(rename="targetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_id: Option<u64>,
    /// [Output Only] URL of the resource the operation is mutating.
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// no description provided
    
    pub user: Option<String>,
    /// no description provided
    
    pub warnings: Option<Vec<OperationWarnings>>,
    /// [Output Only] URL of the zone where the operation resides.
    
    pub zone: Option<String>,
}

impl client::ResponseResult for Operation {}


/// Contains a list of Operation resources.
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
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    pub id: Option<String>,
    /// [Output Only] The Operation resources.
    
    pub items: Option<Vec<Operation>>,
    /// [Output Only] Type of resource. Always replicapoolupdater#operationList for OperationList resources.
    
    pub kind: Option<String>,
    /// [Output Only] A token used to continue a truncate.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// [Output Only] The fully qualified URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for OperationList {}


/// The following represents a resource describing a single update (rollout) of a group of instances to the given template.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel rolling updates](RollingUpdateCancelCall) (none)
/// * [get rolling updates](RollingUpdateGetCall) (response)
/// * [insert rolling updates](RollingUpdateInsertCall) (request)
/// * [list rolling updates](RollingUpdateListCall) (none)
/// * [list instance updates rolling updates](RollingUpdateListInstanceUpdateCall) (none)
/// * [pause rolling updates](RollingUpdatePauseCall) (none)
/// * [resume rolling updates](RollingUpdateResumeCall) (none)
/// * [rollback rolling updates](RollingUpdateRollbackCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollingUpdate {
    /// Specifies the action to take for each instance within the instance group. This can be RECREATE which will recreate each instance and is only available for managed instance groups. It can also be REBOOT which performs a soft reboot for each instance and is only available for regular (non-managed) instance groups.
    #[serde(rename="actionType")]
    
    pub action_type: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// An optional textual description of the resource; provided by the client when the resource is created.
    
    pub description: Option<String>,
    /// [Output Only] Errors that occurred during the rolling update.
    
    pub error: Option<RollingUpdateError>,
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    pub id: Option<String>,
    /// Fully-qualified URL of an instance group being updated. Exactly one of instanceGroupManager and instanceGroup must be set.
    #[serde(rename="instanceGroup")]
    
    pub instance_group: Option<String>,
    /// Fully-qualified URL of an instance group manager being updated. Exactly one of instanceGroupManager and instanceGroup must be set.
    #[serde(rename="instanceGroupManager")]
    
    pub instance_group_manager: Option<String>,
    /// Fully-qualified URL of an instance template to apply.
    #[serde(rename="instanceTemplate")]
    
    pub instance_template: Option<String>,
    /// [Output Only] Type of the resource.
    
    pub kind: Option<String>,
    /// Fully-qualified URL of the instance template encountered while starting the update.
    #[serde(rename="oldInstanceTemplate")]
    
    pub old_instance_template: Option<String>,
    /// Parameters of the update process.
    
    pub policy: Option<RollingUpdatePolicy>,
    /// [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the update will be complete. This number should be monotonically increasing as the update progresses.
    
    pub progress: Option<i32>,
    /// [Output Only] The fully qualified URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] Status of the update. Possible values are:  
    /// - "ROLLING_FORWARD": The update is going forward. 
    /// - "ROLLING_BACK": The update is being rolled back. 
    /// - "PAUSED": The update is temporarily paused (inactive). 
    /// - "ROLLED_OUT": The update is finished, all instances have been updated successfully. 
    /// - "ROLLED_BACK": The update is finished, all instances have been reverted to the previous template. 
    /// - "CANCELLED": The update is paused and no longer can be resumed, undefined how many instances are running in which template.
    
    pub status: Option<String>,
    /// [Output Only] An optional textual description of the current status of the update.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// [Output Only] User who requested the update, for example: user@example.com.
    
    pub user: Option<String>,
}

impl client::RequestValue for RollingUpdate {}
impl client::Resource for RollingUpdate {}
impl client::ResponseResult for RollingUpdate {}


/// Response returned by List method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list rolling updates](RollingUpdateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollingUpdateList {
    /// Collection of requested updates.
    
    pub items: Option<Vec<RollingUpdate>>,
    /// [Output Only] Type of the resource.
    
    pub kind: Option<String>,
    /// A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// [Output Only] The fully qualified URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for RollingUpdateList {}


/// Errors that occurred during the instance update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceUpdateError {
    /// [Output Only] The array of errors encountered while processing this operation.
    
    pub errors: Option<Vec<InstanceUpdateErrorErrors>>,
}

impl client::NestedType for InstanceUpdateError {}
impl client::Part for InstanceUpdateError {}


/// [Output Only] The array of errors encountered while processing this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceUpdateErrorErrors {
    /// [Output Only] The error type identifier for this error.
    
    pub code: Option<String>,
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for InstanceUpdateErrorErrors {}
impl client::Part for InstanceUpdateErrorErrors {}


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
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationErrorErrors {}
impl client::Part for OperationErrorErrors {}


/// There is no detailed description.
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


/// [Output Only] Errors that occurred during the rolling update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollingUpdateError {
    /// [Output Only] The array of errors encountered while processing this operation.
    
    pub errors: Option<Vec<RollingUpdateErrorErrors>>,
}

impl client::NestedType for RollingUpdateError {}
impl client::Part for RollingUpdateError {}


/// [Output Only] The array of errors encountered while processing this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollingUpdateErrorErrors {
    /// [Output Only] The error type identifier for this error.
    
    pub code: Option<String>,
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for RollingUpdateErrorErrors {}
impl client::Part for RollingUpdateErrorErrors {}


/// Parameters of the update process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollingUpdatePolicy {
    /// Number of instances to update before the updater pauses the rolling update.
    #[serde(rename="autoPauseAfterInstances")]
    
    pub auto_pause_after_instances: Option<i32>,
    /// The maximum amount of time that the updater waits for a HEALTHY state after all of the update steps are complete. If the HEALTHY state is not received before the deadline, the instance update is considered a failure.
    #[serde(rename="instanceStartupTimeoutSec")]
    
    pub instance_startup_timeout_sec: Option<i32>,
    /// The maximum number of instances that can be updated simultaneously. An instance update is considered complete only after the instance is restarted and initialized.
    #[serde(rename="maxNumConcurrentInstances")]
    
    pub max_num_concurrent_instances: Option<i32>,
    /// The maximum number of instance updates that can fail before the group update is considered a failure. An instance update is considered failed if any of its update actions (e.g. Stop call on Instance resource in Rolling Reboot) failed with permanent failure, or if the instance is in an UNHEALTHY state after it finishes all of the update actions.
    #[serde(rename="maxNumFailedInstances")]
    
    pub max_num_failed_instances: Option<i32>,
    /// The minimum amount of time that the updater spends to update each instance. Update time is the time it takes to complete all update actions (e.g. Stop call on Instance resource in Rolling Reboot), reboot, and initialize. If the instance update finishes early, the updater pauses for the remainder of the time before it starts the next instance update.
    #[serde(rename="minInstanceUpdateTimeSec")]
    
    pub min_instance_update_time_sec: Option<i32>,
}

impl client::NestedType for RollingUpdatePolicy {}
impl client::Part for RollingUpdatePolicy {}


