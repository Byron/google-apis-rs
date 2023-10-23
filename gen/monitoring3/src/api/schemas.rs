use super::*;
/// Describes how to combine multiple time series to provide a different view of the data. Aggregation of time series is done in two steps. First, each time series in the set is aligned to the same time interval boundaries, then the set of time series is optionally reduced in number.Alignment consists of applying the per_series_aligner operation to each time series after its data has been divided into regular alignment_period time intervals. This process takes all of the data points in an alignment period, applies a mathematical transformation such as averaging, minimum, maximum, delta, etc., and converts them into a single data point per period.Reduction is when the aligned and transformed time series can optionally be combined, reducing the number of time series through similar mathematical transformations. Reduction involves applying a cross_series_reducer to all the time series, optionally sorting the time series into subsets with group_by_fields, and applying the reducer to each subset.The raw time series data can contain a huge amount of information from multiple sources. Alignment and reduction transforms this mass of data into a more manageable and representative collection of data, for example "the 95% latency across the average of all tasks in a cluster". This representative data can be more easily graphed and comprehended, and the individual time series data is still available for later drilldown. For more details, see Filtering and aggregation (https://cloud.google.com/monitoring/api/v3/aggregation).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Aggregation {
    /// The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies.
    #[serde(rename="alignmentPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub alignment_period: Option<client::chrono::Duration>,
    /// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
    #[serde(rename="crossSeriesReducer")]
    
    pub cross_series_reducer: Option<AggregationCrossSeriesReducerEnum>,
    /// The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored.
    #[serde(rename="groupByFields")]
    
    pub group_by_fields: Option<Vec<String>>,
    /// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
    #[serde(rename="perSeriesAligner")]
    
    pub per_series_aligner: Option<AggregationPerSeriesAlignerEnum>,
}

impl client::Part for Aggregation {}


/// A description of the conditions under which some aspect of your system is considered to be “unhealthy” and the ways to notify people or services about this state. For an overview of alert policies, see Introduction to Alerting (https://cloud.google.com/monitoring/alerts/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [alert policies create projects](ProjectAlertPolicyCreateCall) (request|response)
/// * [alert policies get projects](ProjectAlertPolicyGetCall) (response)
/// * [alert policies patch projects](ProjectAlertPolicyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlertPolicy {
    /// Control over how this alert policy's notification channels are notified.
    #[serde(rename="alertStrategy")]
    
    pub alert_strategy: Option<AlertStrategy>,
    /// How to combine the results of multiple conditions to determine if an incident should be opened. If condition_time_series_query_language is present, this must be COMBINE_UNSPECIFIED.
    
    pub combiner: Option<AlertPolicyCombinerEnum>,
    /// A list of conditions for the policy. The conditions are combined by AND or OR according to the combiner field. If the combined conditions evaluate to true, then an incident is created. A policy can have from one to six conditions. If condition_time_series_query_language is present, it must be the only condition.
    
    pub conditions: Option<Vec<Condition>>,
    /// A read-only record of the creation of the alerting policy. If provided in a call to create or update, this field will be ignored.
    #[serde(rename="creationRecord")]
    
    pub creation_record: Option<MutationRecord>,
    /// A short name or phrase used to identify the policy in dashboards, notifications, and incidents. To avoid confusion, don't use the same display name for multiple policies in the same project. The name is limited to 512 Unicode characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Documentation that is included with notifications and incidents related to this policy. Best practice is for the documentation to include information to help responders understand, mitigate, escalate, and correct the underlying problems detected by the alerting policy. Notification channels that have limited capacity might not show this documentation.
    
    pub documentation: Option<Documentation>,
    /// Whether or not the policy is enabled. On write, the default interpretation if unset is that the policy is enabled. On read, clients should not make any assumption about the state if it has not been populated. The field should always be populated on List and Get operations, unless a field projection has been specified that strips it out.
    
    pub enabled: Option<bool>,
    /// A read-only record of the most recent change to the alerting policy. If provided in a call to create or update, this field will be ignored.
    #[serde(rename="mutationRecord")]
    
    pub mutation_record: Option<MutationRecord>,
    /// Required if the policy exists. The resource name for this policy. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] [ALERT_POLICY_ID] is assigned by Cloud Monitoring when the policy is created. When calling the alertPolicies.create method, do not include the name field in the alerting policy passed as part of the request.
    
    pub name: Option<String>,
    /// Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of the NotificationChannel objects that are returned from the ListNotificationChannels method. The format of the entries in this field is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] 
    #[serde(rename="notificationChannels")]
    
    pub notification_channels: Option<Vec<String>>,
    /// User-supplied key/value data to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
    /// Read-only description of how the alert policy is invalid. OK if the alert policy is valid. If not OK, the alert policy will not generate incidents.
    
    pub validity: Option<Status>,
}

impl client::RequestValue for AlertPolicy {}
impl client::ResponseResult for AlertPolicy {}


/// Control over how the notification channels in notification_channels are notified when this alert fires.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlertStrategy {
    /// If an alert policy that was active has no data for this long, any open incidents will close
    #[serde(rename="autoClose")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub auto_close: Option<client::chrono::Duration>,
    /// Required for alert policies with a LogMatch condition.This limit is not implemented for alert policies that are not log-based.
    #[serde(rename="notificationRateLimit")]
    
    pub notification_rate_limit: Option<NotificationRateLimit>,
}

impl client::Part for AlertStrategy {}


/// App Engine service. Learn more at https://cloud.google.com/appengine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppEngine {
    /// The ID of the App Engine module underlying this service. Corresponds to the module_id resource label in the gae_app monitored resource (https://cloud.google.com/monitoring/api/resources#tag_gae_app).
    #[serde(rename="moduleId")]
    
    pub module_id: Option<String>,
}

impl client::Part for AppEngine {}


/// Future parameters for the availability SLI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AvailabilityCriteria { _never_set: Option<bool> }

impl client::Part for AvailabilityCriteria {}


/// The authentication parameters to provide to the specified resource or URL that requires a username and password. Currently, only Basic HTTP authentication (https://tools.ietf.org/html/rfc7617) is supported in Uptime checks.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicAuthentication {
    /// The password to use when authenticating with the HTTP server.
    
    pub password: Option<String>,
    /// The username to use when authenticating with the HTTP server.
    
    pub username: Option<String>,
}

impl client::Part for BasicAuthentication {}


/// A well-known service type, defined by its service type and service labels. Documentation and examples here (https://cloud.google.com/stackdriver/docs/solutions/slo-monitoring/api/api-structures#basic-svc-w-basic-sli).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicService {
    /// Labels that specify the resource that emits the monitoring data which is used for SLO reporting of this Service. Documentation and valid values for given service types here (https://cloud.google.com/stackdriver/docs/solutions/slo-monitoring/api/api-structures#basic-svc-w-basic-sli).
    #[serde(rename="serviceLabels")]
    
    pub service_labels: Option<HashMap<String, String>>,
    /// The type of service that this basic service defines, e.g. APP_ENGINE service type. Documentation and valid values here (https://cloud.google.com/stackdriver/docs/solutions/slo-monitoring/api/api-structures#basic-svc-w-basic-sli).
    #[serde(rename="serviceType")]
    
    pub service_type: Option<String>,
}

impl client::Part for BasicService {}


/// An SLI measuring performance on a well-known service type. Performance will be computed on the basis of pre-defined metrics. The type of the service_resource determines the metrics to use and the service_resource.labels and metric_labels are used to construct a monitoring filter to filter that metric down to just the data relevant to this service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicSli {
    /// Good service is defined to be the count of requests made to this service that return successfully.
    
    pub availability: Option<AvailabilityCriteria>,
    /// Good service is defined to be the count of requests made to this service that are fast enough with respect to latency.threshold.
    
    pub latency: Option<LatencyCriteria>,
    /// OPTIONAL: The set of locations to which this SLI is relevant. Telemetry from other locations will not be used to calculate performance for this SLI. If omitted, this SLI applies to all locations in which the Service has activity. For service types that don't support breaking down by location, setting this field will result in an error.
    
    pub location: Option<Vec<String>>,
    /// OPTIONAL: The set of RPCs to which this SLI is relevant. Telemetry from other methods will not be used to calculate performance for this SLI. If omitted, this SLI applies to all the Service's methods. For service types that don't support breaking down by method, setting this field will result in an error.
    
    pub method: Option<Vec<String>>,
    /// OPTIONAL: The set of API versions to which this SLI is relevant. Telemetry from other API versions will not be used to calculate performance for this SLI. If omitted, this SLI applies to all API versions. For service types that don't support breaking down by version, setting this field will result in an error.
    
    pub version: Option<Vec<String>>,
}

impl client::Part for BasicSli {}


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


/// Cloud Endpoints service. Learn more at https://cloud.google.com/endpoints.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudEndpoints {
    /// The name of the Cloud Endpoints service underlying this service. Corresponds to the service resource label in the api monitored resource (https://cloud.google.com/monitoring/api/resources#tag_api).
    
    pub service: Option<String>,
}

impl client::Part for CloudEndpoints {}


/// Cloud Run service. Learn more at https://cloud.google.com/run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRun {
    /// The location the service is run. Corresponds to the location resource label in the cloud_run_revision monitored resource (https://cloud.google.com/monitoring/api/resources#tag_cloud_run_revision).
    
    pub location: Option<String>,
    /// The name of the Cloud Run service. Corresponds to the service_name resource label in the cloud_run_revision monitored resource (https://cloud.google.com/monitoring/api/resources#tag_cloud_run_revision).
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
}

impl client::Part for CloudRun {}


/// Istio service scoped to a single Kubernetes cluster. Learn more at https://istio.io. Clusters running OSS Istio will have their services ingested as this type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterIstio {
    /// The name of the Kubernetes cluster in which this Istio service is defined. Corresponds to the cluster_name resource label in k8s_cluster resources.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// The location of the Kubernetes cluster in which this Istio service is defined. Corresponds to the location resource label in k8s_cluster resources.
    
    pub location: Option<String>,
    /// The name of the Istio service underlying this service. Corresponds to the destination_service_name metric label in Istio metrics.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
    /// The namespace of the Istio service underlying this service. Corresponds to the destination_service_namespace metric label in Istio metrics.
    #[serde(rename="serviceNamespace")]
    
    pub service_namespace: Option<String>,
}

impl client::Part for ClusterIstio {}


/// A collection of data points sent from a collectd-based plugin. See the collectd documentation for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectdPayload {
    /// The end time of the interval.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The measurement metadata. Example: "process_id" -> 12345
    
    pub metadata: Option<HashMap<String, TypedValue>>,
    /// The name of the plugin. Example: "disk".
    
    pub plugin: Option<String>,
    /// The instance name of the plugin Example: "hdcl".
    #[serde(rename="pluginInstance")]
    
    pub plugin_instance: Option<String>,
    /// The start time of the interval.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The measurement type. Example: "memory".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The measurement type instance. Example: "used".
    #[serde(rename="typeInstance")]
    
    pub type_instance: Option<String>,
    /// The measured values during this time interval. Each value must have a different data_source_name.
    
    pub values: Option<Vec<CollectdValue>>,
}

impl client::Part for CollectdPayload {}


/// Describes the error status for payloads that were not written.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectdPayloadError {
    /// Records the error status for the payload. If this field is present, the partial errors for nested values won't be populated.
    
    pub error: Option<Status>,
    /// The zero-based index in CreateCollectdTimeSeriesRequest.collectd_payloads.
    
    pub index: Option<i32>,
    /// Records the error status for values that were not written due to an error.Failed payloads for which nothing is written will not include partial value errors.
    #[serde(rename="valueErrors")]
    
    pub value_errors: Option<Vec<CollectdValueError>>,
}

impl client::Part for CollectdPayloadError {}


/// A single data point from a collectd-based plugin.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectdValue {
    /// The data source for the collectd value. For example, there are two data sources for network measurements: "rx" and "tx".
    #[serde(rename="dataSourceName")]
    
    pub data_source_name: Option<String>,
    /// The type of measurement.
    #[serde(rename="dataSourceType")]
    
    pub data_source_type: Option<CollectdValueDataSourceTypeEnum>,
    /// The measurement value.
    
    pub value: Option<TypedValue>,
}

impl client::Part for CollectdValue {}


/// Describes the error status for values that were not written.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectdValueError {
    /// Records the error status for the value.
    
    pub error: Option<Status>,
    /// The zero-based index in CollectdPayload.values within the parent CreateCollectdTimeSeriesRequest.collectd_payloads.
    
    pub index: Option<i32>,
}

impl client::Part for CollectdValueError {}


/// A condition is a true/false test that determines when an alerting policy should open an incident. If a condition evaluates to true, it signifies that something is wrong.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    /// A condition that checks that a time series continues to receive new data points.
    #[serde(rename="conditionAbsent")]
    
    pub condition_absent: Option<MetricAbsence>,
    /// A condition that checks for log messages matching given constraints. If set, no other conditions can be present.
    #[serde(rename="conditionMatchedLog")]
    
    pub condition_matched_log: Option<LogMatch>,
    /// A condition that uses the Monitoring Query Language to define alerts.
    #[serde(rename="conditionMonitoringQueryLanguage")]
    
    pub condition_monitoring_query_language: Option<MonitoringQueryLanguageCondition>,
    /// A condition that compares a time series against a threshold.
    #[serde(rename="conditionThreshold")]
    
    pub condition_threshold: Option<MetricThreshold>,
    /// A short name or phrase used to identify the condition in dashboards, notifications, and incidents. To avoid confusion, don't use the same display name for multiple conditions in the same policy.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required if the condition exists. The unique resource name for this condition. Its format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID] [CONDITION_ID] is assigned by Cloud Monitoring when the condition is created as part of a new or updated alerting policy.When calling the alertPolicies.create method, do not include the name field in the conditions of the requested alerting policy. Cloud Monitoring creates the condition identifiers and includes them in the new policy.When calling the alertPolicies.update method to update a policy, including a condition name causes the existing condition to be updated. Conditions without names are added to the updated policy. Existing conditions are deleted if they are not updated.Best practice is to preserve [CONDITION_ID] if you make only small changes, such as those to condition thresholds, durations, or trigger values. Otherwise, treat the change as a new condition and let the existing condition be deleted.
    
    pub name: Option<String>,
}

impl client::Part for Condition {}


/// Optional. Used to perform content matching. This allows matching based on substrings and regular expressions, together with their negations. Only the first 4 MB of an HTTP or HTTPS check's response (and the first 1 MB of a TCP check's response) are examined for purposes of content matching.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentMatcher {
    /// String, regex or JSON content to match. Maximum 1024 bytes. An empty content string indicates no content matching is to be performed.
    
    pub content: Option<String>,
    /// Matcher information for MATCHES_JSON_PATH and NOT_MATCHES_JSON_PATH
    #[serde(rename="jsonPathMatcher")]
    
    pub json_path_matcher: Option<JsonPathMatcher>,
    /// The type of content matcher that will be applied to the server output, compared to the content string when the check is run.
    
    pub matcher: Option<ContentMatcherMatcherEnum>,
}

impl client::Part for ContentMatcher {}


/// The CreateCollectdTimeSeries request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [collectd time series create projects](ProjectCollectdTimeSeryCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCollectdTimeSeriesRequest {
    /// The collectd payloads representing the time series data. You must not include more than a single point for each time series, so no two payloads can have the same values for all of the fields plugin, plugin_instance, type, and type_instance.
    #[serde(rename="collectdPayloads")]
    
    pub collectd_payloads: Option<Vec<CollectdPayload>>,
    /// The version of collectd that collected the data. Example: "5.3.0-192.el6".
    #[serde(rename="collectdVersion")]
    
    pub collectd_version: Option<String>,
    /// The monitored resource associated with the time series.
    
    pub resource: Option<MonitoredResource>,
}

impl client::RequestValue for CreateCollectdTimeSeriesRequest {}


/// The CreateCollectdTimeSeries response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [collectd time series create projects](ProjectCollectdTimeSeryCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCollectdTimeSeriesResponse {
    /// Records the error status for points that were not written due to an error in the request.Failed requests for which nothing is written will return an error response instead. Requests where data points were rejected by the backend will set summary instead.
    #[serde(rename="payloadErrors")]
    
    pub payload_errors: Option<Vec<CollectdPayloadError>>,
    /// Aggregate statistics from writing the payloads. This field is omitted if all points were successfully written, so that the response is empty. This is for backwards compatibility with clients that log errors on any non-empty response.
    
    pub summary: Option<CreateTimeSeriesSummary>,
}

impl client::ResponseResult for CreateCollectdTimeSeriesResponse {}


/// The CreateTimeSeries request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [time series create projects](ProjectTimeSeryCreateCall) (request)
/// * [time series create service projects](ProjectTimeSeryCreateServiceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateTimeSeriesRequest {
    /// Required. The new data to be added to a list of time series. Adds at most one data point to each of several time series. The new data point must be more recent than any other point in its time series. Each TimeSeries value must fully specify a unique time series by supplying all label values for the metric and the monitored resource.The maximum number of TimeSeries objects per Create request is 200.
    #[serde(rename="timeSeries")]
    
    pub time_series: Option<Vec<TimeSeries>>,
}

impl client::RequestValue for CreateTimeSeriesRequest {}


/// Summary of the result of a failed request to write data to a time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateTimeSeriesSummary {
    /// The number of points that failed to be written. Order is not guaranteed.
    
    pub errors: Option<Vec<Error>>,
    /// The number of points that were successfully written.
    #[serde(rename="successPointCount")]
    
    pub success_point_count: Option<i32>,
    /// The number of points in the request.
    #[serde(rename="totalPointCount")]
    
    pub total_point_count: Option<i32>,
}

impl client::Part for CreateTimeSeriesSummary {}


/// Criteria specific to the AlertPolicys that this Snooze applies to. The Snooze will suppress alerts that come from one of the AlertPolicys whose names are supplied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Criteria {
    /// The specific AlertPolicy names for the alert that should be snoozed. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID] There is a limit of 100 policies per snooze. This limit is checked during snooze creation.
    
    pub policies: Option<Vec<String>>,
}

impl client::Part for Criteria {}


/// Use a custom service to designate a service that you want to monitor when none of the other service types (like App Engine, Cloud Run, or a GKE type) matches your intended service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Custom { _never_set: Option<bool> }

impl client::Part for Custom {}


/// Distribution contains summary statistics for a population of values. It optionally contains a histogram representing the distribution of those values across a set of buckets.The summary statistics are the count, mean, sum of the squared deviation from the mean, the minimum, and the maximum of the set of population of values. The histogram is based on a sequence of buckets and gives a count of values that fall into each bucket. The boundaries of the buckets are given either explicitly or by formulas for buckets of fixed or exponentially increasing widths.Although it is not forbidden, it is generally a bad idea to include non-finite values (infinities or NaNs) in the population of values, as this will render the mean and sum_of_squared_deviation fields meaningless.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Distribution {
    /// Required in the Cloud Monitoring API v3. The values for each bucket specified in bucket_options. The sum of the values in bucketCounts must equal the value in the count field of the Distribution object. The order of the bucket counts follows the numbering schemes described for the three bucket types. The underflow bucket has number 0; the finite buckets, if any, have numbers 1 through N-2; and the overflow bucket has number N-1. The size of bucket_counts must not be greater than N. If the size is less than N, then the remaining buckets are assigned values of zero.
    #[serde(rename="bucketCounts")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub bucket_counts: Option<Vec<i64>>,
    /// Required in the Cloud Monitoring API v3. Defines the histogram bucket boundaries.
    #[serde(rename="bucketOptions")]
    
    pub bucket_options: Option<BucketOptions>,
    /// The number of values in the population. Must be non-negative. This value must equal the sum of the values in bucket_counts if a histogram is provided.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Must be in increasing order of value field.
    
    pub exemplars: Option<Vec<Exemplar>>,
    /// The arithmetic mean of the values in the population. If count is zero then this field must be zero.
    
    pub mean: Option<f64>,
    /// If specified, contains the range of the population values. The field must not be present if the count is zero. This field is presently ignored by the Cloud Monitoring API v3.
    
    pub range: Option<Range>,
    /// The sum of squared deviations from the mean of the values in the population. For values x_i this is: Sum\[i=1..n\]((x_i - mean)^2) Knuth, “The Art of Computer Programming”, Vol. 2, page 232, 3rd edition describes Welford’s method for accumulating this sum in one pass.If count is zero then this field must be zero.
    #[serde(rename="sumOfSquaredDeviation")]
    
    pub sum_of_squared_deviation: Option<f64>,
}

impl client::Part for Distribution {}


/// A DistributionCut defines a TimeSeries and thresholds used for measuring good service and total service. The TimeSeries must have ValueType = DISTRIBUTION and MetricKind = DELTA or MetricKind = CUMULATIVE. The computed good_service will be the estimated count of values in the Distribution that fall within the specified min and max.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DistributionCut {
    /// A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries aggregating values. Must have ValueType = DISTRIBUTION and MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[serde(rename="distributionFilter")]
    
    pub distribution_filter: Option<String>,
    /// Range of values considered "good." For a one-sided range, set one bound to an infinite value.
    
    pub range: Option<GoogleMonitoringV3Range>,
}

impl client::Part for DistributionCut {}


/// A content string and a MIME type that describes the content string's format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Documentation {
    /// The body of the documentation, interpreted according to mime_type. The content may not exceed 8,192 Unicode characters and may not exceed more than 10,240 bytes when encoded in UTF-8 format, whichever is smaller. This text can be templatized by using variables (https://cloud.google.com/monitoring/alerts/doc-variables).
    
    pub content: Option<String>,
    /// The format of the content field. Presently, only the value "text/markdown" is supported. See Markdown (https://en.wikipedia.org/wiki/Markdown) for more information.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for Documentation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [alert policies delete projects](ProjectAlertPolicyDeleteCall) (response)
/// * [groups delete projects](ProjectGroupDeleteCall) (response)
/// * [metric descriptors delete projects](ProjectMetricDescriptorDeleteCall) (response)
/// * [notification channels delete projects](ProjectNotificationChannelDeleteCall) (response)
/// * [notification channels send verification code projects](ProjectNotificationChannelSendVerificationCodeCall) (response)
/// * [time series create projects](ProjectTimeSeryCreateCall) (response)
/// * [time series create service projects](ProjectTimeSeryCreateServiceCall) (response)
/// * [uptime check configs delete projects](ProjectUptimeCheckConfigDeleteCall) (response)
/// * [service level objectives delete services](ServiceServiceLevelObjectiveDeleteCall) (response)
/// * [delete services](ServiceDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Detailed information about an error category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    /// The number of points that couldn't be written because of status.
    #[serde(rename="pointCount")]
    
    pub point_count: Option<i32>,
    /// The status of the requested write operation.
    
    pub status: Option<Status>,
}

impl client::Part for Error {}


/// Exemplars are example points that may be used to annotate aggregated distribution values. They are metadata that gives information about a particular value added to a Distribution bucket, such as a trace ID that was active when a value was added. They may contain further information, such as a example values and timestamps, origin, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Exemplar {
    /// Contextual information about the example value. Examples are:Trace: type.googleapis.com/google.monitoring.v3.SpanContextLiteral string: type.googleapis.com/google.protobuf.StringValueLabels dropped during aggregation: type.googleapis.com/google.monitoring.v3.DroppedLabelsThere may be only a single attachment of any given message type in a single exemplar, and this is enforced by the system.
    
    pub attachments: Option<Vec<HashMap<String, json::Value>>>,
    /// The observation (sampling) time of the above value.
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Value of the exemplar point. This value determines to which bucket the exemplar belongs.
    
    pub value: Option<f64>,
}

impl client::Part for Exemplar {}


/// Specifies a set of buckets with arbitrary widths.There are size(bounds) + 1 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): boundsi Lower bound (1 <= i < N); boundsi - 1The bounds field must contain at least one element. If bounds has only one element, then there are no finite buckets, and that single element is the common boundary of the overflow and underflow buckets.
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


/// Specifies an exponential sequence of buckets that have a width that is proportional to the value of the lower bound. Each bucket represents a constant relative uncertainty on a specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): scale * (growth_factor ^ i). Lower bound (1 <= i < N): scale * (growth_factor ^ (i - 1)).
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


/// Options used when forecasting the time series and testing the predicted value against the threshold.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ForecastOptions {
    /// Required. The length of time into the future to forecast whether a time series will violate the threshold. If the predicted value is found to violate the threshold, and the violation is observed in all forecasts made for the configured duration, then the time series is considered to be failing.
    #[serde(rename="forecastHorizon")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub forecast_horizon: Option<client::chrono::Duration>,
}

impl client::Part for ForecastOptions {}


/// The GetNotificationChannelVerificationCode request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channels get verification code projects](ProjectNotificationChannelGetVerificationCodeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetNotificationChannelVerificationCodeRequest {
    /// The desired expiration time. If specified, the API will guarantee that the returned code will not be valid after the specified timestamp; however, the API cannot guarantee that the returned code will be valid for at least as long as the requested time (the API puts an upper bound on the amount of time for which a code may be valid). If omitted, a default expiration will be used, which may be less than the max permissible expiration (so specifying an expiration may extend the code's lifetime over omitting an expiration, even though the API does impose an upper limit on the maximum expiration that is permitted).
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GetNotificationChannelVerificationCodeRequest {}


/// The GetNotificationChannelVerificationCode request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channels get verification code projects](ProjectNotificationChannelGetVerificationCodeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetNotificationChannelVerificationCodeResponse {
    /// The verification code, which may be used to verify other channels that have an equivalent identity (i.e. other channels of the same type with the same fingerprint such as other email channels with the same email address or other sms channels with the same number).
    
    pub code: Option<String>,
    /// The expiration time associated with the code that was returned. If an expiration was provided in the request, this is the minimum of the requested expiration in the request and the max permitted expiration.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GetNotificationChannelVerificationCodeResponse {}


/// GKE Namespace. The field names correspond to the resource metadata labels on monitored resources that fall under a namespace (for example, k8s_container or k8s_pod).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeNamespace {
    /// The name of the parent cluster.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// The location of the parent cluster. This may be a zone or region.
    
    pub location: Option<String>,
    /// The name of this namespace.
    #[serde(rename="namespaceName")]
    
    pub namespace_name: Option<String>,
    /// Output only. The project this resource lives in. For legacy services migrated from the Custom type, this may be a distinct project from the one parenting the service itself.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for GkeNamespace {}


/// GKE Service. The "service" here represents a Kubernetes service object (https://kubernetes.io/docs/concepts/services-networking/service). The field names correspond to the resource labels on k8s_service monitored resources (https://cloud.google.com/monitoring/api/resources#tag_k8s_service).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeService {
    /// The name of the parent cluster.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// The location of the parent cluster. This may be a zone or region.
    
    pub location: Option<String>,
    /// The name of the parent namespace.
    #[serde(rename="namespaceName")]
    
    pub namespace_name: Option<String>,
    /// Output only. The project this resource lives in. For legacy services migrated from the Custom type, this may be a distinct project from the one parenting the service itself.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The name of this service.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
}

impl client::Part for GkeService {}


/// A GKE Workload (Deployment, StatefulSet, etc). The field names correspond to the metadata labels on monitored resources that fall under a workload (for example, k8s_container or k8s_pod).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeWorkload {
    /// The name of the parent cluster.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// The location of the parent cluster. This may be a zone or region.
    
    pub location: Option<String>,
    /// The name of the parent namespace.
    #[serde(rename="namespaceName")]
    
    pub namespace_name: Option<String>,
    /// Output only. The project this resource lives in. For legacy services migrated from the Custom type, this may be a distinct project from the one parenting the service itself.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The name of this workload.
    #[serde(rename="topLevelControllerName")]
    
    pub top_level_controller_name: Option<String>,
    /// The type of this workload (for example, "Deployment" or "DaemonSet")
    #[serde(rename="topLevelControllerType")]
    
    pub top_level_controller_type: Option<String>,
}

impl client::Part for GkeWorkload {}


/// Range of numerical values within min and max.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleMonitoringV3Range {
    /// Range maximum.
    
    pub max: Option<f64>,
    /// Range minimum.
    
    pub min: Option<f64>,
}

impl client::Part for GoogleMonitoringV3Range {}


/// The description of a dynamic collection of monitored resources. Each group has a filter that is matched against monitored resources and their associated metadata. If a group’s filter matches an available monitored resource, then that resource is a member of that group. Groups can contain any number of monitored resources, and each monitored resource can be a member of any number of groups.Groups can be nested in parent-child hierarchies. The parentName field identifies an optional parent for each group. If a group has a parent, then the only monitored resources available to be matched by the group’s filter are the resources contained in the parent group. In other words, a group contains the monitored resources that match its filter and the filters of all the group’s ancestors. A group without a parent can contain any monitored resource.For example, consider an infrastructure running a set of instances with two user-defined tags: “environment” and “role”. A parent group has a filter, environment=“production”. A child of that parent group has a filter, role=“transcoder”. The parent group contains all instances in the production environment, regardless of their roles. The child group contains instances that have the transcoder role and are in the production environment.The monitored resources contained in a group can change at any moment, depending on what resources exist and what filters are associated with the group and its ancestors.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [groups create projects](ProjectGroupCreateCall) (request|response)
/// * [groups get projects](ProjectGroupGetCall) (response)
/// * [groups update projects](ProjectGroupUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// A user-assigned name for this group, used only for display purposes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The filter used to determine which monitored resources belong to this group.
    
    pub filter: Option<String>,
    /// If true, the members of this group are considered to be a cluster. The system can perform additional analysis on groups that are clusters.
    #[serde(rename="isCluster")]
    
    pub is_cluster: Option<bool>,
    /// Output only. The name of this group. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] When creating a group, this field is ignored and a new name is created consisting of the project specified in the call to CreateGroup and a unique [GROUP_ID] that is generated automatically.
    
    pub name: Option<String>,
    /// The name of the group's parent, if it has one. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] For groups with no parent, parent_name is the empty string, "".
    #[serde(rename="parentName")]
    
    pub parent_name: Option<String>,
}

impl client::RequestValue for Group {}
impl client::ResponseResult for Group {}


/// Information involved in an HTTP/HTTPS Uptime check request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpCheck {
    /// If present, the check will only pass if the HTTP response status code is in this set of status codes. If empty, the HTTP status code will only pass if the HTTP status code is 200-299.
    #[serde(rename="acceptedResponseStatusCodes")]
    
    pub accepted_response_status_codes: Option<Vec<ResponseStatusCode>>,
    /// The authentication information. Optional when creating an HTTP check; defaults to empty.
    #[serde(rename="authInfo")]
    
    pub auth_info: Option<BasicAuthentication>,
    /// The request body associated with the HTTP POST request. If content_type is URL_ENCODED, the body passed in must be URL-encoded. Users can provide a Content-Length header via the headers field or the API will do so. If the request_method is GET and body is not empty, the API will return an error. The maximum byte size is 1 megabyte.Note: If client libraries aren't used (which performs the conversion automatically) base64 encode your body data since the field is of bytes type.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub body: Option<Vec<u8>>,
    /// The content type header to use for the check. The following configurations result in errors: 1. Content type is specified in both the headers field and the content_type field. 2. Request method is GET and content_type is not TYPE_UNSPECIFIED 3. Request method is POST and content_type is TYPE_UNSPECIFIED. 4. Request method is POST and a "Content-Type" header is provided via headers field. The content_type field should be used instead.
    #[serde(rename="contentType")]
    
    pub content_type: Option<HttpCheckContentTypeEnum>,
    /// The list of headers to send as part of the Uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described at https://www.w3.org/Protocols/rfc2616/rfc2616.txt (page 31). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100.
    
    pub headers: Option<HashMap<String, String>>,
    /// Boolean specifying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if mask_headers is set to true then the headers will be obscured with ******.
    #[serde(rename="maskHeaders")]
    
    pub mask_headers: Option<bool>,
    /// Optional (defaults to "/"). The path to the page against which to run the check. Will be combined with the host (specified within the monitored_resource) and port to construct the full URL. If the provided path does not begin with "/", a "/" will be prepended automatically.
    
    pub path: Option<String>,
    /// Contains information needed to add pings to an HTTP check.
    #[serde(rename="pingConfig")]
    
    pub ping_config: Option<PingConfig>,
    /// Optional (defaults to 80 when use_ssl is false, and 443 when use_ssl is true). The TCP port on the HTTP server against which to run the check. Will be combined with host (specified within the monitored_resource) and path to construct the full URL.
    
    pub port: Option<i32>,
    /// The HTTP request method to use for the check. If set to METHOD_UNSPECIFIED then request_method defaults to GET.
    #[serde(rename="requestMethod")]
    
    pub request_method: Option<HttpCheckRequestMethodEnum>,
    /// If true, use HTTPS instead of HTTP to run the check.
    #[serde(rename="useSsl")]
    
    pub use_ssl: Option<bool>,
    /// Boolean specifying whether to include SSL certificate validation as a part of the Uptime check. Only applies to checks where monitored_resource is set to uptime_url. If use_ssl is false, setting validate_ssl to true has no effect.
    #[serde(rename="validateSsl")]
    
    pub validate_ssl: Option<bool>,
}

impl client::Part for HttpCheck {}


/// An internal checker allows Uptime checks to run on private/internal GCP resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InternalChecker {
    /// The checker's human-readable name. The display name should be unique within a Cloud Monitoring Metrics Scope in order to make it easier to identify; however, uniqueness is not enforced.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The GCP zone the Uptime check should egress from. Only respected for internal Uptime checks, where internal_network is specified.
    #[serde(rename="gcpZone")]
    
    pub gcp_zone: Option<String>,
    /// A unique resource name for this InternalChecker. The format is: projects/[PROJECT_ID_OR_NUMBER]/internalCheckers/[INTERNAL_CHECKER_ID] [PROJECT_ID_OR_NUMBER] is the Cloud Monitoring Metrics Scope project for the Uptime check config associated with the internal checker.
    
    pub name: Option<String>,
    /// The GCP VPC network (https://cloud.google.com/vpc/docs/vpc) where the internal resource lives (ex: "default").
    
    pub network: Option<String>,
    /// The GCP project ID where the internal checker lives. Not necessary the same as the Metrics Scope project.
    #[serde(rename="peerProjectId")]
    
    pub peer_project_id: Option<String>,
    /// The current operational state of the internal checker.
    
    pub state: Option<InternalCheckerStateEnum>,
}

impl client::Part for InternalChecker {}


/// Canonical service scoped to an Istio mesh. Anthos clusters running ASM >= 1.6.8 will have their services ingested as this type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IstioCanonicalService {
    /// The name of the canonical service underlying this service. Corresponds to the destination_canonical_service_name metric label in label in Istio metrics (https://cloud.google.com/monitoring/api/metrics_istio).
    #[serde(rename="canonicalService")]
    
    pub canonical_service: Option<String>,
    /// The namespace of the canonical service underlying this service. Corresponds to the destination_canonical_service_namespace metric label in Istio metrics (https://cloud.google.com/monitoring/api/metrics_istio).
    #[serde(rename="canonicalServiceNamespace")]
    
    pub canonical_service_namespace: Option<String>,
    /// Identifier for the Istio mesh in which this canonical service is defined. Corresponds to the mesh_uid metric label in Istio metrics (https://cloud.google.com/monitoring/api/metrics_istio).
    #[serde(rename="meshUid")]
    
    pub mesh_uid: Option<String>,
}

impl client::Part for IstioCanonicalService {}


/// Information needed to perform a JSONPath content match. Used for ContentMatcherOption::MATCHES_JSON_PATH and ContentMatcherOption::NOT_MATCHES_JSON_PATH.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JsonPathMatcher {
    /// The type of JSONPath match that will be applied to the JSON output (ContentMatcher.content)
    #[serde(rename="jsonMatcher")]
    
    pub json_matcher: Option<JsonPathMatcherJsonMatcherEnum>,
    /// JSONPath within the response output pointing to the expected ContentMatcher::content to match against.
    #[serde(rename="jsonPath")]
    
    pub json_path: Option<String>,
}

impl client::Part for JsonPathMatcher {}


/// A description of a label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelDescriptor {
    /// A human-readable description for the label.
    
    pub description: Option<String>,
    /// The key for this label. The key must meet the following criteria: Does not exceed 100 characters. Matches the following regular expression: [a-zA-Z][a-zA-Z0-9_]* The first character must be an upper- or lower-case letter. The remaining characters must be letters, digits, or underscores.
    
    pub key: Option<String>,
    /// The type of data that can be assigned to the label.
    #[serde(rename="valueType")]
    
    pub value_type: Option<LabelDescriptorValueTypeEnum>,
}

impl client::Part for LabelDescriptor {}


/// A label value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelValue {
    /// A bool label value.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// An int64 label value.
    #[serde(rename="int64Value")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int64_value: Option<i64>,
    /// A string label value.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for LabelValue {}


/// Parameters for a latency threshold SLI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatencyCriteria {
    /// Good service is defined to be the count of requests made to this service that return in no more than threshold.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub threshold: Option<client::chrono::Duration>,
}

impl client::Part for LatencyCriteria {}


/// Specifies a linear sequence of buckets that all have the same width (except overflow and underflow). Each bucket represents a constant absolute uncertainty on the specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): offset + (width * i). Lower bound (1 <= i < N): offset + (width * (i - 1)).
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


/// The protocol for the ListAlertPolicies response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [alert policies list projects](ProjectAlertPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAlertPoliciesResponse {
    /// The returned alert policies.
    #[serde(rename="alertPolicies")]
    
    pub alert_policies: Option<Vec<AlertPolicy>>,
    /// If there might be more results than were returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of alert policies in all pages. This number is only an estimate, and may change in subsequent pages. https://aip.dev/158
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListAlertPoliciesResponse {}


/// The ListGroupMembers response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [groups members list projects](ProjectGroupMemberListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGroupMembersResponse {
    /// A set of monitored resources in the group.
    
    pub members: Option<Vec<MonitoredResource>>,
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of elements matching this request.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListGroupMembersResponse {}


/// The ListGroups response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [groups list projects](ProjectGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGroupsResponse {
    /// The groups that match the specified filters.
    
    pub group: Option<Vec<Group>>,
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGroupsResponse {}


/// The ListMetricDescriptors response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metric descriptors list projects](ProjectMetricDescriptorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMetricDescriptorsResponse {
    /// The metric descriptors that are available to the project and that match the value of filter, if present.
    #[serde(rename="metricDescriptors")]
    
    pub metric_descriptors: Option<Vec<MetricDescriptor>>,
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMetricDescriptorsResponse {}


/// The ListMonitoredResourceDescriptors response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [monitored resource descriptors list projects](ProjectMonitoredResourceDescriptorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The monitored resource descriptors that are available to this project and that match filter, if present.
    #[serde(rename="resourceDescriptors")]
    
    pub resource_descriptors: Option<Vec<MonitoredResourceDescriptor>>,
}

impl client::ResponseResult for ListMonitoredResourceDescriptorsResponse {}


/// The ListNotificationChannelDescriptors response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channel descriptors list projects](ProjectNotificationChannelDescriptorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNotificationChannelDescriptorsResponse {
    /// The monitored resource descriptors supported for the specified project, optionally filtered.
    #[serde(rename="channelDescriptors")]
    
    pub channel_descriptors: Option<Vec<NotificationChannelDescriptor>>,
    /// If not empty, indicates that there may be more results that match the request. Use the value in the page_token field in a subsequent request to fetch the next set of results. If empty, all results have been returned.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListNotificationChannelDescriptorsResponse {}


/// The ListNotificationChannels response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channels list projects](ProjectNotificationChannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNotificationChannelsResponse {
    /// If not empty, indicates that there may be more results that match the request. Use the value in the page_token field in a subsequent request to fetch the next set of results. If empty, all results have been returned.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The notification channels defined for the specified project.
    #[serde(rename="notificationChannels")]
    
    pub notification_channels: Option<Vec<NotificationChannel>>,
    /// The total number of notification channels in all pages. This number is only an estimate, and may change in subsequent pages. https://aip.dev/158
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListNotificationChannelsResponse {}


/// The ListServiceLevelObjectives response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service level objectives list services](ServiceServiceLevelObjectiveListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceLevelObjectivesResponse {
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The ServiceLevelObjectives matching the specified filter.
    #[serde(rename="serviceLevelObjectives")]
    
    pub service_level_objectives: Option<Vec<ServiceLevelObjective>>,
}

impl client::ResponseResult for ListServiceLevelObjectivesResponse {}


/// The ListServices response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list services](ServiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServicesResponse {
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The Services matching the specified filter.
    
    pub services: Option<Vec<Service>>,
}

impl client::ResponseResult for ListServicesResponse {}


/// The results of a successful ListSnoozes call, containing the matching Snoozes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [snoozes list projects](ProjectSnoozeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSnoozesResponse {
    /// Page token for repeated calls to ListSnoozes, to fetch additional pages of results. If this is empty or missing, there are no more pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Snoozes matching this list call.
    
    pub snoozes: Option<Vec<Snooze>>,
}

impl client::ResponseResult for ListSnoozesResponse {}


/// The ListTimeSeries response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [time series list folders](FolderTimeSeryListCall) (response)
/// * [time series list organizations](OrganizationTimeSeryListCall) (response)
/// * [time series list projects](ProjectTimeSeryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeSeriesResponse {
    /// Query execution errors that may have caused the time series data returned to be incomplete.
    #[serde(rename="executionErrors")]
    
    pub execution_errors: Option<Vec<Status>>,
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// One or more time series that match the filter included in the request.
    #[serde(rename="timeSeries")]
    
    pub time_series: Option<Vec<TimeSeries>>,
    /// The unit in which all time_series point values are reported. unit follows the UCUM format for units as seen in https://unitsofmeasure.org/ucum.html. If different time_series have different units (for example, because they come from different metric types, or a unit is absent), then unit will be "{not_a_unit}".
    
    pub unit: Option<String>,
}

impl client::ResponseResult for ListTimeSeriesResponse {}


/// The protocol for the ListUptimeCheckConfigs response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uptime check configs list projects](ProjectUptimeCheckConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUptimeCheckConfigsResponse {
    /// This field represents the pagination token to retrieve the next page of results. If the value is empty, it means no further results for the request. To retrieve the next page of results, the value of the next_page_token is passed to the subsequent List method call (in the request message's page_token field).
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of Uptime check configurations for the project, irrespective of any pagination.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
    /// The returned Uptime check configurations.
    #[serde(rename="uptimeCheckConfigs")]
    
    pub uptime_check_configs: Option<Vec<UptimeCheckConfig>>,
}

impl client::ResponseResult for ListUptimeCheckConfigsResponse {}


/// The protocol for the ListUptimeCheckIps response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list uptime check ips](UptimeCheckIpListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUptimeCheckIpsResponse {
    /// This field represents the pagination token to retrieve the next page of results. If the value is empty, it means no further results for the request. To retrieve the next page of results, the value of the next_page_token is passed to the subsequent List method call (in the request message's page_token field). NOTE: this field is not yet implemented
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The returned list of IP addresses (including region and location) that the checkers run from.
    #[serde(rename="uptimeCheckIps")]
    
    pub uptime_check_ips: Option<Vec<UptimeCheckIp>>,
}

impl client::ResponseResult for ListUptimeCheckIpsResponse {}


/// A condition type that checks whether a log message in the scoping project (https://cloud.google.com/monitoring/api/v3#project_name) satisfies the given filter. Logs from other projects in the metrics scope are not evaluated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogMatch {
    /// Required. A logs-based filter. See Advanced Logs Queries (https://cloud.google.com/logging/docs/view/advanced-queries) for how this filter should be constructed.
    
    pub filter: Option<String>,
    /// Optional. A map from a label key to an extractor expression, which is used to extract the value for this label key. Each entry in this map is a specification for how data should be extracted from log entries that match filter. Each combination of extracted values is treated as a separate rule for the purposes of triggering notifications. Label keys and corresponding values can be used in notifications generated by this condition.Please see the documentation on logs-based metric valueExtractors (https://cloud.google.com/logging/docs/reference/v2/rest/v2/projects.metrics#LogMetric.FIELDS.value_extractor) for syntax and examples.
    #[serde(rename="labelExtractors")]
    
    pub label_extractors: Option<HashMap<String, String>>,
}

impl client::Part for LogMatch {}


/// Istio service scoped to an Istio mesh. Anthos clusters running ASM < 1.6.8 will have their services ingested as this type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MeshIstio {
    /// Identifier for the mesh in which this Istio service is defined. Corresponds to the mesh_uid metric label in Istio metrics.
    #[serde(rename="meshUid")]
    
    pub mesh_uid: Option<String>,
    /// The name of the Istio service underlying this service. Corresponds to the destination_service_name metric label in Istio metrics.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
    /// The namespace of the Istio service underlying this service. Corresponds to the destination_service_namespace metric label in Istio metrics.
    #[serde(rename="serviceNamespace")]
    
    pub service_namespace: Option<String>,
}

impl client::Part for MeshIstio {}


/// A specific metric, identified by specifying values for all of the labels of a MetricDescriptor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// The set of label values that uniquely identify this metric. All labels listed in the MetricDescriptor must be assigned values.
    
    pub labels: Option<HashMap<String, String>>,
    /// An existing metric type, see google.api.MetricDescriptor. For example, custom.googleapis.com/invoice/paid/amount.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Metric {}


/// A condition type that checks that monitored resources are reporting data. The configuration defines a metric and a set of monitored resources. The predicate is considered in violation when a time series for the specified metric of a monitored resource does not include any data in the specified duration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricAbsence {
    /// Specifies the alignment of data points in individual time series as well as how to combine the retrieved time series together (such as when aggregating multiple streams on each resource to a single stream for each resource or when aggregating streams across all members of a group of resources). Multiple aggregations are applied in the order specified.This field is similar to the one in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list). It is advisable to use the ListTimeSeries method when debugging this field.
    
    pub aggregations: Option<Vec<Aggregation>>,
    /// The amount of time that a time series must fail to report new data to be considered failing. The minimum value of this field is 120 seconds. Larger values that are a multiple of a minute--for example, 240 or 300 seconds--are supported. If an invalid value is given, an error will be returned. The Duration.nanos field is ignored.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Required. A filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies which time series should be compared with the threshold.The filter is similar to the one that is specified in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list) (that call is useful to verify the time series that will be retrieved / processed). The filter must specify the metric type and the resource type. Optionally, it can specify resource labels and metric labels. This field must not exceed 2048 Unicode characters in length.
    
    pub filter: Option<String>,
    /// The number/percent of time series for which the comparison must hold in order for the condition to trigger. If unspecified, then the condition will trigger if the comparison is true for any of the time series that have been identified by filter and aggregations.
    
    pub trigger: Option<Trigger>,
}

impl client::Part for MetricAbsence {}


/// Defines a metric type and its schema. Once a metric descriptor is created, deleting or altering it stops data collection and makes the metric type’s existing data unusable.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [metric descriptors create projects](ProjectMetricDescriptorCreateCall) (request|response)
/// * [metric descriptors get projects](ProjectMetricDescriptorGetCall) (response)
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
    /// Optional. The launch stage of the metric definition.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<MetricDescriptorLaunchStageEnum>,
    /// Optional. Metadata which can be used to guide usage of the metric.
    
    pub metadata: Option<MetricDescriptorMetadata>,
    /// Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported.
    #[serde(rename="metricKind")]
    
    pub metric_kind: Option<MetricDescriptorMetricKindEnum>,
    /// Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here.
    #[serde(rename="monitoredResourceTypes")]
    
    pub monitored_resource_types: Option<Vec<String>>,
    /// The resource name of the metric descriptor.
    
    pub name: Option<String>,
    /// The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name custom.googleapis.com or external.googleapis.com. Metric types should use a natural hierarchical grouping. For example: "custom.googleapis.com/invoice/paid/amount" "external.googleapis.com/prometheus/up" "appengine.googleapis.com/http/server/response_latencies" 
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The units in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored metric values.Different systems might scale the values to be more easily displayed (so a value of 0.02kBy might be displayed as 20By, and a value of 3523kBy might be displayed as 3.5MBy). However, if the unit is kBy, then the value of the metric is always in thousands of bytes, no matter how it might be displayed.If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently 1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as 12005.Alternatively, if you want a custom metric to record data in a more granular way, you can create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value 12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).The supported units are a subset of The Unified Code for Units of Measure (https://unitsofmeasure.org/ucum.html) standard:Basic units (UNIT) bit bit By byte s second min minute h hour d day 1 dimensionlessPrefixes (PREFIX) k kilo (10^3) M mega (10^6) G giga (10^9) T tera (10^12) P peta (10^15) E exa (10^18) Z zetta (10^21) Y yotta (10^24) m milli (10^-3) u micro (10^-6) n nano (10^-9) p pico (10^-12) f femto (10^-15) a atto (10^-18) z zepto (10^-21) y yocto (10^-24) Ki kibi (2^10) Mi mebi (2^20) Gi gibi (2^30) Ti tebi (2^40) Pi pebi (2^50)GrammarThe grammar also includes these connectors: / division or ratio (as an infix operator). For examples, kBy/{email} or MiBy/10ms (although you should almost never have /s in a metric unit; rates should always be computed at query time from the underlying cumulative or delta value). . multiplication or composition (as an infix operator). For examples, GBy.d or k{watt}.h.The grammar for a unit is as follows: Expression = Component { "." Component } { "/" Component } ; Component = ( [ PREFIX ] UNIT | "%" ) [ Annotation ] | Annotation | "1" ; Annotation = "{" NAME "}" ; Notes: Annotation is just a comment if it follows a UNIT. If the annotation is used alone, then the unit is equivalent to 1. For examples, {request}/s == 1/s, By{transmitted}/s == By/s. NAME is a sequence of non-blank printable ASCII characters not containing { or }. 1 represents a unitary dimensionless unit (https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in 1/s. It is typically used when none of the basic units are appropriate. For example, "new users per day" can be represented as 1/d or {new-users}/d (and a metric value 5 would mean "5 new users). Alternatively, "thousands of page views per day" would be represented as 1000/d or k1/d or k{page_views}/d (and a metric value of 5.3 would mean "5300 page views per day"). % represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value 3 means "3 percent"). 10^2.% indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value 0.03 means "3 percent").
    
    pub unit: Option<String>,
    /// Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported.
    #[serde(rename="valueType")]
    
    pub value_type: Option<MetricDescriptorValueTypeEnum>,
}

impl client::RequestValue for MetricDescriptor {}
impl client::ResponseResult for MetricDescriptor {}


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
    /// Deprecated. Must use the MetricDescriptor.launch_stage instead.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<MetricDescriptorMetadataLaunchStageEnum>,
    /// The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period.
    #[serde(rename="samplePeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_period: Option<client::chrono::Duration>,
}

impl client::Part for MetricDescriptorMetadata {}


/// A MetricRange is used when each window is good when the value x of a single TimeSeries satisfies range.min <= x <= range.max. The provided TimeSeries must have ValueType = INT64 or ValueType = DOUBLE and MetricKind = GAUGE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricRange {
    /// Range of values considered "good." For a one-sided range, set one bound to an infinite value.
    
    pub range: Option<GoogleMonitoringV3Range>,
    /// A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying the TimeSeries to use for evaluating window quality.
    #[serde(rename="timeSeries")]
    
    pub time_series: Option<String>,
}

impl client::Part for MetricRange {}


/// A condition type that compares a collection of time series against a threshold.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricThreshold {
    /// Specifies the alignment of data points in individual time series as well as how to combine the retrieved time series together (such as when aggregating multiple streams on each resource to a single stream for each resource or when aggregating streams across all members of a group of resources). Multiple aggregations are applied in the order specified.This field is similar to the one in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list). It is advisable to use the ListTimeSeries method when debugging this field.
    
    pub aggregations: Option<Vec<Aggregation>>,
    /// The comparison to apply between the time series (indicated by filter and aggregation) and the threshold (indicated by threshold_value). The comparison is applied on each time series, with the time series on the left-hand side and the threshold on the right-hand side.Only COMPARISON_LT and COMPARISON_GT are supported currently.
    
    pub comparison: Option<MetricThresholdComparisonEnum>,
    /// Specifies the alignment of data points in individual time series selected by denominatorFilter as well as how to combine the retrieved time series together (such as when aggregating multiple streams on each resource to a single stream for each resource or when aggregating streams across all members of a group of resources).When computing ratios, the aggregations and denominator_aggregations fields must use the same alignment period and produce time series that have the same periodicity and labels.
    #[serde(rename="denominatorAggregations")]
    
    pub denominator_aggregations: Option<Vec<Aggregation>>,
    /// A filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies a time series that should be used as the denominator of a ratio that will be compared with the threshold. If a denominator_filter is specified, the time series specified by the filter field will be used as the numerator.The filter must specify the metric type and optionally may contain restrictions on resource type, resource labels, and metric labels. This field may not exceed 2048 Unicode characters in length.
    #[serde(rename="denominatorFilter")]
    
    pub denominator_filter: Option<String>,
    /// The amount of time that a time series must violate the threshold to be considered failing. Currently, only values that are a multiple of a minute--e.g., 0, 60, 120, or 300 seconds--are supported. If an invalid value is given, an error will be returned. When choosing a duration, it is useful to keep in mind the frequency of the underlying time series data (which may also be affected by any alignments specified in the aggregations field); a good duration is long enough so that a single outlier does not generate spurious alerts, but short enough that unhealthy states are detected and alerted on quickly.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// A condition control that determines how metric-threshold conditions are evaluated when data stops arriving.
    #[serde(rename="evaluationMissingData")]
    
    pub evaluation_missing_data: Option<MetricThresholdEvaluationMissingDataEnum>,
    /// Required. A filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies which time series should be compared with the threshold.The filter is similar to the one that is specified in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list) (that call is useful to verify the time series that will be retrieved / processed). The filter must specify the metric type and the resource type. Optionally, it can specify resource labels and metric labels. This field must not exceed 2048 Unicode characters in length.
    
    pub filter: Option<String>,
    /// When this field is present, the MetricThreshold condition forecasts whether the time series is predicted to violate the threshold within the forecast_horizion. When this field is not set, the MetricThreshold tests the current value of the timeseries against the threshold.
    #[serde(rename="forecastOptions")]
    
    pub forecast_options: Option<ForecastOptions>,
    /// A value against which to compare the time series.
    #[serde(rename="thresholdValue")]
    
    pub threshold_value: Option<f64>,
    /// The number/percent of time series for which the comparison must hold in order for the condition to trigger. If unspecified, then the condition will trigger if the comparison is true for any of the time series that have been identified by filter and aggregations, or by the ratio, if denominator_filter and denominator_aggregations are specified.
    
    pub trigger: Option<Trigger>,
}

impl client::Part for MetricThreshold {}


/// An object representing a resource that can be used for monitoring, logging, billing, or other purposes. Examples include virtual machine instances, databases, and storage devices such as disks. The type field identifies a MonitoredResourceDescriptor object that describes the resource's schema. Information in the labels field identifies the actual resource and its attributes according to the schema. For example, a particular Compute Engine VM instance could be represented by the following object, because the MonitoredResourceDescriptor for "gce_instance" has labels "project_id", "instance_id" and "zone": { "type": "gce_instance", "labels": { "project_id": "my-project", "instance_id": "12345678901234", "zone": "us-central1-a" }} 
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResource {
    /// Required. Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels "project_id", "instance_id", and "zone".
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The monitored resource type. This field must match the type field of a MonitoredResourceDescriptor object. For example, the type of a Compute Engine VM instance is gce_instance. For a list of types, see Monitoring resource types (https://cloud.google.com/monitoring/api/resources) and Logging resource types (https://cloud.google.com/logging/docs/api/v2/resource-list).
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
/// * [monitored resource descriptors get projects](ProjectMonitoredResourceDescriptorGetCall) (response)
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
    /// Optional. The launch stage of the monitored resource definition.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<MonitoredResourceDescriptorLaunchStageEnum>,
    /// Optional. The resource name of the monitored resource descriptor: "projects/{project_id}/monitoredResourceDescriptors/{type}" where {type} is the value of the type field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format "monitoredResourceDescriptors/{type}".
    
    pub name: Option<String>,
    /// Required. The monitored resource type. For example, the type "cloudsql_database" represents databases in Google Cloud SQL. For a list of types, see Monitoring resource types (https://cloud.google.com/monitoring/api/resources) and Logging resource types (https://cloud.google.com/logging/docs/api/v2/resource-list).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for MonitoredResourceDescriptor {}


/// Auxiliary metadata for a MonitoredResource object. MonitoredResource objects contain the minimum set of information to uniquely identify a monitored resource instance. There is some other useful auxiliary metadata. Monitoring and Logging use an ingestion pipeline to extract metadata for cloud resources of all types, and store the metadata in this message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResourceMetadata {
    /// Output only. Values for predefined system metadata labels. System labels are a kind of metadata extracted by Google, including "machine_image", "vpc", "subnet_id", "security_group", "name", etc. System label values can be only strings, Boolean values, or a list of strings. For example: { "name": "my-test-instance", "security_group": ["a", "b", "c"], "spot_instance": false } 
    #[serde(rename="systemLabels")]
    
    pub system_labels: Option<HashMap<String, json::Value>>,
    /// Output only. A map of user-defined metadata labels.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::Part for MonitoredResourceMetadata {}


/// A condition type that allows alert policies to be defined using Monitoring Query Language (https://cloud.google.com/monitoring/mql).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringQueryLanguageCondition {
    /// The amount of time that a time series must violate the threshold to be considered failing. Currently, only values that are a multiple of a minute--e.g., 0, 60, 120, or 300 seconds--are supported. If an invalid value is given, an error will be returned. When choosing a duration, it is useful to keep in mind the frequency of the underlying time series data (which may also be affected by any alignments specified in the aggregations field); a good duration is long enough so that a single outlier does not generate spurious alerts, but short enough that unhealthy states are detected and alerted on quickly.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// A condition control that determines how metric-threshold conditions are evaluated when data stops arriving.
    #[serde(rename="evaluationMissingData")]
    
    pub evaluation_missing_data: Option<MonitoringQueryLanguageConditionEvaluationMissingDataEnum>,
    /// Monitoring Query Language (https://cloud.google.com/monitoring/mql) query that outputs a boolean stream.
    
    pub query: Option<String>,
    /// The number/percent of time series for which the comparison must hold in order for the condition to trigger. If unspecified, then the condition will trigger if the comparison is true for any of the time series that have been identified by filter and aggregations, or by the ratio, if denominator_filter and denominator_aggregations are specified.
    
    pub trigger: Option<Trigger>,
}

impl client::Part for MonitoringQueryLanguageCondition {}


/// Describes a change made to a configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MutationRecord {
    /// When the change occurred.
    #[serde(rename="mutateTime")]
    
    pub mutate_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The email address of the user making the change.
    #[serde(rename="mutatedBy")]
    
    pub mutated_by: Option<String>,
}

impl client::Part for MutationRecord {}


/// A NotificationChannel is a medium through which an alert is delivered when a policy violation is detected. Examples of channels include email, SMS, and third-party messaging applications. Fields containing sensitive information like authentication tokens or contact info are only partially populated on retrieval.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channels create projects](ProjectNotificationChannelCreateCall) (request|response)
/// * [notification channels get projects](ProjectNotificationChannelGetCall) (response)
/// * [notification channels patch projects](ProjectNotificationChannelPatchCall) (request|response)
/// * [notification channels verify projects](ProjectNotificationChannelVerifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Record of the creation of this channel.
    #[serde(rename="creationRecord")]
    
    pub creation_record: Option<MutationRecord>,
    /// An optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters.
    
    pub description: Option<String>,
    /// An optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Whether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future.
    
    pub enabled: Option<bool>,
    /// Configuration fields that define the channel and its behavior. The permissible and required labels are specified in the NotificationChannelDescriptor.labels of the NotificationChannelDescriptor corresponding to the type field.
    
    pub labels: Option<HashMap<String, String>>,
    /// Records of the modification of this channel.
    #[serde(rename="mutationRecords")]
    
    pub mutation_records: Option<Vec<MutationRecord>>,
    /// The full REST resource name for this channel. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] The [CHANNEL_ID] is automatically assigned by the server on creation.
    
    pub name: Option<String>,
    /// The type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// User-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
    /// Indicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel.
    #[serde(rename="verificationStatus")]
    
    pub verification_status: Option<NotificationChannelVerificationStatusEnum>,
}

impl client::RequestValue for NotificationChannel {}
impl client::ResponseResult for NotificationChannel {}


/// A description of a notification channel. The descriptor includes the properties of the channel and the set of labels or fields that must be specified to configure channels of a given type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channel descriptors get projects](ProjectNotificationChannelDescriptorGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationChannelDescriptor {
    /// A human-readable description of the notification channel type. The description may include a description of the properties of the channel and pointers to external documentation.
    
    pub description: Option<String>,
    /// A human-readable name for the notification channel type. This form of the name is suitable for a user interface.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The set of labels that must be defined to identify a particular channel of the corresponding type. Each label includes a description for how that field should be populated.
    
    pub labels: Option<Vec<LabelDescriptor>>,
    /// The product launch stage for channels of this type.
    #[serde(rename="launchStage")]
    
    pub launch_stage: Option<NotificationChannelDescriptorLaunchStageEnum>,
    /// The full REST resource name for this descriptor. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannelDescriptors/[TYPE] In the above, [TYPE] is the value of the type field.
    
    pub name: Option<String>,
    /// The tiers that support this notification channel; the project service tier must be one of the supported_tiers.
    #[serde(rename="supportedTiers")]
    
    pub supported_tiers: Option<Vec<NotificationChannelDescriptorSupportedTiersEnum>>,
    /// The type of notification channel, such as "email" and "sms". To view the full list of channels, see Channel descriptors (https://cloud.google.com/monitoring/alerts/using-channels-api#ncd). Notification channel types are globally unique.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for NotificationChannelDescriptor {}


/// Control over the rate of notifications sent to this alert policy's notification channels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationRateLimit {
    /// Not more than one notification per period.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub period: Option<client::chrono::Duration>,
}

impl client::Part for NotificationRateLimit {}


/// A PerformanceThreshold is used when each window is good when that window has a sufficiently high performance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceThreshold {
    /// BasicSli to evaluate to judge window quality.
    #[serde(rename="basicSliPerformance")]
    
    pub basic_sli_performance: Option<BasicSli>,
    /// RequestBasedSli to evaluate to judge window quality.
    
    pub performance: Option<RequestBasedSli>,
    /// If window performance >= threshold, the window is counted as good.
    
    pub threshold: Option<f64>,
}

impl client::Part for PerformanceThreshold {}


/// Information involved in sending ICMP pings alongside public HTTP/TCP checks. For HTTP, the pings are performed for each part of the redirect chain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PingConfig {
    /// Number of ICMP pings. A maximum of 3 ICMP pings is currently supported.
    #[serde(rename="pingsCount")]
    
    pub pings_count: Option<i32>,
}

impl client::Part for PingConfig {}


/// A single data point in a time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    /// The time interval to which the data point applies. For GAUGE metrics, the start time is optional, but if it is supplied, it must equal the end time. For DELTA metrics, the start and end time should specify a non-zero interval, with subsequent points specifying contiguous and non-overlapping intervals. For CUMULATIVE metrics, the start and end time should specify a non-zero interval, with subsequent points specifying the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points.
    
    pub interval: Option<TimeInterval>,
    /// The value of the data point.
    
    pub value: Option<TypedValue>,
}

impl client::Part for Point {}


/// A point's value columns and time interval. Each point has one or more point values corresponding to the entries in point_descriptors field in the TimeSeriesDescriptor associated with this object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointData {
    /// The time interval associated with the point.
    #[serde(rename="timeInterval")]
    
    pub time_interval: Option<TimeInterval>,
    /// The values that make up the point.
    
    pub values: Option<Vec<TypedValue>>,
}

impl client::Part for PointData {}


/// The QueryTimeSeries request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [time series query projects](ProjectTimeSeryQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryTimeSeriesRequest {
    /// A positive number that is the maximum number of time_series_data to return.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Required. The query in the Monitoring Query Language (https://cloud.google.com/monitoring/mql/reference) format. The default time zone is in UTC.
    
    pub query: Option<String>,
}

impl client::RequestValue for QueryTimeSeriesRequest {}


/// The QueryTimeSeries response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [time series query projects](ProjectTimeSeryQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryTimeSeriesResponse {
    /// If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Query execution errors that may have caused the time series data returned to be incomplete. The available data will be available in the response.
    #[serde(rename="partialErrors")]
    
    pub partial_errors: Option<Vec<Status>>,
    /// The time series data.
    #[serde(rename="timeSeriesData")]
    
    pub time_series_data: Option<Vec<TimeSeriesData>>,
    /// The descriptor for the time series data.
    #[serde(rename="timeSeriesDescriptor")]
    
    pub time_series_descriptor: Option<TimeSeriesDescriptor>,
}

impl client::ResponseResult for QueryTimeSeriesResponse {}


/// The range of the population values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Range {
    /// The maximum of the population values.
    
    pub max: Option<f64>,
    /// The minimum of the population values.
    
    pub min: Option<f64>,
}

impl client::Part for Range {}


/// Service Level Indicators for which atomic units of service are counted directly.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestBasedSli {
    /// distribution_cut is used when good_service is a count of values aggregated in a Distribution that fall into a good range. The total_service is the total count of all values aggregated in the Distribution.
    #[serde(rename="distributionCut")]
    
    pub distribution_cut: Option<DistributionCut>,
    /// good_total_ratio is used when the ratio of good_service to total_service is computed from two TimeSeries.
    #[serde(rename="goodTotalRatio")]
    
    pub good_total_ratio: Option<TimeSeriesRatio>,
}

impl client::Part for RequestBasedSli {}


/// The resource submessage for group checks. It can be used instead of a monitored resource, when multiple resources are being monitored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceGroup {
    /// The group of resources being monitored. Should be only the [GROUP_ID], and not the full-path projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID].
    #[serde(rename="groupId")]
    
    pub group_id: Option<String>,
    /// The resource type of the group members.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<ResourceGroupResourceTypeEnum>,
}

impl client::Part for ResourceGroup {}


/// A status to accept. Either a status code class like "2xx", or an integer status code like "200".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseStatusCode {
    /// A class of status codes to accept.
    #[serde(rename="statusClass")]
    
    pub status_class: Option<ResponseStatusCodeStatusClassEnum>,
    /// A status code to accept.
    #[serde(rename="statusValue")]
    
    pub status_value: Option<i32>,
}

impl client::Part for ResponseStatusCode {}


/// The SendNotificationChannelVerificationCode request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channels send verification code projects](ProjectNotificationChannelSendVerificationCodeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendNotificationChannelVerificationCodeRequest { _never_set: Option<bool> }

impl client::RequestValue for SendNotificationChannelVerificationCodeRequest {}


/// A Service is a discrete, autonomous, and network-accessible unit, designed to solve an individual concern (Wikipedia (https://en.wikipedia.org/wiki/Service-orientation)). In Cloud Monitoring, a Service acts as the root resource under which operational aspects of the service are accessible.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service level objectives create services](ServiceServiceLevelObjectiveCreateCall) (none)
/// * [service level objectives delete services](ServiceServiceLevelObjectiveDeleteCall) (none)
/// * [service level objectives get services](ServiceServiceLevelObjectiveGetCall) (none)
/// * [service level objectives list services](ServiceServiceLevelObjectiveListCall) (none)
/// * [service level objectives patch services](ServiceServiceLevelObjectivePatchCall) (none)
/// * [create services](ServiceCreateCall) (request|response)
/// * [delete services](ServiceDeleteCall) (none)
/// * [get services](ServiceGetCall) (response)
/// * [list services](ServiceListCall) (none)
/// * [patch services](ServicePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// Type used for App Engine services.
    #[serde(rename="appEngine")]
    
    pub app_engine: Option<AppEngine>,
    /// Message that contains the service type and service labels of this service if it is a basic service. Documentation and examples here (https://cloud.google.com/stackdriver/docs/solutions/slo-monitoring/api/api-structures#basic-svc-w-basic-sli).
    #[serde(rename="basicService")]
    
    pub basic_service: Option<BasicService>,
    /// Type used for Cloud Endpoints services.
    #[serde(rename="cloudEndpoints")]
    
    pub cloud_endpoints: Option<CloudEndpoints>,
    /// Type used for Cloud Run services.
    #[serde(rename="cloudRun")]
    
    pub cloud_run: Option<CloudRun>,
    /// Type used for Istio services that live in a Kubernetes cluster.
    #[serde(rename="clusterIstio")]
    
    pub cluster_istio: Option<ClusterIstio>,
    /// Custom service type.
    
    pub custom: Option<Custom>,
    /// Name used for UI elements listing this Service.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Type used for GKE Namespaces.
    #[serde(rename="gkeNamespace")]
    
    pub gke_namespace: Option<GkeNamespace>,
    /// Type used for GKE Services (the Kubernetes concept of a service).
    #[serde(rename="gkeService")]
    
    pub gke_service: Option<GkeService>,
    /// Type used for GKE Workloads.
    #[serde(rename="gkeWorkload")]
    
    pub gke_workload: Option<GkeWorkload>,
    /// Type used for canonical services scoped to an Istio mesh. Metrics for Istio are documented here (https://istio.io/latest/docs/reference/config/metrics/)
    #[serde(rename="istioCanonicalService")]
    
    pub istio_canonical_service: Option<IstioCanonicalService>,
    /// Type used for Istio services scoped to an Istio mesh.
    #[serde(rename="meshIstio")]
    
    pub mesh_istio: Option<MeshIstio>,
    /// Resource name for this Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] 
    
    pub name: Option<String>,
    /// Configuration for how to query telemetry on a Service.
    
    pub telemetry: Option<Telemetry>,
    /// Labels which have been used to annotate the service. Label keys must start with a letter. Label keys and values may contain lowercase letters, numbers, underscores, and dashes. Label keys and values have a maximum length of 63 characters, and must be less than 128 bytes in size. Up to 64 label entries may be stored. For labels which do not have a semantic value, the empty string may be supplied for the label value.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::RequestValue for Service {}
impl client::Resource for Service {}
impl client::ResponseResult for Service {}


/// A Service-Level Indicator (SLI) describes the "performance" of a service. For some services, the SLI is well-defined. In such cases, the SLI can be described easily by referencing the well-known SLI and providing the needed parameters. Alternatively, a "custom" SLI can be defined with a query to the underlying metric store. An SLI is defined to be good_service / total_service over any queried time interval. The value of performance always falls into the range 0 <= performance <= 1. A custom SLI describes how to compute this ratio, whether this is by dividing values from a pair of time series, cutting a Distribution into good and bad counts, or counting time windows in which the service complies with a criterion. For separation of concerns, a single Service-Level Indicator measures performance for only one aspect of service quality, such as fraction of successful queries or fast-enough queries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLevelIndicator {
    /// Basic SLI on a well-known service type.
    #[serde(rename="basicSli")]
    
    pub basic_sli: Option<BasicSli>,
    /// Request-based SLIs
    #[serde(rename="requestBased")]
    
    pub request_based: Option<RequestBasedSli>,
    /// Windows-based SLIs
    #[serde(rename="windowsBased")]
    
    pub windows_based: Option<WindowsBasedSli>,
}

impl client::Part for ServiceLevelIndicator {}


/// A Service-Level Objective (SLO) describes a level of desired good service. It consists of a service-level indicator (SLI), a performance goal, and a period over which the objective is to be evaluated against that goal. The SLO can use SLIs defined in a number of different manners. Typical SLOs might include “99% of requests in each rolling week have latency below 200 milliseconds” or “99.5% of requests in each calendar month return successfully.”
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service level objectives create services](ServiceServiceLevelObjectiveCreateCall) (request|response)
/// * [service level objectives get services](ServiceServiceLevelObjectiveGetCall) (response)
/// * [service level objectives patch services](ServiceServiceLevelObjectivePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLevelObjective {
    /// A calendar period, semantically "since the start of the current ". At this time, only DAY, WEEK, FORTNIGHT, and MONTH are supported.
    #[serde(rename="calendarPeriod")]
    
    pub calendar_period: Option<ServiceLevelObjectiveCalendarPeriodEnum>,
    /// Name used for UI elements listing this SLO.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The fraction of service that must be good in order for this objective to be met. 0 < goal <= 0.999.
    
    pub goal: Option<f64>,
    /// Resource name for this ServiceLevelObjective. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME] 
    
    pub name: Option<String>,
    /// A rolling time period, semantically "in the past ". Must be an integer multiple of 1 day no larger than 30 days.
    #[serde(rename="rollingPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub rolling_period: Option<client::chrono::Duration>,
    /// The definition of good service, used to measure and calculate the quality of the Service's performance with respect to a single aspect of service quality.
    #[serde(rename="serviceLevelIndicator")]
    
    pub service_level_indicator: Option<ServiceLevelIndicator>,
    /// Labels which have been used to annotate the service-level objective. Label keys must start with a letter. Label keys and values may contain lowercase letters, numbers, underscores, and dashes. Label keys and values have a maximum length of 63 characters, and must be less than 128 bytes in size. Up to 64 label entries may be stored. For labels which do not have a semantic value, the empty string may be supplied for the label value.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::RequestValue for ServiceLevelObjective {}
impl client::ResponseResult for ServiceLevelObjective {}


/// A Snooze will prevent any alerts from being opened, and close any that are already open. The Snooze will work on alerts that match the criteria defined in the Snooze. The Snooze will be active from interval.start_time through interval.end_time.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [snoozes create projects](ProjectSnoozeCreateCall) (request|response)
/// * [snoozes get projects](ProjectSnoozeGetCall) (response)
/// * [snoozes patch projects](ProjectSnoozePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Snooze {
    /// Required. This defines the criteria for applying the Snooze. See Criteria for more information.
    
    pub criteria: Option<Criteria>,
    /// Required. A display name for the Snooze. This can be, at most, 512 unicode characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The Snooze will be active from interval.start_time through interval.end_time. interval.start_time cannot be in the past. There is a 15 second clock skew to account for the time it takes for a request to reach the API from the UI.
    
    pub interval: Option<TimeInterval>,
    /// Required. The name of the Snooze. The format is: projects/[PROJECT_ID_OR_NUMBER]/snoozes/[SNOOZE_ID] The ID of the Snooze will be generated by the system.
    
    pub name: Option<String>,
}

impl client::RequestValue for Snooze {}
impl client::ResponseResult for Snooze {}


/// The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors).
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


/// Information required for a TCP Uptime check request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TcpCheck {
    /// Contains information needed to add pings to a TCP check.
    #[serde(rename="pingConfig")]
    
    pub ping_config: Option<PingConfig>,
    /// The TCP port on the server against which to run the check. Will be combined with host (specified within the monitored_resource) to construct the full URL. Required.
    
    pub port: Option<i32>,
}

impl client::Part for TcpCheck {}


/// Configuration for how to query telemetry on a Service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Telemetry {
    /// The full name of the resource that defines this service. Formatted as described in https://cloud.google.com/apis/design/resource_names.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::Part for Telemetry {}


/// Describes a time interval: Reads: A half-open time interval. It includes the end time but excludes the start time: (startTime, endTime]. The start time must be specified, must be earlier than the end time, and should be no older than the data retention period for the metric. Writes: A closed time interval. It extends from the start time to the end time, and includes both: [startTime, endTime]. Valid time intervals depend on the MetricKind (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors#MetricKind) of the metric value. The end time must not be earlier than the start time, and the end time must not be more than 25 hours in the past or more than five minutes in the future. For GAUGE metrics, the startTime value is technically optional; if no value is specified, the start time defaults to the value of the end time, and the interval represents a single point in time. If both start and end times are specified, they must be identical. Such an interval is valid only for GAUGE metrics, which are point-in-time measurements. The end time of a new interval must be at least a millisecond after the end time of the previous interval. For DELTA metrics, the start time and end time must specify a non-zero interval, with subsequent points specifying contiguous and non-overlapping intervals. For DELTA metrics, the start time of the next interval must be at least a millisecond after the end time of the previous interval. For CUMULATIVE metrics, the start time and end time must specify a non-zero interval, with subsequent points specifying the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points. The new start time must be at least a millisecond after the end time of the previous interval. The start time of a new interval must be at least a millisecond after the end time of the previous interval because intervals are closed. If the start time of a new interval is the same as the end time of the previous interval, then data written at the new start time could overwrite data written at the previous end time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeInterval {
    /// Required. The end of the time interval.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The beginning of the time interval. The default value for the start time is the end time. The start time must not be later than the end time.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeInterval {}


/// A collection of data points that describes the time-varying values of a metric. A time series is identified by a combination of a fully-specified monitored resource and a fully-specified metric. This type is used for both listing and creating time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeSeries {
    /// Output only. The associated monitored resource metadata. When reading a time series, this field will include metadata labels that are explicitly named in the reduction. When creating a time series, this field is ignored.
    
    pub metadata: Option<MonitoredResourceMetadata>,
    /// The associated metric. A fully-specified metric used to identify the time series.
    
    pub metric: Option<Metric>,
    /// The metric kind of the time series. When listing time series, this metric kind might be different from the metric kind of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the metric kind of the associated metric. If the associated metric's descriptor must be auto-created, then this field specifies the metric kind of the new descriptor and must be either GAUGE (the default) or CUMULATIVE.
    #[serde(rename="metricKind")]
    
    pub metric_kind: Option<TimeSeryMetricKindEnum>,
    /// The data points of this time series. When listing time series, points are returned in reverse time order.When creating a time series, this field must contain exactly one point and the point's type must be the same as the value type of the associated metric. If the associated metric's descriptor must be auto-created, then the value type of the descriptor is determined by the point's type, which must be BOOL, INT64, DOUBLE, or DISTRIBUTION.
    
    pub points: Option<Vec<Point>>,
    /// The associated monitored resource. Custom metrics can use only certain monitored resource types in their time series data. For more information, see Monitored resources for custom metrics (https://cloud.google.com/monitoring/custom-metrics/creating-metrics#custom-metric-resources).
    
    pub resource: Option<MonitoredResource>,
    /// The units in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored metric values.
    
    pub unit: Option<String>,
    /// The value type of the time series. When listing time series, this value type might be different from the value type of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the type of the data in the points field.
    #[serde(rename="valueType")]
    
    pub value_type: Option<TimeSeryValueTypeEnum>,
}

impl client::Part for TimeSeries {}


/// Represents the values of a time series associated with a TimeSeriesDescriptor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeSeriesData {
    /// The values of the labels in the time series identifier, given in the same order as the label_descriptors field of the TimeSeriesDescriptor associated with this object. Each value must have a value of the type given in the corresponding entry of label_descriptors.
    #[serde(rename="labelValues")]
    
    pub label_values: Option<Vec<LabelValue>>,
    /// The points in the time series.
    #[serde(rename="pointData")]
    
    pub point_data: Option<Vec<PointData>>,
}

impl client::Part for TimeSeriesData {}


/// A descriptor for the labels and points in a time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeSeriesDescriptor {
    /// Descriptors for the labels.
    #[serde(rename="labelDescriptors")]
    
    pub label_descriptors: Option<Vec<LabelDescriptor>>,
    /// Descriptors for the point data value columns.
    #[serde(rename="pointDescriptors")]
    
    pub point_descriptors: Option<Vec<ValueDescriptor>>,
}

impl client::Part for TimeSeriesDescriptor {}


/// A TimeSeriesRatio specifies two TimeSeries to use for computing the good_service / total_service ratio. The specified TimeSeries must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE. The TimeSeriesRatio must specify exactly two of good, bad, and total, and the relationship good_service + bad_service = total_service will be assumed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeSeriesRatio {
    /// A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries quantifying bad service, either demanded service that was not provided or demanded service that was of inadequate quality. Must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[serde(rename="badServiceFilter")]
    
    pub bad_service_filter: Option<String>,
    /// A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries quantifying good service provided. Must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[serde(rename="goodServiceFilter")]
    
    pub good_service_filter: Option<String>,
    /// A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries quantifying total demanded service. Must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[serde(rename="totalServiceFilter")]
    
    pub total_service_filter: Option<String>,
}

impl client::Part for TimeSeriesRatio {}


/// Specifies how many time series must fail a predicate to trigger a condition. If not specified, then a {count: 1} trigger is used.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trigger {
    /// The absolute number of time series that must fail the predicate for the condition to be triggered.
    
    pub count: Option<i32>,
    /// The percentage of time series that must fail the predicate for the condition to be triggered.
    
    pub percent: Option<f64>,
}

impl client::Part for Trigger {}


/// A single strongly-typed value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TypedValue {
    /// A Boolean value: true or false.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// A distribution value.
    #[serde(rename="distributionValue")]
    
    pub distribution_value: Option<Distribution>,
    /// A 64-bit double-precision floating-point number. Its magnitude is approximately ±10±300 and it has 16 significant digits of precision.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// A 64-bit integer. Its range is approximately ±9.2x1018.
    #[serde(rename="int64Value")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int64_value: Option<i64>,
    /// A variable-length string value.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for TypedValue {}


/// This message configures which resources and services to monitor for availability.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uptime check configs create projects](ProjectUptimeCheckConfigCreateCall) (request|response)
/// * [uptime check configs get projects](ProjectUptimeCheckConfigGetCall) (response)
/// * [uptime check configs patch projects](ProjectUptimeCheckConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UptimeCheckConfig {
    /// The type of checkers to use to execute the Uptime check.
    #[serde(rename="checkerType")]
    
    pub checker_type: Option<UptimeCheckConfigCheckerTypeEnum>,
    /// The content that is expected to appear in the data returned by the target server against which the check is run. Currently, only the first entry in the content_matchers list is supported, and additional entries will be ignored. This field is optional and should only be specified if a content match is required as part of the/ Uptime check.
    #[serde(rename="contentMatchers")]
    
    pub content_matchers: Option<Vec<ContentMatcher>>,
    /// A human-friendly name for the Uptime check configuration. The display name should be unique within a Cloud Monitoring Workspace in order to make it easier to identify; however, uniqueness is not enforced. Required.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Contains information needed to make an HTTP or HTTPS check.
    #[serde(rename="httpCheck")]
    
    pub http_check: Option<HttpCheck>,
    /// The internal checkers that this check will egress from. If is_internal is true and this list is empty, the check will egress from all the InternalCheckers configured for the project that owns this UptimeCheckConfig.
    #[serde(rename="internalCheckers")]
    
    pub internal_checkers: Option<Vec<InternalChecker>>,
    /// If this is true, then checks are made only from the 'internal_checkers'. If it is false, then checks are made only from the 'selected_regions'. It is an error to provide 'selected_regions' when is_internal is true, or to provide 'internal_checkers' when is_internal is false.
    #[serde(rename="isInternal")]
    
    pub is_internal: Option<bool>,
    /// The monitored resource (https://cloud.google.com/monitoring/api/resources) associated with the configuration. The following monitored resource types are valid for this field: uptime_url, gce_instance, gae_app, aws_ec2_instance, aws_elb_load_balancer k8s_service servicedirectory_service cloud_run_revision
    #[serde(rename="monitoredResource")]
    
    pub monitored_resource: Option<MonitoredResource>,
    /// A unique resource name for this Uptime check configuration. The format is: projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID] [PROJECT_ID_OR_NUMBER] is the Workspace host project associated with the Uptime check.This field should be omitted when creating the Uptime check configuration; on create, the resource name is assigned by the server and included in the response.
    
    pub name: Option<String>,
    /// How often, in seconds, the Uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 60s.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub period: Option<client::chrono::Duration>,
    /// The group resource associated with the configuration.
    #[serde(rename="resourceGroup")]
    
    pub resource_group: Option<ResourceGroup>,
    /// The list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions must be provided to include a minimum of 3 locations. Not specifying this field will result in Uptime checks running from all available regions.
    #[serde(rename="selectedRegions")]
    
    pub selected_regions: Option<Vec<UptimeCheckConfigSelectedRegionsEnum>>,
    /// Contains information needed to make a TCP check.
    #[serde(rename="tcpCheck")]
    
    pub tcp_check: Option<TcpCheck>,
    /// The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). Required.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
    /// User-supplied key/value data to be used for organizing and identifying the UptimeCheckConfig objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
}

impl client::RequestValue for UptimeCheckConfig {}
impl client::ResponseResult for UptimeCheckConfig {}


/// Contains the region, location, and list of IP addresses where checkers in the location run from.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list uptime check ips](UptimeCheckIpListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UptimeCheckIp {
    /// The IP address from which the Uptime check originates. This is a fully specified IP address (not an IP address range). Most IP addresses, as of this publication, are in IPv4 format; however, one should not rely on the IP addresses being in IPv4 format indefinitely, and should support interpreting this field in either IPv4 or IPv6 format.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// A more specific location within the region that typically encodes a particular city/town/metro (and its containing state/province or country) within the broader umbrella region category.
    
    pub location: Option<String>,
    /// A broad region category in which the IP address is located.
    
    pub region: Option<UptimeCheckIpRegionEnum>,
}

impl client::Resource for UptimeCheckIp {}


/// A descriptor for the value columns in a data point.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueDescriptor {
    /// The value key.
    
    pub key: Option<String>,
    /// The value stream kind.
    #[serde(rename="metricKind")]
    
    pub metric_kind: Option<ValueDescriptorMetricKindEnum>,
    /// The unit in which time_series point values are reported. unit follows the UCUM format for units as seen in https://unitsofmeasure.org/ucum.html. unit is only valid if value_type is INTEGER, DOUBLE, DISTRIBUTION.
    
    pub unit: Option<String>,
    /// The value type.
    #[serde(rename="valueType")]
    
    pub value_type: Option<ValueDescriptorValueTypeEnum>,
}

impl client::Part for ValueDescriptor {}


/// The VerifyNotificationChannel request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notification channels verify projects](ProjectNotificationChannelVerifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyNotificationChannelRequest {
    /// Required. The verification code that was delivered to the channel as a result of invoking the SendNotificationChannelVerificationCode API method or that was retrieved from a verified channel via GetNotificationChannelVerificationCode. For example, one might have "G-123456" or "TKNZGhhd2EyN3I1MnRnMjRv" (in general, one is only guaranteed that the code is valid UTF-8; one should not make any assumptions regarding the structure or format of the code).
    
    pub code: Option<String>,
}

impl client::RequestValue for VerifyNotificationChannelRequest {}


/// A WindowsBasedSli defines good_service as the count of time windows for which the provided service was of good quality. Criteria for determining if service was good are embedded in the window_criterion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsBasedSli {
    /// A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries with ValueType = BOOL. The window is good if any true values appear in the window.
    #[serde(rename="goodBadMetricFilter")]
    
    pub good_bad_metric_filter: Option<String>,
    /// A window is good if its performance is high enough.
    #[serde(rename="goodTotalRatioThreshold")]
    
    pub good_total_ratio_threshold: Option<PerformanceThreshold>,
    /// A window is good if the metric's value is in a good range, averaged across returned streams.
    #[serde(rename="metricMeanInRange")]
    
    pub metric_mean_in_range: Option<MetricRange>,
    /// A window is good if the metric's value is in a good range, summed across returned streams.
    #[serde(rename="metricSumInRange")]
    
    pub metric_sum_in_range: Option<MetricRange>,
    /// Duration over which window quality is evaluated. Must be an integer fraction of a day and at least 60s.
    #[serde(rename="windowPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub window_period: Option<client::chrono::Duration>,
}

impl client::Part for WindowsBasedSli {}


