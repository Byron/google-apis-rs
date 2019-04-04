// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Service Control* crate version *1.0.8+20190323*, where *20190323* is the exact revision of the *servicecontrol:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Service Control* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/service-control/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/servicecontrol1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.ServiceControl.html) ... 
//! 
//! * services
//!  * [*allocate quota*](struct.ServiceAllocateQuotaCall.html), [*check*](struct.ServiceCheckCall.html) and [*report*](struct.ServiceReportCall.html)
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
//! * **[Hub](struct.ServiceControl.html)**
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
//! let r = hub.services().check(...).doit()
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
//! google-servicecontrol1 = "*"
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
//! extern crate google_servicecontrol1 as servicecontrol1;
//! use servicecontrol1::CheckRequest;
//! use servicecontrol1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use servicecontrol1::ServiceControl;
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
//! let mut hub = ServiceControl::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = CheckRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.services().check(req, "serviceName")
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

    /// Manage your Google Service Control data
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::Full => "https://www.googleapis.com/auth/servicecontrol",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all ServiceControl related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_servicecontrol1 as servicecontrol1;
/// use servicecontrol1::CheckRequest;
/// use servicecontrol1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use servicecontrol1::ServiceControl;
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
/// let mut hub = ServiceControl::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CheckRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().check(req, "serviceName")
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
pub struct ServiceControl<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for ServiceControl<C, A> {}

impl<'a, C, A> ServiceControl<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> ServiceControl<C, A> {
        ServiceControl {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://servicecontrol.googleapis.com/".to_string(),
            _root_url: "https://servicecontrol.googleapis.com/".to_string(),
        }
    }

    pub fn services(&'a self) -> ServiceMethods<'a, C, A> {
        ServiceMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://servicecontrol.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://servicecontrol.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Defines the errors to be returned in
/// google.api.servicecontrol.v1.CheckResponse.check_errors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckError {
    /// Contains public information about the check error. If available,
    /// `status.code` will be non zero and client can propagate it out as public
    /// error.
    pub status: Option<Status>,
    /// The error code.
    pub code: Option<String>,
    /// Free-form text providing details on the error cause of the error.
    pub detail: Option<String>,
    /// Subject to whom this error applies. See the specific code enum for more
    /// details on this field. For example:
    ///     - “project:<project-id or project-number>”
    ///     - “folder:<folder-id>”
    ///     - “organization:<organization-id>”
    pub subject: Option<String>,
}

impl Part for CheckError {}


/// Represents a single metric value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    /// A money value.
    #[serde(rename="moneyValue")]
    pub money_value: Option<Money>,
    /// The labels describing the metric value.
    /// See comments on google.api.servicecontrol.v1.Operation.labels for
    /// the overriding relationship.
    pub labels: Option<HashMap<String, String>>,
    /// A double precision floating point value.
    #[serde(rename="doubleValue")]
    pub double_value: Option<f64>,
    /// A boolean value.
    #[serde(rename="boolValue")]
    pub bool_value: Option<bool>,
    /// The start of the time period over which this metric value's measurement
    /// applies. The time period has different semantics for different metric
    /// types (cumulative, delta, and gauge). See the metric definition
    /// documentation in the service configuration for details.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
    /// A distribution value.
    #[serde(rename="distributionValue")]
    pub distribution_value: Option<Distribution>,
    /// A text string value.
    #[serde(rename="stringValue")]
    pub string_value: Option<String>,
    /// A signed 64-bit integer value.
    #[serde(rename="int64Value")]
    pub int64_value: Option<String>,
    /// The end of the time period over which this metric value's measurement
    /// applies.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
}

impl Part for MetricValue {}


/// Represents information regarding a quota operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaOperation {
    /// Quota mode for this operation.
    #[serde(rename="quotaMode")]
    pub quota_mode: Option<String>,
    /// Identity of the consumer for whom this quota operation is being performed.
    /// 
    /// This can be in one of the following formats:
    ///   project:<project_id>,
    ///   project_number:<project_number>,
    ///   api_key:<api_key>.
    #[serde(rename="consumerId")]
    pub consumer_id: Option<String>,
    /// Represents information about this operation. Each MetricValueSet
    /// corresponds to a metric defined in the service configuration.
    /// The data type used in the MetricValueSet must agree with
    /// the data type specified in the metric definition.
    /// 
    /// Within a single operation, it is not allowed to have more than one
    /// MetricValue instances that have the same metric names and identical
    /// label value combinations. If a request has such duplicated MetricValue
    /// instances, the entire request is rejected with
    /// an invalid argument error.
    /// 
    /// This field is mutually exclusive with method_name.
    #[serde(rename="quotaMetrics")]
    pub quota_metrics: Option<Vec<MetricValueSet>>,
    /// Fully qualified name of the API method for which this quota operation is
    /// requested. This name is used for matching quota rules or metric rules and
    /// billing status rules defined in service configuration.
    /// 
    /// This field should not be set if any of the following is true:
    /// (1) the quota operation is performed on non-API resources.
    /// (2) quota_metrics is set because the caller is doing quota override.
    /// 
    /// Example of an RPC method name:
    ///     google.example.library.v1.LibraryService.CreateShelf
    #[serde(rename="methodName")]
    pub method_name: Option<String>,
    /// Labels describing the operation.
    pub labels: Option<HashMap<String, String>>,
    /// Identity of the operation. This is expected to be unique within the scope
    /// of the service that generated the operation, and guarantees idempotency in
    /// case of retries.
    /// 
    /// UUID version 4 is recommended, though not required. In scenarios where an
    /// operation is computed from existing information and an idempotent id is
    /// desirable for deduplication purpose, UUID version 5 is recommended. See
    /// RFC 4122 for details.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl Part for QuotaOperation {}


/// Request message for the Report method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report services](struct.ServiceReportCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    /// Operations to be reported.
    /// 
    /// Typically the service should report one operation per request.
    /// Putting multiple operations into a single request is allowed, but should
    /// be used only when multiple operations are natually available at the time
    /// of the report.
    /// 
    /// If multiple operations are in a single request, the total request size
    /// should be no larger than 1MB. See ReportResponse.report_errors for
    /// partial failure behavior.
    pub operations: Option<Vec<Operation>>,
    /// Specifies which version of service config should be used to process the
    /// request.
    /// 
    /// If unspecified or no matching version can be found, the
    /// latest one will be used.
    #[serde(rename="serviceConfigId")]
    pub service_config_id: Option<String>,
}

impl RequestValue for ReportRequest {}


/// Contains the quota information for a quota check response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaInfo {
    /// Quota metrics to indicate the usage. Depending on the check request, one or
    /// more of the following metrics will be included:
    /// 
    /// 1. For rate quota, per quota group or per quota metric incremental usage
    /// will be specified using the following delta metric:
    ///   "serviceruntime.googleapis.com/api/consumer/quota_used_count"
    /// 
    /// 2. For allocation quota, per quota metric total usage will be specified
    /// using the following gauge metric:
    ///   "serviceruntime.googleapis.com/allocation/consumer/quota_used_count"
    /// 
    /// 3. For both rate quota and allocation quota, the quota limit reached
    /// condition will be specified using the following boolean metric:
    ///   "serviceruntime.googleapis.com/quota/exceeded"
    #[serde(rename="quotaMetrics")]
    pub quota_metrics: Option<Vec<MetricValueSet>>,
    /// Quota Metrics that have exceeded quota limits.
    /// For QuotaGroup-based quota, this is QuotaGroup.name
    /// For QuotaLimit-based quota, this is QuotaLimit.name
    /// See: google.api.Quota
    /// Deprecated: Use quota_metrics to get per quota group limit exceeded status.
    #[serde(rename="limitExceeded")]
    pub limit_exceeded: Option<Vec<String>>,
    /// Map of quota group name to the actual number of tokens consumed. If the
    /// quota check was not successful, then this will not be populated due to no
    /// quota consumption.
    /// 
    /// We are not merging this field with 'quota_metrics' field because of the
    /// complexity of scaling in Chemist client code base. For simplicity, we will
    /// keep this field for Castor (that scales quota usage) and 'quota_metrics'
    /// for SuperQuota (that doesn't scale quota usage).
    /// 
    #[serde(rename="quotaConsumed")]
    pub quota_consumed: Option<HashMap<String, i32>>,
}

impl Part for QuotaInfo {}


/// Distribution represents a frequency distribution of double-valued sample
/// points. It contains the size of the population of sample points plus
/// additional optional information:
/// 
///   - the arithmetic mean of the samples
///   - the minimum and maximum of the samples
///   - the sum-squared-deviation of the samples, used to compute variance
///   - a histogram of the values of the sample points
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Distribution {
    /// The total number of samples in the distribution. Must be >= 0.
    pub count: Option<String>,
    /// The sum of squared deviations from the mean:
    ///   Sum[i=1..count]((x_i - mean)^2)
    /// where each x_i is a sample values. If `count` is zero then this field
    /// must be zero, otherwise validation of the request fails.
    #[serde(rename="sumOfSquaredDeviation")]
    pub sum_of_squared_deviation: Option<f64>,
    /// The number of samples in each histogram bucket. `bucket_counts` are
    /// optional. If present, they must sum to the `count` value.
    /// 
    /// The buckets are defined below in `bucket_option`. There are N buckets.
    /// `bucket_counts[0]` is the number of samples in the underflow bucket.
    /// `bucket_counts[1]` to `bucket_counts[N-1]` are the numbers of samples
    /// in each of the finite buckets. And `bucket_counts[N] is the number
    /// of samples in the overflow bucket. See the comments of `bucket_option`
    /// below for more details.
    /// 
    /// Any suffix of trailing zeros may be omitted.
    #[serde(rename="bucketCounts")]
    pub bucket_counts: Option<Vec<i64>>,
    /// Buckets with exponentially growing width.
    #[serde(rename="exponentialBuckets")]
    pub exponential_buckets: Option<ExponentialBuckets>,
    /// The maximum of the population of values. Ignored if `count` is zero.
    pub maximum: Option<f64>,
    /// The minimum of the population of values. Ignored if `count` is zero.
    pub minimum: Option<f64>,
    /// Buckets with constant width.
    #[serde(rename="linearBuckets")]
    pub linear_buckets: Option<LinearBuckets>,
    /// Buckets with arbitrary user-provided width.
    #[serde(rename="explicitBuckets")]
    pub explicit_buckets: Option<ExplicitBuckets>,
    /// The arithmetic mean of the samples in the distribution. If `count` is
    /// zero then this field must be zero.
    pub mean: Option<f64>,
}

impl Part for Distribution {}


/// Describing buckets with arbitrary user-provided width.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExplicitBuckets {
    /// 'bound' is a list of strictly increasing boundaries between
    /// buckets. Note that a list of length N-1 defines N buckets because
    /// of fenceposting. See comments on `bucket_options` for details.
    /// 
    /// The i'th finite bucket covers the interval
    ///   [bound[i-1], bound[i])
    /// where i ranges from 1 to bound_size() - 1. Note that there are no
    /// finite buckets at all if 'bound' only contains a single element; in
    /// that special case the single bound defines the boundary between the
    /// underflow and overflow buckets.
    /// 
    /// bucket number                   lower bound    upper bound
    ///  i == 0 (underflow)              -inf           bound[i]
    ///  0 < i < bound_size()            bound[i-1]     bound[i]
    ///  i == bound_size() (overflow)    bound[i-1]     +inf
    pub bounds: Option<Vec<f64>>,
}

impl Part for ExplicitBuckets {}


/// Request message for the Check method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check services](struct.ServiceCheckCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckRequest {
    /// Indicates if service activation check should be skipped for this request.
    /// Default behavior is to perform the check and apply relevant quota.
    /// WARNING: Setting this flag to "true" will disable quota enforcement.
    #[serde(rename="skipActivationCheck")]
    pub skip_activation_check: Option<bool>,
    /// The operation to be checked.
    pub operation: Option<Operation>,
    /// Specifies which version of service configuration should be used to process
    /// the request.
    /// 
    /// If unspecified or no matching version can be found, the
    /// latest one will be used.
    #[serde(rename="serviceConfigId")]
    pub service_config_id: Option<String>,
    /// Requests the project settings to be returned as part of the check response.
    #[serde(rename="requestProjectSettings")]
    pub request_project_settings: Option<bool>,
}

impl RequestValue for CheckRequest {}


/// A common proto for logging HTTP requests. Only contains semantics
/// defined by the HTTP specification. Product-specific logging
/// information MUST be defined in a separate message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    /// The response code indicating the status of the response.
    /// Examples: 200, 404.
    pub status: Option<i32>,
    /// Whether or not the response was validated with the origin server before
    /// being served from cache. This field is only meaningful if `cache_hit` is
    /// True.
    #[serde(rename="cacheValidatedWithOriginServer")]
    pub cache_validated_with_origin_server: Option<bool>,
    /// Protocol used for the request. Examples: "HTTP/1.1", "HTTP/2", "websocket"
    pub protocol: Option<String>,
    /// The request processing latency on the server, from the time the request was
    /// received until the response was sent.
    pub latency: Option<String>,
    /// The number of HTTP response bytes inserted into cache. Set only when a
    /// cache fill was attempted.
    #[serde(rename="cacheFillBytes")]
    pub cache_fill_bytes: Option<String>,
    /// The scheme (http, https), the host name, the path, and the query
    /// portion of the URL that was requested.
    /// Example: `"http://example.com/some/info?color=red"`.
    #[serde(rename="requestUrl")]
    pub request_url: Option<String>,
    /// Whether or not an entity was served from cache
    /// (with or without validation).
    #[serde(rename="cacheHit")]
    pub cache_hit: Option<bool>,
    /// The IP address (IPv4 or IPv6) of the origin server that the request was
    /// sent to.
    #[serde(rename="serverIp")]
    pub server_ip: Option<String>,
    /// Whether or not a cache lookup was attempted.
    #[serde(rename="cacheLookup")]
    pub cache_lookup: Option<bool>,
    /// The request method. Examples: `"GET"`, `"HEAD"`, `"PUT"`, `"POST"`.
    #[serde(rename="requestMethod")]
    pub request_method: Option<String>,
    /// The referer URL of the request, as defined in
    /// [HTTP/1.1 Header Field
    /// Definitions](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html).
    pub referer: Option<String>,
    /// The IP address (IPv4 or IPv6) of the client that issued the HTTP
    /// request. Examples: `"192.168.1.1"`, `"FE80::0202:B3FF:FE1E:8329"`.
    #[serde(rename="remoteIp")]
    pub remote_ip: Option<String>,
    /// The user agent sent by the client. Example:
    /// `"Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET
    /// CLR 1.0.3705)"`.
    #[serde(rename="userAgent")]
    pub user_agent: Option<String>,
    /// The size of the HTTP request message in bytes, including the request
    /// headers and the request body.
    #[serde(rename="requestSize")]
    pub request_size: Option<String>,
    /// The size of the HTTP response message sent back to the client, in bytes,
    /// including the response headers and the response body.
    #[serde(rename="responseSize")]
    pub response_size: Option<String>,
}

impl Part for HttpRequest {}


/// Response message for the Report method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report services](struct.ServiceReportCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportResponse {
    /// Partial failures, one for each `Operation` in the request that failed
    /// processing. There are three possible combinations of the RPC status:
    /// 
    /// 1. The combination of a successful RPC status and an empty `report_errors`
    ///    list indicates a complete success where all `Operations` in the
    ///    request are processed successfully.
    /// 2. The combination of a successful RPC status and a non-empty
    ///    `report_errors` list indicates a partial success where some
    ///    `Operations` in the request succeeded. Each
    ///    `Operation` that failed processing has a corresponding item
    ///    in this list.
    /// 3. A failed RPC status indicates a general non-deterministic failure.
    ///    When this happens, it's impossible to know which of the
    ///    'Operations' in the request succeeded or failed.
    #[serde(rename="reportErrors")]
    pub report_errors: Option<Vec<ReportError>>,
    /// Quota usage for each quota release `Operation` request.
    /// 
    /// Fully or partially failed quota release request may or may not be present
    /// in `report_quota_info`. For example, a failed quota release request will
    /// have the current quota usage info when precise quota library returns the
    /// info. A deadline exceeded quota request will not have quota usage info.
    /// 
    /// If there is no quota release request, report_quota_info will be empty.
    /// 
    #[serde(rename="reportInfos")]
    pub report_infos: Option<Vec<ReportInfo>>,
    /// The actual config id used to process the request.
    #[serde(rename="serviceConfigId")]
    pub service_config_id: Option<String>,
}

impl ResponseResult for ReportResponse {}


/// Represents a set of metric values in the same metric.
/// Each metric value in the set should have a unique combination of start time,
/// end time, and label values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValueSet {
    /// The values in this metric.
    #[serde(rename="metricValues")]
    pub metric_values: Option<Vec<MetricValue>>,
    /// The metric name defined in the service configuration.
    #[serde(rename="metricName")]
    pub metric_name: Option<String>,
}

impl Part for MetricValueSet {}


/// Represents error information for QuotaOperation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaError {
    /// Error code.
    pub code: Option<String>,
    /// Free-form text that provides details on the cause of the error.
    pub description: Option<String>,
    /// Subject to whom this error applies. See the specific enum for more details
    /// on this field. For example, "clientip:<ip address of client>" or
    /// "project:<Google developer project id>".
    pub subject: Option<String>,
}

impl Part for QuotaError {}


/// Describes a resource associated with this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceInfo {
    /// The identifier of the parent of this resource instance.
    /// Must be in one of the following formats:
    ///     - “projects/<project-id or project-number>”
    ///     - “folders/<folder-id>”
    ///     - “organizations/<organization-id>”
    #[serde(rename="resourceContainer")]
    pub resource_container: Option<String>,
    /// The location of the resource. If not empty, the resource will be checked
    /// against location policy. The value must be a valid zone, region or
    /// multiregion. For example: "europe-west4" or "northamerica-northeast1-a"
    #[serde(rename="resourceLocation")]
    pub resource_location: Option<String>,
    /// Name of the resource. This is used for auditing purposes.
    #[serde(rename="resourceName")]
    pub resource_name: Option<String>,
}

impl Part for ResourceInfo {}


/// An individual log entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// Optional. Information about the HTTP request associated with this
    /// log entry, if applicable.
    #[serde(rename="httpRequest")]
    pub http_request: Option<HttpRequest>,
    /// Required. The log to which this log entry belongs. Examples: `"syslog"`,
    /// `"book_log"`.
    pub name: Option<String>,
    /// The log entry payload, represented as a Unicode string (UTF-8).
    #[serde(rename="textPayload")]
    pub text_payload: Option<String>,
    /// The time the event described by the log entry occurred. If
    /// omitted, defaults to operation start time.
    pub timestamp: Option<String>,
    /// A set of user-defined (key, value) data that provides additional
    /// information about the log entry.
    pub labels: Option<HashMap<String, String>>,
    /// The log entry payload, represented as a structure that
    /// is expressed as a JSON object.
    #[serde(rename="structPayload")]
    pub struct_payload: Option<HashMap<String, String>>,
    /// A unique ID for the log entry used for deduplication. If omitted,
    /// the implementation will generate one based on operation_id.
    #[serde(rename="insertId")]
    pub insert_id: Option<String>,
    /// The log entry payload, represented as a protocol buffer that is
    /// expressed as a JSON object. The only accepted type currently is
    /// AuditLog.
    #[serde(rename="protoPayload")]
    pub proto_payload: Option<HashMap<String, String>>,
    /// Optional. Resource name of the trace associated with the log entry, if any.
    /// If this field contains a relative resource name, you can assume the name is
    /// relative to `//tracing.googleapis.com`. Example:
    /// `projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824`
    pub trace: Option<String>,
    /// Optional. Information about an operation associated with the log entry, if
    /// applicable.
    pub operation: Option<LogEntryOperation>,
    /// The severity of the log entry. The default value is
    /// `LogSeverity.DEFAULT`.
    pub severity: Option<String>,
}

impl Part for LogEntry {}


/// `ConsumerInfo` provides information about the consumer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsumerInfo {
    /// The consumer identity number, can be Google cloud project number, folder
    /// number or organization number e.g. 1234567890. A value of 0 indicates no
    /// consumer number is found.
    #[serde(rename="consumerNumber")]
    pub consumer_number: Option<String>,
    /// no description provided
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The Google cloud project number, e.g. 1234567890. A value of 0 indicates
    /// no project number is found.
    /// 
    /// NOTE: This field is deprecated after Chemist support flexible consumer
    /// id. New code should not depend on this field anymore.
    #[serde(rename="projectNumber")]
    pub project_number: Option<String>,
}

impl Part for ConsumerInfo {}


/// Response message for the Check method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check services](struct.ServiceCheckCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckResponse {
    /// The same operation_id value used in the CheckRequest.
    /// Used for logging and diagnostics purposes.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    /// The actual config id used to process the request.
    #[serde(rename="serviceConfigId")]
    pub service_config_id: Option<String>,
    /// Feedback data returned from the server during processing a Check request.
    #[serde(rename="checkInfo")]
    pub check_info: Option<CheckInfo>,
    /// Indicate the decision of the check.
    /// 
    /// If no check errors are present, the service should process the operation.
    /// Otherwise the service should use the list of errors to determine the
    /// appropriate action.
    #[serde(rename="checkErrors")]
    pub check_errors: Option<Vec<CheckError>>,
    /// Quota information for the check request associated with this response.
    /// 
    #[serde(rename="quotaInfo")]
    pub quota_info: Option<QuotaInfo>,
}

impl ResponseResult for CheckResponse {}


/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// - Simple to use and understand for most users
/// - Flexible enough to meet unexpected needs
/// 
/// # Overview
/// 
/// The `Status` message contains three pieces of data: error code, error
/// message, and error details. The error code should be an enum value of
/// google.rpc.Code, but it may accept additional error codes if needed.  The
/// error message should be a developer-facing English message that helps
/// developers *understand* and *resolve* the error. If a localized user-facing
/// error message is needed, put the localized message in the error details or
/// localize it in the client. The optional error details may contain arbitrary
/// information about the error. There is a predefined set of error detail types
/// in the package `google.rpc` that can be used for common error conditions.
/// 
/// # Language mapping
/// 
/// The `Status` message is the logical representation of the error model, but it
/// is not necessarily the actual wire format. When the `Status` message is
/// exposed in different client libraries and different wire protocols, it can be
/// mapped differently. For example, it will likely be mapped to some exceptions
/// in Java, but more likely mapped to some error codes in C.
/// 
/// # Other uses
/// 
/// The error model and the `Status` message can be used in a variety of
/// environments, either with or without APIs, to provide a
/// consistent developer experience across different environments.
/// 
/// Example uses of this error model include:
/// 
/// - Partial errors. If a service needs to return partial errors to the client,
///     it may embed the `Status` in the normal response to indicate the partial
///     errors.
/// 
/// - Workflow errors. A typical workflow has multiple steps. Each step may
///     have a `Status` message for error reporting.
/// 
/// - Batch operations. If a client uses batch request and batch response, the
///     `Status` message should be used directly inside batch response, one for
///     each error sub-response.
/// 
/// - Asynchronous operations. If an API call embeds asynchronous operation
///     results in its response, the status of those operations should be
///     represented directly using the `Status` message.
/// 
/// - Logging. If some API errors are stored in logs, the message `Status` could
///     be used directly after any stripping needed for security/privacy reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
}

impl Part for Status {}


/// Request message for the AllocateQuota method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate quota services](struct.ServiceAllocateQuotaCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateQuotaRequest {
    /// Specifies which version of service configuration should be used to process
    /// the request. If unspecified or no matching version can be found, the latest
    /// one will be used.
    #[serde(rename="serviceConfigId")]
    pub service_config_id: Option<String>,
    /// Operation that describes the quota allocation.
    #[serde(rename="allocateOperation")]
    pub allocate_operation: Option<QuotaOperation>,
}

impl RequestValue for AllocateQuotaRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateInfo {
    /// A list of label keys that were unused by the server in processing the
    /// request. Thus, for similar requests repeated in a certain future time
    /// window, the caller can choose to ignore these labels in the requests
    /// to achieve better client-side cache hits and quota aggregation.
    #[serde(rename="unusedArguments")]
    pub unused_arguments: Option<Vec<String>>,
}

impl Part for AllocateInfo {}


/// Additional information about a potentially long-running operation with which
/// a log entry is associated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogEntryOperation {
    /// Optional. Set this to True if this is the last log entry in the operation.
    pub last: Option<bool>,
    /// Optional. An arbitrary operation identifier. Log entries with the
    /// same identifier are assumed to be part of the same operation.
    pub id: Option<String>,
    /// Optional. An arbitrary producer identifier. The combination of
    /// `id` and `producer` must be globally unique.  Examples for `producer`:
    /// `"MyDivision.MyBigCompany.com"`, `"github.com/MyProject/MyApplication"`.
    pub producer: Option<String>,
    /// Optional. Set this to True if this is the first log entry in the operation.
    pub first: Option<bool>,
}

impl Part for LogEntryOperation {}


/// Response message for the AllocateQuota method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [allocate quota services](struct.ServiceAllocateQuotaCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllocateQuotaResponse {
    /// Indicates the decision of the allocate.
    #[serde(rename="allocateErrors")]
    pub allocate_errors: Option<Vec<QuotaError>>,
    /// ID of the actual config used to process the request.
    #[serde(rename="serviceConfigId")]
    pub service_config_id: Option<String>,
    /// Quota metrics to indicate the result of allocation. Depending on the
    /// request, one or more of the following metrics will be included:
    /// 
    /// 1. Per quota group or per quota metric incremental usage will be specified
    /// using the following delta metric :
    ///   "serviceruntime.googleapis.com/api/consumer/quota_used_count"
    /// 
    /// 2. The quota limit reached condition will be specified using the following
    /// boolean metric :
    ///   "serviceruntime.googleapis.com/quota/exceeded"
    #[serde(rename="quotaMetrics")]
    pub quota_metrics: Option<Vec<MetricValueSet>>,
    /// WARNING: DO NOT use this field until this warning message is removed.
    #[serde(rename="allocateInfo")]
    pub allocate_info: Option<AllocateInfo>,
    /// The same operation_id value used in the AllocateQuotaRequest. Used for
    /// logging and diagnostics purposes.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl ResponseResult for AllocateQuotaResponse {}


/// Describing buckets with exponentially growing width.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExponentialBuckets {
    /// The i'th exponential bucket covers the interval
    ///   [scale * growth_factor^(i-1), scale * growth_factor^i)
    /// where i ranges from 1 to num_finite_buckets inclusive.
    /// Must be > 0.
    pub scale: Option<f64>,
    /// The i'th exponential bucket covers the interval
    ///   [scale * growth_factor^(i-1), scale * growth_factor^i)
    /// where i ranges from 1 to num_finite_buckets inclusive.
    /// Must be larger than 1.0.
    #[serde(rename="growthFactor")]
    pub growth_factor: Option<f64>,
    /// The number of finite buckets. With the underflow and overflow buckets,
    /// the total number of buckets is `num_finite_buckets` + 2.
    /// See comments on `bucket_options` for details.
    #[serde(rename="numFiniteBuckets")]
    pub num_finite_buckets: Option<i32>,
}

impl Part for ExponentialBuckets {}


/// Contains additional information about the check operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckInfo {
    /// A list of fields and label keys that are ignored by the server.
    /// The client doesn't need to send them for following requests to improve
    /// performance and allow better aggregation.
    #[serde(rename="unusedArguments")]
    pub unused_arguments: Option<Vec<String>>,
    /// Consumer info of this check.
    #[serde(rename="consumerInfo")]
    pub consumer_info: Option<ConsumerInfo>,
}

impl Part for CheckInfo {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: Option<i32>,
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    pub units: Option<String>,
    /// The 3-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
}

impl Part for Money {}


/// Contains additional info about the report operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportInfo {
    /// Quota usage info when processing the `Operation`.
    #[serde(rename="quotaInfo")]
    pub quota_info: Option<QuotaInfo>,
    /// The Operation.operation_id value from the request.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl Part for ReportInfo {}


/// Represents the processing error of one Operation in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportError {
    /// Details of the error when processing the Operation.
    pub status: Option<Status>,
    /// The Operation.operation_id value from the request.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl Part for ReportError {}


/// Represents the properties needed for quota operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaProperties {
    /// Quota mode for this operation.
    #[serde(rename="quotaMode")]
    pub quota_mode: Option<String>,
}

impl Part for QuotaProperties {}


/// Represents information regarding an operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// Fully qualified name of the operation. Reserved for future use.
    #[serde(rename="operationName")]
    pub operation_name: Option<String>,
    /// Represents information about this operation. Each MetricValueSet
    /// corresponds to a metric defined in the service configuration.
    /// The data type used in the MetricValueSet must agree with
    /// the data type specified in the metric definition.
    /// 
    /// Within a single operation, it is not allowed to have more than one
    /// MetricValue instances that have the same metric names and identical
    /// label value combinations. If a request has such duplicated MetricValue
    /// instances, the entire request is rejected with
    /// an invalid argument error.
    #[serde(rename="metricValueSets")]
    pub metric_value_sets: Option<Vec<MetricValueSet>>,
    /// DO NOT USE. This is an experimental field.
    pub importance: Option<String>,
    /// Labels describing the operation. Only the following labels are allowed:
    /// 
    /// - Labels describing monitored resources as defined in
    ///   the service configuration.
    /// - Default labels of metric values. When specified, labels defined in the
    ///   metric value override these default.
    /// - The following labels defined by Google Cloud Platform:
    ///     - `cloud.googleapis.com/location` describing the location where the
    ///        operation happened,
    ///     - `servicecontrol.googleapis.com/user_agent` describing the user agent
    ///        of the API request,
    ///     - `servicecontrol.googleapis.com/service_agent` describing the service
    ///        used to handle the API request (e.g. ESP),
    ///     - `servicecontrol.googleapis.com/platform` describing the platform
    ///        where the API is served, such as App Engine, Compute Engine, or
    ///        Kubernetes Engine.
    pub labels: Option<HashMap<String, String>>,
    /// Represents the properties needed for quota check. Applicable only if this
    /// operation is for a quota check request. If this is not specified, no quota
    /// check will be performed.
    #[serde(rename="quotaProperties")]
    pub quota_properties: Option<QuotaProperties>,
    /// DO NOT USE. This field is deprecated, use "resources" field instead.
    /// The resource name of the parent of a resource in the resource hierarchy.
    /// 
    /// This can be in one of the following formats:
    ///     - “projects/<project-id or project-number>”
    ///     - “folders/<folder-id>”
    ///     - “organizations/<organization-id>”
    #[serde(rename="resourceContainer")]
    pub resource_container: Option<String>,
    /// User defined labels for the resource that this operation is associated
    /// with. Only a combination of 1000 user labels per consumer project are
    /// allowed.
    #[serde(rename="userLabels")]
    pub user_labels: Option<HashMap<String, String>>,
    /// End time of the operation.
    /// Required when the operation is used in ServiceController.Report,
    /// but optional when the operation is used in ServiceController.Check.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// Represents information to be logged.
    #[serde(rename="logEntries")]
    pub log_entries: Option<Vec<LogEntry>>,
    /// Required. Start time of the operation.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
    /// Identity of the consumer who is using the service.
    /// This field should be filled in for the operations initiated by a
    /// consumer, but not for service-initiated operations that are
    /// not related to a specific consumer.
    /// 
    /// - This can be in one of the following formats:
    ///     - project:PROJECT_ID,
    ///     - project`_`number:PROJECT_NUMBER,
    ///     - projects/PROJECT_ID or PROJECT_NUMBER,
    ///     - folders/FOLDER_NUMBER,
    ///     - organizations/ORGANIZATION_NUMBER,
    ///     - api`_`key:API_KEY.
    #[serde(rename="consumerId")]
    pub consumer_id: Option<String>,
    /// The resources that are involved in the operation.
    /// The maximum supported number of entries in this field is 100.
    pub resources: Option<Vec<ResourceInfo>>,
    /// Identity of the operation. This must be unique within the scope of the
    /// service that generated the operation. If the service calls
    /// Check() and Report() on the same operation, the two calls should carry
    /// the same id.
    /// 
    /// UUID version 4 is recommended, though not required.
    /// In scenarios where an operation is computed from existing information
    /// and an idempotent id is desirable for deduplication purpose, UUID version 5
    /// is recommended. See RFC 4122 for details.
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
}

impl Part for Operation {}


/// Describing buckets with constant width.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinearBuckets {
    /// The i'th linear bucket covers the interval
    ///   [offset + (i-1) * width, offset + i * width)
    /// where i ranges from 1 to num_finite_buckets, inclusive.
    /// Must be strictly positive.
    pub width: Option<f64>,
    /// The number of finite buckets. With the underflow and overflow buckets,
    /// the total number of buckets is `num_finite_buckets` + 2.
    /// See comments on `bucket_options` for details.
    #[serde(rename="numFiniteBuckets")]
    pub num_finite_buckets: Option<i32>,
    /// The i'th linear bucket covers the interval
    ///   [offset + (i-1) * width, offset + i * width)
    /// where i ranges from 1 to num_finite_buckets, inclusive.
    pub offset: Option<f64>,
}

impl Part for LinearBuckets {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *service* resources.
/// It is not used directly, but through the `ServiceControl` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_servicecontrol1 as servicecontrol1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use servicecontrol1::ServiceControl;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ServiceControl::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `allocate_quota(...)`, `check(...)` and `report(...)`
/// // to build up your call.
/// let rb = hub.services();
/// # }
/// ```
pub struct ServiceMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceControl<C, A>,
}

impl<'a, C, A> MethodsBuilder for ServiceMethods<'a, C, A> {}

impl<'a, C, A> ServiceMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks whether an operation on a service should be allowed to proceed
    /// based on the configuration of the service and related policies. It must be
    /// called before the operation is executed.
    /// 
    /// If feasible, the client should cache the check results and reuse them for
    /// 60 seconds. In case of any server errors, the client should rely on the
    /// cached results for much longer time to avoid outage.
    /// WARNING: There is general 60s delay for the configuration and policy
    /// propagation, therefore callers MUST NOT depend on the `Check` method having
    /// the latest policy information.
    /// 
    /// NOTE: the CheckRequest has the size limit of 64KB.
    /// 
    /// This method requires the `servicemanagement.services.check` permission
    /// on the specified service. For more information, see
    /// [Cloud IAM](https://cloud.google.com/iam).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `serviceName` - The service name as specified in its service configuration. For example,
    ///                   `"pubsub.googleapis.com"`.
    ///                   See
    ///                   [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    ///                   for the definition of a service name.
    pub fn check(&self, request: CheckRequest, service_name: &str) -> ServiceCheckCall<'a, C, A> {
        ServiceCheckCall {
            hub: self.hub,
            _request: request,
            _service_name: service_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reports operation results to Google Service Control, such as logs and
    /// metrics. It should be called after an operation is completed.
    /// 
    /// If feasible, the client should aggregate reporting data for up to 5
    /// seconds to reduce API traffic. Limiting aggregation to 5 seconds is to
    /// reduce data loss during client crashes. Clients should carefully choose
    /// the aggregation time window to avoid data loss risk more than 0.01%
    /// for business and compliance reasons.
    /// 
    /// NOTE: the ReportRequest has the size limit of 1MB.
    /// 
    /// This method requires the `servicemanagement.services.report` permission
    /// on the specified service. For more information, see
    /// [Google Cloud IAM](https://cloud.google.com/iam).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `serviceName` - The service name as specified in its service configuration. For example,
    ///                   `"pubsub.googleapis.com"`.
    ///                   See
    ///                   [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    ///                   for the definition of a service name.
    pub fn report(&self, request: ReportRequest, service_name: &str) -> ServiceReportCall<'a, C, A> {
        ServiceReportCall {
            hub: self.hub,
            _request: request,
            _service_name: service_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Attempts to allocate quota for the specified consumer. It should be called
    /// before the operation is executed.
    /// 
    /// This method requires the `servicemanagement.services.quota`
    /// permission on the specified service. For more information, see
    /// [Cloud IAM](https://cloud.google.com/iam).
    /// 
    /// **NOTE:** The client **must** fail-open on server errors `INTERNAL`,
    /// `UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system
    /// reliability, the server may inject these errors to prohibit any hard
    /// dependency on the quota functionality.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `serviceName` - Name of the service as specified in the service configuration. For example,
    ///                   `"pubsub.googleapis.com"`.
    ///                   See google.api.Service for the definition of a service name.
    pub fn allocate_quota(&self, request: AllocateQuotaRequest, service_name: &str) -> ServiceAllocateQuotaCall<'a, C, A> {
        ServiceAllocateQuotaCall {
            hub: self.hub,
            _request: request,
            _service_name: service_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Checks whether an operation on a service should be allowed to proceed
/// based on the configuration of the service and related policies. It must be
/// called before the operation is executed.
/// 
/// If feasible, the client should cache the check results and reuse them for
/// 60 seconds. In case of any server errors, the client should rely on the
/// cached results for much longer time to avoid outage.
/// WARNING: There is general 60s delay for the configuration and policy
/// propagation, therefore callers MUST NOT depend on the `Check` method having
/// the latest policy information.
/// 
/// NOTE: the CheckRequest has the size limit of 64KB.
/// 
/// This method requires the `servicemanagement.services.check` permission
/// on the specified service. For more information, see
/// [Cloud IAM](https://cloud.google.com/iam).
///
/// A builder for the *check* method supported by a *service* resource.
/// It is not used directly, but through a `ServiceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_servicecontrol1 as servicecontrol1;
/// use servicecontrol1::CheckRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use servicecontrol1::ServiceControl;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceControl::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CheckRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().check(req, "serviceName")
///              .doit();
/// # }
/// ```
pub struct ServiceCheckCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceControl<C, A>,
    _request: CheckRequest,
    _service_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceCheckCall<'a, C, A> {}

impl<'a, C, A> ServiceCheckCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CheckResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "servicecontrol.services.check",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("serviceName", self._service_name.to_string()));
        for &field in ["alt", "serviceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/services/{serviceName}:check";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{serviceName}", "serviceName")].iter() {
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
            for param_name in ["serviceName"].iter() {
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
    pub fn request(mut self, new_value: CheckRequest) -> ServiceCheckCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    /// 
    /// See
    /// [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    /// for the definition of a service name.
    ///
    /// Sets the *service name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn service_name(mut self, new_value: &str) -> ServiceCheckCall<'a, C, A> {
        self._service_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceCheckCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceCheckCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceCheckCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Reports operation results to Google Service Control, such as logs and
/// metrics. It should be called after an operation is completed.
/// 
/// If feasible, the client should aggregate reporting data for up to 5
/// seconds to reduce API traffic. Limiting aggregation to 5 seconds is to
/// reduce data loss during client crashes. Clients should carefully choose
/// the aggregation time window to avoid data loss risk more than 0.01%
/// for business and compliance reasons.
/// 
/// NOTE: the ReportRequest has the size limit of 1MB.
/// 
/// This method requires the `servicemanagement.services.report` permission
/// on the specified service. For more information, see
/// [Google Cloud IAM](https://cloud.google.com/iam).
///
/// A builder for the *report* method supported by a *service* resource.
/// It is not used directly, but through a `ServiceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_servicecontrol1 as servicecontrol1;
/// use servicecontrol1::ReportRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use servicecontrol1::ServiceControl;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceControl::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ReportRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().report(req, "serviceName")
///              .doit();
/// # }
/// ```
pub struct ServiceReportCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceControl<C, A>,
    _request: ReportRequest,
    _service_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceReportCall<'a, C, A> {}

impl<'a, C, A> ServiceReportCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ReportResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "servicecontrol.services.report",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("serviceName", self._service_name.to_string()));
        for &field in ["alt", "serviceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/services/{serviceName}:report";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{serviceName}", "serviceName")].iter() {
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
            for param_name in ["serviceName"].iter() {
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
    pub fn request(mut self, new_value: ReportRequest) -> ServiceReportCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    /// 
    /// See
    /// [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    /// for the definition of a service name.
    ///
    /// Sets the *service name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn service_name(mut self, new_value: &str) -> ServiceReportCall<'a, C, A> {
        self._service_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceReportCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceReportCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceReportCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Attempts to allocate quota for the specified consumer. It should be called
/// before the operation is executed.
/// 
/// This method requires the `servicemanagement.services.quota`
/// permission on the specified service. For more information, see
/// [Cloud IAM](https://cloud.google.com/iam).
/// 
/// **NOTE:** The client **must** fail-open on server errors `INTERNAL`,
/// `UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system
/// reliability, the server may inject these errors to prohibit any hard
/// dependency on the quota functionality.
///
/// A builder for the *allocateQuota* method supported by a *service* resource.
/// It is not used directly, but through a `ServiceMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_servicecontrol1 as servicecontrol1;
/// use servicecontrol1::AllocateQuotaRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use servicecontrol1::ServiceControl;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceControl::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AllocateQuotaRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().allocate_quota(req, "serviceName")
///              .doit();
/// # }
/// ```
pub struct ServiceAllocateQuotaCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceControl<C, A>,
    _request: AllocateQuotaRequest,
    _service_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceAllocateQuotaCall<'a, C, A> {}

impl<'a, C, A> ServiceAllocateQuotaCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AllocateQuotaResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "servicecontrol.services.allocateQuota",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("serviceName", self._service_name.to_string()));
        for &field in ["alt", "serviceName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/services/{serviceName}:allocateQuota";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{serviceName}", "serviceName")].iter() {
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
            for param_name in ["serviceName"].iter() {
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
    pub fn request(mut self, new_value: AllocateQuotaRequest) -> ServiceAllocateQuotaCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the service as specified in the service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    /// 
    /// See google.api.Service for the definition of a service name.
    ///
    /// Sets the *service name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn service_name(mut self, new_value: &str) -> ServiceAllocateQuotaCall<'a, C, A> {
        self._service_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceAllocateQuotaCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceAllocateQuotaCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceAllocateQuotaCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


