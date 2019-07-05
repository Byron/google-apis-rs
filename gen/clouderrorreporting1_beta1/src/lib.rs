// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Clouderrorreporting* crate version *1.0.9+20190626*, where *20190626* is the exact revision of the *clouderrorreporting:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.9*.
//! 
//! Everything else about the *Clouderrorreporting* *v1_beta1* API can be found at the
//! [official documentation site](https://cloud.google.com/error-reporting/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/clouderrorreporting1_beta1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Clouderrorreporting.html) ... 
//! 
//! * projects
//!  * [*delete events*](struct.ProjectDeleteEventCall.html), [*events list*](struct.ProjectEventListCall.html), [*events report*](struct.ProjectEventReportCall.html), [*group stats list*](struct.ProjectGroupStatListCall.html), [*groups get*](struct.ProjectGroupGetCall.html) and [*groups update*](struct.ProjectGroupUpdateCall.html)
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
//! * **[Hub](struct.Clouderrorreporting.html)**
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
//! let r = hub.projects().groups_get(...).doit()
//! let r = hub.projects().groups_update(...).doit()
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
//! google-clouderrorreporting1_beta1 = "*"
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
//! extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
//! use clouderrorreporting1_beta1::ErrorGroup;
//! use clouderrorreporting1_beta1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use clouderrorreporting1_beta1::Clouderrorreporting;
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
//! let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = ErrorGroup::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().groups_update(req, "name")
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
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
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

/// Central instance to access all Clouderrorreporting related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// use clouderrorreporting1_beta1::ErrorGroup;
/// use clouderrorreporting1_beta1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use clouderrorreporting1_beta1::Clouderrorreporting;
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
/// let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ErrorGroup::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().groups_update(req, "name")
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
pub struct Clouderrorreporting<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Clouderrorreporting<C, A> {}

impl<'a, C, A> Clouderrorreporting<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Clouderrorreporting<C, A> {
        Clouderrorreporting {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.9".to_string(),
            _base_url: "https://clouderrorreporting.googleapis.com/".to_string(),
            _root_url: "https://clouderrorreporting.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.9`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://clouderrorreporting.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://clouderrorreporting.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The number of errors in a given time period.
/// All numbers are approximate since the error events are sampled
/// before counting them.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimedCount {
    /// Approximate number of occurrences in the given time period.
    pub count: Option<String>,
    /// End of the time period to which `count` refers (excluded).
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// Start of the time period to which `count` refers (included).
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
}

impl Part for TimedCount {}


/// Information related to tracking the progress on resolving the error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrackingIssue {
    /// A URL pointing to a related entry in an issue tracking system.
    /// Example: https://github.com/user/project/issues/4
    pub url: Option<String>,
}

impl Part for TrackingIssue {}


/// Contains a set of requested error events.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [events list projects](struct.ProjectEventListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEventsResponse {
    /// If non-empty, more results are available.
    /// Pass this token, along with the same query parameters as the first
    /// request, to view the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The error events which match the given request.
    #[serde(rename="errorEvents")]
    pub error_events: Option<Vec<ErrorEvent>>,
    /// The timestamp specifies the start time to which the request was restricted.
    #[serde(rename="timeRangeBegin")]
    pub time_range_begin: Option<String>,
}

impl ResponseResult for ListEventsResponse {}


/// A description of the context in which an error occurred.
/// This data should be provided by the application when reporting an error,
/// unless the
/// error report has been generated automatically from Google App Engine logs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorContext {
    /// The HTTP request which was processed when the error was
    /// triggered.
    #[serde(rename="httpRequest")]
    pub http_request: Option<HttpRequestContext>,
    /// Source code that was used to build the executable which has
    /// caused the given error message.
    #[serde(rename="sourceReferences")]
    pub source_references: Option<Vec<SourceReference>>,
    /// The location in the source code where the decision was made to
    /// report the error, usually the place where it was logged.
    /// For a logged exception this would be the source line where the
    /// exception is logged, usually close to the place where it was
    /// caught.
    #[serde(rename="reportLocation")]
    pub report_location: Option<SourceLocation>,
    /// The user who caused or was affected by the crash.
    /// This can be a user ID, an email address, or an arbitrary token that
    /// uniquely identifies the user.
    /// When sending an error report, leave this field empty if the user was not
    /// logged in. In this case the
    /// Error Reporting system will use other data, such as remote IP address, to
    /// distinguish affected users. See `affected_users_count` in
    /// `ErrorGroupStats`.
    pub user: Option<String>,
}

impl Part for ErrorContext {}


/// Description of a group of similar error events.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [groups get projects](struct.ProjectGroupGetCall.html) (response)
/// * [groups update projects](struct.ProjectGroupUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorGroup {
    /// Associated tracking issues.
    #[serde(rename="trackingIssues")]
    pub tracking_issues: Option<Vec<TrackingIssue>>,
    /// Group IDs are unique for a given project. If the same kind of error
    /// occurs in different service contexts, it will receive the same group ID.
    #[serde(rename="groupId")]
    pub group_id: Option<String>,
    /// The group resource name.
    /// Example: <code>projects/my-project-123/groups/my-groupid</code>
    pub name: Option<String>,
}

impl RequestValue for ErrorGroup {}
impl ResponseResult for ErrorGroup {}


/// Data extracted for a specific group based on certain filter criteria,
/// such as a given time period and/or service filter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorGroupStats {
    /// Approximate total number of events in the given group that match
    /// the filter criteria.
    pub count: Option<String>,
    /// Approximate first occurrence that was ever seen for this group
    /// and which matches the given filter criteria, ignoring the
    /// time_range that was specified in the request.
    #[serde(rename="firstSeenTime")]
    pub first_seen_time: Option<String>,
    /// Approximate number of affected users in the given group that
    /// match the filter criteria.
    /// Users are distinguished by data in the `ErrorContext` of the
    /// individual error events, such as their login name or their remote
    /// IP address in case of HTTP requests.
    /// The number of affected users can be zero even if the number of
    /// errors is non-zero if no data was provided from which the
    /// affected user could be deduced.
    /// Users are counted based on data in the request
    /// context that was provided in the error report. If more users are
    /// implicitly affected, such as due to a crash of the whole service,
    /// this is not reflected here.
    #[serde(rename="affectedUsersCount")]
    pub affected_users_count: Option<i64>,
    /// Service contexts with a non-zero error count for the given filter
    /// criteria. This list can be truncated if multiple services are affected.
    /// Refer to `num_affected_services` for the total count.
    #[serde(rename="affectedServices")]
    pub affected_services: Option<Vec<ServiceContext>>,
    /// Approximate number of occurrences over time.
    /// Timed counts returned by ListGroups are guaranteed to be:
    /// 
    /// - Inside the requested time interval
    /// - Non-overlapping, and
    /// - Ordered by ascending time.
    #[serde(rename="timedCounts")]
    pub timed_counts: Option<Vec<TimedCount>>,
    /// Approximate last occurrence that was ever seen for this group and
    /// which matches the given filter criteria, ignoring the time_range
    /// that was specified in the request.
    #[serde(rename="lastSeenTime")]
    pub last_seen_time: Option<String>,
    /// An arbitrary event that is chosen as representative for the whole group.
    /// The representative event is intended to be used as a quick preview for
    /// the whole group. Events in the group are usually sufficiently similar
    /// to each other such that showing an arbitrary representative provides
    /// insight into the characteristics of the group as a whole.
    pub representative: Option<ErrorEvent>,
    /// The total number of services with a non-zero error count for the given
    /// filter criteria.
    #[serde(rename="numAffectedServices")]
    pub num_affected_services: Option<i32>,
    /// Group data that is independent of the filter criteria.
    pub group: Option<ErrorGroup>,
}

impl Part for ErrorGroupStats {}


/// Response message for deleting error events.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete events projects](struct.ProjectDeleteEventCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteEventsResponse { _never_set: Option<bool> }

impl ResponseResult for DeleteEventsResponse {}


/// Describes a running service that sends errors.
/// Its version changes over time and multiple versions can run in parallel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceContext {
    /// Type of the MonitoredResource. List of possible values:
    /// https://cloud.google.com/monitoring/api/resources
    /// 
    /// Value is set automatically for incoming errors and must not be set when
    /// reporting errors.
    #[serde(rename="resourceType")]
    pub resource_type: Option<String>,
    /// Represents the source code version that the developer provided,
    /// which could represent a version label or a Git SHA-1 hash, for example.
    /// For App Engine standard environment, the version is set to the version of
    /// the app.
    pub version: Option<String>,
    /// An identifier of the service, such as the name of the
    /// executable, job, or Google App Engine service name. This field is expected
    /// to have a low number of values that are relatively stable over time, as
    /// opposed to `version`, which can be changed whenever new code is deployed.
    /// 
    /// Contains the service name for error reports extracted from Google
    /// App Engine logs or `default` if the App Engine default service is used.
    pub service: Option<String>,
}

impl Part for ServiceContext {}


/// A reference to a particular snapshot of the source tree used to build and
/// deploy an application.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceReference {
    /// The canonical and persistent identifier of the deployed revision.
    /// Example (git): "0035781c50ec7aa23385dc841529ce8a4b70db1b"
    #[serde(rename="revisionId")]
    pub revision_id: Option<String>,
    /// Optional. A URI string identifying the repository.
    /// Example: "https://github.com/GoogleCloudPlatform/kubernetes.git"
    pub repository: Option<String>,
}

impl Part for SourceReference {}


/// An error event which is returned by the Error Reporting system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorEvent {
    /// The `ServiceContext` for which this error was reported.
    #[serde(rename="serviceContext")]
    pub service_context: Option<ServiceContext>,
    /// Time when the event occurred as provided in the error report.
    /// If the report did not contain a timestamp, the time the error was received
    /// by the Error Reporting system is used.
    #[serde(rename="eventTime")]
    pub event_time: Option<String>,
    /// The stack trace that was reported or logged by the service.
    pub message: Option<String>,
    /// Data about the context in which the error occurred.
    pub context: Option<ErrorContext>,
}

impl Part for ErrorEvent {}


/// Contains a set of requested error group stats.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [group stats list projects](struct.ProjectGroupStatListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGroupStatsResponse {
    /// If non-empty, more results are available.
    /// Pass this token, along with the same query parameters as the first
    /// request, to view the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The error group stats which match the given request.
    #[serde(rename="errorGroupStats")]
    pub error_group_stats: Option<Vec<ErrorGroupStats>>,
    /// The timestamp specifies the start time to which the request was restricted.
    /// The start time is set based on the requested time range. It may be adjusted
    /// to a later time if a project has exceeded the storage quota and older data
    /// has been deleted.
    #[serde(rename="timeRangeBegin")]
    pub time_range_begin: Option<String>,
}

impl ResponseResult for ListGroupStatsResponse {}


/// An error event which is reported to the Error Reporting system.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [events report projects](struct.ProjectEventReportCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportedErrorEvent {
    /// [Required] The service context in which this error has occurred.
    #[serde(rename="serviceContext")]
    pub service_context: Option<ServiceContext>,
    /// [Optional] Time when the event occurred.
    /// If not provided, the time when the event was received by the
    /// Error Reporting system will be used.
    #[serde(rename="eventTime")]
    pub event_time: Option<String>,
    /// [Required] The error message.
    /// If no `context.reportLocation` is provided, the message must contain a
    /// header (typically consisting of the exception type name and an error
    /// message) and an exception stack trace in one of the supported programming
    /// languages and formats.
    /// Supported languages are Java, Python, JavaScript, Ruby, C#, PHP, and Go.
    /// Supported stack trace formats are:
    /// 
    /// * **Java**: Must be the return value of
    /// [`Throwable.printStackTrace()`](https://docs.oracle.com/javase/7/docs/api/java/lang/Throwable.html#printStackTrace%28%29).
    /// * **Python**: Must be the return value of
    /// [`traceback.format_exc()`](https://docs.python.org/2/library/traceback.html#traceback.format_exc).
    /// * **JavaScript**: Must be the value of
    /// [`error.stack`](https://github.com/v8/v8/wiki/Stack-Trace-API) as returned
    /// by V8.
    /// * **Ruby**: Must contain frames returned by
    /// [`Exception.backtrace`](https://ruby-doc.org/core-2.2.0/Exception.html#method-i-backtrace).
    /// * **C#**: Must be the return value of
    /// [`Exception.ToString()`](https://msdn.microsoft.com/en-us/library/system.exception.tostring.aspx).
    /// * **PHP**: Must start with `PHP (Notice|Parse error|Fatal error|Warning)`
    /// and contain the result of
    /// [`(string)$exception`](http://php.net/manual/en/exception.tostring.php).
    /// * **Go**: Must be the return value of
    /// [`runtime.Stack()`](https://golang.org/pkg/runtime/debug/#Stack).
    pub message: Option<String>,
    /// [Optional] A description of the context in which the error occurred.
    pub context: Option<ErrorContext>,
}

impl RequestValue for ReportedErrorEvent {}


/// Response for reporting an individual error event.
/// Data may be added to this message in the future.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [events report projects](struct.ProjectEventReportCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReportErrorEventResponse { _never_set: Option<bool> }

impl ResponseResult for ReportErrorEventResponse {}


/// HTTP request data that is related to a reported error.
/// This data should be provided by the application when reporting an error,
/// unless the
/// error report has been generated automatically from Google App Engine logs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRequestContext {
    /// The HTTP response status code for the request.
    #[serde(rename="responseStatusCode")]
    pub response_status_code: Option<i32>,
    /// The IP address from which the request originated.
    /// This can be IPv4, IPv6, or a token which is derived from the
    /// IP address, depending on the data that has been provided
    /// in the error report.
    #[serde(rename="remoteIp")]
    pub remote_ip: Option<String>,
    /// The user agent information that is provided with the request.
    #[serde(rename="userAgent")]
    pub user_agent: Option<String>,
    /// The URL of the request.
    pub url: Option<String>,
    /// The referrer information that is provided with the request.
    pub referrer: Option<String>,
    /// The type of HTTP request, such as `GET`, `POST`, etc.
    pub method: Option<String>,
}

impl Part for HttpRequestContext {}


/// Indicates a location in the source code of the service for which errors are
/// reported. `functionName` must be provided by the application when reporting
/// an error, unless the error report contains a `message` with a supported
/// exception stack trace. All fields are optional for the later case.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    /// The source code filename, which can include a truncated relative
    /// path, or a full path from a production machine.
    #[serde(rename="filePath")]
    pub file_path: Option<String>,
    /// Human-readable name of a function or method.
    /// The value can include optional context like the class or package name.
    /// For example, `my.package.MyClass.method` in case of Java.
    #[serde(rename="functionName")]
    pub function_name: Option<String>,
    /// 1-based. 0 indicates that the line number is unknown.
    #[serde(rename="lineNumber")]
    pub line_number: Option<i32>,
}

impl Part for SourceLocation {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Clouderrorreporting` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use clouderrorreporting1_beta1::Clouderrorreporting;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete_events(...)`, `events_list(...)`, `events_report(...)`, `group_stats_list(...)`, `groups_get(...)` and `groups_update(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Clouderrorreporting<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the specified group.
    /// 
    /// # Arguments
    ///
    /// * `groupName` - [Required] The group resource name. Written as
    ///                 <code>projects/<var>projectID</var>/groups/<var>group_name</var></code>.
    ///                 Call
    ///                 <a href="/error-reporting/reference/rest/v1beta1/projects.groupStats/list">
    ///                 <code>groupStats.list</code></a> to return a list of groups belonging to
    ///                 this project.
    ///                 Example: <code>projects/my-project-123/groups/my-group</code>
    pub fn groups_get(&self, group_name: &str) -> ProjectGroupGetCall<'a, C, A> {
        ProjectGroupGetCall {
            hub: self.hub,
            _group_name: group_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the specified groups.
    /// 
    /// # Arguments
    ///
    /// * `projectName` - [Required] The resource name of the Google Cloud Platform project. Written
    ///                   as <code>projects/</code> plus the
    ///                   <a href="https://support.google.com/cloud/answer/6158840">Google Cloud
    ///                   Platform project ID</a>.
    ///                   Example: <code>projects/my-project-123</code>.
    pub fn group_stats_list(&self, project_name: &str) -> ProjectGroupStatListCall<'a, C, A> {
        ProjectGroupStatListCall {
            hub: self.hub,
            _project_name: project_name.to_string(),
            _timed_count_duration: Default::default(),
            _time_range_period: Default::default(),
            _service_filter_version: Default::default(),
            _service_filter_service: Default::default(),
            _service_filter_resource_type: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order: Default::default(),
            _group_id: Default::default(),
            _alignment_time: Default::default(),
            _alignment: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all error events of a given project.
    /// 
    /// # Arguments
    ///
    /// * `projectName` - [Required] The resource name of the Google Cloud Platform project. Written
    ///                   as `projects/` plus the
    ///                   [Google Cloud Platform project
    ///                   ID](https://support.google.com/cloud/answer/6158840).
    ///                   Example: `projects/my-project-123`.
    pub fn delete_events(&self, project_name: &str) -> ProjectDeleteEventCall<'a, C, A> {
        ProjectDeleteEventCall {
            hub: self.hub,
            _project_name: project_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the specified events.
    /// 
    /// # Arguments
    ///
    /// * `projectName` - [Required] The resource name of the Google Cloud Platform project. Written
    ///                   as `projects/` plus the
    ///                   [Google Cloud Platform project
    ///                   ID](https://support.google.com/cloud/answer/6158840).
    ///                   Example: `projects/my-project-123`.
    pub fn events_list(&self, project_name: &str) -> ProjectEventListCall<'a, C, A> {
        ProjectEventListCall {
            hub: self.hub,
            _project_name: project_name.to_string(),
            _time_range_period: Default::default(),
            _service_filter_version: Default::default(),
            _service_filter_service: Default::default(),
            _service_filter_resource_type: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _group_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replace the data for the specified group.
    /// Fails if the group does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The group resource name.
    ///            Example: <code>projects/my-project-123/groups/my-groupid</code>
    pub fn groups_update(&self, request: ErrorGroup, name: &str) -> ProjectGroupUpdateCall<'a, C, A> {
        ProjectGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Report an individual error event.
    /// 
    /// This endpoint accepts **either** an OAuth token,
    /// **or** an [API key](https://support.google.com/cloud/answer/6158862)
    /// for authentication. To use an API key, append it to the URL as the value of
    /// a `key` parameter. For example:
    /// 
    /// `POST
    /// https://clouderrorreporting.googleapis.com/v1beta1/projects/example-project/events:report?key=123ABC456`
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectName` - [Required] The resource name of the Google Cloud Platform project. Written
    ///                   as `projects/` plus the
    ///                   [Google Cloud Platform project
    ///                   ID](https://support.google.com/cloud/answer/6158840). Example:
    ///                   `projects/my-project-123`.
    pub fn events_report(&self, request: ReportedErrorEvent, project_name: &str) -> ProjectEventReportCall<'a, C, A> {
        ProjectEventReportCall {
            hub: self.hub,
            _request: request,
            _project_name: project_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Get the specified group.
///
/// A builder for the *groups.get* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouderrorreporting1_beta1::Clouderrorreporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().groups_get("groupName")
///              .doit();
/// # }
/// ```
pub struct ProjectGroupGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Clouderrorreporting<C, A>,
    _group_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectGroupGetCall<'a, C, A> {}

impl<'a, C, A> ProjectGroupGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ErrorGroup)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouderrorreporting.projects.groups.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("groupName", self._group_name.to_string()));
        for &field in ["alt", "groupName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+groupName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+groupName}", "groupName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["groupName"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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


    /// [Required] The group resource name. Written as
    /// <code>projects/<var>projectID</var>/groups/<var>group_name</var></code>.
    /// Call
    /// <a href="/error-reporting/reference/rest/v1beta1/projects.groupStats/list">
    /// <code>groupStats.list</code></a> to return a list of groups belonging to
    /// this project.
    /// 
    /// Example: <code>projects/my-project-123/groups/my-group</code>
    ///
    /// Sets the *group name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn group_name(mut self, new_value: &str) -> ProjectGroupGetCall<'a, C, A> {
        self._group_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectGroupGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGroupGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectGroupGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the specified groups.
///
/// A builder for the *groupStats.list* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouderrorreporting1_beta1::Clouderrorreporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().group_stats_list("projectName")
///              .timed_count_duration(-18)
///              .time_range_period("kasd")
///              .service_filter_version("accusam")
///              .service_filter_service("takimata")
///              .service_filter_resource_type("justo")
///              .page_token("amet.")
///              .page_size(-81)
///              .order("labore")
///              .add_group_id("sea")
///              .alignment_time("nonumy")
///              .alignment("dolores")
///              .doit();
/// # }
/// ```
pub struct ProjectGroupStatListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Clouderrorreporting<C, A>,
    _project_name: String,
    _timed_count_duration: Option<i64>,
    _time_range_period: Option<String>,
    _service_filter_version: Option<String>,
    _service_filter_service: Option<String>,
    _service_filter_resource_type: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order: Option<String>,
    _group_id: Vec<String>,
    _alignment_time: Option<String>,
    _alignment: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectGroupStatListCall<'a, C, A> {}

impl<'a, C, A> ProjectGroupStatListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListGroupStatsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouderrorreporting.projects.groupStats.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(14 + self._additional_params.len());
        params.push(("projectName", self._project_name.to_string()));
        if let Some(value) = self._timed_count_duration {
            params.push(("timedCountDuration", value.to_string()));
        }
        if let Some(value) = self._time_range_period {
            params.push(("timeRange.period", value.to_string()));
        }
        if let Some(value) = self._service_filter_version {
            params.push(("serviceFilter.version", value.to_string()));
        }
        if let Some(value) = self._service_filter_service {
            params.push(("serviceFilter.service", value.to_string()));
        }
        if let Some(value) = self._service_filter_resource_type {
            params.push(("serviceFilter.resourceType", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order {
            params.push(("order", value.to_string()));
        }
        if self._group_id.len() > 0 {
            for f in self._group_id.iter() {
                params.push(("groupId", f.to_string()));
            }
        }
        if let Some(value) = self._alignment_time {
            params.push(("alignmentTime", value.to_string()));
        }
        if let Some(value) = self._alignment {
            params.push(("alignment", value.to_string()));
        }
        for &field in ["alt", "projectName", "timedCountDuration", "timeRange.period", "serviceFilter.version", "serviceFilter.service", "serviceFilter.resourceType", "pageToken", "pageSize", "order", "groupId", "alignmentTime", "alignment"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+projectName}/groupStats";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+projectName}", "projectName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectName"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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


    /// [Required] The resource name of the Google Cloud Platform project. Written
    /// as <code>projects/</code> plus the
    /// <a href="https://support.google.com/cloud/answer/6158840">Google Cloud
    /// Platform project ID</a>.
    /// 
    /// Example: <code>projects/my-project-123</code>.
    ///
    /// Sets the *project name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_name(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._project_name = new_value.to_string();
        self
    }
    /// [Optional] The preferred duration for a single returned `TimedCount`.
    /// If not set, no timed counts are returned.
    ///
    /// Sets the *timed count duration* query property to the given value.
    pub fn timed_count_duration(mut self, new_value: i64) -> ProjectGroupStatListCall<'a, C, A> {
        self._timed_count_duration = Some(new_value);
        self
    }
    /// Restricts the query to the specified time range.
    ///
    /// Sets the *time range.period* query property to the given value.
    pub fn time_range_period(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._time_range_period = Some(new_value.to_string());
        self
    }
    /// [Optional] The exact value to match against
    /// [`ServiceContext.version`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.version).
    ///
    /// Sets the *service filter.version* query property to the given value.
    pub fn service_filter_version(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._service_filter_version = Some(new_value.to_string());
        self
    }
    /// [Optional] The exact value to match against
    /// [`ServiceContext.service`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.service).
    ///
    /// Sets the *service filter.service* query property to the given value.
    pub fn service_filter_service(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._service_filter_service = Some(new_value.to_string());
        self
    }
    /// [Optional] The exact value to match against
    /// [`ServiceContext.resource_type`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.resource_type).
    ///
    /// Sets the *service filter.resource type* query property to the given value.
    pub fn service_filter_resource_type(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._service_filter_resource_type = Some(new_value.to_string());
        self
    }
    /// [Optional] A `next_page_token` provided by a previous response. To view
    /// additional results, pass this token along with the identical query
    /// parameters as the first request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// [Optional] The maximum number of results to return per response.
    /// Default is 20.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectGroupStatListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// [Optional] The sort order in which the results are returned.
    /// Default is `COUNT_DESC`.
    ///
    /// Sets the *order* query property to the given value.
    pub fn order(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._order = Some(new_value.to_string());
        self
    }
    /// [Optional] List all <code>ErrorGroupStats</code> with these IDs.
    ///
    /// Append the given value to the *group id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_group_id(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._group_id.push(new_value.to_string());
        self
    }
    /// [Optional] Time where the timed counts shall be aligned if rounded
    /// alignment is chosen. Default is 00:00 UTC.
    ///
    /// Sets the *alignment time* query property to the given value.
    pub fn alignment_time(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._alignment_time = Some(new_value.to_string());
        self
    }
    /// [Optional] The alignment of the timed counts to be returned.
    /// Default is `ALIGNMENT_EQUAL_AT_END`.
    ///
    /// Sets the *alignment* query property to the given value.
    pub fn alignment(mut self, new_value: &str) -> ProjectGroupStatListCall<'a, C, A> {
        self._alignment = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectGroupStatListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGroupStatListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectGroupStatListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes all error events of a given project.
///
/// A builder for the *deleteEvents* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouderrorreporting1_beta1::Clouderrorreporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().delete_events("projectName")
///              .doit();
/// # }
/// ```
pub struct ProjectDeleteEventCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Clouderrorreporting<C, A>,
    _project_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectDeleteEventCall<'a, C, A> {}

impl<'a, C, A> ProjectDeleteEventCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DeleteEventsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouderrorreporting.projects.deleteEvents",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("projectName", self._project_name.to_string()));
        for &field in ["alt", "projectName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+projectName}/events";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+projectName}", "projectName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectName"].iter() {
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


    /// [Required] The resource name of the Google Cloud Platform project. Written
    /// as `projects/` plus the
    /// [Google Cloud Platform project
    /// ID](https://support.google.com/cloud/answer/6158840).
    /// Example: `projects/my-project-123`.
    ///
    /// Sets the *project name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_name(mut self, new_value: &str) -> ProjectDeleteEventCall<'a, C, A> {
        self._project_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectDeleteEventCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeleteEventCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectDeleteEventCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the specified events.
///
/// A builder for the *events.list* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouderrorreporting1_beta1::Clouderrorreporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().events_list("projectName")
///              .time_range_period("aliquyam")
///              .service_filter_version("ea")
///              .service_filter_service("no")
///              .service_filter_resource_type("justo")
///              .page_token("justo")
///              .page_size(-34)
///              .group_id("et")
///              .doit();
/// # }
/// ```
pub struct ProjectEventListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Clouderrorreporting<C, A>,
    _project_name: String,
    _time_range_period: Option<String>,
    _service_filter_version: Option<String>,
    _service_filter_service: Option<String>,
    _service_filter_resource_type: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _group_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectEventListCall<'a, C, A> {}

impl<'a, C, A> ProjectEventListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListEventsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouderrorreporting.projects.events.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("projectName", self._project_name.to_string()));
        if let Some(value) = self._time_range_period {
            params.push(("timeRange.period", value.to_string()));
        }
        if let Some(value) = self._service_filter_version {
            params.push(("serviceFilter.version", value.to_string()));
        }
        if let Some(value) = self._service_filter_service {
            params.push(("serviceFilter.service", value.to_string()));
        }
        if let Some(value) = self._service_filter_resource_type {
            params.push(("serviceFilter.resourceType", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._group_id {
            params.push(("groupId", value.to_string()));
        }
        for &field in ["alt", "projectName", "timeRange.period", "serviceFilter.version", "serviceFilter.service", "serviceFilter.resourceType", "pageToken", "pageSize", "groupId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+projectName}/events";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+projectName}", "projectName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectName"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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


    /// [Required] The resource name of the Google Cloud Platform project. Written
    /// as `projects/` plus the
    /// [Google Cloud Platform project
    /// ID](https://support.google.com/cloud/answer/6158840).
    /// Example: `projects/my-project-123`.
    ///
    /// Sets the *project name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_name(mut self, new_value: &str) -> ProjectEventListCall<'a, C, A> {
        self._project_name = new_value.to_string();
        self
    }
    /// Restricts the query to the specified time range.
    ///
    /// Sets the *time range.period* query property to the given value.
    pub fn time_range_period(mut self, new_value: &str) -> ProjectEventListCall<'a, C, A> {
        self._time_range_period = Some(new_value.to_string());
        self
    }
    /// [Optional] The exact value to match against
    /// [`ServiceContext.version`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.version).
    ///
    /// Sets the *service filter.version* query property to the given value.
    pub fn service_filter_version(mut self, new_value: &str) -> ProjectEventListCall<'a, C, A> {
        self._service_filter_version = Some(new_value.to_string());
        self
    }
    /// [Optional] The exact value to match against
    /// [`ServiceContext.service`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.service).
    ///
    /// Sets the *service filter.service* query property to the given value.
    pub fn service_filter_service(mut self, new_value: &str) -> ProjectEventListCall<'a, C, A> {
        self._service_filter_service = Some(new_value.to_string());
        self
    }
    /// [Optional] The exact value to match against
    /// [`ServiceContext.resource_type`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.resource_type).
    ///
    /// Sets the *service filter.resource type* query property to the given value.
    pub fn service_filter_resource_type(mut self, new_value: &str) -> ProjectEventListCall<'a, C, A> {
        self._service_filter_resource_type = Some(new_value.to_string());
        self
    }
    /// [Optional] A `next_page_token` provided by a previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectEventListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// [Optional] The maximum number of results to return per response.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectEventListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// [Required] The group for which events shall be returned.
    ///
    /// Sets the *group id* query property to the given value.
    pub fn group_id(mut self, new_value: &str) -> ProjectEventListCall<'a, C, A> {
        self._group_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectEventListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectEventListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectEventListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Replace the data for the specified group.
/// Fails if the group does not exist.
///
/// A builder for the *groups.update* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// use clouderrorreporting1_beta1::ErrorGroup;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouderrorreporting1_beta1::Clouderrorreporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ErrorGroup::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().groups_update(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectGroupUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Clouderrorreporting<C, A>,
    _request: ErrorGroup,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectGroupUpdateCall<'a, C, A> {}

impl<'a, C, A> ProjectGroupUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ErrorGroup)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouderrorreporting.projects.groups.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
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
    pub fn request(mut self, new_value: ErrorGroup) -> ProjectGroupUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The group resource name.
    /// Example: <code>projects/my-project-123/groups/my-groupid</code>
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectGroupUpdateCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectGroupUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGroupUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectGroupUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Report an individual error event.
/// 
/// This endpoint accepts **either** an OAuth token,
/// **or** an [API key](https://support.google.com/cloud/answer/6158862)
/// for authentication. To use an API key, append it to the URL as the value of
/// a `key` parameter. For example:
/// 
/// `POST
/// https://clouderrorreporting.googleapis.com/v1beta1/projects/example-project/events:report?key=123ABC456`
///
/// A builder for the *events.report* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouderrorreporting1_beta1 as clouderrorreporting1_beta1;
/// use clouderrorreporting1_beta1::ReportedErrorEvent;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouderrorreporting1_beta1::Clouderrorreporting;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Clouderrorreporting::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ReportedErrorEvent::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().events_report(req, "projectName")
///              .doit();
/// # }
/// ```
pub struct ProjectEventReportCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Clouderrorreporting<C, A>,
    _request: ReportedErrorEvent,
    _project_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectEventReportCall<'a, C, A> {}

impl<'a, C, A> ProjectEventReportCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ReportErrorEventResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouderrorreporting.projects.events.report",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectName", self._project_name.to_string()));
        for &field in ["alt", "projectName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+projectName}/events:report";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+projectName}", "projectName")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectName"].iter() {
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
    pub fn request(mut self, new_value: ReportedErrorEvent) -> ProjectEventReportCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// [Required] The resource name of the Google Cloud Platform project. Written
    /// as `projects/` plus the
    /// [Google Cloud Platform project
    /// ID](https://support.google.com/cloud/answer/6158840). Example:
    /// `projects/my-project-123`.
    ///
    /// Sets the *project name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_name(mut self, new_value: &str) -> ProjectEventReportCall<'a, C, A> {
        self._project_name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectEventReportCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectEventReportCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectEventReportCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


