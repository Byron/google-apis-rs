use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [patch traces projects](ProjectPatchTraceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The response message for the `ListTraces` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traces list projects](ProjectTraceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTracesResponse {
    /// If defined, indicates that there are more traces that match the request and that this value should be passed to the next request to continue retrieving additional traces.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of trace records as specified by the view parameter.
    
    pub traces: Option<Vec<Trace>>,
}

impl client::ResponseResult for ListTracesResponse {}


/// A trace describes how long it takes for an application to perform an operation. It consists of a set of spans, each of which represent a single timed event within the operation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traces get projects](ProjectTraceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trace {
    /// Project ID of the Cloud project where the trace data is stored.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Collection of spans in the trace.
    
    pub spans: Option<Vec<TraceSpan>>,
    /// Globally unique identifier for the trace. This identifier is a 128-bit numeric value formatted as a 32-byte hex string. For example, `382d4f4c6b7bb2f4a972559d9085001d`. The numeric value should not be zero.
    #[serde(rename="traceId")]
    
    pub trace_id: Option<String>,
}

impl client::ResponseResult for Trace {}


/// A span represents a single timed event within a trace. Spans can be nested and form a trace tree. Often, a trace contains a root span that describes the end-to-end latency of an operation and, optionally, one or more subspans for its suboperations. Spans do not need to be contiguous. There may be gaps between spans in a trace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TraceSpan {
    /// End time of the span in nanoseconds from the UNIX epoch.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `RPC_CLIENT` and `RPC_SERVER` to identify queueing latency associated with the span.
    
    pub kind: Option<String>,
    /// Collection of labels associated with the span. Label keys must be less than 128 bytes. Label values must be less than 16 kilobytes (10MB for `/stacktrace` values). Some predefined label keys exist, or you may create your own. When creating your own, we recommend the following formats: * `/category/product/key` for agents of well-known products (e.g. `/db/mongodb/read_size`). * `short_host/path/key` for domain-specific keys (e.g. `foo.com/myproduct/bar`) Predefined labels include: * `/agent` * `/component` * `/error/message` * `/error/name` * `/http/client_city` * `/http/client_country` * `/http/client_protocol` * `/http/client_region` * `/http/host` * `/http/method` * `/http/path` * `/http/redirected_url` * `/http/request/size` * `/http/response/size` * `/http/route` * `/http/status_code` * `/http/url` * `/http/user_agent` * `/pid` * `/stacktrace` * `/tid`
    
    pub labels: Option<HashMap<String, String>>,
    /// Name of the span. Must be less than 128 bytes. The span name is sanitized and displayed in the Trace tool in the Google Cloud Platform Console. The name may be a method name or some other per-call site name. For the same executable and the same call point, a best practice is to use a consistent name, which makes it easier to correlate cross-trace spans.
    
    pub name: Option<String>,
    /// Optional. ID of the parent span, if any.
    #[serde(rename="parentSpanId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parent_span_id: Option<u64>,
    /// Identifier for the span. Must be a 64-bit integer other than 0 and unique within a trace. For example, `2205310701640571284`.
    #[serde(rename="spanId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub span_id: Option<u64>,
    /// Start time of the span in nanoseconds from the UNIX epoch.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TraceSpan {}


/// List of new or updated traces.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [patch traces projects](ProjectPatchTraceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Traces {
    /// List of traces.
    
    pub traces: Option<Vec<Trace>>,
}

impl client::RequestValue for Traces {}


