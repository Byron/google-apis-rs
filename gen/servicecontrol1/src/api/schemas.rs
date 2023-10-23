use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateInfo {
    /// A list of label keys that were unused by the server in processing the request. Thus, for similar requests repeated in a certain future time window, the caller can choose to ignore these labels in the requests to achieve better client-side cache hits and quota aggregation for rate quota. This field is not populated for allocation quota checks.
    #[serde(rename="unusedArguments")]
    
    pub unused_arguments: Option<Vec<String>>,
}

impl client::Part for AllocateInfo {}


/// Request message for the AllocateQuota method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate quota services](ServiceAllocateQuotaCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateQuotaRequest {
    /// Operation that describes the quota allocation.
    #[serde(rename="allocateOperation")]
    
    pub allocate_operation: Option<QuotaOperation>,
    /// Specifies which version of service configuration should be used to process the request. If unspecified or no matching version can be found, the latest one will be used.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
}

impl client::RequestValue for AllocateQuotaRequest {}


/// Response message for the AllocateQuota method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate quota services](ServiceAllocateQuotaCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateQuotaResponse {
    /// Indicates the decision of the allocate.
    #[serde(rename="allocateErrors")]
    
    pub allocate_errors: Option<Vec<QuotaError>>,
    /// WARNING: DO NOT use this field until this warning message is removed.
    #[serde(rename="allocateInfo")]
    
    pub allocate_info: Option<AllocateInfo>,
    /// The same operation_id value used in the AllocateQuotaRequest. Used for logging and diagnostics purposes.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Quota metrics to indicate the result of allocation. Depending on the request, one or more of the following metrics will be included: 1. Per quota group or per quota metric incremental usage will be specified using the following delta metric : "serviceruntime.googleapis.com/api/consumer/quota_used_count" 2. The quota limit reached condition will be specified using the following boolean metric : "serviceruntime.googleapis.com/quota/exceeded"
    #[serde(rename="quotaMetrics")]
    
    pub quota_metrics: Option<Vec<MetricValueSet>>,
    /// ID of the actual config used to process the request.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
}

impl client::ResponseResult for AllocateQuotaResponse {}


/// The allowed types for [VALUE] in a `[KEY]:[VALUE]` attribute.
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


/// A set of attributes, each in the format `[KEY]:[VALUE]`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    /// The set of attributes. Each attribute's key can be up to 128 bytes long. The value can be a string up to 256 bytes, a signed 64-bit integer, or the Boolean values `true` and `false`. For example: "/instance_id": "my-instance" "/http/user_agent": "" "/http/request_bytes": 300 "abc.com/myattribute": true
    #[serde(rename="attributeMap")]
    
    pub attribute_map: Option<HashMap<String, AttributeValue>>,
    /// The number of attributes that were discarded. Attributes can be discarded because their keys are too long or because there are too many attributes. If this value is 0 then all attributes are valid.
    #[serde(rename="droppedAttributesCount")]
    
    pub dropped_attributes_count: Option<i32>,
}

impl client::Part for Attributes {}


/// Defines the errors to be returned in google.api.servicecontrol.v1.CheckResponse.check_errors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckError {
    /// The error code.
    
    pub code: Option<CheckErrorCodeEnum>,
    /// Free-form text providing details on the error cause of the error.
    
    pub detail: Option<String>,
    /// Contains public information about the check error. If available, `status.code` will be non zero and client can propagate it out as public error.
    
    pub status: Option<Status>,
    /// Subject to whom this error applies. See the specific code enum for more details on this field. For example: - "project:" - "folder:" - "organization:"
    
    pub subject: Option<String>,
}

impl client::Part for CheckError {}


/// Contains additional information about the check operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckInfo {
    /// Consumer info of this check.
    #[serde(rename="consumerInfo")]
    
    pub consumer_info: Option<ConsumerInfo>,
    /// A list of fields and label keys that are ignored by the server. The client doesn't need to send them for following requests to improve performance and allow better aggregation.
    #[serde(rename="unusedArguments")]
    
    pub unused_arguments: Option<Vec<String>>,
}

impl client::Part for CheckInfo {}


/// Request message for the Check method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check services](ServiceCheckCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckRequest {
    /// The operation to be checked.
    
    pub operation: Option<Operation>,
    /// Requests the project settings to be returned as part of the check response.
    #[serde(rename="requestProjectSettings")]
    
    pub request_project_settings: Option<bool>,
    /// Specifies which version of service configuration should be used to process the request. If unspecified or no matching version can be found, the latest one will be used.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
    /// Indicates if service activation check should be skipped for this request. Default behavior is to perform the check and apply relevant quota. WARNING: Setting this flag to "true" will disable quota enforcement.
    #[serde(rename="skipActivationCheck")]
    
    pub skip_activation_check: Option<bool>,
}

impl client::RequestValue for CheckRequest {}


/// Response message for the Check method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check services](ServiceCheckCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckResponse {
    /// Indicate the decision of the check. If no check errors are present, the service should process the operation. Otherwise the service should use the list of errors to determine the appropriate action.
    #[serde(rename="checkErrors")]
    
    pub check_errors: Option<Vec<CheckError>>,
    /// Feedback data returned from the server during processing a Check request.
    #[serde(rename="checkInfo")]
    
    pub check_info: Option<CheckInfo>,
    /// The same operation_id value used in the CheckRequest. Used for logging and diagnostics purposes.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Quota information for the check request associated with this response. 
    #[serde(rename="quotaInfo")]
    
    pub quota_info: Option<QuotaInfo>,
    /// The actual config id used to process the request.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
    /// The current service rollout id used to process the request.
    #[serde(rename="serviceRolloutId")]
    
    pub service_rollout_id: Option<String>,
}

impl client::ResponseResult for CheckResponse {}


/// `ConsumerInfo` provides information about the consumer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsumerInfo {
    /// The consumer identity number, can be Google cloud project number, folder number or organization number e.g. 1234567890. A value of 0 indicates no consumer number is found.
    #[serde(rename="consumerNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub consumer_number: Option<i64>,
    /// The Google cloud project number, e.g. 1234567890. A value of 0 indicates no project number is found. NOTE: This field is deprecated after Chemist support flexible consumer id. New code should not depend on this field anymore.
    #[serde(rename="projectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub project_number: Option<i64>,
    /// The type of the consumer which should have been defined in [Google Resource Manager](https://cloud.google.com/resource-manager/).
    #[serde(rename="type")]
    
    pub type_: Option<ConsumerInfoTypeEnum>,
}

impl client::Part for ConsumerInfo {}


/// Distribution represents a frequency distribution of double-valued sample points. It contains the size of the population of sample points plus additional optional information: * the arithmetic mean of the samples * the minimum and maximum of the samples * the sum-squared-deviation of the samples, used to compute variance * a histogram of the values of the sample points
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Distribution {
    /// The number of samples in each histogram bucket. `bucket_counts` are optional. If present, they must sum to the `count` value. The buckets are defined below in `bucket_option`. There are N buckets. `bucket_counts[0]` is the number of samples in the underflow bucket. `bucket_counts[1]` to `bucket_counts[N-1]` are the numbers of samples in each of the finite buckets. And `bucket_counts[N] is the number of samples in the overflow bucket. See the comments of `bucket_option` below for more details. Any suffix of trailing zeros may be omitted.
    #[serde(rename="bucketCounts")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub bucket_counts: Option<Vec<i64>>,
    /// The total number of samples in the distribution. Must be >= 0.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Example points. Must be in increasing order of `value` field.
    
    pub exemplars: Option<Vec<Exemplar>>,
    /// Buckets with arbitrary user-provided width.
    #[serde(rename="explicitBuckets")]
    
    pub explicit_buckets: Option<ExplicitBuckets>,
    /// Buckets with exponentially growing width.
    #[serde(rename="exponentialBuckets")]
    
    pub exponential_buckets: Option<ExponentialBuckets>,
    /// Buckets with constant width.
    #[serde(rename="linearBuckets")]
    
    pub linear_buckets: Option<LinearBuckets>,
    /// The maximum of the population of values. Ignored if `count` is zero.
    
    pub maximum: Option<f64>,
    /// The arithmetic mean of the samples in the distribution. If `count` is zero then this field must be zero.
    
    pub mean: Option<f64>,
    /// The minimum of the population of values. Ignored if `count` is zero.
    
    pub minimum: Option<f64>,
    /// The sum of squared deviations from the mean: Sum\[i=1..count\]((x_i - mean)^2) where each x_i is a sample values. If `count` is zero then this field must be zero, otherwise validation of the request fails.
    #[serde(rename="sumOfSquaredDeviation")]
    
    pub sum_of_squared_deviation: Option<f64>,
}

impl client::Part for Distribution {}


/// Exemplars are example points that may be used to annotate aggregated distribution values. They are metadata that gives information about a particular value added to a Distribution bucket, such as a trace ID that was active when a value was added. They may contain further information, such as a example values and timestamps, origin, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Exemplar {
    /// Contextual information about the example value. Examples are: Trace: type.googleapis.com/google.monitoring.v3.SpanContext Literal string: type.googleapis.com/google.protobuf.StringValue Labels dropped during aggregation: type.googleapis.com/google.monitoring.v3.DroppedLabels There may be only a single attachment of any given message type in a single exemplar, and this is enforced by the system.
    
    pub attachments: Option<Vec<HashMap<String, json::Value>>>,
    /// The observation (sampling) time of the above value.
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Value of the exemplar point. This value determines to which bucket the exemplar belongs.
    
    pub value: Option<f64>,
}

impl client::Part for Exemplar {}


/// Describing buckets with arbitrary user-provided width.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExplicitBuckets {
    /// 'bound' is a list of strictly increasing boundaries between buckets. Note that a list of length N-1 defines N buckets because of fenceposting. See comments on `bucket_options` for details. The i'th finite bucket covers the interval [bound[i-1], bound[i]) where i ranges from 1 to bound_size() - 1. Note that there are no finite buckets at all if 'bound' only contains a single element; in that special case the single bound defines the boundary between the underflow and overflow buckets. bucket number lower bound upper bound i == 0 (underflow) -inf bound[i] 0 < i < bound_size() bound[i-1] bound[i] i == bound_size() (overflow) bound[i-1] +inf
    
    pub bounds: Option<Vec<f64>>,
}

impl client::Part for ExplicitBuckets {}


/// Describing buckets with exponentially growing width.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExponentialBuckets {
    /// The i'th exponential bucket covers the interval [scale * growth_factor^(i-1), scale * growth_factor^i) where i ranges from 1 to num_finite_buckets inclusive. Must be larger than 1.0.
    #[serde(rename="growthFactor")]
    
    pub growth_factor: Option<f64>,
    /// The number of finite buckets. With the underflow and overflow buckets, the total number of buckets is `num_finite_buckets` + 2. See comments on `bucket_options` for details.
    #[serde(rename="numFiniteBuckets")]
    
    pub num_finite_buckets: Option<i32>,
    /// The i'th exponential bucket covers the interval [scale * growth_factor^(i-1), scale * growth_factor^i) where i ranges from 1 to num_finite_buckets inclusive. Must be > 0.
    
    pub scale: Option<f64>,
}

impl client::Part for ExponentialBuckets {}


/// A common proto for logging HTTP requests. Only contains semantics defined by the HTTP specification. Product-specific logging information MUST be defined in a separate message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    /// The number of HTTP response bytes inserted into cache. Set only when a cache fill was attempted.
    #[serde(rename="cacheFillBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cache_fill_bytes: Option<i64>,
    /// Whether or not an entity was served from cache (with or without validation).
    #[serde(rename="cacheHit")]
    
    pub cache_hit: Option<bool>,
    /// Whether or not a cache lookup was attempted.
    #[serde(rename="cacheLookup")]
    
    pub cache_lookup: Option<bool>,
    /// Whether or not the response was validated with the origin server before being served from cache. This field is only meaningful if `cache_hit` is True.
    #[serde(rename="cacheValidatedWithOriginServer")]
    
    pub cache_validated_with_origin_server: Option<bool>,
    /// The request processing latency on the server, from the time the request was received until the response was sent.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub latency: Option<client::chrono::Duration>,
    /// Protocol used for the request. Examples: "HTTP/1.1", "HTTP/2", "websocket"
    
    pub protocol: Option<String>,
    /// The referer URL of the request, as defined in [HTTP/1.1 Header Field Definitions](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html).
    
    pub referer: Option<String>,
    /// The IP address (IPv4 or IPv6) of the client that issued the HTTP request. Examples: `"192.168.1.1"`, `"FE80::0202:B3FF:FE1E:8329"`.
    #[serde(rename="remoteIp")]
    
    pub remote_ip: Option<String>,
    /// The request method. Examples: `"GET"`, `"HEAD"`, `"PUT"`, `"POST"`.
    #[serde(rename="requestMethod")]
    
    pub request_method: Option<String>,
    /// The size of the HTTP request message in bytes, including the request headers and the request body.
    #[serde(rename="requestSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_size: Option<i64>,
    /// The scheme (http, https), the host name, the path, and the query portion of the URL that was requested. Example: `"http://example.com/some/info?color=red"`.
    #[serde(rename="requestUrl")]
    
    pub request_url: Option<String>,
    /// The size of the HTTP response message sent back to the client, in bytes, including the response headers and the response body.
    #[serde(rename="responseSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub response_size: Option<i64>,
    /// The IP address (IPv4 or IPv6) of the origin server that the request was sent to.
    #[serde(rename="serverIp")]
    
    pub server_ip: Option<String>,
    /// The response code indicating the status of the response. Examples: 200, 404.
    
    pub status: Option<i32>,
    /// The user agent sent by the client. Example: `"Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET CLR 1.0.3705)"`.
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
}

impl client::Part for HttpRequest {}


/// Describing buckets with constant width.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinearBuckets {
    /// The number of finite buckets. With the underflow and overflow buckets, the total number of buckets is `num_finite_buckets` + 2. See comments on `bucket_options` for details.
    #[serde(rename="numFiniteBuckets")]
    
    pub num_finite_buckets: Option<i32>,
    /// The i'th linear bucket covers the interval [offset + (i-1) * width, offset + i * width) where i ranges from 1 to num_finite_buckets, inclusive.
    
    pub offset: Option<f64>,
    /// The i'th linear bucket covers the interval [offset + (i-1) * width, offset + i * width) where i ranges from 1 to num_finite_buckets, inclusive. Must be strictly positive.
    
    pub width: Option<f64>,
}

impl client::Part for LinearBuckets {}


/// An individual log entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// Optional. Information about the HTTP request associated with this log entry, if applicable.
    #[serde(rename="httpRequest")]
    
    pub http_request: Option<HttpRequest>,
    /// A unique ID for the log entry used for deduplication. If omitted, the implementation will generate one based on operation_id.
    #[serde(rename="insertId")]
    
    pub insert_id: Option<String>,
    /// A set of user-defined (key, value) data that provides additional information about the log entry.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The log to which this log entry belongs. Examples: `"syslog"`, `"book_log"`.
    
    pub name: Option<String>,
    /// Optional. Information about an operation associated with the log entry, if applicable.
    
    pub operation: Option<LogEntryOperation>,
    /// The log entry payload, represented as a protocol buffer that is expressed as a JSON object. The only accepted type currently is AuditLog.
    #[serde(rename="protoPayload")]
    
    pub proto_payload: Option<HashMap<String, json::Value>>,
    /// The severity of the log entry. The default value is `LogSeverity.DEFAULT`.
    
    pub severity: Option<LogEntrySeverityEnum>,
    /// Optional. Source code location information associated with the log entry, if any.
    #[serde(rename="sourceLocation")]
    
    pub source_location: Option<LogEntrySourceLocation>,
    /// The log entry payload, represented as a structure that is expressed as a JSON object.
    #[serde(rename="structPayload")]
    
    pub struct_payload: Option<HashMap<String, json::Value>>,
    /// The log entry payload, represented as a Unicode string (UTF-8).
    #[serde(rename="textPayload")]
    
    pub text_payload: Option<String>,
    /// The time the event described by the log entry occurred. If omitted, defaults to operation start time.
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Resource name of the trace associated with the log entry, if any. If this field contains a relative resource name, you can assume the name is relative to `//tracing.googleapis.com`. Example: `projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824`
    
    pub trace: Option<String>,
}

impl client::Part for LogEntry {}


/// Additional information about a potentially long-running operation with which a log entry is associated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntryOperation {
    /// Optional. Set this to True if this is the first log entry in the operation.
    
    pub first: Option<bool>,
    /// Optional. An arbitrary operation identifier. Log entries with the same identifier are assumed to be part of the same operation.
    
    pub id: Option<String>,
    /// Optional. Set this to True if this is the last log entry in the operation.
    
    pub last: Option<bool>,
    /// Optional. An arbitrary producer identifier. The combination of `id` and `producer` must be globally unique. Examples for `producer`: `"MyDivision.MyBigCompany.com"`, `"github.com/MyProject/MyApplication"`.
    
    pub producer: Option<String>,
}

impl client::Part for LogEntryOperation {}


/// Additional information about the source code location that produced the log entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntrySourceLocation {
    /// Optional. Source file name. Depending on the runtime environment, this might be a simple name or a fully-qualified name.
    
    pub file: Option<String>,
    /// Optional. Human-readable name of the function or method being invoked, with optional context such as the class or package name. This information may be used in contexts such as the logs viewer, where a file and line number are less meaningful. The format can vary by language. For example: `qual.if.ied.Class.method` (Java), `dir/package.func` (Go), `function` (Python).
    
    pub function: Option<String>,
    /// Optional. Line within the source file. 1-based; 0 indicates no line number available.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line: Option<i64>,
}

impl client::Part for LogEntrySourceLocation {}


/// Represents a single metric value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    /// A boolean value.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// A distribution value.
    #[serde(rename="distributionValue")]
    
    pub distribution_value: Option<Distribution>,
    /// A double precision floating point value.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// The end of the time period over which this metric value's measurement applies. If not specified, google.api.servicecontrol.v1.Operation.end_time will be used.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A signed 64-bit integer value.
    #[serde(rename="int64Value")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int64_value: Option<i64>,
    /// The labels describing the metric value. See comments on google.api.servicecontrol.v1.Operation.labels for the overriding relationship. Note that this map must not contain monitored resource labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// A money value.
    #[serde(rename="moneyValue")]
    
    pub money_value: Option<Money>,
    /// The start of the time period over which this metric value's measurement applies. The time period has different semantics for different metric types (cumulative, delta, and gauge). See the metric definition documentation in the service configuration for details. If not specified, google.api.servicecontrol.v1.Operation.start_time will be used.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A text string value.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for MetricValue {}


/// Represents a set of metric values in the same metric. Each metric value in the set should have a unique combination of start time, end time, and label values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValueSet {
    /// The metric name defined in the service configuration.
    #[serde(rename="metricName")]
    
    pub metric_name: Option<String>,
    /// The values in this metric.
    #[serde(rename="metricValues")]
    
    pub metric_values: Option<Vec<MetricValue>>,
}

impl client::Part for MetricValueSet {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Money {}


/// Represents information regarding an operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// Identity of the consumer who is using the service. This field should be filled in for the operations initiated by a consumer, but not for service-initiated operations that are not related to a specific consumer. - This can be in one of the following formats: - project:PROJECT_ID, - project`_`number:PROJECT_NUMBER, - projects/PROJECT_ID or PROJECT_NUMBER, - folders/FOLDER_NUMBER, - organizations/ORGANIZATION_NUMBER, - api`_`key:API_KEY.
    #[serde(rename="consumerId")]
    
    pub consumer_id: Option<String>,
    /// End time of the operation. Required when the operation is used in ServiceController.Report, but optional when the operation is used in ServiceController.Check.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// DO NOT USE. This is an experimental field.
    
    pub importance: Option<OperationImportanceEnum>,
    /// Labels describing the operation. Only the following labels are allowed: - Labels describing monitored resources as defined in the service configuration. - Default labels of metric values. When specified, labels defined in the metric value override these default. - The following labels defined by Google Cloud Platform: - `cloud.googleapis.com/location` describing the location where the operation happened, - `servicecontrol.googleapis.com/user_agent` describing the user agent of the API request, - `servicecontrol.googleapis.com/service_agent` describing the service used to handle the API request (e.g. ESP), - `servicecontrol.googleapis.com/platform` describing the platform where the API is served, such as App Engine, Compute Engine, or Kubernetes Engine.
    
    pub labels: Option<HashMap<String, String>>,
    /// Represents information to be logged.
    #[serde(rename="logEntries")]
    
    pub log_entries: Option<Vec<LogEntry>>,
    /// Represents information about this operation. Each MetricValueSet corresponds to a metric defined in the service configuration. The data type used in the MetricValueSet must agree with the data type specified in the metric definition. Within a single operation, it is not allowed to have more than one MetricValue instances that have the same metric names and identical label value combinations. If a request has such duplicated MetricValue instances, the entire request is rejected with an invalid argument error.
    #[serde(rename="metricValueSets")]
    
    pub metric_value_sets: Option<Vec<MetricValueSet>>,
    /// Identity of the operation. This must be unique within the scope of the service that generated the operation. If the service calls Check() and Report() on the same operation, the two calls should carry the same id. UUID version 4 is recommended, though not required. In scenarios where an operation is computed from existing information and an idempotent id is desirable for deduplication purpose, UUID version 5 is recommended. See RFC 4122 for details.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Fully qualified name of the operation. Reserved for future use.
    #[serde(rename="operationName")]
    
    pub operation_name: Option<String>,
    /// Represents the properties needed for quota check. Applicable only if this operation is for a quota check request. If this is not specified, no quota check will be performed.
    #[serde(rename="quotaProperties")]
    
    pub quota_properties: Option<QuotaProperties>,
    /// The resources that are involved in the operation. The maximum supported number of entries in this field is 100.
    
    pub resources: Option<Vec<ResourceInfo>>,
    /// Required. Start time of the operation.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Unimplemented. A list of Cloud Trace spans. The span names shall contain the id of the destination project which can be either the produce or the consumer project.
    #[serde(rename="traceSpans")]
    
    pub trace_spans: Option<Vec<TraceSpan>>,
    /// Private Preview. This feature is only available for approved services. User defined labels for the resource that this operation is associated with.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::Part for Operation {}


/// Represents error information for QuotaOperation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaError {
    /// Error code.
    
    pub code: Option<QuotaErrorCodeEnum>,
    /// Free-form text that provides details on the cause of the error.
    
    pub description: Option<String>,
    /// Contains additional information about the quota error. If available, `status.code` will be non zero.
    
    pub status: Option<Status>,
    /// Subject to whom this error applies. See the specific enum for more details on this field. For example, "clientip:" or "project:".
    
    pub subject: Option<String>,
}

impl client::Part for QuotaError {}


/// Contains the quota information for a quota check response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaInfo {
    /// Quota Metrics that have exceeded quota limits. For QuotaGroup-based quota, this is QuotaGroup.name For QuotaLimit-based quota, this is QuotaLimit.name See: google.api.Quota Deprecated: Use quota_metrics to get per quota group limit exceeded status.
    #[serde(rename="limitExceeded")]
    
    pub limit_exceeded: Option<Vec<String>>,
    /// Map of quota group name to the actual number of tokens consumed. If the quota check was not successful, then this will not be populated due to no quota consumption. We are not merging this field with 'quota_metrics' field because of the complexity of scaling in Chemist client code base. For simplicity, we will keep this field for Castor (that scales quota usage) and 'quota_metrics' for SuperQuota (that doesn't scale quota usage). 
    #[serde(rename="quotaConsumed")]
    
    pub quota_consumed: Option<HashMap<String, i32>>,
    /// Quota metrics to indicate the usage. Depending on the check request, one or more of the following metrics will be included: 1. For rate quota, per quota group or per quota metric incremental usage will be specified using the following delta metric: "serviceruntime.googleapis.com/api/consumer/quota_used_count" 2. For allocation quota, per quota metric total usage will be specified using the following gauge metric: "serviceruntime.googleapis.com/allocation/consumer/quota_used_count" 3. For both rate quota and allocation quota, the quota limit reached condition will be specified using the following boolean metric: "serviceruntime.googleapis.com/quota/exceeded"
    #[serde(rename="quotaMetrics")]
    
    pub quota_metrics: Option<Vec<MetricValueSet>>,
}

impl client::Part for QuotaInfo {}


/// Represents information regarding a quota operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaOperation {
    /// Identity of the consumer for whom this quota operation is being performed. This can be in one of the following formats: project:, project_number:, api_key:.
    #[serde(rename="consumerId")]
    
    pub consumer_id: Option<String>,
    /// Labels describing the operation.
    
    pub labels: Option<HashMap<String, String>>,
    /// Fully qualified name of the API method for which this quota operation is requested. This name is used for matching quota rules or metric rules and billing status rules defined in service configuration. This field should not be set if any of the following is true: (1) the quota operation is performed on non-API resources. (2) quota_metrics is set because the caller is doing quota override. Example of an RPC method name: google.example.library.v1.LibraryService.CreateShelf
    #[serde(rename="methodName")]
    
    pub method_name: Option<String>,
    /// Identity of the operation. For Allocation Quota, this is expected to be unique within the scope of the service that generated the operation, and guarantees idempotency in case of retries. In order to ensure best performance and latency in the Quota backends, operation_ids are optimally associated with time, so that related operations can be accessed fast in storage. For this reason, the recommended token for services that intend to operate at a high QPS is Unix time in nanos + UUID
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Represents information about this operation. Each MetricValueSet corresponds to a metric defined in the service configuration. The data type used in the MetricValueSet must agree with the data type specified in the metric definition. Within a single operation, it is not allowed to have more than one MetricValue instances that have the same metric names and identical label value combinations. If a request has such duplicated MetricValue instances, the entire request is rejected with an invalid argument error. This field is mutually exclusive with method_name.
    #[serde(rename="quotaMetrics")]
    
    pub quota_metrics: Option<Vec<MetricValueSet>>,
    /// Quota mode for this operation.
    #[serde(rename="quotaMode")]
    
    pub quota_mode: Option<QuotaOperationQuotaModeEnum>,
}

impl client::Part for QuotaOperation {}


/// Represents the properties needed for quota operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaProperties {
    /// Quota mode for this operation.
    #[serde(rename="quotaMode")]
    
    pub quota_mode: Option<QuotaPropertyQuotaModeEnum>,
}

impl client::Part for QuotaProperties {}


/// Represents the processing error of one Operation in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportError {
    /// The Operation.operation_id value from the request.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Details of the error when processing the Operation.
    
    pub status: Option<Status>,
}

impl client::Part for ReportError {}


/// Request message for the Report method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report services](ServiceReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    /// Operations to be reported. Typically the service should report one operation per request. Putting multiple operations into a single request is allowed, but should be used only when multiple operations are natually available at the time of the report. There is no limit on the number of operations in the same ReportRequest, however the ReportRequest size should be no larger than 1MB. See ReportResponse.report_errors for partial failure behavior.
    
    pub operations: Option<Vec<Operation>>,
    /// Specifies which version of service config should be used to process the request. If unspecified or no matching version can be found, the latest one will be used.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
}

impl client::RequestValue for ReportRequest {}


/// Response message for the Report method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report services](ServiceReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportResponse {
    /// Partial failures, one for each `Operation` in the request that failed processing. There are three possible combinations of the RPC status: 1. The combination of a successful RPC status and an empty `report_errors` list indicates a complete success where all `Operations` in the request are processed successfully. 2. The combination of a successful RPC status and a non-empty `report_errors` list indicates a partial success where some `Operations` in the request succeeded. Each `Operation` that failed processing has a corresponding item in this list. 3. A failed RPC status indicates a general non-deterministic failure. When this happens, it's impossible to know which of the 'Operations' in the request succeeded or failed.
    #[serde(rename="reportErrors")]
    
    pub report_errors: Option<Vec<ReportError>>,
    /// The actual config id used to process the request.
    #[serde(rename="serviceConfigId")]
    
    pub service_config_id: Option<String>,
    /// The current service rollout id used to process the request.
    #[serde(rename="serviceRolloutId")]
    
    pub service_rollout_id: Option<String>,
}

impl client::ResponseResult for ReportResponse {}


/// Describes a resource associated with this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceInfo {
    /// The resource permission required for this request.
    
    pub permission: Option<String>,
    /// The identifier of the parent of this resource instance. Must be in one of the following formats: - `projects/` - `folders/` - `organizations/`
    #[serde(rename="resourceContainer")]
    
    pub resource_container: Option<String>,
    /// The location of the resource. If not empty, the resource will be checked against location policy. The value must be a valid zone, region or multiregion. For example: "europe-west4" or "northamerica-northeast1-a"
    #[serde(rename="resourceLocation")]
    
    pub resource_location: Option<String>,
    /// Name of the resource. This is used for auditing purposes.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::Part for ResourceInfo {}


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


/// A span represents a single operation within a trace. Spans can be nested to form a trace tree. Often, a trace contains a root span that describes the end-to-end latency, and one or more subspans for its sub-operations. A trace can also contain multiple root spans, or none at all. Spans do not need to be contiguous—there may be gaps or overlaps between spans in a trace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TraceSpan {
    /// A set of attributes on the span. You can have up to 32 attributes per span.
    
    pub attributes: Option<Attributes>,
    /// An optional number of child spans that were generated while this span was active. If set, allows implementation to detect missing child spans.
    #[serde(rename="childSpanCount")]
    
    pub child_span_count: Option<i32>,
    /// A description of the span's operation (up to 128 bytes). Stackdriver Trace displays the description in the Google Cloud Platform Console. For example, the display name can be a qualified method name or a file name and a line number where the operation is called. A best practice is to use the same display name within an application and at the same call point. This makes it easier to correlate spans in different traces.
    #[serde(rename="displayName")]
    
    pub display_name: Option<TruncatableString>,
    /// The end time of the span. On the client side, this is the time kept by the local machine where the span execution ends. On the server side, this is the time when the server application handler stops running.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The resource name of the span in the following format: projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/SPAN_ID is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. [SPAN_ID] is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array.
    
    pub name: Option<String>,
    /// The [SPAN_ID] of this span's parent span. If this is a root span, then this field must be empty.
    #[serde(rename="parentSpanId")]
    
    pub parent_span_id: Option<String>,
    /// (Optional) Set this parameter to indicate whether this span is in the same process as its parent. If you do not set this parameter, Stackdriver Trace is unable to take advantage of this helpful information.
    #[serde(rename="sameProcessAsParentSpan")]
    
    pub same_process_as_parent_span: Option<bool>,
    /// The [SPAN_ID] portion of the span's resource name.
    #[serde(rename="spanId")]
    
    pub span_id: Option<String>,
    /// Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call.
    #[serde(rename="spanKind")]
    
    pub span_kind: Option<TraceSpanSpanKindEnum>,
    /// The start time of the span. On the client side, this is the time kept by the local machine where the span execution starts. On the server side, this is the time when the server's application handler starts running.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An optional final status for this span.
    
    pub status: Option<Status>,
}

impl client::Part for TraceSpan {}


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


