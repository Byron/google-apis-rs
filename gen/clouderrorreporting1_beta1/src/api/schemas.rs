use super::*;
/// Response message for deleting error events.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete events projects](ProjectDeleteEventCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteEventsResponse { _never_set: Option<bool> }

impl client::ResponseResult for DeleteEventsResponse {}


/// A description of the context in which an error occurred. This data should be provided by the application when reporting an error, unless the error report has been generated automatically from Google App Engine logs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorContext {
    /// The HTTP request which was processed when the error was triggered.
    #[serde(rename="httpRequest")]
    
    pub http_request: Option<HttpRequestContext>,
    /// The location in the source code where the decision was made to report the error, usually the place where it was logged. For a logged exception this would be the source line where the exception is logged, usually close to the place where it was caught.
    #[serde(rename="reportLocation")]
    
    pub report_location: Option<SourceLocation>,
    /// Source code that was used to build the executable which has caused the given error message.
    #[serde(rename="sourceReferences")]
    
    pub source_references: Option<Vec<SourceReference>>,
    /// The user who caused or was affected by the crash. This can be a user ID, an email address, or an arbitrary token that uniquely identifies the user. When sending an error report, leave this field empty if the user was not logged in. In this case the Error Reporting system will use other data, such as remote IP address, to distinguish affected users. See `affected_users_count` in `ErrorGroupStats`.
    
    pub user: Option<String>,
}

impl client::Part for ErrorContext {}


/// An error event which is returned by the Error Reporting system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorEvent {
    /// Data about the context in which the error occurred.
    
    pub context: Option<ErrorContext>,
    /// Time when the event occurred as provided in the error report. If the report did not contain a timestamp, the time the error was received by the Error Reporting system is used.
    #[serde(rename="eventTime")]
    
    pub event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The stack trace that was reported or logged by the service.
    
    pub message: Option<String>,
    /// The `ServiceContext` for which this error was reported.
    #[serde(rename="serviceContext")]
    
    pub service_context: Option<ServiceContext>,
}

impl client::Part for ErrorEvent {}


/// Description of a group of similar error events.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [groups get projects](ProjectGroupGetCall) (response)
/// * [groups update projects](ProjectGroupUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorGroup {
    /// Group IDs are unique for a given project. If the same kind of error occurs in different service contexts, it will receive the same group ID.
    #[serde(rename="groupId")]
    
    pub group_id: Option<String>,
    /// The group resource name. Example: projects/my-project-123/groups/CNSgkpnppqKCUw
    
    pub name: Option<String>,
    /// Error group's resolution status. An unspecified resolution status will be interpreted as OPEN
    #[serde(rename="resolutionStatus")]
    
    pub resolution_status: Option<ErrorGroupResolutionStatusEnum>,
    /// Associated tracking issues.
    #[serde(rename="trackingIssues")]
    
    pub tracking_issues: Option<Vec<TrackingIssue>>,
}

impl client::RequestValue for ErrorGroup {}
impl client::ResponseResult for ErrorGroup {}


/// Data extracted for a specific group based on certain filter criteria, such as a given time period and/or service filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorGroupStats {
    /// Service contexts with a non-zero error count for the given filter criteria. This list can be truncated if multiple services are affected. Refer to `num_affected_services` for the total count.
    #[serde(rename="affectedServices")]
    
    pub affected_services: Option<Vec<ServiceContext>>,
    /// Approximate number of affected users in the given group that match the filter criteria. Users are distinguished by data in the ErrorContext of the individual error events, such as their login name or their remote IP address in case of HTTP requests. The number of affected users can be zero even if the number of errors is non-zero if no data was provided from which the affected user could be deduced. Users are counted based on data in the request context that was provided in the error report. If more users are implicitly affected, such as due to a crash of the whole service, this is not reflected here.
    #[serde(rename="affectedUsersCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub affected_users_count: Option<i64>,
    /// Approximate total number of events in the given group that match the filter criteria.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Approximate first occurrence that was ever seen for this group and which matches the given filter criteria, ignoring the time_range that was specified in the request.
    #[serde(rename="firstSeenTime")]
    
    pub first_seen_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Group data that is independent of the filter criteria.
    
    pub group: Option<ErrorGroup>,
    /// Approximate last occurrence that was ever seen for this group and which matches the given filter criteria, ignoring the time_range that was specified in the request.
    #[serde(rename="lastSeenTime")]
    
    pub last_seen_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The total number of services with a non-zero error count for the given filter criteria.
    #[serde(rename="numAffectedServices")]
    
    pub num_affected_services: Option<i32>,
    /// An arbitrary event that is chosen as representative for the whole group. The representative event is intended to be used as a quick preview for the whole group. Events in the group are usually sufficiently similar to each other such that showing an arbitrary representative provides insight into the characteristics of the group as a whole.
    
    pub representative: Option<ErrorEvent>,
    /// Approximate number of occurrences over time. Timed counts returned by ListGroups are guaranteed to be: - Inside the requested time interval - Non-overlapping, and - Ordered by ascending time.
    #[serde(rename="timedCounts")]
    
    pub timed_counts: Option<Vec<TimedCount>>,
}

impl client::Part for ErrorGroupStats {}


/// HTTP request data that is related to a reported error. This data should be provided by the application when reporting an error, unless the error report has been generated automatically from Google App Engine logs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRequestContext {
    /// The type of HTTP request, such as `GET`, `POST`, etc.
    
    pub method: Option<String>,
    /// The referrer information that is provided with the request.
    
    pub referrer: Option<String>,
    /// The IP address from which the request originated. This can be IPv4, IPv6, or a token which is derived from the IP address, depending on the data that has been provided in the error report.
    #[serde(rename="remoteIp")]
    
    pub remote_ip: Option<String>,
    /// The HTTP response status code for the request.
    #[serde(rename="responseStatusCode")]
    
    pub response_status_code: Option<i32>,
    /// The URL of the request.
    
    pub url: Option<String>,
    /// The user agent information that is provided with the request.
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
}

impl client::Part for HttpRequestContext {}


/// Contains a set of requested error events.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [events list projects](ProjectEventListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEventsResponse {
    /// The error events which match the given request.
    #[serde(rename="errorEvents")]
    
    pub error_events: Option<Vec<ErrorEvent>>,
    /// If non-empty, more results are available. Pass this token, along with the same query parameters as the first request, to view the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The timestamp specifies the start time to which the request was restricted.
    #[serde(rename="timeRangeBegin")]
    
    pub time_range_begin: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ListEventsResponse {}


/// Contains a set of requested error group stats.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [group stats list projects](ProjectGroupStatListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGroupStatsResponse {
    /// The error group stats which match the given request.
    #[serde(rename="errorGroupStats")]
    
    pub error_group_stats: Option<Vec<ErrorGroupStats>>,
    /// If non-empty, more results are available. Pass this token, along with the same query parameters as the first request, to view the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The timestamp specifies the start time to which the request was restricted. The start time is set based on the requested time range. It may be adjusted to a later time if a project has exceeded the storage quota and older data has been deleted.
    #[serde(rename="timeRangeBegin")]
    
    pub time_range_begin: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ListGroupStatsResponse {}


/// Response for reporting an individual error event. Data may be added to this message in the future.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [events report projects](ProjectEventReportCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportErrorEventResponse { _never_set: Option<bool> }

impl client::ResponseResult for ReportErrorEventResponse {}


/// An error event which is reported to the Error Reporting system.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [events report projects](ProjectEventReportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportedErrorEvent {
    /// Optional. A description of the context in which the error occurred.
    
    pub context: Option<ErrorContext>,
    /// Optional. Time when the event occurred. If not provided, the time when the event was received by the Error Reporting system is used. If provided, the time must not exceed the [logs retention period](https://cloud.google.com/logging/quotas#logs_retention_periods) in the past, or be more than 24 hours in the future. If an invalid time is provided, then an error is returned.
    #[serde(rename="eventTime")]
    
    pub event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The error message. If no `context.reportLocation` is provided, the message must contain a header (typically consisting of the exception type name and an error message) and an exception stack trace in one of the supported programming languages and formats. Supported languages are Java, Python, JavaScript, Ruby, C#, PHP, and Go. Supported stack trace formats are: * **Java**: Must be the return value of [`Throwable.printStackTrace()`](https://docs.oracle.com/javase/7/docs/api/java/lang/Throwable.html#printStackTrace%28%29). * **Python**: Must be the return value of [`traceback.format_exc()`](https://docs.python.org/2/library/traceback.html#traceback.format_exc). * **JavaScript**: Must be the value of [`error.stack`](https://github.com/v8/v8/wiki/Stack-Trace-API) as returned by V8. * **Ruby**: Must contain frames returned by [`Exception.backtrace`](https://ruby-doc.org/core-2.2.0/Exception.html#method-i-backtrace). * **C#**: Must be the return value of [`Exception.ToString()`](https://msdn.microsoft.com/en-us/library/system.exception.tostring.aspx). * **PHP**: Must be prefixed with `"PHP (Notice|Parse error|Fatal error|Warning): "` and contain the result of [`(string)$exception`](https://php.net/manual/en/exception.tostring.php). * **Go**: Must be the return value of [`runtime.Stack()`](https://golang.org/pkg/runtime/debug/#Stack).
    
    pub message: Option<String>,
    /// Required. The service context in which this error has occurred.
    #[serde(rename="serviceContext")]
    
    pub service_context: Option<ServiceContext>,
}

impl client::RequestValue for ReportedErrorEvent {}


/// Describes a running service that sends errors. Its version changes over time and multiple versions can run in parallel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceContext {
    /// Type of the MonitoredResource. List of possible values: https://cloud.google.com/monitoring/api/resources Value is set automatically for incoming errors and must not be set when reporting errors.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
    /// An identifier of the service, such as the name of the executable, job, or Google App Engine service name. This field is expected to have a low number of values that are relatively stable over time, as opposed to `version`, which can be changed whenever new code is deployed. Contains the service name for error reports extracted from Google App Engine logs or `default` if the App Engine default service is used.
    
    pub service: Option<String>,
    /// Represents the source code version that the developer provided, which could represent a version label or a Git SHA-1 hash, for example. For App Engine standard environment, the version is set to the version of the app.
    
    pub version: Option<String>,
}

impl client::Part for ServiceContext {}


/// Indicates a location in the source code of the service for which errors are reported. `functionName` must be provided by the application when reporting an error, unless the error report contains a `message` with a supported exception stack trace. All fields are optional for the later case.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    /// The source code filename, which can include a truncated relative path, or a full path from a production machine.
    #[serde(rename="filePath")]
    
    pub file_path: Option<String>,
    /// Human-readable name of a function or method. The value can include optional context like the class or package name. For example, `my.package.MyClass.method` in case of Java.
    #[serde(rename="functionName")]
    
    pub function_name: Option<String>,
    /// 1-based. 0 indicates that the line number is unknown.
    #[serde(rename="lineNumber")]
    
    pub line_number: Option<i32>,
}

impl client::Part for SourceLocation {}


/// A reference to a particular snapshot of the source tree used to build and deploy an application.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceReference {
    /// Optional. A URI string identifying the repository. Example: "https://github.com/GoogleCloudPlatform/kubernetes.git"
    
    pub repository: Option<String>,
    /// The canonical and persistent identifier of the deployed revision. Example (git): "0035781c50ec7aa23385dc841529ce8a4b70db1b"
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for SourceReference {}


/// The number of errors in a given time period. All numbers are approximate since the error events are sampled before counting them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimedCount {
    /// Approximate number of occurrences in the given time period.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// End of the time period to which `count` refers (excluded).
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Start of the time period to which `count` refers (included).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimedCount {}


/// Information related to tracking the progress on resolving the error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackingIssue {
    /// A URL pointing to a related entry in an issue tracking system. Example: `https://github.com/user/project/issues/4`
    
    pub url: Option<String>,
}

impl client::Part for TrackingIssue {}


