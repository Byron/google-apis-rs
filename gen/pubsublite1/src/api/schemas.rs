use super::*;
/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations operations cancel admin](AdminProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// The throughput capacity configuration for each partition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Capacity {
    /// Publish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16.
    #[serde(rename="publishMibPerSec")]
    
    pub publish_mib_per_sec: Option<i32>,
    /// Subscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 32.
    #[serde(rename="subscribeMibPerSec")]
    
    pub subscribe_mib_per_sec: Option<i32>,
}

impl client::Part for Capacity {}


/// Request for CommitCursor.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations subscriptions commit cursor cursor](CursorProjectLocationSubscriptionCommitCursorCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitCursorRequest {
    /// The new value for the committed cursor.
    
    pub cursor: Option<Cursor>,
    /// The partition for which to update the cursor. Partitions are zero indexed, so `partition` must be in the range [0, topic.num_partitions).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition: Option<i64>,
}

impl client::RequestValue for CommitCursorRequest {}


/// Response for CommitCursor.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations subscriptions commit cursor cursor](CursorProjectLocationSubscriptionCommitCursorCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitCursorResponse { _never_set: Option<bool> }

impl client::ResponseResult for CommitCursorResponse {}


/// Compute the current head cursor for a partition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics compute head cursor topic stats](TopicStatProjectLocationTopicComputeHeadCursorCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComputeHeadCursorRequest {
    /// Required. The partition for which we should compute the head cursor.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition: Option<i64>,
}

impl client::RequestValue for ComputeHeadCursorRequest {}


/// Response containing the head cursor for the requested topic and partition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics compute head cursor topic stats](TopicStatProjectLocationTopicComputeHeadCursorCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComputeHeadCursorResponse {
    /// The head cursor.
    #[serde(rename="headCursor")]
    
    pub head_cursor: Option<Cursor>,
}

impl client::ResponseResult for ComputeHeadCursorResponse {}


/// Compute statistics about a range of messages in a given topic and partition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics compute message stats topic stats](TopicStatProjectLocationTopicComputeMessageStatCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComputeMessageStatsRequest {
    /// The exclusive end of the range. The range is empty if end_cursor <= start_cursor. Specifying a start_cursor before the first message and an end_cursor after the last message will retrieve all messages.
    #[serde(rename="endCursor")]
    
    pub end_cursor: Option<Cursor>,
    /// Required. The partition for which we should compute message stats.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition: Option<i64>,
    /// The inclusive start of the range.
    #[serde(rename="startCursor")]
    
    pub start_cursor: Option<Cursor>,
}

impl client::RequestValue for ComputeMessageStatsRequest {}


/// Response containing stats for messages in the requested topic and partition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics compute message stats topic stats](TopicStatProjectLocationTopicComputeMessageStatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComputeMessageStatsResponse {
    /// The number of quota bytes accounted to these messages.
    #[serde(rename="messageBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub message_bytes: Option<i64>,
    /// The count of messages.
    #[serde(rename="messageCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub message_count: Option<i64>,
    /// The minimum event timestamp across these messages. For the purposes of this computation, if a message does not have an event time, we use the publish time. The timestamp will be unset if there are no messages.
    #[serde(rename="minimumEventTime")]
    
    pub minimum_event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The minimum publish timestamp across these messages. Note that publish timestamps within a partition are not guaranteed to be non-decreasing. The timestamp will be unset if there are no messages.
    #[serde(rename="minimumPublishTime")]
    
    pub minimum_publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ComputeMessageStatsResponse {}


/// Compute the corresponding cursor for a publish or event time in a topic partition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics compute time cursor topic stats](TopicStatProjectLocationTopicComputeTimeCursorCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComputeTimeCursorRequest {
    /// Required. The partition for which we should compute the cursor.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition: Option<i64>,
    /// Required. The target publish or event time. Specifying a future time will return an unset cursor.
    
    pub target: Option<TimeTarget>,
}

impl client::RequestValue for ComputeTimeCursorRequest {}


/// Response containing the cursor corresponding to a publish or event time in a topic partition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics compute time cursor topic stats](TopicStatProjectLocationTopicComputeTimeCursorCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComputeTimeCursorResponse {
    /// If present, the cursor references the first message with time greater than or equal to the specified target time. If such a message cannot be found, the cursor will be unset (i.e. `cursor` is not present).
    
    pub cursor: Option<Cursor>,
}

impl client::ResponseResult for ComputeTimeCursorResponse {}


/// A cursor that describes the position of a message within a topic partition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cursor {
    /// The offset of a message within a topic partition. Must be greater than or equal 0.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub offset: Option<i64>,
}

impl client::Part for Cursor {}


/// The settings for a subscription's message delivery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryConfig {
    /// The DeliveryRequirement for this subscription.
    #[serde(rename="deliveryRequirement")]
    
    pub delivery_requirement: Option<String>,
}

impl client::Part for DeliveryConfig {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations operations cancel admin](AdminProjectLocationOperationCancelCall) (response)
/// * [projects locations operations delete admin](AdminProjectLocationOperationDeleteCall) (response)
/// * [projects locations reservations delete admin](AdminProjectLocationReservationDeleteCall) (response)
/// * [projects locations subscriptions delete admin](AdminProjectLocationSubscriptionDeleteCall) (response)
/// * [projects locations topics delete admin](AdminProjectLocationTopicDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Configuration for a Pub/Sub Lite subscription that writes messages to a destination. User subscriber clients must not connect to this subscription.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Output only. The current state of the export, which may be different to the desired state due to errors. This field is output only.
    #[serde(rename="currentState")]
    
    pub current_state: Option<String>,
    /// Optional. The name of an optional Pub/Sub Lite topic to publish messages that can not be exported to the destination. For example, the message can not be published to the Pub/Sub service because it does not satisfy the constraints documented at https://cloud.google.com/pubsub/docs/publisher. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}. Must be within the same project and location as the subscription. The topic may be changed or removed.
    #[serde(rename="deadLetterTopic")]
    
    pub dead_letter_topic: Option<String>,
    /// The desired state of this export. Setting this to values other than `ACTIVE` and `PAUSED` will result in an error.
    #[serde(rename="desiredState")]
    
    pub desired_state: Option<String>,
    /// Messages are automatically written from the Pub/Sub Lite topic associated with this subscription to a Pub/Sub topic.
    #[serde(rename="pubsubConfig")]
    
    pub pubsub_config: Option<PubSubConfig>,
}

impl client::Part for ExportConfig {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations operations list admin](AdminProjectLocationOperationListCall) (response)
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


/// Response for ListPartitionCursors
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations subscriptions cursors list cursor](CursorProjectLocationSubscriptionCursorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPartitionCursorsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The partition cursors from this request.
    #[serde(rename="partitionCursors")]
    
    pub partition_cursors: Option<Vec<PartitionCursor>>,
}

impl client::ResponseResult for ListPartitionCursorsResponse {}


/// Response for ListReservationTopics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations reservations topics list admin](AdminProjectLocationReservationTopicListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReservationTopicsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The names of topics attached to the reservation. The order of the topics is unspecified.
    
    pub topics: Option<Vec<String>>,
}

impl client::ResponseResult for ListReservationTopicsResponse {}


/// Response for ListReservations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations reservations list admin](AdminProjectLocationReservationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReservationsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of reservation in the requested parent. The order of the reservations is unspecified.
    
    pub reservations: Option<Vec<Reservation>>,
}

impl client::ResponseResult for ListReservationsResponse {}


/// Response for ListSubscriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations subscriptions list admin](AdminProjectLocationSubscriptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSubscriptionsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of subscriptions in the requested parent. The order of the subscriptions is unspecified.
    
    pub subscriptions: Option<Vec<Subscription>>,
}

impl client::ResponseResult for ListSubscriptionsResponse {}


/// Response for ListTopicSubscriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics subscriptions list admin](AdminProjectLocationTopicSubscriptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTopicSubscriptionsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The names of subscriptions attached to the topic. The order of the subscriptions is unspecified.
    
    pub subscriptions: Option<Vec<String>>,
}

impl client::ResponseResult for ListTopicSubscriptionsResponse {}


/// Response for ListTopics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics list admin](AdminProjectLocationTopicListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTopicsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of topic in the requested parent. The order of the topics is unspecified.
    
    pub topics: Option<Vec<Topic>>,
}

impl client::ResponseResult for ListTopicsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations operations get admin](AdminProjectLocationOperationGetCall) (response)
/// * [projects locations subscriptions seek admin](AdminProjectLocationSubscriptionSeekCall) (response)
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


/// The settings for a topic's partitions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionConfig {
    /// The capacity configuration.
    
    pub capacity: Option<Capacity>,
    /// The number of partitions in the topic. Must be at least 1. Once a topic has been created the number of partitions can be increased but not decreased. Message ordering is not guaranteed across a topic resize. For more information see https://cloud.google.com/pubsub/lite/docs/topics#scaling_capacity
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// DEPRECATED: Use capacity instead which can express a superset of configurations. Every partition in the topic is allocated throughput equivalent to `scale` times the standard partition throughput (4 MiB/s). This is also reflected in the cost of this topic; a topic with `scale` of 2 and count of 10 is charged for 20 partitions. This value must be in the range [1,4].
    
    pub scale: Option<i32>,
}

impl client::Part for PartitionConfig {}


/// A pair of a Cursor and the partition it is for.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionCursor {
    /// The value of the cursor.
    
    pub cursor: Option<Cursor>,
    /// The partition this is for.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition: Option<i64>,
}

impl client::Part for PartitionCursor {}


/// Configuration for exporting to a Pub/Sub topic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PubSubConfig {
    /// The name of the Pub/Sub topic. Structured like: projects/{project_number}/topics/{topic_id}. The topic may be changed.
    
    pub topic: Option<String>,
}

impl client::Part for PubSubConfig {}


/// Metadata about a reservation resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations reservations create admin](AdminProjectLocationReservationCreateCall) (request|response)
/// * [projects locations reservations get admin](AdminProjectLocationReservationGetCall) (response)
/// * [projects locations reservations patch admin](AdminProjectLocationReservationPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Reservation {
    /// The name of the reservation. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id}
    
    pub name: Option<String>,
    /// The reserved throughput capacity. Every unit of throughput capacity is equivalent to 1 MiB/s of published messages or 2 MiB/s of subscribed messages. Any topics which are declared as using capacity from a Reservation will consume resources from this reservation instead of being charged individually.
    #[serde(rename="throughputCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub throughput_capacity: Option<i64>,
}

impl client::RequestValue for Reservation {}
impl client::ResponseResult for Reservation {}


/// The settings for this topic's Reservation usage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReservationConfig {
    /// The Reservation to use for this topic's throughput capacity. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id}
    #[serde(rename="throughputReservation")]
    
    pub throughput_reservation: Option<String>,
}

impl client::Part for ReservationConfig {}


/// The settings for a topic's message retention.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RetentionConfig {
    /// The provisioned storage, in bytes, per partition. If the number of bytes stored in any of the topic's partitions grows beyond this value, older messages will be dropped to make room for newer ones, regardless of the value of `period`.
    #[serde(rename="perPartitionBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub per_partition_bytes: Option<i64>,
    /// How long a published message is retained. If unset, messages will be retained as long as the bytes retained for each partition is below `per_partition_bytes`.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub period: Option<client::chrono::Duration>,
}

impl client::Part for RetentionConfig {}


/// Request for SeekSubscription.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations subscriptions seek admin](AdminProjectLocationSubscriptionSeekCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeekSubscriptionRequest {
    /// Seek to a named position with respect to the message backlog.
    #[serde(rename="namedTarget")]
    
    pub named_target: Option<String>,
    /// Seek to the first message whose publish or event time is greater than or equal to the specified query time. If no such message can be located, will seek to the end of the message backlog.
    #[serde(rename="timeTarget")]
    
    pub time_target: Option<TimeTarget>,
}

impl client::RequestValue for SeekSubscriptionRequest {}


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


/// Metadata about a subscription resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations subscriptions create admin](AdminProjectLocationSubscriptionCreateCall) (request|response)
/// * [projects locations subscriptions get admin](AdminProjectLocationSubscriptionGetCall) (response)
/// * [projects locations subscriptions patch admin](AdminProjectLocationSubscriptionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// The settings for this subscription's message delivery.
    #[serde(rename="deliveryConfig")]
    
    pub delivery_config: Option<DeliveryConfig>,
    /// If present, messages are automatically written from the Pub/Sub Lite topic associated with this subscription to a destination.
    #[serde(rename="exportConfig")]
    
    pub export_config: Option<ExportConfig>,
    /// The name of the subscription. Structured like: projects/{project_number}/locations/{location}/subscriptions/{subscription_id}
    
    pub name: Option<String>,
    /// The name of the topic this subscription is attached to. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}
    
    pub topic: Option<String>,
}

impl client::RequestValue for Subscription {}
impl client::ResponseResult for Subscription {}


/// A target publish or event time. Can be used for seeking to or retrieving the corresponding cursor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeTarget {
    /// Request the cursor of the first message with event time greater than or equal to `event_time`. If messages are missing an event time, the publish time is used as a fallback. As event times are user supplied, subsequent messages may have event times less than `event_time` and should be filtered by the client, if necessary.
    #[serde(rename="eventTime")]
    
    pub event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Request the cursor of the first message with publish time greater than or equal to `publish_time`. All messages thereafter are guaranteed to have publish times >= `publish_time`.
    #[serde(rename="publishTime")]
    
    pub publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeTarget {}


/// Metadata about a topic resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics create admin](AdminProjectLocationTopicCreateCall) (request|response)
/// * [projects locations topics get admin](AdminProjectLocationTopicGetCall) (response)
/// * [projects locations topics patch admin](AdminProjectLocationTopicPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    /// The name of the topic. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}
    
    pub name: Option<String>,
    /// The settings for this topic's partitions.
    #[serde(rename="partitionConfig")]
    
    pub partition_config: Option<PartitionConfig>,
    /// The settings for this topic's Reservation usage.
    #[serde(rename="reservationConfig")]
    
    pub reservation_config: Option<ReservationConfig>,
    /// The settings for this topic's message retention.
    #[serde(rename="retentionConfig")]
    
    pub retention_config: Option<RetentionConfig>,
}

impl client::RequestValue for Topic {}
impl client::ResponseResult for Topic {}


/// Response for GetTopicPartitions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects locations topics get partitions admin](AdminProjectLocationTopicGetPartitionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TopicPartitions {
    /// The number of partitions in the topic.
    #[serde(rename="partitionCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition_count: Option<i64>,
}

impl client::ResponseResult for TopicPartitions {}


