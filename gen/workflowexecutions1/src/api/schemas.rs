use super::*;
/// Request for the CancelExecution method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workflows executions cancel projects](ProjectLocationWorkflowExecutionCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelExecutionRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelExecutionRequest {}


/// Error describes why the execution was abnormally terminated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    /// Human-readable stack trace string.
    
    pub context: Option<String>,
    /// Error message and data returned represented as a JSON string.
    
    pub payload: Option<String>,
    /// Stack trace with detailed information of where error was generated.
    #[serde(rename="stackTrace")]
    
    pub stack_trace: Option<StackTrace>,
}

impl client::Part for Error {}


/// A running instance of a [Workflow](https://cloud.google.com/workflows/docs/reference/rest/v1/projects.locations.workflows).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workflows executions cancel projects](ProjectLocationWorkflowExecutionCancelCall) (response)
/// * [locations workflows executions create projects](ProjectLocationWorkflowExecutionCreateCall) (request|response)
/// * [locations workflows executions get projects](ProjectLocationWorkflowExecutionGetCall) (response)
/// * [locations workflows trigger pubsub execution projects](ProjectLocationWorkflowTriggerPubsubExecutionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Execution {
    /// Input parameters of the execution represented as a JSON string. The size limit is 32KB. *Note*: If you are using the REST API directly to run your workflow, you must escape any JSON string value of `argument`. Example: `'{"argument":"{\"firstName\":\"FIRST\",\"lastName\":\"LAST\"}"}'`
    
    pub argument: Option<String>,
    /// The call logging level associated to this execution.
    #[serde(rename="callLogLevel")]
    
    pub call_log_level: Option<ExecutionCallLogLevelEnum>,
    /// Output only. Measures the duration of the execution.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Output only. Marks the end of execution, successful or not.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The error which caused the execution to finish prematurely. The value is only present if the execution's state is `FAILED` or `CANCELLED`.
    
    pub error: Option<Error>,
    /// Output only. The resource name of the execution. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    
    pub name: Option<String>,
    /// Output only. Output of the execution represented as a JSON string. The value can only be present if the execution's state is `SUCCEEDED`.
    
    pub result: Option<String>,
    /// Output only. Marks the beginning of execution.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Current state of the execution.
    
    pub state: Option<ExecutionStateEnum>,
    /// Output only. Status tracks the current steps and progress data of this execution.
    
    pub status: Option<Status>,
    /// Output only. Revision of the workflow this execution is using.
    #[serde(rename="workflowRevisionId")]
    
    pub workflow_revision_id: Option<String>,
}

impl client::RequestValue for Execution {}
impl client::ResponseResult for Execution {}


/// Response for the ListExecutions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workflows executions list projects](ProjectLocationWorkflowExecutionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListExecutionsResponse {
    /// The executions which match the request.
    
    pub executions: Option<Vec<Execution>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListExecutionsResponse {}


/// Position contains source position information about the stack trace element such as line number, column number and length of the code block in bytes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    /// The source code column position (of the line) the current instruction was generated from.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub column: Option<i64>,
    /// The number of bytes of source code making up this stack trace element.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// The source code line number the current instruction was generated from.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line: Option<i64>,
}

impl client::Part for Position {}


/// A message that is published by publishers and consumed by subscribers. The message must contain either a non-empty data field or at least one attribute. Note that client libraries represent this object differently depending on the language. See the corresponding [client library documentation](https://cloud.google.com/pubsub/docs/reference/libraries) for more information. See [quotas and limits] (https://cloud.google.com/pubsub/quotas) for more information about message limits.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PubsubMessage {
    /// Attributes for this message. If this field is empty, the message must contain non-empty data. This can be used to filter messages on the subscription.
    
    pub attributes: Option<HashMap<String, String>>,
    /// The message data field. If this field is empty, the message must contain at least one attribute.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// ID of this message, assigned by the server when the message is published. Guaranteed to be unique within the topic. This value may be read by a subscriber that receives a `PubsubMessage` via a `Pull` call or a push delivery. It must not be populated by the publisher in a `Publish` call.
    #[serde(rename="messageId")]
    
    pub message_id: Option<String>,
    /// If non-empty, identifies related messages for which publish order should be respected. If a `Subscription` has `enable_message_ordering` set to `true`, messages published with the same non-empty `ordering_key` value will be delivered to subscribers in the order in which they are received by the Pub/Sub system. All `PubsubMessage`s published in a given `PublishRequest` must specify the same `ordering_key` value. For more information, see [ordering messages](https://cloud.google.com/pubsub/docs/ordering).
    #[serde(rename="orderingKey")]
    
    pub ordering_key: Option<String>,
    /// The time at which the message was published, populated by the server when it receives the `Publish` call. It must not be populated by the publisher in a `Publish` call.
    #[serde(rename="publishTime")]
    
    pub publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PubsubMessage {}


/// A collection of stack elements (frames) where an error occurred.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackTrace {
    /// An array of stack elements.
    
    pub elements: Option<Vec<StackTraceElement>>,
}

impl client::Part for StackTrace {}


/// A single stack element (frame) where an error occurred.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackTraceElement {
    /// The source position information of the stack trace element.
    
    pub position: Option<Position>,
    /// The routine where the error occurred.
    
    pub routine: Option<String>,
    /// The step the error occurred at.
    
    pub step: Option<String>,
}

impl client::Part for StackTraceElement {}


/// Represents the current status of this execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// A list of currently executing or last executed step names for the workflow execution currently running. If the workflow has succeeded or failed, this is the last attempted or executed step. Presently, if the current step is inside a subworkflow, the list only includes that step. In the future, the list will contain items for each step in the call stack, starting with the outermost step in the `main` subworkflow, and ending with the most deeply nested step.
    #[serde(rename="currentSteps")]
    
    pub current_steps: Option<Vec<Step>>,
}

impl client::Part for Status {}


/// Represents a step of the workflow this execution is running.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Step {
    /// Name of a routine within the workflow.
    
    pub routine: Option<String>,
    /// Name of a step within the routine.
    
    pub step: Option<String>,
}

impl client::Part for Step {}


/// Request for the TriggerPubsubExecution method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workflows trigger pubsub execution projects](ProjectLocationWorkflowTriggerPubsubExecutionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TriggerPubsubExecutionRequest {
    /// Required. LINT: LEGACY_NAMES The query parameter value for __GCP_CloudEventsMode, set by the Eventarc service when configuring triggers.
    #[serde(rename="GCPCloudEventsMode")]
    
    pub gcp_cloud_events_mode: Option<String>,
    /// Required. The message of the Pub/Sub push notification.
    
    pub message: Option<PubsubMessage>,
    /// Required. The subscription of the Pub/Sub push notification. Format: projects/{project}/subscriptions/{sub}
    
    pub subscription: Option<String>,
}

impl client::RequestValue for TriggerPubsubExecutionRequest {}


