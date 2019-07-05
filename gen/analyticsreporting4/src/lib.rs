// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *AnalyticsReporting* crate version *1.0.9+20190625*, where *20190625* is the exact revision of the *analyticsreporting:v4* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.9*.
//! 
//! Everything else about the *AnalyticsReporting* *v4* API can be found at the
//! [official documentation site](https://developers.google.com/analytics/devguides/reporting/core/v4/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/analyticsreporting4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.AnalyticsReporting.html) ... 
//! 
//! * [reports](struct.Report.html)
//!  * [*batch get*](struct.ReportBatchGetCall.html)
//! * user activity
//!  * [*search*](struct.UserActivitySearchCall.html)
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
//! * **[Hub](struct.AnalyticsReporting.html)**
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
//! let r = hub.user_activity().search(...).doit()
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
//! google-analyticsreporting4 = "*"
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
//! extern crate google_analyticsreporting4 as analyticsreporting4;
//! use analyticsreporting4::SearchUserActivityRequest;
//! use analyticsreporting4::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use analyticsreporting4::AnalyticsReporting;
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
//! let mut hub = AnalyticsReporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = SearchUserActivityRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.user_activity().search(req)
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

pub use cmn::*;


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your Google Analytics data
    Analytic,

    /// View your Google Analytics data
    AnalyticReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Analytic => "https://www.googleapis.com/auth/analytics",
            Scope::AnalyticReadonly => "https://www.googleapis.com/auth/analytics.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AnalyticReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all AnalyticsReporting related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_analyticsreporting4 as analyticsreporting4;
/// use analyticsreporting4::SearchUserActivityRequest;
/// use analyticsreporting4::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use analyticsreporting4::AnalyticsReporting;
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
/// let mut hub = AnalyticsReporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SearchUserActivityRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_activity().search(req)
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
pub struct AnalyticsReporting<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for AnalyticsReporting<C, A> {}

impl<'a, C, A> AnalyticsReporting<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> AnalyticsReporting<C, A> {
        AnalyticsReporting {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.9".to_string(),
            _base_url: "https://analyticsreporting.googleapis.com/".to_string(),
            _root_url: "https://analyticsreporting.googleapis.com/".to_string(),
        }
    }

    pub fn reports(&'a self) -> ReportMethods<'a, C, A> {
        ReportMethods { hub: &self }
    }
    pub fn user_activity(&'a self) -> UserActivityMethods<'a, C, A> {
        UserActivityMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.9`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://analyticsreporting.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://analyticsreporting.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Defines a cohort. A cohort is a group of users who share a common
/// characteristic. For example, all users with the same acquisition date
/// belong to the same cohort.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cohort {
    /// This is used for `FIRST_VISIT_DATE` cohort, the cohort selects users
    /// whose first visit date is between start date and end date defined in the
    /// DateRange. The date ranges should be aligned for cohort requests. If the
    /// request contains `ga:cohortNthDay` it should be exactly one day long,
    /// if `ga:cohortNthWeek` it should be aligned to the week boundary (starting
    /// at Sunday and ending Saturday), and for `ga:cohortNthMonth` the date range
    /// should be aligned to the month (starting at the first and ending on the
    /// last day of the month).
    /// For LTV requests there are no such restrictions.
    /// You do not need to supply a date range for the
    /// `reportsRequest.dateRanges` field.
    #[serde(rename="dateRange")]
    pub date_range: Option<DateRange>,
    /// Type of the cohort. The only supported type as of now is
    /// `FIRST_VISIT_DATE`. If this field is unspecified the cohort is treated
    /// as `FIRST_VISIT_DATE` type cohort.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// A unique name for the cohort. If not defined name will be auto-generated
    /// with values cohort_[1234...].
    pub name: Option<String>,
}

impl Part for Cohort {}


/// The data part of the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportData {
    /// There's one ReportRow for every unique combination of dimensions.
    pub rows: Option<Vec<ReportRow>>,
    /// Minimum and maximum values seen over all matching rows. These are both
    /// empty when `hideValueRanges` in the request is false, or when
    /// rowCount is zero.
    pub maximums: Option<Vec<DateRangeValues>>,
    /// Minimum and maximum values seen over all matching rows. These are both
    /// empty when `hideValueRanges` in the request is false, or when
    /// rowCount is zero.
    pub minimums: Option<Vec<DateRangeValues>>,
    /// Indicates if response to this request is golden or not. Data is
    /// golden when the exact same request will not produce any new results if
    /// asked at a later point in time.
    #[serde(rename="isDataGolden")]
    pub is_data_golden: Option<bool>,
    /// If the results are
    /// [sampled](https://support.google.com/analytics/answer/2637192),
    /// this returns the total number of
    /// samples present, one entry per date range. If the results are not sampled
    /// this field will not be defined. See
    /// [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)
    /// for details.
    #[serde(rename="samplingSpaceSizes")]
    pub sampling_space_sizes: Option<Vec<String>>,
    /// For each requested date range, for the set of all rows that match
    /// the query, every requested value format gets a total. The total
    /// for a value format is computed by first totaling the metrics
    /// mentioned in the value format and then evaluating the value
    /// format as a scalar expression.  E.g., The "totals" for
    /// `3 / (ga:sessions + 2)` we compute
    /// `3 / ((sum of all relevant ga:sessions) + 2)`.
    /// Totals are computed before pagination.
    pub totals: Option<Vec<DateRangeValues>>,
    /// Total number of matching rows for this query.
    #[serde(rename="rowCount")]
    pub row_count: Option<i32>,
    /// The last time the data in the report was refreshed. All the hits received
    /// before this timestamp are included in the calculation of the report.
    #[serde(rename="dataLastRefreshed")]
    pub data_last_refreshed: Option<String>,
    /// If the results are
    /// [sampled](https://support.google.com/analytics/answer/2637192),
    /// this returns the total number of samples read, one entry per date range.
    /// If the results are not sampled this field will not be defined. See
    /// [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)
    /// for details.
    #[serde(rename="samplesReadCounts")]
    pub samples_read_counts: Option<Vec<String>>,
}

impl Part for ReportData {}


/// The batch request containing multiple report request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get reports](struct.ReportBatchGetCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetReportsRequest {
    /// Requests, each request will have a separate response.
    /// There can be a maximum of 5 requests. All requests should have the same
    /// `dateRanges`, `viewId`, `segments`, `samplingLevel`, and `cohortGroup`.
    #[serde(rename="reportRequests")]
    pub report_requests: Option<Vec<ReportRequest>>,
    /// Enables
    /// [resource based quotas](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4),
    /// (defaults to `False`). If this field is set to `True` the
    /// per view (profile) quotas are governed by the computational
    /// cost of the request. Note that using cost based quotas will
    /// higher enable sampling rates. (10 Million for `SMALL`,
    /// 100M for `LARGE`. See the
    /// [limits and quotas documentation](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4) for details.
    #[serde(rename="useResourceQuotas")]
    pub use_resource_quotas: Option<bool>,
}

impl RequestValue for GetReportsRequest {}


/// A contiguous set of days: startDate, startDate + 1 day, ..., endDate.
/// The start and end dates are specified in
/// [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) date format `YYYY-MM-DD`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// The start date for the query in the format `YYYY-MM-DD`.
    #[serde(rename="startDate")]
    pub start_date: Option<String>,
    /// The end date for the query in the format `YYYY-MM-DD`.
    #[serde(rename="endDate")]
    pub end_date: Option<String>,
}

impl Part for DateRange {}


/// The headers for the metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricHeader {
    /// Headers for the metrics in the response.
    #[serde(rename="metricHeaderEntries")]
    pub metric_header_entries: Option<Vec<MetricHeaderEntry>>,
    /// Headers for the pivots in the response.
    #[serde(rename="pivotHeaders")]
    pub pivot_headers: Option<Vec<PivotHeader>>,
}

impl Part for MetricHeader {}


/// The main response class which holds the reports from the Reporting API
/// `batchGet` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get reports](struct.ReportBatchGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetReportsResponse {
    /// The amount of resource quota tokens deducted to execute the query. Includes
    /// all responses.
    #[serde(rename="queryCost")]
    pub query_cost: Option<i32>,
    /// Responses corresponding to each of the request.
    pub reports: Option<Vec<Report>>,
    /// The amount of resource quota remaining for the property.
    #[serde(rename="resourceQuotasRemaining")]
    pub resource_quotas_remaining: Option<ResourceQuotasRemaining>,
}

impl ResponseResult for GetReportsResponse {}


/// SegmentDefinition defines the segment to be a set of SegmentFilters which
/// are combined together with a logical `AND` operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentDefinition {
    /// A segment is defined by a set of segment filters which are combined
    /// together with a logical `AND` operation.
    #[serde(rename="segmentFilters")]
    pub segment_filters: Option<Vec<SegmentFilter>>,
}

impl Part for SegmentDefinition {}


/// The main request class which specifies the Reporting API request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    /// The metrics requested.
    /// Requests must specify at least one metric. Requests can have a
    /// total of 10 metrics.
    pub metrics: Option<Vec<Metric>>,
    /// Dimension or metric filters that restrict the data returned for your
    /// request. To use the `filtersExpression`, supply a dimension or metric on
    /// which to filter, followed by the filter expression. For example, the
    /// following expression selects `ga:browser` dimension which starts with
    /// Firefox; `ga:browser=~^Firefox`. For more information on dimensions
    /// and metric filters, see
    /// [Filters reference](https://developers.google.com/analytics/devguides/reporting/core/v3/reference#filters).
    #[serde(rename="filtersExpression")]
    pub filters_expression: Option<String>,
    /// The Analytics
    /// [view ID](https://support.google.com/analytics/answer/1009618)
    /// from which to retrieve data. Every [ReportRequest](#ReportRequest)
    /// within a `batchGet` method must contain the same `viewId`.
    #[serde(rename="viewId")]
    pub view_id: Option<String>,
    /// Cohort group associated with this request. If there is a cohort group
    /// in the request the `ga:cohort` dimension must be present.
    /// Every [ReportRequest](#ReportRequest) within a `batchGet` method must
    /// contain the same `cohortGroup` definition.
    #[serde(rename="cohortGroup")]
    pub cohort_group: Option<CohortGroup>,
    /// Date ranges in the request. The request can have a maximum of 2 date
    /// ranges. The response will contain a set of metric values for each
    /// combination of the dimensions for each date range in the request. So, if
    /// there are two date ranges, there will be two set of metric values, one for
    /// the original date range and one for the second date range.
    /// The `reportRequest.dateRanges` field should not be specified for cohorts
    /// or Lifetime value requests.
    /// If a date range is not provided, the default date range is (startDate:
    /// current date - 7 days, endDate: current date - 1 day). Every
    /// [ReportRequest](#ReportRequest) within a `batchGet` method must
    /// contain the same `dateRanges` definition.
    #[serde(rename="dateRanges")]
    pub date_ranges: Option<Vec<DateRange>>,
    /// The desired report
    /// [sample](https://support.google.com/analytics/answer/2637192) size.
    /// If the the `samplingLevel` field is unspecified the `DEFAULT` sampling
    /// level is used. Every [ReportRequest](#ReportRequest) within a
    /// `batchGet` method must contain the same `samplingLevel` definition. See
    /// [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)
    ///  for details.
    #[serde(rename="samplingLevel")]
    pub sampling_level: Option<String>,
    /// The metric filter clauses. They are logically combined with the `AND`
    /// operator.  Metric filters look at only the first date range and not the
    /// comparing date range. Note that filtering on metrics occurs after the
    /// metrics are aggregated.
    #[serde(rename="metricFilterClauses")]
    pub metric_filter_clauses: Option<Vec<MetricFilterClause>>,
    /// The dimensions requested.
    /// Requests can have a total of 7 dimensions.
    pub dimensions: Option<Vec<Dimension>>,
    /// The pivot definitions. Requests can have a maximum of 2 pivots.
    pub pivots: Option<Vec<Pivot>>,
    /// If set to true, hides the total of all metrics for all the matching rows,
    /// for every date range. The default false and will return the totals.
    #[serde(rename="hideTotals")]
    pub hide_totals: Option<bool>,
    /// Page size is for paging and specifies the maximum number of returned rows.
    /// Page size should be >= 0. A query returns the default of 1,000 rows.
    /// The Analytics Core Reporting API returns a maximum of 100,000 rows per
    /// request, no matter how many you ask for. It can also return fewer rows
    /// than requested, if there aren't as many dimension segments as you expect.
    /// For instance, there are fewer than 300 possible values for `ga:country`,
    /// so when segmenting only by country, you can't get more than 300 rows,
    /// even if you set `pageSize` to a higher value.
    #[serde(rename="pageSize")]
    pub page_size: Option<i32>,
    /// The dimension filter clauses for filtering Dimension Values. They are
    /// logically combined with the `AND` operator. Note that filtering occurs
    /// before any dimensions are aggregated, so that the returned metrics
    /// represent the total for only the relevant dimensions.
    #[serde(rename="dimensionFilterClauses")]
    pub dimension_filter_clauses: Option<Vec<DimensionFilterClause>>,
    /// If set to false, the response does not include rows if all the retrieved
    /// metrics are equal to zero. The default is false which will exclude these
    /// rows.
    #[serde(rename="includeEmptyRows")]
    pub include_empty_rows: Option<bool>,
    /// Segment the data returned for the request. A segment definition helps look
    /// at a subset of the segment request. A request can contain up to four
    /// segments. Every [ReportRequest](#ReportRequest) within a
    /// `batchGet` method must contain the same `segments` definition. Requests
    /// with segments must have the `ga:segment` dimension.
    pub segments: Option<Vec<Segment>>,
    /// Sort order on output rows. To compare two rows, the elements of the
    /// following are applied in order until a difference is found.  All date
    /// ranges in the output get the same row order.
    #[serde(rename="orderBys")]
    pub order_bys: Option<Vec<OrderBy>>,
    /// A continuation token to get the next page of the results. Adding this to
    /// the request will return the rows after the pageToken. The pageToken should
    /// be the value returned in the nextPageToken parameter in the response to
    /// the GetReports request.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
    /// If set to true, hides the minimum and maximum across all matching rows.
    /// The default is false and the value ranges are returned.
    #[serde(rename="hideValueRanges")]
    pub hide_value_ranges: Option<bool>,
}

impl Part for ReportRequest {}


/// The response from `userActivity:get` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search user activity](struct.UserActivitySearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchUserActivityResponse {
    /// This token should be passed to
    /// [SearchUserActivityRequest](#SearchUserActivityRequest) to retrieve the
    /// next page.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// This field represents the
    /// [sampling rate](https://support.google.com/analytics/answer/2637192) for
    /// the given request and is a number between 0.0 to 1.0. See
    /// [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling)
    /// for details.
    #[serde(rename="sampleRate")]
    pub sample_rate: Option<f64>,
    /// Total rows returned by this query (across different pages).
    #[serde(rename="totalRows")]
    pub total_rows: Option<i32>,
    /// Each record represents a session (device details, duration, etc).
    pub sessions: Option<Vec<UserActivitySession>>,
}

impl ResponseResult for SearchUserActivityResponse {}


/// Details of the products in an e-commerce transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductData {
    /// Unique code that represents the product.
    #[serde(rename="productSku")]
    pub product_sku: Option<String>,
    /// The total revenue from purchased product items.
    #[serde(rename="itemRevenue")]
    pub item_revenue: Option<f64>,
    /// Total number of this product units in the transaction.
    #[serde(rename="productQuantity")]
    pub product_quantity: Option<String>,
    /// The product name, supplied by the e-commerce tracking application, for
    /// the purchased items.
    #[serde(rename="productName")]
    pub product_name: Option<String>,
}

impl Part for ProductData {}


/// This represents a user session performed on a specific device at a certain
/// time over a period of time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserActivitySession {
    /// Represents a detailed view into each of the activity in this session.
    pub activities: Option<Vec<Activity>>,
    /// Unique ID of the session.
    #[serde(rename="sessionId")]
    pub session_id: Option<String>,
    /// Date of this session in ISO-8601 format.
    #[serde(rename="sessionDate")]
    pub session_date: Option<String>,
    /// The data source of a hit. By default, hits sent from analytics.js are
    /// reported as "web" and hits sent from the mobile SDKs are reported as "app".
    /// These values can be overridden in the Measurement Protocol.
    #[serde(rename="dataSource")]
    pub data_source: Option<String>,
    /// Platform on which the activity happened: "android", "ios" etc.
    pub platform: Option<String>,
    /// The type of device used: "mobile", "tablet" etc.
    #[serde(rename="deviceCategory")]
    pub device_category: Option<String>,
}

impl Part for UserActivitySession {}


/// Represents a group of metric filters.
/// Set the operator value to specify how the filters are logically combined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricFilterClause {
    /// The operator for combining multiple metric filters. If unspecified, it is
    /// treated as an `OR`.
    pub operator: Option<String>,
    /// The repeated set of filters. They are logically combined based on the
    /// operator specified.
    pub filters: Option<Vec<MetricFilter>>,
}

impl Part for MetricFilterClause {}


/// SegmentFilter defines the segment to be either a simple or a sequence
/// segment. A simple segment condition contains dimension and metric conditions
/// to select the sessions or users. A sequence segment condition can be used to
/// select users or sessions based on sequential conditions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentFilter {
    /// If true, match the complement of simple or sequence segment.
    /// For example, to match all visits not from "New York", we can define the
    /// segment as follows:
    /// 
    ///       "sessionSegment": {
    ///         "segmentFilters": [{
    ///           "simpleSegment" :{
    ///             "orFiltersForSegment": [{
    ///               "segmentFilterClauses":[{
    ///                 "dimensionFilter": {
    ///                   "dimensionName": "ga:city",
    ///                   "expressions": ["New York"]
    ///                 }
    ///               }]
    ///             }]
    ///           },
    ///           "not": "True"
    ///         }]
    ///       },
    pub not: Option<bool>,
    /// A Simple segment conditions consist of one or more dimension/metric
    /// conditions that can be combined
    #[serde(rename="simpleSegment")]
    pub simple_segment: Option<SimpleSegment>,
    /// Sequence conditions consist of one or more steps, where each step is
    /// defined by one or more dimension/metric conditions. Multiple steps can
    /// be combined with special sequence operators.
    #[serde(rename="sequenceSegment")]
    pub sequence_segment: Option<SequenceSegment>,
}

impl Part for SegmentFilter {}


/// Custom dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDimension {
    /// Slot number of custom dimension.
    pub index: Option<i32>,
    /// Value of the custom dimension. Default value (i.e. empty string) indicates
    /// clearing sesion/visitor scope custom dimension value.
    pub value: Option<String>,
}

impl Part for CustomDimension {}


/// Sequence conditions consist of one or more steps, where each step is defined
/// by one or more dimension/metric conditions. Multiple steps can be combined
/// with special sequence operators.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SequenceSegment {
    /// If set, first step condition must match the first hit of the visitor (in
    /// the date range).
    #[serde(rename="firstStepShouldMatchFirstHit")]
    pub first_step_should_match_first_hit: Option<bool>,
    /// The list of steps in the sequence.
    #[serde(rename="segmentSequenceSteps")]
    pub segment_sequence_steps: Option<Vec<SegmentSequenceStep>>,
}

impl Part for SequenceSegment {}


/// The metric values in the pivot region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotValueRegion {
    /// The values of the metrics in each of the pivot regions.
    pub values: Option<Vec<String>>,
}

impl Part for PivotValueRegion {}


/// The request to fetch User Report from Reporting API `userActivity:get` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search user activity](struct.UserActivitySearchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchUserActivityRequest {
    /// A continuation token to get the next page of the results. Adding this to
    /// the request will return the rows after the pageToken. The pageToken should
    /// be the value returned in the nextPageToken parameter in the response to
    /// the [SearchUserActivityRequest](#SearchUserActivityRequest) request.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
    /// Required. Unique user Id to query for. Every
    /// [SearchUserActivityRequest](#SearchUserActivityRequest) must contain this
    /// field.
    pub user: Option<User>,
    /// Page size is for paging and specifies the maximum number of returned rows.
    /// Page size should be > 0. If the value is 0 or if the field isn't specified,
    /// the request returns the default of 1000 rows per page.
    #[serde(rename="pageSize")]
    pub page_size: Option<i32>,
    /// Date range for which to retrieve the user activity. If a date range is not
    /// provided, the default date range is (startDate: current date - 7 days,
    /// endDate: current date - 1 day).
    #[serde(rename="dateRange")]
    pub date_range: Option<DateRange>,
    /// Set of all activity types being requested. Only acvities matching these
    /// types will be returned in the response. If empty, all activies will be
    /// returned.
    #[serde(rename="activityTypes")]
    pub activity_types: Option<Vec<String>>,
    /// Required. The Analytics
    /// [view ID](https://support.google.com/analytics/answer/1009618)
    /// from which to retrieve data. Every
    /// [SearchUserActivityRequest](#SearchUserActivityRequest) must contain the
    /// `viewId`.
    #[serde(rename="viewId")]
    pub view_id: Option<String>,
}

impl RequestValue for SearchUserActivityRequest {}


/// Column headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnHeader {
    /// The dimension names in the response.
    pub dimensions: Option<Vec<String>>,
    /// Metric headers for the metrics in the response.
    #[serde(rename="metricHeader")]
    pub metric_header: Option<MetricHeader>,
}

impl Part for ColumnHeader {}


/// Represents details collected when the visitor performs a transaction on the
/// page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionData {
    /// Total tax for the transaction.
    #[serde(rename="transactionTax")]
    pub transaction_tax: Option<f64>,
    /// The transaction ID, supplied by the e-commerce tracking method, for the
    /// purchase in the shopping cart.
    #[serde(rename="transactionId")]
    pub transaction_id: Option<String>,
    /// The total sale revenue (excluding shipping and tax) of the transaction.
    #[serde(rename="transactionRevenue")]
    pub transaction_revenue: Option<f64>,
    /// Total cost of shipping.
    #[serde(rename="transactionShipping")]
    pub transaction_shipping: Option<f64>,
}

impl Part for TransactionData {}


/// Dynamic segment definition for defining the segment within the request.
/// A segment can select users, sessions or both.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicSegment {
    /// Session Segment to select sessions to include in the segment.
    #[serde(rename="sessionSegment")]
    pub session_segment: Option<SegmentDefinition>,
    /// The name of the dynamic segment.
    pub name: Option<String>,
    /// User Segment to select users to include in the segment.
    #[serde(rename="userSegment")]
    pub user_segment: Option<SegmentDefinition>,
}

impl Part for DynamicSegment {}


/// Header for the metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricHeaderEntry {
    /// The type of the metric, for example `INTEGER`.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The name of the header.
    pub name: Option<String>,
}

impl Part for MetricHeaderEntry {}


/// Contains information to identify a particular user uniquely.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Type of the user in the request. The field `userId` is associated with this
    /// type.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Unique Id of the user for which the data is being requested.
    #[serde(rename="userId")]
    pub user_id: Option<String>,
}

impl Part for User {}


/// The data response corresponding to the request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get reports](struct.ReportBatchGetCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// Page token to retrieve the next page of results in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Response data.
    pub data: Option<ReportData>,
    /// The column headers.
    #[serde(rename="columnHeader")]
    pub column_header: Option<ColumnHeader>,
}

impl Resource for Report {}


/// A row in the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportRow {
    /// List of metrics for each requested DateRange.
    pub metrics: Option<Vec<DateRangeValues>>,
    /// List of requested dimensions.
    pub dimensions: Option<Vec<String>>,
}

impl Part for ReportRow {}


/// The segment definition, if the report needs to be segmented.
/// A Segment is a subset of the Analytics data. For example, of the entire
/// set of users, one Segment might be users from a particular country or city.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    /// A dynamic segment definition in the request.
    #[serde(rename="dynamicSegment")]
    pub dynamic_segment: Option<DynamicSegment>,
    /// The segment ID of a built-in or custom segment, for example `gaid::-3`.
    #[serde(rename="segmentId")]
    pub segment_id: Option<String>,
}

impl Part for Segment {}


/// The resource quota tokens remaining for the property after the request is
/// completed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceQuotasRemaining {
    /// Daily resource quota remaining remaining.
    #[serde(rename="dailyQuotaTokensRemaining")]
    pub daily_quota_tokens_remaining: Option<i32>,
    /// Hourly resource quota tokens remaining.
    #[serde(rename="hourlyQuotaTokensRemaining")]
    pub hourly_quota_tokens_remaining: Option<i32>,
}

impl Part for ResourceQuotasRemaining {}


/// The headers for each of the pivot sections defined in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotHeader {
    /// The total number of groups for this pivot.
    #[serde(rename="totalPivotGroupsCount")]
    pub total_pivot_groups_count: Option<i32>,
    /// A single pivot section header.
    #[serde(rename="pivotHeaderEntries")]
    pub pivot_header_entries: Option<Vec<PivotHeaderEntry>>,
}

impl Part for PivotHeader {}


/// E-commerce details associated with the user activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EcommerceData {
    /// The type of this e-commerce activity.
    #[serde(rename="ecommerceType")]
    pub ecommerce_type: Option<String>,
    /// Transaction details of this e-commerce action.
    pub transaction: Option<TransactionData>,
    /// Action associated with this e-commerce action.
    #[serde(rename="actionType")]
    pub action_type: Option<String>,
    /// Details of the products in this transaction.
    pub products: Option<Vec<ProductData>>,
}

impl Part for EcommerceData {}


/// A Simple segment conditions consist of one or more dimension/metric
/// conditions that can be combined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SimpleSegment {
    /// A list of segment filters groups which are combined with logical `AND`
    /// operator.
    #[serde(rename="orFiltersForSegment")]
    pub or_filters_for_segment: Option<Vec<OrFiltersForSegment>>,
}

impl Part for SimpleSegment {}


/// The headers for the each of the metric column corresponding to the metrics
/// requested in the pivots section of the response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotHeaderEntry {
    /// The values for the dimensions in the pivot.
    #[serde(rename="dimensionValues")]
    pub dimension_values: Option<Vec<String>>,
    /// The name of the dimensions in the pivot response.
    #[serde(rename="dimensionNames")]
    pub dimension_names: Option<Vec<String>>,
    /// The metric header for the metric in the pivot.
    pub metric: Option<MetricHeaderEntry>,
}

impl Part for PivotHeaderEntry {}


/// Represents details collected when the visitor views a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageviewData {
    /// The title of the page that the visitor viewed.
    #[serde(rename="pageTitle")]
    pub page_title: Option<String>,
    /// The URL of the page that the visitor viewed.
    #[serde(rename="pagePath")]
    pub page_path: Option<String>,
}

impl Part for PageviewData {}


/// Used to return a list of metrics for a single DateRange / dimension
/// combination
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRangeValues {
    /// Each value corresponds to each Metric in the request.
    pub values: Option<Vec<String>>,
    /// The values of each pivot region.
    #[serde(rename="pivotValueRegions")]
    pub pivot_value_regions: Option<Vec<PivotValueRegion>>,
}

impl Part for DateRangeValues {}


/// Defines a cohort group.
/// For example:
/// 
///     "cohortGroup": {
///       "cohorts": [{
///         "name": "cohort 1",
///         "type": "FIRST_VISIT_DATE",
///         "dateRange": { "startDate": "2015-08-01", "endDate": "2015-08-01" }
///       },{
///         "name": "cohort 2"
///          "type": "FIRST_VISIT_DATE"
///          "dateRange": { "startDate": "2015-07-01", "endDate": "2015-07-01" }
///       }]
///     }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CohortGroup {
    /// The definition for the cohort.
    pub cohorts: Option<Vec<Cohort>>,
    /// Enable Life Time Value (LTV).  LTV measures lifetime value for users
    /// acquired through different channels.
    /// Please see:
    /// [Cohort Analysis](https://support.google.com/analytics/answer/6074676) and
    /// [Lifetime Value](https://support.google.com/analytics/answer/6182550)
    /// If the value of lifetimeValue is false:
    /// 
    /// - The metric values are similar to the values in the web interface cohort
    ///   report.
    /// - The cohort definition date ranges must be aligned to the calendar week
    ///   and month. i.e. while requesting `ga:cohortNthWeek` the `startDate` in
    ///   the cohort definition should be a Sunday and the `endDate` should be the
    ///   following Saturday, and for `ga:cohortNthMonth`, the `startDate`
    ///   should be the 1st of the month and `endDate` should be the last day
    ///   of the month.
    /// 
    /// When the lifetimeValue is true:
    /// 
    /// - The metric values will correspond to the values in the web interface
    ///   LifeTime value report.
    /// - The Lifetime Value report shows you how user value (Revenue) and
    ///   engagement (Appviews, Goal Completions, Sessions, and Session Duration)
    ///   grow during the 90 days after a user is acquired.
    /// - The metrics are calculated as a cumulative average per user per the time
    ///   increment.
    /// - The cohort definition date ranges need not be aligned to the calendar
    ///   week and month boundaries.
    /// - The `viewId` must be an
    ///   [app view ID](https://support.google.com/analytics/answer/2649553#WebVersusAppViews)
    #[serde(rename="lifetimeValue")]
    pub lifetime_value: Option<bool>,
}

impl Part for CohortGroup {}


/// Represents all the details pertaining to a goal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalData {
    /// Total number of goal completions in this activity.
    #[serde(rename="goalCompletions")]
    pub goal_completions: Option<String>,
    /// Value in this goal.
    #[serde(rename="goalValue")]
    pub goal_value: Option<f64>,
    /// Name of the goal.
    #[serde(rename="goalName")]
    pub goal_name: Option<String>,
    /// URL of the page one step prior to the goal completion.
    #[serde(rename="goalPreviousStep1")]
    pub goal_previous_step1: Option<String>,
    /// URL of the page three steps prior to the goal completion.
    #[serde(rename="goalPreviousStep3")]
    pub goal_previous_step3: Option<String>,
    /// URL of the page two steps prior to the goal completion.
    #[serde(rename="goalPreviousStep2")]
    pub goal_previous_step2: Option<String>,
    /// This identifies the goal as configured for the profile.
    #[serde(rename="goalIndex")]
    pub goal_index: Option<i32>,
    /// URL of the page where this goal was completed.
    #[serde(rename="goalCompletionLocation")]
    pub goal_completion_location: Option<String>,
}

impl Part for GoalData {}


/// Dimension filter specifies the filtering options on a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionFilter {
    /// The dimension to filter on. A DimensionFilter must contain a dimension.
    #[serde(rename="dimensionName")]
    pub dimension_name: Option<String>,
    /// Should the match be case sensitive? Default is false.
    #[serde(rename="caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// Strings or regular expression to match against. Only the first value of
    /// the list is used for comparison unless the operator is `IN_LIST`.
    /// If `IN_LIST` operator, then the entire list is used to filter the
    /// dimensions as explained in the description of the `IN_LIST` operator.
    pub expressions: Option<Vec<String>>,
    /// Logical `NOT` operator. If this boolean is set to true, then the matching
    /// dimension values will be excluded in the report. The default is false.
    pub not: Option<bool>,
    /// How to match the dimension to the expression. The default is REGEXP.
    pub operator: Option<String>,
}

impl Part for DimensionFilter {}


/// An Activity represents data for an activity of a user. Note that an
/// Activity is different from a hit.
/// A hit might result in multiple Activity's. For example, if a hit
/// includes a transaction and a goal completion, there will be two
/// Activity protos for this hit, one for ECOMMERCE and one for GOAL.
/// Conversely, multiple hits can also construct one Activity. In classic
/// e-commerce, data for one transaction might be sent through multiple hits.
/// These hits will be merged into one ECOMMERCE Activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// A list of all custom dimensions associated with this activity.
    #[serde(rename="customDimension")]
    pub custom_dimension: Option<Vec<CustomDimension>>,
    /// The first page in users' sessions, or the landing page.
    #[serde(rename="landingPagePath")]
    pub landing_page_path: Option<String>,
    /// The hostname from which the tracking request was made.
    pub hostname: Option<String>,
    /// This will be set if `activity_type` equals `ECOMMERCE`.
    pub ecommerce: Option<EcommerceData>,
    /// For manual campaign tracking, it is the value of the utm_term campaign
    /// tracking parameter. For AdWords traffic, it contains the best matching
    /// targeting criteria. For the display network, where multiple targeting
    /// criteria could have caused the ad to show up, it returns the best matching
    /// targeting criteria as selected by Ads. This could be display_keyword, site
    /// placement, boomuserlist, user_interest, age, or gender. Otherwise its value
    /// is (not set).
    pub keyword: Option<String>,
    /// For manual campaign tracking, it is the value of the utm_campaign campaign
    /// tracking parameter. For AdWords autotagging, it is the name(s) of the
    /// online ad campaign(s) you use for the property. If you use neither, its
    /// value is (not set).
    pub campaign: Option<String>,
    /// Type of this activity.
    #[serde(rename="activityType")]
    pub activity_type: Option<String>,
    /// This will be set if `activity_type` equals `PAGEVIEW`. This field
    /// contains all the details about the visitor and the page that was visited.
    pub pageview: Option<PageviewData>,
    /// This will be set if `activity_type` equals `SCREEN_VIEW`.
    pub appview: Option<ScreenviewData>,
    /// The source of referrals. For manual campaign tracking, it is the value of
    /// the utm_source campaign tracking parameter. For AdWords autotagging, it is
    /// google. If you use neither, it is the domain of the source
    /// (e.g., document.referrer) referring the users. It may also contain a port
    /// address. If users arrived without a referrer, its value is (direct).
    pub source: Option<String>,
    /// The type of referrals. For manual campaign tracking, it is the value of the
    /// utm_medium campaign tracking parameter. For AdWords autotagging, it is cpc.
    /// If users came from a search engine detected by Google Analytics, it is
    /// organic. If the referrer is not a search engine, it is referral. If users
    /// came directly to the property and document.referrer is empty, its value is
    /// (none).
    pub medium: Option<String>,
    /// The Channel Group associated with an end user's session for this View
    /// (defined by the View's Channel Groupings).
    #[serde(rename="channelGrouping")]
    pub channel_grouping: Option<String>,
    /// Timestamp of the activity.
    #[serde(rename="activityTime")]
    pub activity_time: Option<String>,
    /// This field contains all the details pertaining to an event and will be
    /// set if `activity_type` equals `EVENT`.
    pub event: Option<EventData>,
    /// This field contains a list of all the goals that were reached in this
    /// activity when `activity_type` equals `GOAL`.
    pub goals: Option<GoalSetData>,
}

impl Part for Activity {}


/// A segment sequence definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentSequenceStep {
    /// Specifies if the step immediately precedes or can be any time before the
    /// next step.
    #[serde(rename="matchType")]
    pub match_type: Option<String>,
    /// A sequence is specified with a list of Or grouped filters which are
    /// combined with `AND` operator.
    #[serde(rename="orFiltersForSegment")]
    pub or_filters_for_segment: Option<Vec<OrFiltersForSegment>>,
}

impl Part for SegmentSequenceStep {}


/// [Dimensions](https://support.google.com/analytics/answer/1033861)
/// are attributes of your data. For example, the dimension `ga:city`
/// indicates the city, for example, "Paris" or "New York", from which
/// a session originates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// Name of the dimension to fetch, for example `ga:browser`.
    pub name: Option<String>,
    /// If non-empty, we place dimension values into buckets after string to
    /// int64. Dimension values that are not the string representation of an
    /// integral value will be converted to zero.  The bucket values have to be in
    /// increasing order.  Each bucket is closed on the lower end, and open on the
    /// upper end. The "first" bucket includes all values less than the first
    /// boundary, the "last" bucket includes all values up to infinity. Dimension
    /// values that fall in a bucket get transformed to a new dimension value. For
    /// example, if one gives a list of "0, 1, 3, 4, 7", then we return the
    /// following buckets:
    /// 
    /// - bucket #1: values < 0, dimension value "<0"
    /// - bucket #2: values in [0,1), dimension value "0"
    /// - bucket #3: values in [1,3), dimension value "1-2"
    /// - bucket #4: values in [3,4), dimension value "3"
    /// - bucket #5: values in [4,7), dimension value "4-6"
    /// - bucket #6: values >= 7, dimension value "7+"
    /// 
    /// NOTE: If you are applying histogram mutation on any dimension, and using
    /// that dimension in sort, you will want to use the sort type
    /// `HISTOGRAM_BUCKET` for that purpose. Without that the dimension values
    /// will be sorted according to dictionary
    /// (lexicographic) order. For example the ascending dictionary order is:
    /// 
    ///    "<50", "1001+", "121-1000", "50-120"
    /// 
    /// And the ascending `HISTOGRAM_BUCKET` order is:
    /// 
    ///    "<50", "50-120", "121-1000", "1001+"
    /// 
    /// The client has to explicitly request `"orderType": "HISTOGRAM_BUCKET"`
    /// for a histogram-mutated dimension.
    #[serde(rename="histogramBuckets")]
    pub histogram_buckets: Option<Vec<String>>,
}

impl Part for Dimension {}


/// MetricFilter specifies the filter on a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricFilter {
    /// Logical `NOT` operator. If this boolean is set to true, then the matching
    /// metric values will be excluded in the report. The default is false.
    pub not: Option<bool>,
    /// Is the metric `EQUAL`, `LESS_THAN` or `GREATER_THAN` the
    /// comparisonValue, the default is `EQUAL`. If the operator is
    /// `IS_MISSING`, checks if the metric is missing and would ignore the
    /// comparisonValue.
    pub operator: Option<String>,
    /// The value to compare against.
    #[serde(rename="comparisonValue")]
    pub comparison_value: Option<String>,
    /// The metric that will be filtered on. A metricFilter must contain a metric
    /// name. A metric name can be an alias earlier defined as a metric or it can
    /// also be a metric expression.
    #[serde(rename="metricName")]
    pub metric_name: Option<String>,
}

impl Part for MetricFilter {}


/// Represents all the details pertaining to an event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventData {
    /// Label attached with the event.
    #[serde(rename="eventLabel")]
    pub event_label: Option<String>,
    /// Numeric value associated with the event.
    #[serde(rename="eventValue")]
    pub event_value: Option<String>,
    /// Type of interaction with the object. Eg: 'play'.
    #[serde(rename="eventAction")]
    pub event_action: Option<String>,
    /// The object on the page that was interacted with. Eg: 'Video'.
    #[serde(rename="eventCategory")]
    pub event_category: Option<String>,
    /// Number of such events in this activity.
    #[serde(rename="eventCount")]
    pub event_count: Option<String>,
}

impl Part for EventData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScreenviewData {
    /// Mobile manufacturer or branded name. Eg: "Google", "Apple" etc.
    #[serde(rename="mobileDeviceBranding")]
    pub mobile_device_branding: Option<String>,
    /// Mobile device model. Eg: "Pixel", "iPhone" etc.
    #[serde(rename="mobileDeviceModel")]
    pub mobile_device_model: Option<String>,
    /// The application name.
    #[serde(rename="appName")]
    pub app_name: Option<String>,
    /// The name of the screen.
    #[serde(rename="screenName")]
    pub screen_name: Option<String>,
}

impl Part for ScreenviewData {}


/// [Metrics](https://support.google.com/analytics/answer/1033861)
/// are the quantitative measurements. For example, the metric `ga:users`
/// indicates the total number of users for the requested time period.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// An alias for the metric expression is an alternate name for the
    /// expression. The alias can be used for filtering and sorting. This field
    /// is optional and is useful if the expression is not a single metric but
    /// a complex expression which cannot be used in filtering and sorting.
    /// The alias is also used in the response column header.
    pub alias: Option<String>,
    /// A metric expression in the request. An expression is constructed from one
    /// or more metrics and numbers. Accepted operators include: Plus (+), Minus
    /// (-), Negation (Unary -), Divided by (/), Multiplied by (*), Parenthesis,
    /// Positive cardinal numbers (0-9), can include decimals and is limited to
    /// 1024 characters. Example `ga:totalRefunds/ga:users`, in most cases the
    /// metric expression is just a single metric name like `ga:users`.
    /// Adding mixed `MetricType` (E.g., `CURRENCY` + `PERCENTAGE`) metrics
    /// will result in unexpected results.
    pub expression: Option<String>,
    /// Specifies how the metric expression should be formatted, for example
    /// `INTEGER`.
    #[serde(rename="formattingType")]
    pub formatting_type: Option<String>,
}

impl Part for Metric {}


/// Dimension filter specifies the filtering options on a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentDimensionFilter {
    /// Name of the dimension for which the filter is being applied.
    #[serde(rename="dimensionName")]
    pub dimension_name: Option<String>,
    /// Should the match be case sensitive, ignored for `IN_LIST` operator.
    #[serde(rename="caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// Minimum comparison values for `BETWEEN` match type.
    #[serde(rename="minComparisonValue")]
    pub min_comparison_value: Option<String>,
    /// The operator to use to match the dimension with the expressions.
    pub operator: Option<String>,
    /// The list of expressions, only the first element is used for all operators
    pub expressions: Option<Vec<String>>,
    /// Maximum comparison values for `BETWEEN` match type.
    #[serde(rename="maxComparisonValue")]
    pub max_comparison_value: Option<String>,
}

impl Part for SegmentDimensionFilter {}


/// A list of segment filters in the `OR` group are combined with the logical OR
/// operator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrFiltersForSegment {
    /// List of segment filters to be combined with a `OR` operator.
    #[serde(rename="segmentFilterClauses")]
    pub segment_filter_clauses: Option<Vec<SegmentFilterClause>>,
}

impl Part for OrFiltersForSegment {}


/// Represents a set of goals that were reached in an activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalSetData {
    /// All the goals that were reached in the current activity.
    pub goals: Option<Vec<GoalData>>,
}

impl Part for GoalSetData {}


/// Filter Clause to be used in a segment definition, can be wither a metric or
/// a dimension filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentFilterClause {
    /// Matches the complement (`!`) of the filter.
    pub not: Option<bool>,
    /// Dimension Filter for the segment definition.
    #[serde(rename="dimensionFilter")]
    pub dimension_filter: Option<SegmentDimensionFilter>,
    /// Metric Filter for the segment definition.
    #[serde(rename="metricFilter")]
    pub metric_filter: Option<SegmentMetricFilter>,
}

impl Part for SegmentFilterClause {}


/// Specifies the sorting options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderBy {
    /// The order type. The default orderType is `VALUE`.
    #[serde(rename="orderType")]
    pub order_type: Option<String>,
    /// The field which to sort by. The default sort order is ascending. Example:
    /// `ga:browser`.
    /// Note, that you can only specify one field for sort here. For example,
    /// `ga:browser, ga:city` is not valid.
    #[serde(rename="fieldName")]
    pub field_name: Option<String>,
    /// The sorting order for the field.
    #[serde(rename="sortOrder")]
    pub sort_order: Option<String>,
}

impl Part for OrderBy {}


/// A group of dimension filters. Set the operator value to specify how
/// the filters are logically combined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionFilterClause {
    /// The operator for combining multiple dimension filters. If unspecified, it
    /// is treated as an `OR`.
    pub operator: Option<String>,
    /// The repeated set of filters. They are logically combined based on the
    /// operator specified.
    pub filters: Option<Vec<DimensionFilter>>,
}

impl Part for DimensionFilterClause {}


/// Metric filter to be used in a segment filter clause.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentMetricFilter {
    /// Specifies is the operation to perform to compare the metric. The default
    /// is `EQUAL`.
    pub operator: Option<String>,
    /// Scope for a metric defines the level at which that metric is defined.  The
    /// specified metric scope must be equal to or greater than its primary scope
    /// as defined in the data model. The primary scope is defined by if the
    /// segment is selecting users or sessions.
    pub scope: Option<String>,
    /// The value to compare against. If the operator is `BETWEEN`, this value is
    /// treated as minimum comparison value.
    #[serde(rename="comparisonValue")]
    pub comparison_value: Option<String>,
    /// Max comparison value is only used for `BETWEEN` operator.
    #[serde(rename="maxComparisonValue")]
    pub max_comparison_value: Option<String>,
    /// The metric that will be filtered on. A `metricFilter` must contain a
    /// metric name.
    #[serde(rename="metricName")]
    pub metric_name: Option<String>,
}

impl Part for SegmentMetricFilter {}


/// The Pivot describes the pivot section in the request.
/// The Pivot helps rearrange the information in the table for certain reports
/// by pivoting your data on a second dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pivot {
    /// The pivot metrics. Pivot metrics are part of the
    /// restriction on total number of metrics allowed in the request.
    pub metrics: Option<Vec<Metric>>,
    /// A list of dimensions to show as pivot columns. A Pivot can have a maximum
    /// of 4 dimensions. Pivot dimensions are part of the restriction on the
    /// total number of dimensions allowed in the request.
    pub dimensions: Option<Vec<Dimension>>,
    /// Specifies the maximum number of groups to return.
    /// The default value is 10, also the maximum value is 1,000.
    #[serde(rename="maxGroupCount")]
    pub max_group_count: Option<i32>,
    /// DimensionFilterClauses are logically combined with an `AND` operator: only
    /// data that is included by all these DimensionFilterClauses contributes to
    /// the values in this pivot region. Dimension filters can be used to restrict
    /// the columns shown in the pivot region. For example if you have
    /// `ga:browser` as the requested dimension in the pivot region, and you
    /// specify key filters to restrict `ga:browser` to only "IE" or "Firefox",
    /// then only those two browsers would show up as columns.
    #[serde(rename="dimensionFilterClauses")]
    pub dimension_filter_clauses: Option<Vec<DimensionFilterClause>>,
    /// If k metrics were requested, then the response will contain some
    /// data-dependent multiple of k columns in the report.  E.g., if you pivoted
    /// on the dimension `ga:browser` then you'd get k columns for "Firefox", k
    /// columns for "IE", k columns for "Chrome", etc. The ordering of the groups
    /// of columns is determined by descending order of "total" for the first of
    /// the k values.  Ties are broken by lexicographic ordering of the first
    /// pivot dimension, then lexicographic ordering of the second pivot
    /// dimension, and so on.  E.g., if the totals for the first value for
    /// Firefox, IE, and Chrome were 8, 2, 8, respectively, the order of columns
    /// would be Chrome, Firefox, IE.
    /// 
    /// The following let you choose which of the groups of k columns are
    /// included in the response.
    #[serde(rename="startGroup")]
    pub start_group: Option<i32>,
}

impl Part for Pivot {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *userActivity* resources.
/// It is not used directly, but through the `AnalyticsReporting` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_analyticsreporting4 as analyticsreporting4;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use analyticsreporting4::AnalyticsReporting;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = AnalyticsReporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.user_activity();
/// # }
/// ```
pub struct UserActivityMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a AnalyticsReporting<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserActivityMethods<'a, C, A> {}

impl<'a, C, A> UserActivityMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns User Activity data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: SearchUserActivityRequest) -> UserActivitySearchCall<'a, C, A> {
        UserActivitySearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the `AnalyticsReporting` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_analyticsreporting4 as analyticsreporting4;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use analyticsreporting4::AnalyticsReporting;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = AnalyticsReporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a AnalyticsReporting<C, A>,
}

impl<'a, C, A> MethodsBuilder for ReportMethods<'a, C, A> {}

impl<'a, C, A> ReportMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the Analytics data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_get(&self, request: GetReportsRequest) -> ReportBatchGetCall<'a, C, A> {
        ReportBatchGetCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns User Activity data.
///
/// A builder for the *search* method supported by a *userActivity* resource.
/// It is not used directly, but through a `UserActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsreporting4 as analyticsreporting4;
/// use analyticsreporting4::SearchUserActivityRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use analyticsreporting4::AnalyticsReporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = AnalyticsReporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SearchUserActivityRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_activity().search(req)
///              .doit();
/// # }
/// ```
pub struct UserActivitySearchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a AnalyticsReporting<C, A>,
    _request: SearchUserActivityRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserActivitySearchCall<'a, C, A> {}

impl<'a, C, A> UserActivitySearchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SearchUserActivityResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "analyticsreporting.userActivity.search",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/userActivity:search";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Analytic.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: SearchUserActivityRequest) -> UserActivitySearchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserActivitySearchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> UserActivitySearchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Analytic`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> UserActivitySearchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the Analytics data.
///
/// A builder for the *batchGet* method supported by a *report* resource.
/// It is not used directly, but through a `ReportMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsreporting4 as analyticsreporting4;
/// use analyticsreporting4::GetReportsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use analyticsreporting4::AnalyticsReporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = AnalyticsReporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetReportsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.reports().batch_get(req)
///              .doit();
/// # }
/// ```
pub struct ReportBatchGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a AnalyticsReporting<C, A>,
    _request: GetReportsRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ReportBatchGetCall<'a, C, A> {}

impl<'a, C, A> ReportBatchGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetReportsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "analyticsreporting.reports.batchGet",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/reports:batchGet";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Analytic.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GetReportsRequest) -> ReportBatchGetCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ReportBatchGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ReportBatchGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Analytic`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ReportBatchGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


