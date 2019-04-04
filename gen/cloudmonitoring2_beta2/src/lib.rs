// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Monitoring* crate version *1.0.8+20170501*, where *20170501* is the exact revision of the *cloudmonitoring:v2beta2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Cloud Monitoring* *v2_beta2* API can be found at the
//! [official documentation site](https://cloud.google.com/monitoring/v2beta2/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/cloudmonitoring2_beta2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CloudMonitoring.html) ... 
//! 
//! * [metric descriptors](struct.MetricDescriptor.html)
//!  * [*create*](struct.MetricDescriptorCreateCall.html), [*delete*](struct.MetricDescriptorDeleteCall.html) and [*list*](struct.MetricDescriptorListCall.html)
//! * timeseries
//!  * [*list*](struct.TimeseryListCall.html) and [*write*](struct.TimeseryWriteCall.html)
//! * [timeseries descriptors](struct.TimeseriesDescriptor.html)
//!  * [*list*](struct.TimeseriesDescriptorListCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.CloudMonitoring.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.metric_descriptors().list(...).doit()
//! let r = hub.metric_descriptors().delete(...).doit()
//! let r = hub.metric_descriptors().create(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-cloudmonitoring2_beta2 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
//! use cloudmonitoring2_beta2::ListMetricDescriptorsRequest;
//! use cloudmonitoring2_beta2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use cloudmonitoring2_beta2::CloudMonitoring;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = ListMetricDescriptorsRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.metric_descriptors().list(req, "project")
//!              .query("dolores")
//!              .page_token("kasd")
//!              .count(-22)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


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
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::ListMetricDescriptorsRequest;
/// use cloudmonitoring2_beta2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListMetricDescriptorsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().list(req, "project")
///              .query("justo")
///              .page_token("amet.")
///              .count(-81)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct CloudMonitoring<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for CloudMonitoring<C, A> {}

impl<'a, C, A> CloudMonitoring<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CloudMonitoring<C, A> {
        CloudMonitoring {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://www.googleapis.com/cloudmonitoring/v2beta2/projects/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn metric_descriptors(&'a self) -> MetricDescriptorMethods<'a, C, A> {
        MetricDescriptorMethods { hub: &self }
    }
    pub fn timeseries(&'a self) -> TimeseryMethods<'a, C, A> {
        TimeseryMethods { hub: &self }
    }
    pub fn timeseries_descriptors(&'a self) -> TimeseriesDescriptorMethods<'a, C, A> {
        TimeseriesDescriptorMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
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
/// The underflow bucket is a special bucket that does not have the lowerBound field; it includes all of the events that are less than its upper bound.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistributionUnderflowBucket {
    /// The number of events whose values are in the interval defined by this bucket.
    pub count: Option<String>,
    /// The upper bound of the value interval of this bucket (exclusive).
    #[serde(rename="upperBound")]
    pub upper_bound: Option<f64>,
}

impl Part for PointDistributionUnderflowBucket {}


/// The response of cloudmonitoring.metricDescriptors.list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list metric descriptors](struct.MetricDescriptorListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMetricDescriptorsResponse {
    /// The returned metric descriptors.
    pub metrics: Option<Vec<MetricDescriptor>>,
    /// Pagination token. If present, indicates that additional results are available for retrieval. To access the results past the pagination limit, pass this value to the pageToken query parameter.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listMetricDescriptorsResponse".
    pub kind: Option<String>,
}

impl ResponseResult for ListMetricDescriptorsResponse {}


/// The request of cloudmonitoring.metricDescriptors.list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list metric descriptors](struct.MetricDescriptorListCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMetricDescriptorsRequest {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listMetricDescriptorsRequest".
    pub kind: Option<String>,
}

impl RequestValue for ListMetricDescriptorsRequest {}


/// The request of cloudmonitoring.timeseriesDescriptors.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries descriptors](struct.TimeseriesDescriptorListCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesDescriptorsRequest {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesDescriptorsRequest".
    pub kind: Option<String>,
}

impl RequestValue for ListTimeseriesDescriptorsRequest {}


/// When writing time series, TimeseriesPoint should be used instead of Timeseries, to enforce single point for each time series in the timeseries.write request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeseriesPoint {
    /// The descriptor of this time series.
    #[serde(rename="timeseriesDesc")]
    pub timeseries_desc: Option<TimeseriesDescriptor>,
    /// The data point in this time series snapshot.
    pub point: Option<Point>,
}

impl Part for TimeseriesPoint {}


/// Distribution data point value type. When writing distribution points, try to be consistent with the boundaries of your buckets. If you must modify the bucket boundaries, then do so by merging, partitioning, or appending rather than skewing them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistribution {
    /// The finite buckets.
    pub buckets: Option<Vec<PointDistributionBucket>>,
    /// The underflow bucket.
    #[serde(rename="underflowBucket")]
    pub underflow_bucket: Option<PointDistributionUnderflowBucket>,
    /// The overflow bucket.
    #[serde(rename="overflowBucket")]
    pub overflow_bucket: Option<PointDistributionOverflowBucket>,
}

impl Part for PointDistribution {}


/// A type in a metric contains information about how the metric is collected and what its data points look like.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptorTypeDescriptor {
    /// The data type of of individual points in the metric's time series. See Metric value types.
    #[serde(rename="valueType")]
    pub value_type: Option<String>,
    /// The method of collecting data for the metric. See Metric types.
    #[serde(rename="metricType")]
    pub metric_type: Option<String>,
}

impl Part for MetricDescriptorTypeDescriptor {}


/// A label in a metric is a description of this metric, including the key of this description (what the description is), and the value for this description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptorLabelDescriptor {
    /// Label description.
    pub description: Option<String>,
    /// Label key.
    pub key: Option<String>,
}

impl Part for MetricDescriptorLabelDescriptor {}


/// The histogram's bucket. Buckets that form the histogram of a distribution value. If the upper bound of a bucket, say U1, does not equal the lower bound of the next bucket, say L2, this means that there is no event in [U1, L2).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistributionBucket {
    /// The number of events whose values are in the interval defined by this bucket.
    pub count: Option<String>,
    /// The lower bound of the value interval of this bucket (inclusive).
    #[serde(rename="lowerBound")]
    pub lower_bound: Option<f64>,
    /// The upper bound of the value interval of this bucket (exclusive).
    #[serde(rename="upperBound")]
    pub upper_bound: Option<f64>,
}

impl Part for PointDistributionBucket {}


/// The request of cloudmonitoring.timeseries.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries](struct.TimeseryListCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesRequest {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesRequest".
    pub kind: Option<String>,
}

impl RequestValue for ListTimeseriesRequest {}


/// A metricDescriptor defines the name, label keys, and data type of a particular metric.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list metric descriptors](struct.MetricDescriptorListCall.html) (none)
/// * [delete metric descriptors](struct.MetricDescriptorDeleteCall.html) (none)
/// * [create metric descriptors](struct.MetricDescriptorCreateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptor {
    /// The project ID to which the metric belongs.
    pub project: Option<String>,
    /// Labels defined for this metric.
    pub labels: Option<Vec<MetricDescriptorLabelDescriptor>>,
    /// Description of this metric.
    pub description: Option<String>,
    /// The name of this metric.
    pub name: Option<String>,
    /// Type description for this metric.
    #[serde(rename="typeDescriptor")]
    pub type_descriptor: Option<MetricDescriptorTypeDescriptor>,
}

impl RequestValue for MetricDescriptor {}
impl Resource for MetricDescriptor {}
impl ResponseResult for MetricDescriptor {}


/// The response of cloudmonitoring.timeseries.write
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write timeseries](struct.TimeseryWriteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteTimeseriesResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#writeTimeseriesResponse".
    pub kind: Option<String>,
}

impl ResponseResult for WriteTimeseriesResponse {}


/// The response of cloudmonitoring.metricDescriptors.delete.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete metric descriptors](struct.MetricDescriptorDeleteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteMetricDescriptorResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#deleteMetricDescriptorResponse".
    pub kind: Option<String>,
}

impl ResponseResult for DeleteMetricDescriptorResponse {}


/// The request of cloudmonitoring.timeseries.write
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [write timeseries](struct.TimeseryWriteCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WriteTimeseriesRequest {
    /// The label's name.
    #[serde(rename="commonLabels")]
    pub common_labels: Option<HashMap<String, String>>,
    /// Provide time series specific labels and the data points for each time series. The labels in timeseries and the common_labels should form a complete list of labels that required by the metric.
    pub timeseries: Option<Vec<TimeseriesPoint>>,
}

impl RequestValue for WriteTimeseriesRequest {}


/// Point is a single point in a time series. It consists of a start time, an end time, and a value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    /// The interval [start, end] is the time period to which the point's value applies. For gauge metrics, whose values are instantaneous measurements, this interval should be empty (start should equal end). For cumulative metrics (of which deltas and rates are special cases), the interval should be non-empty. Both start and end are RFC 3339 strings.
    pub start: Option<String>,
    /// The value of this data point. Either "true" or "false".
    #[serde(rename="boolValue")]
    pub bool_value: Option<bool>,
    /// The interval [start, end] is the time period to which the point's value applies. For gauge metrics, whose values are instantaneous measurements, this interval should be empty (start should equal end). For cumulative metrics (of which deltas and rates are special cases), the interval should be non-empty. Both start and end are RFC 3339 strings.
    pub end: Option<String>,
    /// The value of this data point as a distribution. A distribution value can contain a list of buckets and/or an underflowBucket and an overflowBucket. The values of these points can be used to create a histogram.
    #[serde(rename="distributionValue")]
    pub distribution_value: Option<PointDistribution>,
    /// The value of this data point in string format.
    #[serde(rename="stringValue")]
    pub string_value: Option<String>,
    /// The value of this data point as a 64-bit integer.
    #[serde(rename="int64Value")]
    pub int64_value: Option<String>,
    /// The value of this data point as a double-precision floating-point number.
    #[serde(rename="doubleValue")]
    pub double_value: Option<f64>,
}

impl Part for Point {}


/// TimeseriesDescriptor identifies a single time series.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries descriptors](struct.TimeseriesDescriptorListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeseriesDescriptor {
    /// The Developers Console project number to which this time series belongs.
    pub project: Option<String>,
    /// The name of the metric.
    pub metric: Option<String>,
    /// The label's name.
    pub labels: Option<HashMap<String, String>>,
}

impl Resource for TimeseriesDescriptor {}


/// The monitoring data is organized as metrics and stored as data points that are recorded over time. Each data point represents information like the CPU utilization of your virtual machine. A historical record of these data points is called a time series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Timeseries {
    /// The descriptor of this time series.
    #[serde(rename="timeseriesDesc")]
    pub timeseries_desc: Option<TimeseriesDescriptor>,
    /// The data points of this time series. The points are listed in order of their end timestamp, from younger to older.
    pub points: Option<Vec<Point>>,
}

impl Part for Timeseries {}


/// The overflow bucket is a special bucket that does not have the upperBound field; it includes all of the events that are no less than its lower bound.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointDistributionOverflowBucket {
    /// The number of events whose values are in the interval defined by this bucket.
    pub count: Option<String>,
    /// The lower bound of the value interval of this bucket (inclusive).
    #[serde(rename="lowerBound")]
    pub lower_bound: Option<f64>,
}

impl Part for PointDistributionOverflowBucket {}


/// The response of cloudmonitoring.timeseriesDescriptors.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries descriptors](struct.TimeseriesDescriptorListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesDescriptorsResponse {
    /// Pagination token. If present, indicates that additional results are available for retrieval. To access the results past the pagination limit, set this value to the pageToken query parameter.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The youngest timestamp of the interval of this query, as an RFC 3339 string.
    pub youngest: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesDescriptorsResponse".
    pub kind: Option<String>,
    /// The oldest timestamp of the interval of this query, as an RFC 3339 string.
    pub oldest: Option<String>,
    /// The returned time series descriptors.
    pub timeseries: Option<Vec<TimeseriesDescriptor>>,
}

impl ResponseResult for ListTimeseriesDescriptorsResponse {}


/// The response of cloudmonitoring.timeseries.list
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list timeseries](struct.TimeseryListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTimeseriesResponse {
    /// Pagination token. If present, indicates that additional results are available for retrieval. To access the results past the pagination limit, set the pageToken query parameter to this value. All of the points of a time series will be returned before returning any point of the subsequent time series.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The youngest timestamp of the interval of this query as an RFC 3339 string.
    pub youngest: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "cloudmonitoring#listTimeseriesResponse".
    pub kind: Option<String>,
    /// The oldest timestamp of the interval of this query as an RFC 3339 string.
    pub oldest: Option<String>,
    /// The returned time series.
    pub timeseries: Option<Vec<Timeseries>>,
}

impl ResponseResult for ListTimeseriesResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *timeseriesDescriptor* resources.
/// It is not used directly, but through the `CloudMonitoring` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.timeseries_descriptors();
/// # }
/// ```
pub struct TimeseriesDescriptorMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
}

impl<'a, C, A> MethodsBuilder for TimeseriesDescriptorMethods<'a, C, A> {}

impl<'a, C, A> TimeseriesDescriptorMethods<'a, C, A> {
    
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
    pub fn list(&self, request: ListTimeseriesDescriptorsRequest, project: &str, metric: &str, youngest: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
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
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *timesery* resources.
/// It is not used directly, but through the `CloudMonitoring` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `write(...)`
/// // to build up your call.
/// let rb = hub.timeseries();
/// # }
/// ```
pub struct TimeseryMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
}

impl<'a, C, A> MethodsBuilder for TimeseryMethods<'a, C, A> {}

impl<'a, C, A> TimeseryMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Put data points to one or more time series for one or more metrics. If a time series does not exist, a new time series will be created. It is not allowed to write a time series point that is older than the existing youngest point of that time series. Points that are older than the existing youngest point of that time series will be discarded silently. Therefore, users should make sure that points of a time series are written sequentially in the order of their end time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID. The value can be the numeric project ID or string-based project name.
    pub fn write(&self, request: WriteTimeseriesRequest, project: &str) -> TimeseryWriteCall<'a, C, A> {
        TimeseryWriteCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
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
    pub fn list(&self, request: ListTimeseriesRequest, project: &str, metric: &str, youngest: &str) -> TimeseryListCall<'a, C, A> {
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
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *metricDescriptor* resources.
/// It is not used directly, but through the `CloudMonitoring` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.metric_descriptors();
/// # }
/// ```
pub struct MetricDescriptorMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
}

impl<'a, C, A> MethodsBuilder for MetricDescriptorMethods<'a, C, A> {}

impl<'a, C, A> MetricDescriptorMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List metric descriptors that match the query. If the query is not set, then all of the metric descriptors will be returned. Large responses will be paginated, use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project id. The value can be the numeric project ID or string-based project name.
    pub fn list(&self, request: ListMetricDescriptorsRequest, project: &str) -> MetricDescriptorListCall<'a, C, A> {
        MetricDescriptorListCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
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
    pub fn delete(&self, project: &str, metric: &str) -> MetricDescriptorDeleteCall<'a, C, A> {
        MetricDescriptorDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _metric: metric.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new metric.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project id. The value can be the numeric project ID or string-based project name.
    pub fn create(&self, request: MetricDescriptor, project: &str) -> MetricDescriptorCreateCall<'a, C, A> {
        MetricDescriptorCreateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// List the descriptors of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
///
/// A builder for the *list* method supported by a *timeseriesDescriptor* resource.
/// It is not used directly, but through a `TimeseriesDescriptorMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::ListTimeseriesDescriptorsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListTimeseriesDescriptorsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.timeseries_descriptors().list(req, "project", "metric", "youngest")
///              .window("dolores")
///              .timespan("gubergren")
///              .page_token("sadipscing")
///              .oldest("aliquyam")
///              .add_labels("ea")
///              .count(-61)
///              .aggregator("justo")
///              .doit();
/// # }
/// ```
pub struct TimeseriesDescriptorListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
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
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TimeseriesDescriptorListCall<'a, C, A> {}

impl<'a, C, A> TimeseriesDescriptorListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListTimeseriesDescriptorsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudmonitoring.timeseriesDescriptors.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("metric", self._metric.to_string()));
        params.push(("youngest", self._youngest.to_string()));
        if let Some(value) = self._window {
            params.push(("window", value.to_string()));
        }
        if let Some(value) = self._timespan {
            params.push(("timespan", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._oldest {
            params.push(("oldest", value.to_string()));
        }
        if self._labels.len() > 0 {
            for f in self._labels.iter() {
                params.push(("labels", f.to_string()));
            }
        }
        if let Some(value) = self._count {
            params.push(("count", value.to_string()));
        }
        if let Some(value) = self._aggregator {
            params.push(("aggregator", value.to_string()));
        }
        for &field in ["alt", "project", "metric", "youngest", "window", "timespan", "pageToken", "oldest", "labels", "count", "aggregator"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/timeseriesDescriptors/{metric}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{metric}", "metric")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["metric", "project"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: ListTimeseriesDescriptorsRequest) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    ///
    /// Sets the *metric* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metric(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._metric = new_value.to_string();
        self
    }
    /// End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    ///
    /// Sets the *youngest* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn youngest(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
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
    pub fn window(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
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
    pub fn timespan(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._timespan = Some(new_value.to_string());
        self
    }
    /// The pagination token, which is used to page through large result sets. Set this value to the value of the nextPageToken to retrieve the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Start of the time interval (exclusive), which is expressed as an RFC 3339 timestamp. If neither oldest nor timespan is specified, the default time interval will be (youngest - 4 hours, youngest]
    ///
    /// Sets the *oldest* query property to the given value.
    pub fn oldest(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
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
    pub fn add_labels(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._labels.push(new_value.to_string());
        self
    }
    /// Maximum number of time series descriptors per page. Used for pagination. If not specified, count = 100.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._count = Some(new_value);
        self
    }
    /// The aggregation function that will reduce the data points in each window to a single point. This parameter is only valid for non-cumulative metrics with a value type of INT64 or DOUBLE.
    ///
    /// Sets the *aggregator* query property to the given value.
    pub fn aggregator(mut self, new_value: &str) -> TimeseriesDescriptorListCall<'a, C, A> {
        self._aggregator = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TimeseriesDescriptorListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TimeseriesDescriptorListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> TimeseriesDescriptorListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Put data points to one or more time series for one or more metrics. If a time series does not exist, a new time series will be created. It is not allowed to write a time series point that is older than the existing youngest point of that time series. Points that are older than the existing youngest point of that time series will be discarded silently. Therefore, users should make sure that points of a time series are written sequentially in the order of their end time.
///
/// A builder for the *write* method supported by a *timesery* resource.
/// It is not used directly, but through a `TimeseryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::WriteTimeseriesRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = WriteTimeseriesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.timeseries().write(req, "project")
///              .doit();
/// # }
/// ```
pub struct TimeseryWriteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
    _request: WriteTimeseriesRequest,
    _project: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TimeseryWriteCall<'a, C, A> {}

impl<'a, C, A> TimeseryWriteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, WriteTimeseriesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudmonitoring.timeseries.write",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        for &field in ["alt", "project"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/timeseries:write";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["project"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: WriteTimeseriesRequest) -> TimeseryWriteCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project ID. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> TimeseryWriteCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TimeseryWriteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TimeseryWriteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> TimeseryWriteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List the data points of the time series that match the metric and labels values and that have data points in the interval. Large responses are paginated; use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
///
/// A builder for the *list* method supported by a *timesery* resource.
/// It is not used directly, but through a `TimeseryMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::ListTimeseriesRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListTimeseriesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.timeseries().list(req, "project", "metric", "youngest")
///              .window("ipsum")
///              .timespan("Lorem")
///              .page_token("et")
///              .oldest("duo")
///              .add_labels("aliquyam")
///              .count(-9)
///              .aggregator("Lorem")
///              .doit();
/// # }
/// ```
pub struct TimeseryListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
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
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TimeseryListCall<'a, C, A> {}

impl<'a, C, A> TimeseryListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListTimeseriesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudmonitoring.timeseries.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("metric", self._metric.to_string()));
        params.push(("youngest", self._youngest.to_string()));
        if let Some(value) = self._window {
            params.push(("window", value.to_string()));
        }
        if let Some(value) = self._timespan {
            params.push(("timespan", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._oldest {
            params.push(("oldest", value.to_string()));
        }
        if self._labels.len() > 0 {
            for f in self._labels.iter() {
                params.push(("labels", f.to_string()));
            }
        }
        if let Some(value) = self._count {
            params.push(("count", value.to_string()));
        }
        if let Some(value) = self._aggregator {
            params.push(("aggregator", value.to_string()));
        }
        for &field in ["alt", "project", "metric", "youngest", "window", "timespan", "pageToken", "oldest", "labels", "count", "aggregator"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/timeseries/{metric}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{metric}", "metric")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["metric", "project"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: ListTimeseriesRequest) -> TimeseryListCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project ID to which this time series belongs. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// Metric names are protocol-free URLs as listed in the Supported Metrics page. For example, compute.googleapis.com/instance/disk/read_ops_count.
    ///
    /// Sets the *metric* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metric(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
        self._metric = new_value.to_string();
        self
    }
    /// End of the time interval (inclusive), which is expressed as an RFC 3339 timestamp.
    ///
    /// Sets the *youngest* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn youngest(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
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
    pub fn window(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
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
    pub fn timespan(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
        self._timespan = Some(new_value.to_string());
        self
    }
    /// The pagination token, which is used to page through large result sets. Set this value to the value of the nextPageToken to retrieve the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Start of the time interval (exclusive), which is expressed as an RFC 3339 timestamp. If neither oldest nor timespan is specified, the default time interval will be (youngest - 4 hours, youngest]
    ///
    /// Sets the *oldest* query property to the given value.
    pub fn oldest(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
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
    pub fn add_labels(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
        self._labels.push(new_value.to_string());
        self
    }
    /// Maximum number of data points per page, which is used for pagination of results.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> TimeseryListCall<'a, C, A> {
        self._count = Some(new_value);
        self
    }
    /// The aggregation function that will reduce the data points in each window to a single point. This parameter is only valid for non-cumulative metrics with a value type of INT64 or DOUBLE.
    ///
    /// Sets the *aggregator* query property to the given value.
    pub fn aggregator(mut self, new_value: &str) -> TimeseryListCall<'a, C, A> {
        self._aggregator = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TimeseryListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TimeseryListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> TimeseryListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List metric descriptors that match the query. If the query is not set, then all of the metric descriptors will be returned. Large responses will be paginated, use the nextPageToken returned in the response to request subsequent pages of results by setting the pageToken query parameter to the value of the nextPageToken.
///
/// A builder for the *list* method supported by a *metricDescriptor* resource.
/// It is not used directly, but through a `MetricDescriptorMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::ListMetricDescriptorsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ListMetricDescriptorsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().list(req, "project")
///              .query("erat")
///              .page_token("sadipscing")
///              .count(-48)
///              .doit();
/// # }
/// ```
pub struct MetricDescriptorListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
    _request: ListMetricDescriptorsRequest,
    _project: String,
    _query: Option<String>,
    _page_token: Option<String>,
    _count: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MetricDescriptorListCall<'a, C, A> {}

impl<'a, C, A> MetricDescriptorListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListMetricDescriptorsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudmonitoring.metricDescriptors.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        if let Some(value) = self._query {
            params.push(("query", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._count {
            params.push(("count", value.to_string()));
        }
        for &field in ["alt", "project", "query", "pageToken", "count"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/metricDescriptors";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["project"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: ListMetricDescriptorsRequest) -> MetricDescriptorListCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project id. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> MetricDescriptorListCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// The query used to search against existing metrics. Separate keywords with a space; the service joins all keywords with AND, meaning that all keywords must match for a metric to be returned. If this field is omitted, all metrics are returned. If an empty string is passed with this field, no metrics are returned.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> MetricDescriptorListCall<'a, C, A> {
        self._query = Some(new_value.to_string());
        self
    }
    /// The pagination token, which is used to page through large result sets. Set this value to the value of the nextPageToken to retrieve the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> MetricDescriptorListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of metric descriptors per page. Used for pagination. If not specified, count = 100.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> MetricDescriptorListCall<'a, C, A> {
        self._count = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MetricDescriptorListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MetricDescriptorListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> MetricDescriptorListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Delete an existing metric.
///
/// A builder for the *delete* method supported by a *metricDescriptor* resource.
/// It is not used directly, but through a `MetricDescriptorMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().delete("project", "metric")
///              .doit();
/// # }
/// ```
pub struct MetricDescriptorDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
    _project: String,
    _metric: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MetricDescriptorDeleteCall<'a, C, A> {}

impl<'a, C, A> MetricDescriptorDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DeleteMetricDescriptorResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudmonitoring.metricDescriptors.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("metric", self._metric.to_string()));
        for &field in ["alt", "project", "metric"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/metricDescriptors/{metric}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{metric}", "metric")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["metric", "project"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn project(mut self, new_value: &str) -> MetricDescriptorDeleteCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// Name of the metric.
    ///
    /// Sets the *metric* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metric(mut self, new_value: &str) -> MetricDescriptorDeleteCall<'a, C, A> {
        self._metric = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MetricDescriptorDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MetricDescriptorDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> MetricDescriptorDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Create a new metric.
///
/// A builder for the *create* method supported by a *metricDescriptor* resource.
/// It is not used directly, but through a `MetricDescriptorMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudmonitoring2_beta2 as cloudmonitoring2_beta2;
/// use cloudmonitoring2_beta2::MetricDescriptor;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudmonitoring2_beta2::CloudMonitoring;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudMonitoring::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = MetricDescriptor::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metric_descriptors().create(req, "project")
///              .doit();
/// # }
/// ```
pub struct MetricDescriptorCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudMonitoring<C, A>,
    _request: MetricDescriptor,
    _project: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MetricDescriptorCreateCall<'a, C, A> {}

impl<'a, C, A> MetricDescriptorCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, MetricDescriptor)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudmonitoring.metricDescriptors.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        for &field in ["alt", "project"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/metricDescriptors";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["project"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: MetricDescriptor) -> MetricDescriptorCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project id. The value can be the numeric project ID or string-based project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> MetricDescriptorCreateCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MetricDescriptorCreateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MetricDescriptorCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> MetricDescriptorCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


