use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// View and write monitoring data for all of your Google and third-party Cloud and API projects
    Monitoring,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::Monitoring => "https://www.googleapis.com/auth/monitoring",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Monitoring
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all CloudMonitoring related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::api::ListMetricDescriptorsRequest;
/// use cloudmonitoring2_beta2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListMetricDescriptorsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().list(req, "project")
///              .query("takimata")
///              .page_token("amet.")
///              .count(-20)
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct CloudMonitoring<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for CloudMonitoring<S> {}

impl<'a, S> CloudMonitoring<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> CloudMonitoring<S> {
        CloudMonitoring {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2-beta-1".to_string(),
            _base_url: "https://www.googleapis.com/cloudmonitoring/v2beta2/projects/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn metric_descriptors(&'a self) -> MetricDescriptorMethods<'a, S> {
        MetricDescriptorMethods { hub: &self }
    }
    pub fn timeseries(&'a self) -> TimeseryMethods<'a, S> {
        TimeseryMethods { hub: &self }
    }
    pub fn timeseries_descriptors(&'a self) -> TimeseriesDescriptorMethods<'a, S> {
        TimeseriesDescriptorMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2-beta-1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/cloudmonitoring/v2beta2/projects/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The response of cloudmonitoring.metricDescriptors.delete.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete metric descriptors](MetricDescriptorDeleteCall) (response)
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteTimeseriesResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#writeTimeseriesResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for WriteTimeseriesResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *metricDescriptor* resources.
/// It is not used directly, but through the [`CloudMonitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.metric_descriptors();
/// # }
/// ```
pub struct MetricDescriptorMethods<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
}

impl<'a, S> client::MethodsBuilder for MetricDescriptorMethods<'a, S> {}

impl<'a, S> MetricDescriptorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project id. The value can be the numeric project ID or string-based project name.
    pub fn create(&self, request: MetricDescriptor, project: &str) -> MetricDescriptorCreateCall<'a, S> {
        MetricDescriptorCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an existing metric.
    /// 
    /// # Arguments
    ///
    /// * `project` - The project ID to which the metric belongs.
    /// * `metric` - Name of the metric.
    pub fn delete(&self, project: &str, metric: &str) -> MetricDescriptorDeleteCall<'a, S> {
        MetricDescriptorDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _metric: metric.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List metric descriptors that match the query. If the query is not set, then all of the metric descriptors will be returned. Large responses will be paginated, use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project id. The value can be the numeric project ID or string-based project name.
    pub fn list(&self, request: ListMetricDescriptorsRequest, project: &str) -> MetricDescriptorListCall<'a, S> {
        MetricDescriptorListCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *timesery* resources.
/// It is not used directly, but through the [`CloudMonitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `write(...)`
/// // to build up your call.
/// let rb = hub.timeseries();
/// # }
/// ```
pub struct TimeseryMethods<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
}

impl<'a, S> client::MethodsBuilder for TimeseryMethods<'a, S> {}

impl<'a, S> TimeseryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the data points of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    /// * `metric` - Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    /// * `youngest` - End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    pub fn list(&self, request: ListTimeseriesRequest, project: &str, metric: &str, youngest: &str) -> TimeseryListCall<'a, S> {
        TimeseryListCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _metric: metric.to_string(),
            _youngest: youngest.to_string(),
            _window: Default::default(),
            _timespan: Default::default(),
            _page_token: Default::default(),
            _oldest: Default::default(),
            _labels: Default::default(),
            _count: Default::default(),
            _aggregator: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Put data points to one or more time series for one or more metrics. If a time series does not exist, a new time series will be created. It is not allowed to write a time series point that is older than the existing youngest point of that time series. Points that are older than the existing youngest point of that time series will be discarded silently. Therefore, users should make sure that points of a time series are written sequentially in the order of their end time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID. The value can be the numeric project ID or string-based project name.
    pub fn write(&self, request: WriteTimeseriesRequest, project: &str) -> TimeseryWriteCall<'a, S> {
        TimeseryWriteCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *timeseriesDescriptor* resources.
/// It is not used directly, but through the [`CloudMonitoring`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.timeseries_descriptors();
/// # }
/// ```
pub struct TimeseriesDescriptorMethods<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
}

impl<'a, S> client::MethodsBuilder for TimeseriesDescriptorMethods<'a, S> {}

impl<'a, S> TimeseriesDescriptorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the descriptors of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    /// * `metric` - Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    /// * `youngest` - End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    pub fn list(&self, request: ListTimeseriesDescriptorsRequest, project: &str, metric: &str, youngest: &str) -> TimeseriesDescriptorListCall<'a, S> {
        TimeseriesDescriptorListCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _metric: metric.to_string(),
            _youngest: youngest.to_string(),
            _window: Default::default(),
            _timespan: Default::default(),
            _page_token: Default::default(),
            _oldest: Default::default(),
            _labels: Default::default(),
            _count: Default::default(),
            _aggregator: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Create a new metric.
///
/// A builder for the *create* method supported by a *metricDescriptor* resource.
/// It is not used directly, but through a [`MetricDescriptorMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::api::MetricDescriptor;
/// # async fn dox() {
/// # use std::default::Default;
/// # use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = MetricDescriptor::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().create(req, "project")
///              .doit().await;
/// # }
/// ```
pub struct MetricDescriptorCreateCall<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
    _request: MetricDescriptor,
    _project: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MetricDescriptorCreateCall<'a, S> {}

impl<'a, S> MetricDescriptorCreateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, MetricDescriptor)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "cloudmonitoring.metricDescriptors.create",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "project"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("project", self._project);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{project}/metricDescriptors";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{project}", "project")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: MetricDescriptor) -> MetricDescriptorCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The project id. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> MetricDescriptorCreateCall<'a, S> {
        self._project = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MetricDescriptorCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> MetricDescriptorCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MetricDescriptorCreateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MetricDescriptorCreateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MetricDescriptorCreateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Delete an existing metric.
///
/// A builder for the *delete* method supported by a *metricDescriptor* resource.
/// It is not used directly, but through a [`MetricDescriptorMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().delete("project", "metric")
///              .doit().await;
/// # }
/// ```
pub struct MetricDescriptorDeleteCall<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
    _project: String,
    _metric: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MetricDescriptorDeleteCall<'a, S> {}

impl<'a, S> MetricDescriptorDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DeleteMetricDescriptorResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "cloudmonitoring.metricDescriptors.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["alt", "project", "metric"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("project", self._project);
        params.push("metric", self._metric);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{project}/metricDescriptors/{metric}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{metric}", "metric")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["metric", "project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The project ID to which the metric belongs.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> MetricDescriptorDeleteCall<'a, S> {
        self._project = new_value.to_string();
        self
    }
    /// Name of the metric.
    ///
    /// Sets the *metric* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metric(mut self, new_value: &str) -> MetricDescriptorDeleteCall<'a, S> {
        self._metric = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MetricDescriptorDeleteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> MetricDescriptorDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MetricDescriptorDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MetricDescriptorDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MetricDescriptorDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List metric descriptors that match the query. If the query is not set, then all of the metric descriptors will be returned. Large responses will be paginated, use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
///
/// A builder for the *list* method supported by a *metricDescriptor* resource.
/// It is not used directly, but through a [`MetricDescriptorMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::api::ListMetricDescriptorsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListMetricDescriptorsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().list(req, "project")
///              .query("eos")
///              .page_token("dolor")
///              .count(-17)
///              .doit().await;
/// # }
/// ```
pub struct MetricDescriptorListCall<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
    _request: ListMetricDescriptorsRequest,
    _project: String,
    _query: Option<String>,
    _page_token: Option<String>,
    _count: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MetricDescriptorListCall<'a, S> {}

impl<'a, S> MetricDescriptorListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListMetricDescriptorsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "cloudmonitoring.metricDescriptors.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "project", "query", "pageToken", "count"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("project", self._project);
        if let Some(value) = self._query.as_ref() {
            params.push("query", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._count.as_ref() {
            params.push("count", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{project}/metricDescriptors";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{project}", "project")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ListMetricDescriptorsRequest) -> MetricDescriptorListCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The project id. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> MetricDescriptorListCall<'a, S> {
        self._project = new_value.to_string();
        self
    }
    /// The query used to search against existing metrics. Separate keywords with a space; the service joins all keywords with AND, meaning that all keywords must match for a metric to be returned. If this field is omitted, all metrics are returned. If an empty string is passed with this field, no metrics are returned.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> MetricDescriptorListCall<'a, S> {
        self._query = Some(new_value.to_string());
        self
    }
    /// The pagination token, which is used to page through large result sets. Set this value to the value of the nextPageToken to retrieve the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> MetricDescriptorListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of metric descriptors per page. Used for pagination. If not specified, count = 100.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> MetricDescriptorListCall<'a, S> {
        self._count = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MetricDescriptorListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> MetricDescriptorListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MetricDescriptorListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MetricDescriptorListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MetricDescriptorListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List the data points of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
///
/// A builder for the *list* method supported by a *timesery* resource.
/// It is not used directly, but through a [`TimeseryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::api::ListTimeseriesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListTimeseriesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.timeseries().list(req, "project", "metric", "youngest")
///              .window("duo")
///              .timespan("ipsum")
///              .page_token("sed")
///              .oldest("ut")
///              .add_labels("gubergren")
///              .count(-16)
///              .aggregator("est")
///              .doit().await;
/// # }
/// ```
pub struct TimeseryListCall<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
    _request: ListTimeseriesRequest,
    _project: String,
    _metric: String,
    _youngest: String,
    _window: Option<String>,
    _timespan: Option<String>,
    _page_token: Option<String>,
    _oldest: Option<String>,
    _labels: Vec<String>,
    _count: Option<i32>,
    _aggregator: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TimeseryListCall<'a, S> {}

impl<'a, S> TimeseryListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListTimeseriesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "cloudmonitoring.timeseries.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "project", "metric", "youngest", "window", "timespan", "pageToken", "oldest", "labels", "count", "aggregator"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(13 + self._additional_params.len());
        params.push("project", self._project);
        params.push("metric", self._metric);
        params.push("youngest", self._youngest);
        if let Some(value) = self._window.as_ref() {
            params.push("window", value);
        }
        if let Some(value) = self._timespan.as_ref() {
            params.push("timespan", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._oldest.as_ref() {
            params.push("oldest", value);
        }
        if self._labels.len() > 0 {
            for f in self._labels.iter() {
                params.push("labels", f);
            }
        }
        if let Some(value) = self._count.as_ref() {
            params.push("count", value.to_string());
        }
        if let Some(value) = self._aggregator.as_ref() {
            params.push("aggregator", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{project}/timeseries/{metric}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{metric}", "metric")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["metric", "project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ListTimeseriesRequest) -> TimeseryListCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._project = new_value.to_string();
        self
    }
    /// Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    ///
    /// Sets the *metric* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metric(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._metric = new_value.to_string();
        self
    }
    /// End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    ///
    /// Sets the *youngest* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn youngest(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._youngest = new_value.to_string();
        self
    }
    /// The sampling window. At most one data point will be returned for each window in the requested time interval. This parameter is only valid for non-cumulative metric types. Units:  
    /// - m: minute 
    /// - h: hour 
    /// - d: day 
    /// - w: week  Examples: 3m, 4w. Only one unit is allowed, for example: 2w3d is not allowed; you should use 17d instead.
    ///
    /// Sets the *window* query property to the given value.
    pub fn window(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._window = Some(new_value.to_string());
        self
    }
    /// Length of the time interval to query, which is an alternative way to declare the interval: (youngest - timespan, youngest]. The timespan and oldest parameters should not be used together. Units:  
    /// - s: second 
    /// - m: minute 
    /// - h: hour 
    /// - d: day 
    /// - w: week  Examples: 2s, 3m, 4w. Only one unit is allowed, for example: 2w3d is not allowed; you should use 17d instead.
    /// 
    /// If neither oldest nor timespan is specified, the default time interval will be (youngest - 4 hours, youngest].
    ///
    /// Sets the *timespan* query property to the given value.
    pub fn timespan(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._timespan = Some(new_value.to_string());
        self
    }
    /// The pagination token, which is used to page through large result sets. Set this value to the value of the nextPageToken to retrieve the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Start of the time interval (exclusive), which is expressed as an RFC 3339 timestamp. If neither oldest nor timespan is specified, the default time interval will be (youngest - 4 hours, youngest]
    ///
    /// Sets the *oldest* query property to the given value.
    pub fn oldest(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._oldest = Some(new_value.to_string());
        self
    }
    /// A collection of labels for the matching time series, which are represented as:  
    /// - key==value: key equals the value 
    /// - key=~value: key regex matches the value 
    /// - key!=value: key does not equal the value 
    /// - key!~value: key regex does not match the value  For example, to list all of the time series descriptors for the region us-central1, you could specify:
    /// label=cloud.googleapis.com%2Flocation=~us-central1.*
    ///
    /// Append the given value to the *labels* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_labels(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._labels.push(new_value.to_string());
        self
    }
    /// Maximum number of data points per page, which is used for pagination of results.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> TimeseryListCall<'a, S> {
        self._count = Some(new_value);
        self
    }
    /// The aggregation function that will reduce the data points in each window to a single point. This parameter is only valid for non-cumulative metrics with a value type of INT64 or DOUBLE.
    ///
    /// Sets the *aggregator* query property to the given value.
    pub fn aggregator(mut self, new_value: &str) -> TimeseryListCall<'a, S> {
        self._aggregator = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TimeseryListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> TimeseryListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TimeseryListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TimeseryListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TimeseryListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Put data points to one or more time series for one or more metrics. If a time series does not exist, a new time series will be created. It is not allowed to write a time series point that is older than the existing youngest point of that time series. Points that are older than the existing youngest point of that time series will be discarded silently. Therefore, users should make sure that points of a time series are written sequentially in the order of their end time.
///
/// A builder for the *write* method supported by a *timesery* resource.
/// It is not used directly, but through a [`TimeseryMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::api::WriteTimeseriesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = WriteTimeseriesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.timeseries().write(req, "project")
///              .doit().await;
/// # }
/// ```
pub struct TimeseryWriteCall<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
    _request: WriteTimeseriesRequest,
    _project: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TimeseryWriteCall<'a, S> {}

impl<'a, S> TimeseryWriteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, WriteTimeseriesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "cloudmonitoring.timeseries.write",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "project"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("project", self._project);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{project}/timeseries:write";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{project}", "project")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: WriteTimeseriesRequest) -> TimeseryWriteCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The project ID. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> TimeseryWriteCall<'a, S> {
        self._project = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TimeseryWriteCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> TimeseryWriteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TimeseryWriteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TimeseryWriteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TimeseryWriteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List the descriptors of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
///
/// A builder for the *list* method supported by a *timeseriesDescriptor* resource.
/// It is not used directly, but through a [`TimeseriesDescriptorMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::api::ListTimeseriesDescriptorsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use cloudmonitoring2_beta2::{CloudMonitoring, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CloudMonitoring::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListTimeseriesDescriptorsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.timeseries_descriptors().list(req, "project", "metric", "youngest")
///              .window("ea")
///              .timespan("dolor")
///              .page_token("Lorem")
///              .oldest("eos")
///              .add_labels("labore")
///              .count(-43)
///              .aggregator("duo")
///              .doit().await;
/// # }
/// ```
pub struct TimeseriesDescriptorListCall<'a, S>
    where S: 'a {

    hub: &'a CloudMonitoring<S>,
    _request: ListTimeseriesDescriptorsRequest,
    _project: String,
    _metric: String,
    _youngest: String,
    _window: Option<String>,
    _timespan: Option<String>,
    _page_token: Option<String>,
    _oldest: Option<String>,
    _labels: Vec<String>,
    _count: Option<i32>,
    _aggregator: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TimeseriesDescriptorListCall<'a, S> {}

impl<'a, S> TimeseriesDescriptorListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListTimeseriesDescriptorsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "cloudmonitoring.timeseriesDescriptors.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "project", "metric", "youngest", "window", "timespan", "pageToken", "oldest", "labels", "count", "aggregator"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(13 + self._additional_params.len());
        params.push("project", self._project);
        params.push("metric", self._metric);
        params.push("youngest", self._youngest);
        if let Some(value) = self._window.as_ref() {
            params.push("window", value);
        }
        if let Some(value) = self._timespan.as_ref() {
            params.push("timespan", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._oldest.as_ref() {
            params.push("oldest", value);
        }
        if self._labels.len() > 0 {
            for f in self._labels.iter() {
                params.push("labels", f);
            }
        }
        if let Some(value) = self._count.as_ref() {
            params.push("count", value.to_string());
        }
        if let Some(value) = self._aggregator.as_ref() {
            params.push("aggregator", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{project}/timeseriesDescriptors/{metric}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{metric}", "metric")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["metric", "project"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ListTimeseriesDescriptorsRequest) -> TimeseriesDescriptorListCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._project = new_value.to_string();
        self
    }
    /// Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    ///
    /// Sets the *metric* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metric(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._metric = new_value.to_string();
        self
    }
    /// End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    ///
    /// Sets the *youngest* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn youngest(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._youngest = new_value.to_string();
        self
    }
    /// The sampling window. At most one data point will be returned for each window in the requested time interval. This parameter is only valid for non-cumulative metric types. Units:  
    /// - m: minute 
    /// - h: hour 
    /// - d: day 
    /// - w: week  Examples: 3m, 4w. Only one unit is allowed, for example: 2w3d is not allowed; you should use 17d instead.
    ///
    /// Sets the *window* query property to the given value.
    pub fn window(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._window = Some(new_value.to_string());
        self
    }
    /// Length of the time interval to query, which is an alternative way to declare the interval: (youngest - timespan, youngest]. The timespan and oldest parameters should not be used together. Units:  
    /// - s: second 
    /// - m: minute 
    /// - h: hour 
    /// - d: day 
    /// - w: week  Examples: 2s, 3m, 4w. Only one unit is allowed, for example: 2w3d is not allowed; you should use 17d instead.
    /// 
    /// If neither oldest nor timespan is specified, the default time interval will be (youngest - 4 hours, youngest].
    ///
    /// Sets the *timespan* query property to the given value.
    pub fn timespan(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._timespan = Some(new_value.to_string());
        self
    }
    /// The pagination token, which is used to page through large result sets. Set this value to the value of the nextPageToken to retrieve the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Start of the time interval (exclusive), which is expressed as an RFC 3339 timestamp. If neither oldest nor timespan is specified, the default time interval will be (youngest - 4 hours, youngest]
    ///
    /// Sets the *oldest* query property to the given value.
    pub fn oldest(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._oldest = Some(new_value.to_string());
        self
    }
    /// A collection of labels for the matching time series, which are represented as:  
    /// - key==value: key equals the value 
    /// - key=~value: key regex matches the value 
    /// - key!=value: key does not equal the value 
    /// - key!~value: key regex does not match the value  For example, to list all of the time series descriptors for the region us-central1, you could specify:
    /// label=cloud.googleapis.com%2Flocation=~us-central1.*
    ///
    /// Append the given value to the *labels* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_labels(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._labels.push(new_value.to_string());
        self
    }
    /// Maximum number of time series descriptors per page. Used for pagination. If not specified, count = 100.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> TimeseriesDescriptorListCall<'a, S> {
        self._count = Some(new_value);
        self
    }
    /// The aggregation function that will reduce the data points in each window to a single point. This parameter is only valid for non-cumulative metrics with a value type of INT64 or DOUBLE.
    ///
    /// Sets the *aggregator* query property to the given value.
    pub fn aggregator(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, S> {
        self._aggregator = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TimeseriesDescriptorListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> TimeseriesDescriptorListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TimeseriesDescriptorListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TimeseriesDescriptorListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TimeseriesDescriptorListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


