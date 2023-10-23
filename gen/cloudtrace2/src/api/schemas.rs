use super::*;
/// Text annotation with a set of attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Annotation {
    /// A set of attributes on the annotation. You can have up to 4 attributes per Annotation.
    
    pub attributes: Option<Attributes>,
    /// A user-supplied message describing the event. The maximum length for the description is 256 bytes.
    
    pub description: Option<TruncatableString>,
}

impl client::Part for Annotation {}


/// The allowed types for `[VALUE]` in a `[KEY]:[VALUE]` attribute.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeValue {
    /// A Boolean value represented by `true` or `false`.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// A 64-bit signed integer.
    #[serde(rename="intValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int_value: Option<i64>,
    /// A string up to 256 bytes long.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<TruncatableString>,
}

impl client::Part for AttributeValue {}


/// A set of attributes as key-value pairs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    /// A set of attributes. Each attribute's key can be up to 128 bytes long. The value can be a string up to 256 bytes, a signed 64-bit integer, or the boolean values `true` or `false`. For example: "/instance_id": { "string_value": { "value": "my-instance" } } "/http/request_bytes": { "int_value": 300 } "abc.com/myattribute": { "bool_value": false }
    #[serde(rename="attributeMap")]
    
    pub attribute_map: Option<HashMap<String, AttributeValue>>,
    /// The number of attributes that were discarded. Attributes can be discarded because their keys are too long or because there are too many attributes. If this value is 0 then all attributes are valid.
    #[serde(rename="droppedAttributesCount")]
    
    pub dropped_attributes_count: Option<i32>,
}

impl client::Part for Attributes {}


/// The request message for the `BatchWriteSpans` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traces batch write projects](ProjectTraceBatchWriteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchWriteSpansRequest {
    /// Required. A list of new spans. The span names must not match existing spans, otherwise the results are undefined.
    
    pub spans: Option<Vec<Span>>,
}

impl client::RequestValue for BatchWriteSpansRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traces batch write projects](ProjectTraceBatchWriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A pointer from the current span to another span in the same trace or in a different trace. For example, this can be used in batching operations, where a single batch handler processes multiple requests from different traces or when the handler receives a request from a different project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// A set of attributes on the link. Up to 32 attributes can be specified per link.
    
    pub attributes: Option<Attributes>,
    /// The `[SPAN_ID]` for a span within a trace.
    #[serde(rename="spanId")]
    
    pub span_id: Option<String>,
    /// The `[TRACE_ID]` for a trace within a project.
    #[serde(rename="traceId")]
    
    pub trace_id: Option<String>,
    /// The relationship of the current span relative to the linked span.
    #[serde(rename="type")]
    
    pub type_: Option<LinkTypeEnum>,
}

impl client::Part for Link {}


/// A collection of links, which are references from this span to a span in the same or different trace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Links {
    /// The number of dropped links after the maximum size was enforced. If this value is 0, then no links were dropped.
    #[serde(rename="droppedLinksCount")]
    
    pub dropped_links_count: Option<i32>,
    /// A collection of links.
    
    pub link: Option<Vec<Link>>,
}

impl client::Part for Links {}


/// An event describing a message sent/received between Spans.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MessageEvent {
    /// The number of compressed bytes sent or received. If missing, the compressed size is assumed to be the same size as the uncompressed size.
    #[serde(rename="compressedSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub compressed_size_bytes: Option<i64>,
    /// An identifier for the MessageEvent's message that can be used to match `SENT` and `RECEIVED` MessageEvents.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Type of MessageEvent. Indicates whether the message was sent or received.
    #[serde(rename="type")]
    
    pub type_: Option<MessageEventTypeEnum>,
    /// The number of uncompressed bytes sent or received.
    #[serde(rename="uncompressedSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub uncompressed_size_bytes: Option<i64>,
}

impl client::Part for MessageEvent {}


/// Binary module.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    /// A unique identifier for the module, usually a hash of its contents (up to 128 bytes).
    #[serde(rename="buildId")]
    
    pub build_id: Option<TruncatableString>,
    /// For example: main binary, kernel modules, and dynamic libraries such as libc.so, sharedlib.so (up to 256 bytes).
    
    pub module: Option<TruncatableString>,
}

impl client::Part for Module {}


/// A span represents a single operation within a trace. Spans can be nested to form a trace tree. Often, a trace contains a root span that describes the end-to-end latency, and one or more subspans for its sub-operations. A trace can also contain multiple root spans, or none at all. Spans do not need to be contiguous—there might be gaps or overlaps between spans in a trace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traces spans create span projects](ProjectTraceSpanCreateSpanCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Span {
    /// A set of attributes on the span. You can have up to 32 attributes per span.
    
    pub attributes: Option<Attributes>,
    /// Optional. The number of child spans that were generated while this span was active. If set, allows implementation to detect missing child spans.
    #[serde(rename="childSpanCount")]
    
    pub child_span_count: Option<i32>,
    /// Required. A description of the span's operation (up to 128 bytes). Cloud Trace displays the description in the Cloud console. For example, the display name can be a qualified method name or a file name and a line number where the operation is called. A best practice is to use the same display name within an application and at the same call point. This makes it easier to correlate spans in different traces.
    #[serde(rename="displayName")]
    
    pub display_name: Option<TruncatableString>,
    /// Required. The end time of the span. On the client side, this is the time kept by the local machine where the span execution ends. On the server side, this is the time when the server application handler stops running.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Links associated with the span. You can have up to 128 links per Span.
    
    pub links: Option<Links>,
    /// Required. The resource name of the span in the following format: * `projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/[SPAN_ID]` `[TRACE_ID]` is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. It should not be zero. `[SPAN_ID]` is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array. It should not be zero. .
    
    pub name: Option<String>,
    /// The `[SPAN_ID]` of this span's parent span. If this is a root span, then this field must be empty.
    #[serde(rename="parentSpanId")]
    
    pub parent_span_id: Option<String>,
    /// Optional. Set this parameter to indicate whether this span is in the same process as its parent. If you do not set this parameter, Trace is unable to take advantage of this helpful information.
    #[serde(rename="sameProcessAsParentSpan")]
    
    pub same_process_as_parent_span: Option<bool>,
    /// Required. The `[SPAN_ID]` portion of the span's resource name.
    #[serde(rename="spanId")]
    
    pub span_id: Option<String>,
    /// Optional. Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call.
    #[serde(rename="spanKind")]
    
    pub span_kind: Option<SpanSpanKindEnum>,
    /// Stack trace captured at the start of the span.
    #[serde(rename="stackTrace")]
    
    pub stack_trace: Option<StackTrace>,
    /// Required. The start time of the span. On the client side, this is the time kept by the local machine where the span execution starts. On the server side, this is the time when the server's application handler starts running.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The final status for this span.
    
    pub status: Option<Status>,
    /// A set of time events. You can have up to 32 annotations and 128 message events per span.
    #[serde(rename="timeEvents")]
    
    pub time_events: Option<TimeEvents>,
}

impl client::RequestValue for Span {}
impl client::ResponseResult for Span {}


/// Represents a single stack frame in a stack trace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackFrame {
    /// The column number where the function call appears, if available. This is important in JavaScript because of its anonymous functions.
    #[serde(rename="columnNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub column_number: Option<i64>,
    /// The name of the source file where the function call appears (up to 256 bytes).
    #[serde(rename="fileName")]
    
    pub file_name: Option<TruncatableString>,
    /// The fully-qualified name that uniquely identifies the function or method that is active in this frame (up to 1024 bytes).
    #[serde(rename="functionName")]
    
    pub function_name: Option<TruncatableString>,
    /// The line number in `file_name` where the function call appears.
    #[serde(rename="lineNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line_number: Option<i64>,
    /// The binary module from where the code was loaded.
    #[serde(rename="loadModule")]
    
    pub load_module: Option<Module>,
    /// An un-mangled function name, if `function_name` is [mangled](http://www.avabodh.com/cxxin/namemangling.html). The name can be fully-qualified (up to 1024 bytes).
    #[serde(rename="originalFunctionName")]
    
    pub original_function_name: Option<TruncatableString>,
    /// The version of the deployed source code (up to 128 bytes).
    #[serde(rename="sourceVersion")]
    
    pub source_version: Option<TruncatableString>,
}

impl client::Part for StackFrame {}


/// A collection of stack frames, which can be truncated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackFrames {
    /// The number of stack frames that were dropped because there were too many stack frames. If this value is 0, then no stack frames were dropped.
    #[serde(rename="droppedFramesCount")]
    
    pub dropped_frames_count: Option<i32>,
    /// Stack frames in this call stack.
    
    pub frame: Option<Vec<StackFrame>>,
}

impl client::Part for StackFrames {}


/// A call stack appearing in a trace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackTrace {
    /// Stack frames in this stack trace. A maximum of 128 frames are allowed.
    #[serde(rename="stackFrames")]
    
    pub stack_frames: Option<StackFrames>,
    /// The hash ID is used to conserve network bandwidth for duplicate stack traces within a single trace. Often multiple spans will have identical stack traces. The first occurrence of a stack trace should contain both the `stackFrame` content and a value in `stackTraceHashId`. Subsequent spans within the same request can refer to that stack trace by only setting `stackTraceHashId`.
    #[serde(rename="stackTraceHashId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub stack_trace_hash_id: Option<i64>,
}

impl client::Part for StackTrace {}


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


/// A time-stamped annotation or message event in the Span.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeEvent {
    /// Text annotation with a set of attributes.
    
    pub annotation: Option<Annotation>,
    /// An event describing a message sent/received between Spans.
    #[serde(rename="messageEvent")]
    
    pub message_event: Option<MessageEvent>,
    /// The timestamp indicating the time the event occurred.
    
    pub time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeEvent {}


/// A collection of `TimeEvent`s. A `TimeEvent` is a time-stamped annotation on the span, consisting of either user-supplied key:value pairs, or details of a message sent/received between Spans.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeEvents {
    /// The number of dropped annotations in all the included time events. If the value is 0, then no annotations were dropped.
    #[serde(rename="droppedAnnotationsCount")]
    
    pub dropped_annotations_count: Option<i32>,
    /// The number of dropped message events in all the included time events. If the value is 0, then no message events were dropped.
    #[serde(rename="droppedMessageEventsCount")]
    
    pub dropped_message_events_count: Option<i32>,
    /// A collection of `TimeEvent`s.
    #[serde(rename="timeEvent")]
    
    pub time_event: Option<Vec<TimeEvent>>,
}

impl client::Part for TimeEvents {}


/// Represents a string that might be shortened to a specified length.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TruncatableString {
    /// The number of bytes removed from the original string. If this value is 0, then the string was not shortened.
    #[serde(rename="truncatedByteCount")]
    
    pub truncated_byte_count: Option<i32>,
    /// The shortened string. For example, if the original string is 500 bytes long and the limit of the string is 128 bytes, then `value` contains the first 128 bytes of the 500-byte string. Truncation always happens on a UTF8 character boundary. If there are multi-byte characters in the string, then the length of the shortened string might be less than the size limit.
    
    pub value: Option<String>,
}

impl client::Part for TruncatableString {}


