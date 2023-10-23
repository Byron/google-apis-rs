use super::*;
/// BucketOptions describes the bucket boundaries used to create a histogram for the distribution. The buckets can be in a linear sequence, an exponential sequence, or each bucket can be specified explicitly. BucketOptions does not include the number of values in each bucket.A bucket has an inclusive lower bound and exclusive upper bound for the values that are counted for that bucket. The upper bound of a bucket must be strictly greater than the lower bound. The sequence of N buckets for a distribution consists of an underflow bucket (number 0), zero or more finite buckets (number 1 through N - 2) and an overflow bucket (number N - 1). The buckets are contiguous: the lower bound of bucket i (i > 0) is the same as the upper bound of bucket i - 1. The buckets span the whole range of finite values: lower bound of the underflow bucket is -infinity and the upper bound of the overflow bucket is +infinity. The finite buckets are so-called because both bounds are finite.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketOptions {
    /// The explicit buckets.
    #[serde(rename="explicitBuckets")]
    
    pub explicit_buckets: Option<Explicit>,
    /// The exponential buckets.
    #[serde(rename="exponentialBuckets")]
    
    pub exponential_buckets: Option<Exponential>,
    /// The linear bucket.
    #[serde(rename="linearBuckets")]
    
    pub linear_buckets: Option<Linear>,
}

impl client::Part for BucketOptions {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance:
/// service Foo {
/// rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// The JSON representation for Empty is empty JSON object {}.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metrics delete projects](ProjectMetricDeleteCall) (response)
/// * [sinks delete projects](ProjectSinkDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Specifies a set of buckets with arbitrary widths.There are size(bounds) + 1 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): boundsi  Lower bound (1 <= i < N); boundsi - 1The bounds field must contain at least one element. If bounds has only one element, then there are no finite buckets, and that single element is the common boundary of the overflow and underflow buckets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Explicit {
    /// The values must be monotonically increasing.
    
    pub bounds: Option<Vec<f64>>,
}

impl client::Part for Explicit {}


/// Specifies an exponential sequence of buckets that have a width that is proportional to the value of the lower bound. Each bucket represents a constant relative uncertainty on a specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): scale * (growth_factor ^ i).  Lower bound (1 <= i < N): scale * (growth_factor ^ (i - 1)).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Exponential {
    /// Must be greater than 1.
    #[serde(rename="growthFactor")]
    
    pub growth_factor: Option<f64>,
    /// Must be greater than 0.
    #[serde(rename="numFiniteBuckets")]
    
    pub num_finite_buckets: Option<i32>,
    /// Must be greater than 0.
    
    pub scale: Option<f64>,
}

impl client::Part for Exponential {}


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
    /// Whether or not the response was validated with the origin server before being served from cache. This field is only meaningful if cache_hit is True.
    #[serde(rename="cacheValidatedWithOriginServer")]
    
    pub cache_validated_with_origin_server: Option<bool>,
    /// The request processing latency on the server, from the time the request was received until the response was sent.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub latency: Option<client::chrono::Duration>,
    /// Protocol used for the request. Examples: "HTTP/1.1", "HTTP/2", "websocket"
    
    pub protocol: Option<String>,
    /// The referer URL of the request, as defined in HTTP/1.1 Header Field Definitions (http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html).
    
    pub referer: Option<String>,
    /// The IP address (IPv4 or IPv6) of the client that issued the HTTP request. Examples: "192.168.1.1", "FE80::0202:B3FF:FE1E:8329".
    #[serde(rename="remoteIp")]
    
    pub remote_ip: Option<String>,
    /// The request method. Examples: "GET", "HEAD", "PUT", "POST".
    #[serde(rename="requestMethod")]
    
    pub request_method: Option<String>,
    /// The size of the HTTP request message in bytes, including the request headers and the request body.
    #[serde(rename="requestSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_size: Option<i64>,
    /// The scheme (http, https), the host name, the path and the query portion of the URL that was requested. Example: "http://example.com/some/info?color=red".
    #[serde(rename="requestUrl")]
    
    pub request_url: Option<String>,
    /// The size of the HTTP response message sent back to the client, in bytes, including the response headers and the response body.
    #[serde(rename="responseSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub response_size: Option<i64>,
    /// The IP address (IPv4 or IPv6) of the origin server that the request was sent to.
    #[serde(rename="serverIp")]
    
    pub server_ip: Option<String>,
    /// The response code indicating the status of response. Examples: 200, 404.
    
    pub status: Option<i32>,
    /// The user agent sent by the client. Example: "Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET
    /// CLR 1.0.3705)".
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
}

impl client::Part for HttpRequest {}


/// A description of a label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelDescriptor {
    /// A human-readable description for the label.
    
    pub description: Option<String>,
    /// The label key.
    
    pub key: Option<String>,
    /// The type of data that can be assigned to the label.
    #[serde(rename="valueType")]
    
    pub value_type: Option<LabelDescriptorValueTypeEnum>,
}

impl client::Part for LabelDescriptor {}


/// Specifies a linear sequence of buckets that all have the same width (except overflow and underflow). Each bucket represents a constant absolute uncertainty on the specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): offset + (width * i).  Lower bound (1 <= i < N): offset + (width * (i - 1)).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Linear {
    /// Must be greater than 0.
    #[serde(rename="numFiniteBuckets")]
    
    pub num_finite_buckets: Option<i32>,
    /// Lower bound of the first bucket.
    
    pub offset: Option<f64>,
    /// Must be greater than 0.
    
    pub width: Option<f64>,
}

impl client::Part for Linear {}


/// The parameters to ListLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list entries](EntryListCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLogEntriesRequest {
    /// Optional. A filter that chooses which log entries to return. See Advanced Logs Filters. Only log entries that match the filter are returned. An empty filter matches all log entries in the resources listed in resource_names. Referencing a parent resource that is not listed in resource_names will cause the filter to return no results. The maximum length of the filter is 20000 characters.
    
    pub filter: Option<String>,
    /// Optional. How the results should be sorted. Presently, the only permitted values are "timestamp asc" (default) and "timestamp desc". The first option returns entries in order of increasing values of LogEntry.timestamp (oldest first), and the second option returns entries in order of decreasing timestamps (newest first). Entries with equal timestamps are returned in order of their insert_id values.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<String>,
    /// Optional. The maximum number of results to return from this request. Non-positive values are ignored. The presence of next_page_token in the response indicates that more results might be available.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. If present, then retrieve the next batch of results from the preceding call to this method. page_token must be the value of next_page_token from the previous response. The values of other method parameters should be identical to those in the previous call.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Deprecated. Use resource_names instead. One or more project identifiers or project numbers from which to retrieve log entries. Example: "my-project-1A".
    #[serde(rename="projectIds")]
    
    pub project_ids: Option<Vec<String>>,
    /// Required. Names of one or more parent resources from which to retrieve log entries:
    /// "projects/[PROJECT_ID]"
    /// "organizations/[ORGANIZATION_ID]"
    /// "billingAccounts/[BILLING_ACCOUNT_ID]"
    /// "folders/[FOLDER_ID]"
    /// Projects listed in the project_ids field are added to this list.
    #[serde(rename="resourceNames")]
    
    pub resource_names: Option<Vec<String>>,
}

impl client::RequestValue for ListLogEntriesRequest {}


/// Result returned from ListLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list entries](EntryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLogEntriesResponse {
    /// A list of log entries. If entries is empty, nextPageToken may still be returned, indicating that more entries may exist. See nextPageToken for more information.
    
    pub entries: Option<Vec<LogEntry>>,
    /// If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken.If a value for next_page_token appears and the entries field is empty, it means that the search found no log entries so far but it did not have time to search all the possible log entries. Retry the method with this value for page_token to continue the search. Alternatively, consider speeding up the search by changing your filter to specify a single log name or resource type, or to narrow the time range of the search.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLogEntriesResponse {}


/// Result returned from ListLogMetrics.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metrics list projects](ProjectMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLogMetricsResponse {
    /// A list of logs-based metrics.
    
    pub metrics: Option<Vec<LogMetric>>,
    /// If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLogMetricsResponse {}


/// Result returned from ListMonitoredResourceDescriptors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list monitored resource descriptors](MonitoredResourceDescriptorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// If there might be more results than those appearing in this response, then nextPageToken is included. To get the next set of results, call this method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of resource descriptors.
    #[serde(rename="resourceDescriptors")]
    
    pub resource_descriptors: Option<Vec<MonitoredResourceDescriptor>>,
}

impl client::ResponseResult for ListMonitoredResourceDescriptorsResponse {}


/// Result returned from ListSinks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sinks list projects](ProjectSinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSinksResponse {
    /// If there might be more results than appear in this response, then nextPageToken is included. To get the next set of results, call the same method again using the value of nextPageToken as pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of sinks.
    
    pub sinks: Option<Vec<LogSink>>,
}

impl client::ResponseResult for ListSinksResponse {}


/// An individual entry in a log.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// Optional. Information about the HTTP request associated with this log entry, if applicable.
    #[serde(rename="httpRequest")]
    
    pub http_request: Option<HttpRequest>,
    /// Optional. A unique identifier for the log entry. If you provide a value, then Logging considers other log entries in the same project, with the same timestamp, and with the same insert_id to be duplicates which can be removed. If omitted in new log entries, then Logging assigns its own unique identifier. The insert_id is also used to order log entries that have the same timestamp value.
    #[serde(rename="insertId")]
    
    pub insert_id: Option<String>,
    /// The log entry payload, represented as a structure that is expressed as a JSON object.
    #[serde(rename="jsonPayload")]
    
    pub json_payload: Option<HashMap<String, json::Value>>,
    /// Optional. A set of user-defined (key, value) data that provides additional information about the log entry.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The resource name of the log to which this log entry belongs:
    /// "projects/[PROJECT_ID]/logs/[LOG_ID]"
    /// "organizations/[ORGANIZATION_ID]/logs/[LOG_ID]"
    /// "billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID]"
    /// "folders/[FOLDER_ID]/logs/[LOG_ID]"
    /// A project number may optionally be used in place of PROJECT_ID. The project number is translated to its corresponding PROJECT_ID internally and the log_name field will contain PROJECT_ID in queries and exports.[LOG_ID] must be URL-encoded within log_name. Example: "organizations/1234567890/logs/cloudresourcemanager.googleapis.com%2Factivity". [LOG_ID] must be less than 512 characters long and can only include the following characters: upper and lower case alphanumeric characters, forward-slash, underscore, hyphen, and period.For backward compatibility, if log_name begins with a forward-slash, such as /projects/..., then the log entry is ingested as usual but the forward-slash is removed. Listing the log entry will not show the leading slash and filtering for a log name with a leading slash will never return any results.
    #[serde(rename="logName")]
    
    pub log_name: Option<String>,
    /// Deprecated. Output only. Additional metadata about the monitored resource.Only k8s_container, k8s_pod, and k8s_node MonitoredResources have this field populated for GKE versions older than 1.12.6. For GKE versions 1.12.6 and above, the metadata field has been deprecated. The Kubernetes pod labels that used to be in metadata.userLabels will now be present in the labels field with a key prefix of k8s-pod/. The Stackdriver system labels that were present in the metadata.systemLabels field will no longer be available in the LogEntry.
    
    pub metadata: Option<MonitoredResourceMetadata>,
    /// Optional. Information about an operation associated with the log entry, if applicable.
    
    pub operation: Option<LogEntryOperation>,
    /// The log entry payload, represented as a protocol buffer. Some Google Cloud Platform services use this field for their log entry payloads.
    #[serde(rename="protoPayload")]
    
    pub proto_payload: Option<HashMap<String, json::Value>>,
    /// Output only. The time the log entry was received by Logging.
    #[serde(rename="receiveTimestamp")]
    
    pub receive_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The primary monitored resource associated with this log entry.Example: a log entry that reports a database error would be associated with the monitored resource designating the particular database that reported the error.
    
    pub resource: Option<MonitoredResource>,
    /// Optional. The severity of the log entry. The default value is LogSeverity.DEFAULT.
    
    pub severity: Option<LogEntrySeverityEnum>,
    /// Optional. Source code location information associated with the log entry, if any.
    #[serde(rename="sourceLocation")]
    
    pub source_location: Option<LogEntrySourceLocation>,
    /// Optional. The span ID within the trace associated with the log entry.For Trace spans, this is the same format that the Trace API v2 uses: a 16-character hexadecimal encoding of an 8-byte array, such as <code>"000000000000004a"</code>.
    #[serde(rename="spanId")]
    
    pub span_id: Option<String>,
    /// The log entry payload, represented as a Unicode string (UTF-8).
    #[serde(rename="textPayload")]
    
    pub text_payload: Option<String>,
    /// Optional. The time the event described by the log entry occurred. This time is used to compute the log entry's age and to enforce the logs retention period. If this field is omitted in a new log entry, then Logging assigns it the current time. Timestamps have nanosecond accuracy, but trailing zeros in the fractional seconds might be omitted when the timestamp is displayed.Incoming log entries should have timestamps that are no more than the logs retention period in the past, and no more than 24 hours in the future. Log entries outside those time boundaries will not be available when calling entries.list, but those log entries can still be exported with LogSinks.
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Resource name of the trace associated with the log entry, if any. If it contains a relative resource name, the name is assumed to be relative to //tracing.googleapis.com. Example: projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824
    
    pub trace: Option<String>,
    /// Optional. The sampling decision of the trace associated with the log entry.True means that the trace resource name in the trace field was sampled for storage in a trace backend. False means that the trace was not sampled for storage when this log entry was written, or the sampling decision was unknown at the time. A non-sampled trace value is still useful as a request correlation identifier. The default is False.
    #[serde(rename="traceSampled")]
    
    pub trace_sampled: Option<bool>,
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
    /// Optional. An arbitrary producer identifier. The combination of id and producer must be globally unique. Examples for producer: "MyDivision.MyBigCompany.com", "github.com/MyProject/MyApplication".
    
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
    /// Optional. Human-readable name of the function or method being invoked, with optional context such as the class or package name. This information may be used in contexts such as the logs viewer, where a file and line number are less meaningful. The format can vary by language. For example: qual.if.ied.Class.method (Java), dir/package.func (Go), function (Python).
    
    pub function: Option<String>,
    /// Optional. Line within the source file. 1-based; 0 indicates no line number available.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line: Option<i64>,
}

impl client::Part for LogEntrySourceLocation {}


/// Describes a logs-based metric. The value of the metric is the number of log entries that match a logs filter in a given time interval.Logs-based metric can also be used to extract values from logs and create a a distribution of the values. The distribution records the statistics of the extracted values along with an optional histogram of the values as specified by the bucket options.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metrics create projects](ProjectMetricCreateCall) (request|response)
/// * [metrics get projects](ProjectMetricGetCall) (response)
/// * [metrics update projects](ProjectMetricUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMetric {
    /// Optional. The bucket_options are required when the logs-based metric is using a DISTRIBUTION value type and it describes the bucket boundaries used to create a histogram of the extracted values.
    #[serde(rename="bucketOptions")]
    
    pub bucket_options: Option<BucketOptions>,
    /// Output only. The creation timestamp of the metric.This field may not be present for older metrics.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A description of this metric, which is used in documentation. The maximum length of the description is 8000 characters.
    
    pub description: Option<String>,
    /// Required. An advanced logs filter which is used to match log entries. Example:
    /// "resource.type=gae_app AND severity>=ERROR"
    /// The maximum length of the filter is 20000 characters.
    
    pub filter: Option<String>,
    /// Optional. A map from a label key string to an extractor expression which is used to extract data from a log entry field and assign as the label value. Each label key specified in the LabelDescriptor must have an associated extractor expression in this map. The syntax of the extractor expression is the same as for the value_extractor field.The extracted value is converted to the type defined in the label descriptor. If the either the extraction or the type conversion fails, the label will have a default value. The default value for a string label is an empty string, for an integer label its 0, and for a boolean label its false.Note that there are upper bounds on the maximum number of labels and the number of active time series that are allowed in a project.
    #[serde(rename="labelExtractors")]
    
    pub label_extractors: Option<HashMap<String, String>>,
    /// Optional. The metric descriptor associated with the logs-based metric. If unspecified, it uses a default metric descriptor with a DELTA metric kind, INT64 value type, with no labels and a unit of "1". Such a metric counts the number of log entries matching the filter expression.The name, type, and description fields in the metric_descriptor are output only, and is constructed using the name and description field in the LogMetric.To create a logs-based metric that records a distribution of log values, a DELTA metric kind with a DISTRIBUTION value type must be used along with a value_extractor expression in the LogMetric.Each label in the metric descriptor must have a matching label name as the key and an extractor expression as the value in the label_extractors map.The metric_kind and value_type fields in the metric_descriptor cannot be updated once initially configured. New labels can be added in the metric_descriptor, but existing labels cannot be modified except for their description.
    #[serde(rename="metricDescriptor")]
    
    pub metric_descriptor: Option<MetricDescriptor>,
    /// Required. The client-assigned metric identifier. Examples: "error_count", "nginx/requests".Metric identifiers are limited to 100 characters and can include only the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the first character of the name.The metric identifier in this field must not be URL-encoded (https://en.wikipedia.org/wiki/Percent-encoding). However, when the metric identifier appears as the [METRIC_ID] part of a metric_name API parameter, then the metric identifier must be URL-encoded. Example: "projects/my-project/metrics/nginx%2Frequests".
    
    pub name: Option<String>,
    /// Output only. The last update timestamp of the metric.This field may not be present for older metrics.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. A value_extractor is required when using a distribution logs-based metric to extract the values to record from a log entry. Two functions are supported for value extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The argument are:  1. field: The name of the log entry field from which the value is to be  extracted.  2. regex: A regular expression using the Google RE2 syntax  (https://github.com/google/re2/wiki/Syntax) with a single capture  group to extract data from the specified log entry field. The value  of the field is converted to a string before applying the regex.  It is an error to specify a regex that does not include exactly one  capture group.The result of the extraction must be convertible to a double type, as the distribution always records double values. If either the extraction or the conversion to double fails, then those values are not recorded in the distribution.Example: REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*")
    #[serde(rename="valueExtractor")]
    
    pub value_extractor: Option<String>,
    /// Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed.
    
    pub version: Option<LogMetricVersionEnum>,
}

impl client::RequestValue for LogMetric {}
impl client::ResponseResult for LogMetric {}


/// Describes a sink used to export log entries to one of the following destinations in any project: a Cloud Storage bucket, a BigQuery dataset, or a Cloud Pub/Sub topic. A logs filter controls which log entries are exported. The sink must be created within a project, organization, billing account, or folder.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sinks create projects](ProjectSinkCreateCall) (request|response)
/// * [sinks get projects](ProjectSinkGetCall) (response)
/// * [sinks update projects](ProjectSinkUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogSink {
    /// Output only. The creation timestamp of the sink.This field may not be present for older sinks.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The export destination:
    /// "storage.googleapis.com/[GCS_BUCKET]"
    /// "bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]"
    /// "pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]"
    /// The sink's writer_identity, set when the sink is created, must have permission to write to the destination or else the log entries are not exported. For more information, see Exporting Logs with Sinks.
    
    pub destination: Option<String>,
    /// Optional. An advanced logs filter. The only exported log entries are those that are in the resource owning the sink and that match the filter. For example:
    /// logName="projects/[PROJECT_ID]/logs/[LOG_ID]" AND severity>=ERROR
    /// 
    
    pub filter: Option<String>,
    /// Optional. This field applies only to sinks owned by organizations and folders. If the field is false, the default, only the logs owned by the sink's parent resource are available for export. If the field is true, then logs from all the projects, folders, and billing accounts contained in the sink's parent resource are also available for export. Whether a particular log entry from the children is exported depends on the sink's filter expression. For example, if this field is true, then the filter resource.type=gce_instance would export all Compute Engine VM instance log entries from all projects in the sink's parent. To only export entries from certain child projects, filter on the project part of the log name:
    /// logName:("projects/test-project1/" OR "projects/test-project2/") AND
    /// resource.type=gce_instance
    /// 
    #[serde(rename="includeChildren")]
    
    pub include_children: Option<bool>,
    /// Required. The client-assigned sink identifier, unique within the project. Example: "my-syslog-errors-to-pubsub". Sink identifiers are limited to 100 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods.
    
    pub name: Option<String>,
    /// Deprecated. The log entry format to use for this sink's exported log entries. The v2 format is used by default and cannot be changed.
    #[serde(rename="outputVersionFormat")]
    
    pub output_version_format: Option<LogSinkOutputVersionFormatEnum>,
    /// Output only. The last update timestamp of the sink.This field may not be present for older sinks.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. An IAM identity&mdash;a service account or group&mdash;under which Logging writes the exported log entries to the sink's destination. This field is set by sinks.create and sinks.update based on the value of unique_writer_identity in those methods.Until you grant this identity write-access to the destination, log entry exports from this sink will fail. For more information, see Granting Access for a Resource. Consult the destination service's documentation to determine the appropriate IAM roles to assign to the identity.
    #[serde(rename="writerIdentity")]
    
    pub writer_identity: Option<String>,
}

impl client::RequestValue for LogSink {}
impl client::ResponseResult for LogSink {}


/// Defines a metric type and its schema. Once a metric descriptor is created, deleting or altering it stops data collection and makes the metric type's existing data unusable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptor {
    /// A detailed description of the metric, which can be used in documentation.
    
    pub description: Option<String>,
    /// A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example "Request count". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The set of labels that can be used to describe a specific instance of this metric type. For example, the appengine.googleapis.com/http/server/response_latencies metric type has a label for the HTTP response code, response_code, so you can look at latencies for successful responses or just for responses that failed.
    
    pub labels: Option<Vec<LabelDescriptor>>,
    /// Optional. Metadata which can be used to guide usage of the metric.
    
    pub metadata: Option<MetricDescriptorMetadata>,
    /// Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported.
    #[serde(rename="metricKind")]
    
    pub metric_kind: Option<MetricDescriptorMetricKindEnum>,
    /// The resource name of the metric descriptor.
    
    pub name: Option<String>,
    /// The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name custom.googleapis.com or external.googleapis.com. Metric types should use a natural hierarchical grouping. For example:
    /// "custom.googleapis.com/invoice/paid/amount"
    /// "external.googleapis.com/prometheus/up"
    /// "appengine.googleapis.com/http/server/response_latencies"
    /// 
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The unit in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The supported units are a subset of The Unified Code for Units of Measure (http://unitsofmeasure.org/ucum.html) standard:Basic units (UNIT)
    /// bit bit
    /// By byte
    /// s second
    /// min minute
    /// h hour
    /// d dayPrefixes (PREFIX)
    /// k kilo (10**3)
    /// M mega (10**6)
    /// G giga (10**9)
    /// T tera (10**12)
    /// P peta (10**15)
    /// E exa (10**18)
    /// Z zetta (10**21)
    /// Y yotta (10**24)
    /// m milli (10\*\*-3)
    /// u micro (10\*\*-6)
    /// n nano (10\*\*-9)
    /// p pico (10\*\*-12)
    /// f femto (10\*\*-15)
    /// a atto (10\*\*-18)
    /// z zepto (10\*\*-21)
    /// y yocto (10\*\*-24)
    /// Ki kibi (2**10)
    /// Mi mebi (2**20)
    /// Gi gibi (2**30)
    /// Ti tebi (2**40)GrammarThe grammar also includes these connectors:
    /// / division (as an infix operator, e.g. 1/s).
    /// . multiplication (as an infix operator, e.g. GBy.d)The grammar for a unit is as follows:
    /// Expression = Component { “.” Component } { “/” Component } ;
    /// 
    /// Component = ( \[ PREFIX \] UNIT | “%” ) \[ Annotation \]
    /// \| Annotation
    /// \| “1”
    /// ;
    /// 
    /// Annotation = “{” NAME “}” ;
    /// Notes:
    /// Annotation is just a comment if it follows a UNIT and is  equivalent to 1 if it is used alone. For examples,  {requests}/s == 1/s, By{transmitted}/s == By/s.
    /// NAME is a sequence of non-blank printable ASCII characters not  containing ‘{’ or ‘}’.
    /// 1 represents dimensionless value 1, such as in 1/s.
    /// % represents dimensionless value 1/100, and annotates values giving  a percentage.
    
    pub unit: Option<String>,
    /// Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported.
    #[serde(rename="valueType")]
    
    pub value_type: Option<MetricDescriptorValueTypeEnum>,
}

impl client::Part for MetricDescriptor {}


/// Additional annotations that can be used to guide the usage of a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptorMetadata {
    /// The delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors.
    #[serde(rename="ingestDelay")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ingest_delay: Option<client::chrono::Duration>,
    /// The launch stage of the metric definition.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<MetricDescriptorMetadataLaunchStageEnum>,
    /// The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period.
    #[serde(rename="samplePeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_period: Option<client::chrono::Duration>,
}

impl client::Part for MetricDescriptorMetadata {}


/// An object representing a resource that can be used for monitoring, logging, billing, or other purposes. Examples include virtual machine instances, databases, and storage devices such as disks. The type field identifies a MonitoredResourceDescriptor object that describes the resource’s schema. Information in the labels field identifies the actual resource and its attributes according to the schema. For example, a particular Compute Engine VM instance could be represented by the following object, because the MonitoredResourceDescriptor for “gce_instance” has labels “instance_id” and “zone”:
/// { “type”: “gce_instance”,
/// “labels”: { “instance_id”: “12345678901234”,
/// “zone”: “us-central1-a” }}
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResource {
    /// Required. Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels "project_id", "instance_id", and "zone".
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The monitored resource type. This field must match the type field of a MonitoredResourceDescriptor object. For example, the type of a Compute Engine VM instance is gce_instance.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for MonitoredResource {}


/// An object that describes the schema of a MonitoredResource object using a type name and a set of labels. For example, the monitored resource descriptor for Google Compute Engine VM instances has a type of “gce_instance” and specifies the use of the labels “instance_id” and “zone” to identify particular VM instances.Different APIs can support different monitored resource types. APIs generally provide a list method that returns the monitored resource descriptors used by the API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list monitored resource descriptors](MonitoredResourceDescriptorListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResourceDescriptor {
    /// Optional. A detailed description of the monitored resource type that might be used in documentation.
    
    pub description: Option<String>,
    /// Optional. A concise name for the monitored resource type that might be displayed in user interfaces. It should be a Title Cased Noun Phrase, without any article or other determiners. For example, "Google Cloud SQL Database".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. A set of labels used to describe instances of this monitored resource type. For example, an individual Google Cloud SQL database is identified by values for the labels "database_id" and "zone".
    
    pub labels: Option<Vec<LabelDescriptor>>,
    /// Optional. The resource name of the monitored resource descriptor: "projects/{project_id}/monitoredResourceDescriptors/{type}" where {type} is the value of the type field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format "monitoredResourceDescriptors/{type}".
    
    pub name: Option<String>,
    /// Required. The monitored resource type. For example, the type "cloudsql_database" represents databases in Google Cloud SQL. The maximum length of this value is 256 characters.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Resource for MonitoredResourceDescriptor {}


/// Auxiliary metadata for a MonitoredResource object. MonitoredResource objects contain the minimum set of information to uniquely identify a monitored resource instance. There is some other useful auxiliary metadata. Monitoring and Logging use an ingestion pipeline to extract metadata for cloud resources of all types, and store the metadata in this message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResourceMetadata {
    /// Output only. Values for predefined system metadata labels. System labels are a kind of metadata extracted by Google, including "machine_image", "vpc", "subnet_id", "security_group", "name", etc. System label values can be only strings, Boolean values, or a list of strings. For example:
    /// { "name": "my-test-instance",
    ///   "security_group": ["a", "b", "c"],
    ///   "spot_instance": false }
    /// 
    #[serde(rename="systemLabels")]
    
    pub system_labels: Option<HashMap<String, json::Value>>,
    /// Output only. A map of user-defined metadata labels.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::Part for MonitoredResourceMetadata {}


/// The parameters to WriteLogEntries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write entries](EntryWriteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteLogEntriesRequest {
    /// Optional. If true, the request should expect normal response, but the entries won't be persisted nor exported. Useful for checking whether the logging API endpoints are working properly before sending valuable data.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// Required. The log entries to send to Logging. The order of log entries in this list does not matter. Values supplied in this method's log_name, resource, and labels fields are copied into those log entries in this list that do not include values for their corresponding fields. For more information, see the LogEntry type.If the timestamp or insert_id fields are missing in log entries, then this method supplies the current time or a unique identifier, respectively. The supplied values are chosen so that, among the log entries that did not supply their own values, the entries earlier in the list will sort before the entries later in the list. See the entries.list method.Log entries with timestamps that are more than the logs retention period in the past or more than 24 hours in the future will not be available when calling entries.list. However, those log entries can still be exported with LogSinks.To improve throughput and to avoid exceeding the quota limit for calls to entries.write, you should try to include several log entries in this list, rather than calling this method for each individual log entry.
    
    pub entries: Option<Vec<LogEntry>>,
    /// Optional. Default labels that are added to the labels field of all log entries in entries. If a log entry already has a label with the same key as a label in this parameter, then the log entry's label is not changed. See LogEntry.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. A default log resource name that is assigned to all log entries in entries that do not specify a value for log_name:
    /// "projects/[PROJECT_ID]/logs/[LOG_ID]"
    /// "organizations/[ORGANIZATION_ID]/logs/[LOG_ID]"
    /// "billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID]"
    /// "folders/[FOLDER_ID]/logs/[LOG_ID]"
    /// [LOG_ID] must be URL-encoded. For example:
    /// "projects/my-project-id/logs/syslog"
    /// "organizations/1234567890/logs/cloudresourcemanager.googleapis.com%2Factivity"
    /// The permission <code>logging.logEntries.create</code> is needed on each project, organization, billing account, or folder that is receiving new log entries, whether the resource is specified in <code>logName</code> or in an individual log entry.
    #[serde(rename="logName")]
    
    pub log_name: Option<String>,
    /// Optional. Whether valid entries should be written even if some other entries fail due to INVALID_ARGUMENT or PERMISSION_DENIED errors. If any entry is not written, then the response status is the error associated with one of the failed entries and the response includes error details keyed by the entries' zero-based index in the entries.write method.
    #[serde(rename="partialSuccess")]
    
    pub partial_success: Option<bool>,
    /// Optional. A default monitored resource object that is assigned to all log entries in entries that do not specify a value for resource. Example:
    /// { “type”: “gce_instance”,
    /// “labels”: {
    /// “zone”: “us-central1-a”, “instance_id”: “00000000000000000000” }}
    /// See LogEntry.
    
    pub resource: Option<MonitoredResource>,
}

impl client::RequestValue for WriteLogEntriesRequest {}


/// Result returned from WriteLogEntries. empty
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write entries](EntryWriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteLogEntriesResponse { _never_set: Option<bool> }

impl client::ResponseResult for WriteLogEntriesResponse {}


