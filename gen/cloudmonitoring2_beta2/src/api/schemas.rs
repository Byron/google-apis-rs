use super::*;
/// The response of cloudmonitoring.metricDescriptors.delete.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete metric descriptors](MetricDescriptorDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteMetricDescriptorResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#deleteMetricDescriptorResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DeleteMetricDescriptorResponse {}


/// The request of cloudmonitoring.metricDescriptors.list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list metric descriptors](MetricDescriptorListCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMetricDescriptorsRequest {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listMetricDescriptorsRequest".
    
    pub kind: Option<String>,
}

impl client::RequestValue for ListMetricDescriptorsRequest {}


/// The response of cloudmonitoring.metricDescriptors.list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list metric descriptors](MetricDescriptorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMetricDescriptorsResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listMetricDescriptorsResponse".
    
    pub kind: Option<String>,
    /// The returned metric descriptors.
    
    pub metrics: Option<Vec<MetricDescriptor>>,
    /// Pagination token. If present, indicates that additional results are available for retrieval. To access the results past the pagination limit, pass this value to the pageToken query parameter.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMetricDescriptorsResponse {}


/// The request of cloudmonitoring.timeseriesDescriptors.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries descriptors](TimeseriesDescriptorListCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesDescriptorsRequest {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesDescriptorsRequest".
    
    pub kind: Option<String>,
}

impl client::RequestValue for ListTimeseriesDescriptorsRequest {}


/// The response of cloudmonitoring.timeseriesDescriptors.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries descriptors](TimeseriesDescriptorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesDescriptorsResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesDescriptorsResponse".
    
    pub kind: Option<String>,
    /// Pagination token. If present, indicates that additional results are available for retrieval. To access the results past the pagination limit, set this value to the pageToken query parameter.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The oldest timestamp of the interval of this query, as an RFC 3339 string.
    
    pub oldest: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The returned time series descriptors.
    
    pub timeseries: Option<Vec<TimeseriesDescriptor>>,
    /// The youngest timestamp of the interval of this query, as an RFC 3339 string.
    
    pub youngest: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ListTimeseriesDescriptorsResponse {}


/// The request of cloudmonitoring.timeseries.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries](TimeseryListCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesRequest {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesRequest".
    
    pub kind: Option<String>,
}

impl client::RequestValue for ListTimeseriesRequest {}


/// The response of cloudmonitoring.timeseries.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries](TimeseryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesResponse".
    
    pub kind: Option<String>,
    /// Pagination token. If present, indicates that additional results are available for retrieval. To access the results past the pagination limit, set the pageToken query parameter to this value. All of the points of a time series will be returned before returning any point of the subsequent time series.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The oldest timestamp of the interval of this query as an RFC 3339 string.
    
    pub oldest: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The returned time series.
    
    pub timeseries: Option<Vec<Timeseries>>,
    /// The youngest timestamp of the interval of this query as an RFC 3339 string.
    
    pub youngest: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ListTimeseriesResponse {}


/// A metricDescriptor defines the name, label keys, and data type of a particular metric.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create metric descriptors](MetricDescriptorCreateCall) (request|response)
/// * [delete metric descriptors](MetricDescriptorDeleteCall) (none)
/// * [list metric descriptors](MetricDescriptorListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptor {
    /// Description of this metric.
    
    pub description: Option<String>,
    /// Labels defined for this metric.
    
    pub labels: Option<Vec<MetricDescriptorLabelDescriptor>>,
    /// The name of this metric.
    
    pub name: Option<String>,
    /// The project ID to which the metric belongs.
    
    pub project: Option<String>,
    /// Type description for this metric.
    #[serde(rename="typeDescriptor")]
    
    pub type_descriptor: Option<MetricDescriptorTypeDescriptor>,
}

impl client::RequestValue for MetricDescriptor {}
impl client::Resource for MetricDescriptor {}
impl client::ResponseResult for MetricDescriptor {}


/// A label in a metric is a description of this metric, including the key of this description (what the description is), and the value for this description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptorLabelDescriptor {
    /// Label description.
    
    pub description: Option<String>,
    /// Label key.
    
    pub key: Option<String>,
}

impl client::Part for MetricDescriptorLabelDescriptor {}


/// A type in a metric contains information about how the metric is collected and what its data points look like.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptorTypeDescriptor {
    /// The method of collecting data for the metric. See Metric types.
    #[serde(rename="metricType")]
    
    pub metric_type: Option<String>,
    /// The data type of of individual points in the metric's time series. See Metric value types.
    #[serde(rename="valueType")]
    
    pub value_type: Option<String>,
}

impl client::Part for MetricDescriptorTypeDescriptor {}


/// Point is a single point in a time series. It consists of a start time, an end time, and a value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    /// The value of this data point. Either "true" or "false".
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// The value of this data point as a distribution. A distribution value can contain a list of buckets and/or an underflowBucket and an overflowBucket. The values of these points can be used to create a histogram.
    #[serde(rename="distributionValue")]
    
    pub distribution_value: Option<PointDistribution>,
    /// The value of this data point as a double-precision floating-point number.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// The interval [start, end] is the time period to which the point's value applies. For gauge metrics, whose values are instantaneous measurements, this interval should be empty (start should equal end). For cumulative metrics (of which deltas and rates are special cases), the interval should be non-empty. Both start and end are RFC 3339 strings.
    
    pub end: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The value of this data point as a 64-bit integer.
    #[serde(rename="int64Value")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int64_value: Option<i64>,
    /// The interval [start, end] is the time period to which the point's value applies. For gauge metrics, whose values are instantaneous measurements, this interval should be empty (start should equal end). For cumulative metrics (of which deltas and rates are special cases), the interval should be non-empty. Both start and end are RFC 3339 strings.
    
    pub start: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The value of this data point in string format.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for Point {}


/// Distribution data point value type. When writing distribution points, try to be consistent with the boundaries of your buckets. If you must modify the bucket boundaries, then do so by merging, partitioning, or appending rather than skewing them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistribution {
    /// The finite buckets.
    
    pub buckets: Option<Vec<PointDistributionBucket>>,
    /// The overflow bucket.
    #[serde(rename="overflowBucket")]
    
    pub overflow_bucket: Option<PointDistributionOverflowBucket>,
    /// The underflow bucket.
    #[serde(rename="underflowBucket")]
    
    pub underflow_bucket: Option<PointDistributionUnderflowBucket>,
}

impl client::Part for PointDistribution {}


/// The histogram's bucket. Buckets that form the histogram of a distribution value. If the upper bound of a bucket, say U1, does not equal the lower bound of the next bucket, say L2, this means that there is no event in [U1, L2).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistributionBucket {
    /// The number of events whose values are in the interval defined by this bucket.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The lower bound of the value interval of this bucket (inclusive).
    #[serde(rename="lowerBound")]
    
    pub lower_bound: Option<f64>,
    /// The upper bound of the value interval of this bucket (exclusive).
    #[serde(rename="upperBound")]
    
    pub upper_bound: Option<f64>,
}

impl client::Part for PointDistributionBucket {}


/// The overflow bucket is a special bucket that does not have the upperBound field; it includes all of the events that are no less than its lower bound.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistributionOverflowBucket {
    /// The number of events whose values are in the interval defined by this bucket.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The lower bound of the value interval of this bucket (inclusive).
    #[serde(rename="lowerBound")]
    
    pub lower_bound: Option<f64>,
}

impl client::Part for PointDistributionOverflowBucket {}


/// The underflow bucket is a special bucket that does not have the lowerBound field; it includes all of the events that are less than its upper bound.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistributionUnderflowBucket {
    /// The number of events whose values are in the interval defined by this bucket.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The upper bound of the value interval of this bucket (exclusive).
    #[serde(rename="upperBound")]
    
    pub upper_bound: Option<f64>,
}

impl client::Part for PointDistributionUnderflowBucket {}


/// The monitoring data is organized as metrics and stored as data points that are recorded over time. Each data point represents information like the CPU utilization of your virtual machine. A historical record of these data points is called a time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Timeseries {
    /// The data points of this time series. The points are listed in order of their end timestamp, from younger to older.
    
    pub points: Option<Vec<Point>>,
    /// The descriptor of this time series.
    #[serde(rename="timeseriesDesc")]
    
    pub timeseries_desc: Option<TimeseriesDescriptor>,
}

impl client::Part for Timeseries {}


/// TimeseriesDescriptor identifies a single time series.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries descriptors](TimeseriesDescriptorListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeseriesDescriptor {
    /// The label's name.
    
    pub labels: Option<HashMap<String, String>>,
    /// The name of the metric.
    
    pub metric: Option<String>,
    /// The Developers Console project number to which this time series belongs.
    
    pub project: Option<String>,
}

impl client::Resource for TimeseriesDescriptor {}


/// When writing time series, TimeseriesPoint should be used instead of Timeseries, to enforce single point for each time series in the timeseries.write request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeseriesPoint {
    /// The data point in this time series snapshot.
    
    pub point: Option<Point>,
    /// The descriptor of this time series.
    #[serde(rename="timeseriesDesc")]
    
    pub timeseries_desc: Option<TimeseriesDescriptor>,
}

impl client::Part for TimeseriesPoint {}


/// The request of cloudmonitoring.timeseries.write
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write timeseries](TimeseryWriteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteTimeseriesRequest {
    /// The label's name.
    #[serde(rename="commonLabels")]
    
    pub common_labels: Option<HashMap<String, String>>,
    /// Provide time series specific labels and the data points for each time series. The labels in timeseries and the common_labels should form a complete list of labels that required by the metric.
    
    pub timeseries: Option<Vec<TimeseriesPoint>>,
}

impl client::RequestValue for WriteTimeseriesRequest {}


/// The response of cloudmonitoring.timeseries.write
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write timeseries](TimeseryWriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteTimeseriesResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#writeTimeseriesResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for WriteTimeseriesResponse {}


