use super::*;
/// Cloud Autoscaler resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete autoscalers](AutoscalerDeleteCall) (none)
/// * [get autoscalers](AutoscalerGetCall) (response)
/// * [insert autoscalers](AutoscalerInsertCall) (request)
/// * [list autoscalers](AutoscalerListCall) (none)
/// * [patch autoscalers](AutoscalerPatchCall) (request)
/// * [update autoscalers](AutoscalerUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Autoscaler {
    /// Configuration parameters for autoscaling algorithm.
    #[serde(rename="autoscalingPolicy")]
    
    pub autoscaling_policy: Option<AutoscalingPolicy>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// An optional textual description of the resource provided by the client.
    
    pub description: Option<String>,
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// Name of the Autoscaler resource. Must be unique per project and zone.
    
    pub name: Option<String>,
    /// [Output Only] A self-link to the Autoscaler configuration resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// URL to the entity which will be autoscaled. Currently the only supported value is ReplicaPool?s URL. Note: it is illegal to specify multiple Autoscalers for the same target.
    
    pub target: Option<String>,
}

impl client::RequestValue for Autoscaler {}
impl client::Resource for Autoscaler {}
impl client::ResponseResult for Autoscaler {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list autoscalers](AutoscalerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalerListResponse {
    /// Autoscaler resources.
    
    pub items: Option<Vec<Autoscaler>>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// [Output only] A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AutoscalerListResponse {}


/// Cloud Autoscaler policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingPolicy {
    /// The number of seconds that the Autoscaler should wait between two succeeding changes to the number of virtual machines. You should define an interval that is at least as long as the initialization time of a virtual machine and the time it may take for replica pool to create the virtual machine. The default is 60 seconds.
    #[serde(rename="coolDownPeriodSec")]
    
    pub cool_down_period_sec: Option<i32>,
    /// Exactly one utilization policy should be provided. Configuration parameters of CPU based autoscaling policy.
    #[serde(rename="cpuUtilization")]
    
    pub cpu_utilization: Option<AutoscalingPolicyCpuUtilization>,
    /// Configuration parameters of autoscaling based on custom metric.
    #[serde(rename="customMetricUtilizations")]
    
    pub custom_metric_utilizations: Option<Vec<AutoscalingPolicyCustomMetricUtilization>>,
    /// Configuration parameters of autoscaling based on load balancer.
    #[serde(rename="loadBalancingUtilization")]
    
    pub load_balancing_utilization: Option<AutoscalingPolicyLoadBalancingUtilization>,
    /// The maximum number of replicas that the Autoscaler can scale up to.
    #[serde(rename="maxNumReplicas")]
    
    pub max_num_replicas: Option<i32>,
    /// The minimum number of replicas that the Autoscaler can scale down to.
    #[serde(rename="minNumReplicas")]
    
    pub min_num_replicas: Option<i32>,
}

impl client::Part for AutoscalingPolicy {}


/// CPU utilization policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingPolicyCpuUtilization {
    /// The target utilization that the Autoscaler should maintain. It is represented as a fraction of used cores. For example: 6 cores used in 8-core VM are represented here as 0.75. Must be a float value between (0, 1]. If not defined, the default is 0.8.
    #[serde(rename="utilizationTarget")]
    
    pub utilization_target: Option<f64>,
}

impl client::Part for AutoscalingPolicyCpuUtilization {}


/// Custom utilization metric policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingPolicyCustomMetricUtilization {
    /// Identifier of the metric. It should be a Cloud Monitoring metric. The metric can not have negative values. The metric should be an utilization metric (increasing number of VMs handling requests x times should reduce average value of the metric roughly x times). For example you could use: compute.googleapis.com/instance/network/received_bytes_count.
    
    pub metric: Option<String>,
    /// Target value of the metric which Autoscaler should maintain. Must be a positive value.
    #[serde(rename="utilizationTarget")]
    
    pub utilization_target: Option<f64>,
    /// Defines type in which utilization_target is expressed.
    #[serde(rename="utilizationTargetType")]
    
    pub utilization_target_type: Option<String>,
}

impl client::Part for AutoscalingPolicyCustomMetricUtilization {}


/// Load balancing utilization policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingPolicyLoadBalancingUtilization {
    /// Fraction of backend capacity utilization (set in HTTP load balancing configuration) that Autoscaler should maintain. Must be a positive float value. If not defined, the default is 0.8. For example if your maxRatePerInstance capacity (in HTTP Load Balancing configuration) is set at 10 and you would like to keep number of instances such that each instance receives 7 QPS on average, set this to 0.7.
    #[serde(rename="utilizationTarget")]
    
    pub utilization_target: Option<f64>,
}

impl client::Part for AutoscalingPolicyLoadBalancingUtilization {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeprecationStatus {
    /// no description provided
    
    pub deleted: Option<String>,
    /// no description provided
    
    pub deprecated: Option<String>,
    /// no description provided
    
    pub obsolete: Option<String>,
    /// no description provided
    
    pub replacement: Option<String>,
    /// no description provided
    
    pub state: Option<String>,
}

impl client::Part for DeprecationStatus {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete autoscalers](AutoscalerDeleteCall) (response)
/// * [insert autoscalers](AutoscalerInsertCall) (response)
/// * [patch autoscalers](AutoscalerPatchCall) (response)
/// * [update autoscalers](AutoscalerUpdateCall) (response)
/// * [get zone operations](ZoneOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// no description provided
    #[serde(rename="clientOperationId")]
    
    pub client_operation_id: Option<String>,
    /// no description provided
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// no description provided
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// no description provided
    
    pub error: Option<OperationError>,
    /// no description provided
    #[serde(rename="httpErrorMessage")]
    
    pub http_error_message: Option<String>,
    /// no description provided
    #[serde(rename="httpErrorStatusCode")]
    
    pub http_error_status_code: Option<i32>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// no description provided
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// [Output Only] Type of the resource. Always compute#Operation for Operation resources.
    
    pub kind: Option<String>,
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// no description provided
    
    pub progress: Option<i32>,
    /// no description provided
    
    pub region: Option<String>,
    /// no description provided
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// no description provided
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// no description provided
    
    pub status: Option<String>,
    /// no description provided
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// no description provided
    #[serde(rename="targetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_id: Option<u64>,
    /// no description provided
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// no description provided
    
    pub user: Option<String>,
    /// no description provided
    
    pub warnings: Option<Vec<OperationWarnings>>,
    /// no description provided
    
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
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<Operation>>,
    /// [Output Only] Type of resource. Always compute#operations for Operations resource.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for OperationList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list zones](ZoneListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Zone {
    /// no description provided
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// no description provided
    
    pub deprecated: Option<DeprecationStatus>,
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] Type of the resource. Always kind#zone for zones.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="maintenanceWindows")]
    
    pub maintenance_windows: Option<Vec<ZoneMaintenanceWindows>>,
    /// no description provided
    
    pub name: Option<String>,
    /// no description provided
    
    pub region: Option<String>,
    /// [Output Only] Server defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// no description provided
    
    pub status: Option<String>,
}

impl client::Resource for Zone {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list zones](ZoneListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneList {
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<Zone>>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Server defined URL for this resource (output only).
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for ZoneList {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationError {
    /// no description provided
    
    pub errors: Option<Vec<OperationErrorErrors>>,
}

impl client::NestedType for OperationError {}
impl client::Part for OperationError {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationErrorErrors {
    /// no description provided
    
    pub code: Option<String>,
    /// no description provided
    
    pub location: Option<String>,
    /// no description provided
    
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
    /// no description provided
    
    pub code: Option<String>,
    /// no description provided
    
    pub data: Option<Vec<OperationWarningsData>>,
    /// no description provided
    
    pub message: Option<String>,
}

impl client::NestedType for OperationWarnings {}
impl client::Part for OperationWarnings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarningsData {
    /// no description provided
    
    pub key: Option<String>,
    /// no description provided
    
    pub value: Option<String>,
}

impl client::NestedType for OperationWarningsData {}
impl client::Part for OperationWarningsData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneMaintenanceWindows {
    /// no description provided
    #[serde(rename="beginTime")]
    
    pub begin_time: Option<String>,
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// no description provided
    
    pub name: Option<String>,
}

impl client::NestedType for ZoneMaintenanceWindows {}
impl client::Part for ZoneMaintenanceWindows {}


