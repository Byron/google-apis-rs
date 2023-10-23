use super::*;
/// An alert affecting a customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [feedback create alerts](AlertFeedbackCreateCall) (none)
/// * [feedback list alerts](AlertFeedbackListCall) (none)
/// * [batch delete alerts](AlertBatchDeleteCall) (none)
/// * [batch undelete alerts](AlertBatchUndeleteCall) (none)
/// * [delete alerts](AlertDeleteCall) (none)
/// * [get alerts](AlertGetCall) (response)
/// * [get metadata alerts](AlertGetMetadataCall) (none)
/// * [list alerts](AlertListCall) (none)
/// * [undelete alerts](AlertUndeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    /// Output only. The unique identifier for the alert.
    #[serde(rename="alertId")]
    
    pub alert_id: Option<String>,
    /// Output only. The time this alert was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The unique identifier of the Google Workspace account of the customer.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// Optional. The data associated with this alert, for example google.apps.alertcenter.type.DeviceCompromised.
    
    pub data: Option<HashMap<String, json::Value>>,
    /// Output only. `True` if this alert is marked for deletion.
    
    pub deleted: Option<bool>,
    /// Optional. The time the event that caused this alert ceased being active. If provided, the end time must not be earlier than the start time. If not provided, it indicates an ongoing alert.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of an alert from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform alert updates in order to avoid race conditions: An `etag` is returned in the response which contains alerts, and systems are expected to put that etag in the request to update alert to ensure that their change will be applied to the same version of the alert. If no `etag` is provided in the call to update alert, then the existing alert is overwritten blindly.
    
    pub etag: Option<String>,
    /// Output only. The metadata associated with this alert.
    
    pub metadata: Option<AlertMetadata>,
    /// Output only. An optional [Security Investigation Tool](https://support.google.com/a/answer/7575955) query for this alert.
    #[serde(rename="securityInvestigationToolLink")]
    
    pub security_investigation_tool_link: Option<String>,
    /// Required. A unique identifier for the system that reported the alert. This is output only after alert is created. Supported sources are any of the following: * Google Operations * Mobile device management * Gmail phishing * Data Loss Prevention * Domain wide takeout * State sponsored attack * Google identity * Apps outage
    
    pub source: Option<String>,
    /// Required. The time the event that caused this alert was started or detected.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The type of the alert. This is output only after alert is created. For a list of available alert types see [Google Workspace Alert types](https://developers.google.com/admin-sdk/alertcenter/reference/alert-types).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. The time this alert was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Alert {}
impl client::ResponseResult for Alert {}


/// A customer feedback about an alert.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [feedback create alerts](AlertFeedbackCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlertFeedback {
    /// Output only. The alert identifier.
    #[serde(rename="alertId")]
    
    pub alert_id: Option<String>,
    /// Output only. The time this feedback was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The unique identifier of the Google Workspace account of the customer.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// Output only. The email of the user that provided the feedback.
    
    pub email: Option<String>,
    /// Output only. The unique identifier for the feedback.
    #[serde(rename="feedbackId")]
    
    pub feedback_id: Option<String>,
    /// Required. The type of the feedback.
    #[serde(rename="type")]
    
    pub type_: Option<AlertFeedbackTypeEnum>,
}

impl client::RequestValue for AlertFeedback {}
impl client::ResponseResult for AlertFeedback {}


/// An alert metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get metadata alerts](AlertGetMetadataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlertMetadata {
    /// Output only. The alert identifier.
    #[serde(rename="alertId")]
    
    pub alert_id: Option<String>,
    /// The email address of the user assigned to the alert.
    
    pub assignee: Option<String>,
    /// Output only. The unique identifier of the Google Workspace account of the customer.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// Optional. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of an alert metadata from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform metadata updates in order to avoid race conditions: An `etag` is returned in the response which contains alert metadata, and systems are expected to put that etag in the request to update alert metadata to ensure that their change will be applied to the same version of the alert metadata. If no `etag` is provided in the call to update alert metadata, then the existing alert metadata is overwritten blindly.
    
    pub etag: Option<String>,
    /// The severity value of the alert. Alert Center will set this field at alert creation time, default's to an empty string when it could not be determined. The supported values for update actions on this field are the following: * HIGH * MEDIUM * LOW
    
    pub severity: Option<String>,
    /// The current status of the alert. The supported values are the following: * NOT_STARTED * IN_PROGRESS * CLOSED
    
    pub status: Option<String>,
    /// Output only. The time this metadata was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for AlertMetadata {}


/// A request to perform batch delete on alerts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch delete alerts](AlertBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteAlertsRequest {
    /// Required. The list of alert IDs to delete.
    #[serde(rename="alertId")]
    
    pub alert_id: Option<Vec<String>>,
    /// Optional. The unique identifier of the Google Workspace account of the customer the alerts are associated with. The `customer_id` must have the initial "C" stripped (for example, `046psxkn`). Inferred from the caller identity if not provided. [Find your customer ID](https://support.google.com/cloudidentity/answer/10070793).
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
}

impl client::RequestValue for BatchDeleteAlertsRequest {}


/// Response to batch delete operation on alerts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch delete alerts](AlertBatchDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteAlertsResponse {
    /// The status details for each failed `alert_id`.
    #[serde(rename="failedAlertStatus")]
    
    pub failed_alert_status: Option<HashMap<String, Status>>,
    /// The successful list of alert IDs.
    #[serde(rename="successAlertIds")]
    
    pub success_alert_ids: Option<Vec<String>>,
}

impl client::ResponseResult for BatchDeleteAlertsResponse {}


/// A request to perform batch undelete on alerts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch undelete alerts](AlertBatchUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUndeleteAlertsRequest {
    /// Required. The list of alert IDs to undelete.
    #[serde(rename="alertId")]
    
    pub alert_id: Option<Vec<String>>,
    /// Optional. The unique identifier of the Google Workspace account of the customer the alerts are associated with. The `customer_id` must have the initial "C" stripped (for example, `046psxkn`). Inferred from the caller identity if not provided. [Find your customer ID](https://support.google.com/cloudidentity/answer/10070793).
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
}

impl client::RequestValue for BatchUndeleteAlertsRequest {}


/// Response to batch undelete operation on alerts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch undelete alerts](AlertBatchUndeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUndeleteAlertsResponse {
    /// The status details for each failed `alert_id`.
    #[serde(rename="failedAlertStatus")]
    
    pub failed_alert_status: Option<HashMap<String, Status>>,
    /// The successful list of alert IDs.
    #[serde(rename="successAlertIds")]
    
    pub success_alert_ids: Option<Vec<String>>,
}

impl client::ResponseResult for BatchUndeleteAlertsResponse {}


/// A reference to a Cloud Pubsub topic. To register for notifications, the owner of the topic must grant `alerts-api-push-notifications@system.gserviceaccount.com` the `projects.topics.publish` permission.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudPubsubTopic {
    /// Optional. The format of the payload that would be sent. If not specified the format will be JSON.
    #[serde(rename="payloadFormat")]
    
    pub payload_format: Option<CloudPubsubTopicPayloadFormatEnum>,
    /// The `name` field of a Cloud Pubsub [Topic] (https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic).
    #[serde(rename="topicName")]
    
    pub topic_name: Option<String>,
}

impl client::Part for CloudPubsubTopic {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete alerts](AlertDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Response message for an alert feedback listing request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [feedback list alerts](AlertFeedbackListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAlertFeedbackResponse {
    /// The list of alert feedback. Feedback entries for each alert are ordered by creation time descending.
    
    pub feedback: Option<Vec<AlertFeedback>>,
}

impl client::ResponseResult for ListAlertFeedbackResponse {}


/// Response message for an alert listing request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list alerts](AlertListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAlertsResponse {
    /// The list of alerts.
    
    pub alerts: Option<Vec<Alert>>,
    /// The token for the next page. If not empty, indicates that there may be more alerts that match the listing request; this value can be used in a subsequent ListAlertsRequest to get alerts continuing from last result of the current list call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAlertsResponse {}


/// Settings for callback notifications. For more details see [Google Workspace Alert Notification](https://developers.google.com/admin-sdk/alertcenter/guides/notifications).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// A Google Cloud Pub/sub topic destination.
    #[serde(rename="cloudPubsubTopic")]
    
    pub cloud_pubsub_topic: Option<CloudPubsubTopic>,
}

impl client::Part for Notification {}


/// Customer-level settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get settings](MethodGetSettingCall) (response)
/// * [update settings](MethodUpdateSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    /// The list of notifications.
    
    pub notifications: Option<Vec<Notification>>,
}

impl client::RequestValue for Settings {}
impl client::ResponseResult for Settings {}


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


/// A request to undelete a specific alert that was marked for deletion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [undelete alerts](AlertUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteAlertRequest {
    /// Optional. The unique identifier of the Google Workspace account of the customer the alert is associated with. The `customer_id` must have the initial "C" stripped (for example, `046psxkn`). Inferred from the caller identity if not provided. [Find your customer ID](https://support.google.com/cloudidentity/answer/10070793).
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
}

impl client::RequestValue for UndeleteAlertRequest {}


