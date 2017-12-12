// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Service Consumer Management* crate version *1.0.6+20171211*, where *20171211* is the exact revision of the *serviceconsumermanagement:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.6*.
//! 
//! Everything else about the *Service Consumer Management* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/service-consumer-management/docs/overview).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/serviceconsumermanagement1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.ServiceConsumerManagement.html) ... 
//! 
//! * [operations](struct.Operation.html)
//!  * [*cancel*](struct.OperationCancelCall.html), [*delete*](struct.OperationDeleteCall.html), [*get*](struct.OperationGetCall.html) and [*list*](struct.OperationListCall.html)
//! * [services](struct.Service.html)
//!  * [*search*](struct.ServiceSearchCall.html), [*tenancy units add project*](struct.ServiceTenancyUnitAddProjectCall.html), [*tenancy units create*](struct.ServiceTenancyUnitCreateCall.html), [*tenancy units delete*](struct.ServiceTenancyUnitDeleteCall.html), [*tenancy units list*](struct.ServiceTenancyUnitListCall.html) and [*tenancy units remove project*](struct.ServiceTenancyUnitRemoveProjectCall.html)
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
//! * **[Hub](struct.ServiceConsumerManagement.html)**
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
//! let r = hub.operations().delete(...).doit()
//! let r = hub.services().tenancy_units_add_project(...).doit()
//! let r = hub.operations().list(...).doit()
//! let r = hub.services().tenancy_units_remove_project(...).doit()
//! let r = hub.services().tenancy_units_delete(...).doit()
//! let r = hub.operations().cancel(...).doit()
//! let r = hub.operations().get(...).doit()
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
//! google-serviceconsumermanagement1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
//! use serviceconsumermanagement1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use serviceconsumermanagement1::ServiceConsumerManagement;
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
//! let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.operations().list("name")
//!              .page_token("dolores")
//!              .page_size(-63)
//!              .filter("accusam")
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

    /// Manage your Google API service configuration
    ServiceManagement,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::ServiceManagement => "https://www.googleapis.com/auth/service.management",
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

/// Central instance to access all ServiceConsumerManagement related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// use serviceconsumermanagement1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use serviceconsumermanagement1::ServiceConsumerManagement;
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
/// let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().list("name")
///              .page_token("justo")
///              .page_size(-1)
///              .filter("erat")
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
pub struct ServiceConsumerManagement<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for ServiceConsumerManagement<C, A> {}

impl<'a, C, A> ServiceConsumerManagement<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> ServiceConsumerManagement<C, A> {
        ServiceConsumerManagement {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.6".to_string(),
            _base_url: "https://serviceconsumermanagement.googleapis.com/".to_string(),
            _root_url: "https://serviceconsumermanagement.googleapis.com/".to_string(),
        }
    }

    pub fn operations(&'a self) -> OperationMethods<'a, C, A> {
        OperationMethods { hub: &self }
    }
    pub fn services(&'a self) -> ServiceMethods<'a, C, A> {
        ServiceMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.6`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://serviceconsumermanagement.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://serviceconsumermanagement.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Translates to IAM Policy bindings (without auditing at this level)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyBinding {
    /// Role. (https://cloud.google.com/iam/docs/understanding-roles)
    /// For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    pub role: Option<String>,
    /// Uses the same format as in IAM policy.
    /// `member` must include both prefix and id. E.g., `user:{emailId}`,
    /// `serviceAccount:{emailId}`, `group:{emailId}`.
    pub members: Option<Vec<String>>,
}

impl Part for PolicyBinding {}


/// Define a parameter's name and location. The parameter may be passed as either
/// an HTTP header or a URL query parameter, and if both are passed the behavior
/// is implementation-dependent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemParameter {
    /// Define the URL query parameter name to use for the parameter. It is case
    /// sensitive.
    #[serde(rename="urlQueryParameter")]
    pub url_query_parameter: Option<String>,
    /// Define the HTTP header name to use for the parameter. It is case
    /// insensitive.
    #[serde(rename="httpHeader")]
    pub http_header: Option<String>,
    /// Define the name of the parameter, such as "api_key" . It is case sensitive.
    pub name: Option<String>,
}

impl Part for SystemParameter {}


/// Monitoring configuration of the service.
/// 
/// The example below shows how to configure monitored resources and metrics
/// for monitoring. In the example, a monitored resource and two metrics are
/// defined. The `library.googleapis.com/book/returned_count` metric is sent
/// to both producer and consumer projects, whereas the
/// `library.googleapis.com/book/overdue_count` metric is only sent to the
/// consumer project.
/// 
///     monitored_resources:
///     - type: library.googleapis.com/branch
///       labels:
///       - key: /city
///         description: The city where the library branch is located in.
///       - key: /name
///         description: The name of the branch.
///     metrics:
///     - name: library.googleapis.com/book/returned_count
///       metric_kind: DELTA
///       value_type: INT64
///       labels:
///       - key: /customer_id
///     - name: library.googleapis.com/book/overdue_count
///       metric_kind: GAUGE
///       value_type: INT64
///       labels:
///       - key: /customer_id
///     monitoring:
///       producer_destinations:
///       - monitored_resource: library.googleapis.com/branch
///         metrics:
///         - library.googleapis.com/book/returned_count
///       consumer_destinations:
///       - monitored_resource: library.googleapis.com/branch
///         metrics:
///         - library.googleapis.com/book/returned_count
///         - library.googleapis.com/book/overdue_count
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Monitoring {
    /// Monitoring configurations for sending metrics to the producer project.
    /// There can be multiple producer destinations, each one must have a
    /// different monitored resource type. A metric can be used in at most
    /// one producer destination.
    #[serde(rename="producerDestinations")]
    pub producer_destinations: Option<Vec<MonitoringDestination>>,
    /// Monitoring configurations for sending metrics to the consumer project.
    /// There can be multiple consumer destinations, each one must have a
    /// different monitored resource type. A metric can be used in at most
    /// one consumer destination.
    #[serde(rename="consumerDestinations")]
    pub consumer_destinations: Option<Vec<MonitoringDestination>>,
}

impl Part for Monitoring {}


/// Billing related configuration of the service.
/// 
/// The following example shows how to configure monitored resources and metrics
/// for billing:
/// 
///     monitored_resources:
///     - type: library.googleapis.com/branch
///       labels:
///       - key: /city
///         description: The city where the library branch is located in.
///       - key: /name
///         description: The name of the branch.
///     metrics:
///     - name: library.googleapis.com/book/borrowed_count
///       metric_kind: DELTA
///       value_type: INT64
///     billing:
///       consumer_destinations:
///       - monitored_resource: library.googleapis.com/branch
///         metrics:
///         - library.googleapis.com/book/borrowed_count
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Billing {
    /// Billing configurations for sending metrics to the consumer project.
    /// There can be multiple consumer destinations per service, each one must have
    /// a different monitored resource type. A metric can be used in at most
    /// one consumer destination.
    #[serde(rename="consumerDestinations")]
    pub consumer_destinations: Option<Vec<BillingDestination>>,
}

impl Part for Billing {}


/// Request message to remove tenant project resource from the tenancy unit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenancy units remove project services](struct.ServiceTenancyUnitRemoveProjectCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveTenantProjectRequest {
    /// Tag of the resource within the tenancy unit.
    pub tag: Option<String>,
}

impl RequestValue for RemoveTenantProjectRequest {}


/// `Authentication` defines the authentication configuration for an API.
/// 
/// Example for an API targeted for external use:
/// 
///     name: calendar.googleapis.com
///     authentication:
///       providers:
///       - id: google_calendar_auth
///         jwks_uri: https://www.googleapis.com/oauth2/v1/certs
///         issuer: https://securetoken.google.com
///       rules:
///       - selector: "*"
///         requirements:
///           provider_id: google_calendar_auth
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Authentication {
    /// A list of authentication rules that apply to individual API methods.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<AuthenticationRule>>,
    /// Defines a set of authentication providers that a service supports.
    pub providers: Option<Vec<AuthProvider>>,
}

impl Part for Authentication {}


/// `QuotaLimit` defines a specific limit that applies over a specified duration
/// for a limit type. There can be at most one limit for a duration and limit
/// type combination defined within a `QuotaGroup`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaLimit {
    /// User-visible display name for this limit.
    /// Optional. If not set, the UI will provide a default display name based on
    /// the quota configuration. This field can be used to override the default
    /// display name generated from the configuration.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Optional. User-visible, extended description for this quota limit.
    /// Should be used only when more context is needed to understand this limit
    /// than provided by the limit's display name (see: `display_name`).
    pub description: Option<String>,
    /// Default number of tokens that can be consumed during the specified
    /// duration. This is the number of tokens assigned when a client
    /// application developer activates the service for his/her project.
    /// 
    /// Specifying a value of 0 will block all requests. This can be used if you
    /// are provisioning quota to selected consumers and blocking others.
    /// Similarly, a value of -1 will indicate an unlimited quota. No other
    /// negative values are allowed.
    /// 
    /// Used by group-based quotas only.
    #[serde(rename="defaultLimit")]
    pub default_limit: Option<String>,
    /// The name of the metric this quota limit applies to. The quota limits with
    /// the same metric will be checked together during runtime. The metric must be
    /// defined within the service config.
    pub metric: Option<String>,
    /// Tiered limit values. You must specify this as a key:value pair, with an
    /// integer value that is the maximum number of requests allowed for the
    /// specified unit. Currently only STANDARD is supported.
    pub values: Option<HashMap<String, String>>,
    /// Maximum number of tokens that can be consumed during the specified
    /// duration. Client application developers can override the default limit up
    /// to this maximum. If specified, this value cannot be set to a value less
    /// than the default limit. If not specified, it is set to the default limit.
    /// 
    /// To allow clients to apply overrides with no upper bound, set this to -1,
    /// indicating unlimited maximum quota.
    /// 
    /// Used by group-based quotas only.
    #[serde(rename="maxLimit")]
    pub max_limit: Option<String>,
    /// Duration of this limit in textual notation. Example: "100s", "24h", "1d".
    /// For duration longer than a day, only multiple of days is supported. We
    /// support only "100s" and "1d" for now. Additional support will be added in
    /// the future. "0" indicates indefinite duration.
    /// 
    /// Used by group-based quotas only.
    pub duration: Option<String>,
    /// Free tier value displayed in the Developers Console for this limit.
    /// The free tier is the number of tokens that will be subtracted from the
    /// billed amount when billing is enabled.
    /// This field can only be set on a limit with duration "1d", in a billable
    /// group; it is invalid on any other limit. If this field is not set, it
    /// defaults to 0, indicating that there is no free tier for this service.
    /// 
    /// Used by group-based quotas only.
    #[serde(rename="freeTier")]
    pub free_tier: Option<String>,
    /// Specify the unit of the quota limit. It uses the same syntax as
    /// Metric.unit. The supported unit kinds are determined by the quota
    /// backend system.
    /// 
    /// Here are some examples:
    /// * "1/min/{project}" for quota per minute per project.
    /// 
    /// Note: the order of unit components is insignificant.
    /// The "1" at the beginning is required to follow the metric unit syntax.
    pub unit: Option<String>,
    /// Name of the quota limit.
    /// 
    /// The name must be provided, and it must be unique within the service. The
    /// name can only include alphanumeric characters as well as '-'.
    /// 
    /// The maximum length of the limit name is 64 characters.
    pub name: Option<String>,
}

impl Part for QuotaLimit {}


/// Describes service account configuration for the tenant project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountConfig {
    /// Roles for the service account above on tenant project.
    #[serde(rename="tenantProjectRoles")]
    pub tenant_project_roles: Option<Vec<String>>,
    /// ID of the IAM service account to be created in tenant project.
    /// The email format of the service account will be
    /// "<account-id>@<tenant-project-id>.iam.gserviceaccount.com".
    /// This account id has to be unique within tenant project and producers
    /// have to guarantee it.
    #[serde(rename="accountId")]
    pub account_id: Option<String>,
}

impl Part for ServiceAccountConfig {}


/// Configuration controlling usage of a service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    /// A list of usage rules that apply to individual API methods.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<UsageRule>>,
    /// The full resource name of a channel used for sending notifications to the
    /// service producer.
    /// 
    /// Google Service Management currently only supports
    /// [Google Cloud Pub/Sub](https://cloud.google.com/pubsub) as a notification
    /// channel. To use Google Cloud Pub/Sub as the channel, this must be the name
    /// of a Cloud Pub/Sub topic that uses the Cloud Pub/Sub topic name format
    /// documented in https://cloud.google.com/pubsub/docs/overview.
    #[serde(rename="producerNotificationChannel")]
    pub producer_notification_channel: Option<String>,
    /// Requirements that must be satisfied before a consumer project can use the
    /// service. Each requirement is of the form <service.name>/<requirement-id>;
    /// for example 'serviceusage.googleapis.com/billing-enabled'.
    pub requirements: Option<Vec<String>>,
}

impl Part for Usage {}


/// Method represents a method of an API interface.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Method {
    /// A URL of the input message type.
    #[serde(rename="requestTypeUrl")]
    pub request_type_url: Option<String>,
    /// If true, the response is streamed.
    #[serde(rename="responseStreaming")]
    pub response_streaming: Option<bool>,
    /// Any metadata attached to the method.
    pub options: Option<Vec<Option>>,
    /// The simple name of this method.
    pub name: Option<String>,
    /// The URL of the output message type.
    #[serde(rename="responseTypeUrl")]
    pub response_type_url: Option<String>,
    /// If true, the request is streamed.
    #[serde(rename="requestStreaming")]
    pub request_streaming: Option<bool>,
    /// The source syntax of this method.
    pub syntax: Option<String>,
}

impl Part for Method {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete operations](struct.OperationDeleteCall.html) (response)
/// * [cancel operations](struct.OperationCancelCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// Response for the list request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenancy units list services](struct.ServiceTenancyUnitListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTenancyUnitsResponse {
    /// Pagination token for large results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Tenancy Units matching the request.
    #[serde(rename="tenancyUnits")]
    pub tenancy_units: Option<Vec<TenancyUnit>>,
}

impl ResponseResult for ListTenancyUnitsResponse {}


/// Defines the HTTP configuration for an API service. It contains a list of
/// HttpRule, each specifying the mapping of an RPC method
/// to one or more HTTP REST API methods.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Http {
    /// A list of HTTP configuration rules that apply to individual API methods.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<HttpRule>>,
    /// When set to true, URL path parmeters will be fully URI-decoded except in
    /// cases of single segment matches in reserved expansion, where "%2F" will be
    /// left encoded.
    /// 
    /// The default behavior is to not decode RFC 6570 reserved characters in multi
    /// segment matches.
    #[serde(rename="fullyDecodeReservedExpansion")]
    pub fully_decode_reserved_expansion: Option<bool>,
}

impl Part for Http {}


/// Configuration for an anthentication provider, including support for
/// [JSON Web Token (JWT)](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthProvider {
    /// The list of JWT
    /// [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3).
    /// that are allowed to access. A JWT containing any of these audiences will
    /// be accepted. When this setting is absent, only JWTs with audience
    /// "https://Service_name/API_name"
    /// will be accepted. For example, if no audiences are in the setting,
    /// LibraryService API will only accept JWTs with the following audience
    /// "https://library-example.googleapis.com/google.example.library.v1.LibraryService".
    /// 
    /// Example:
    /// 
    ///     audiences: bookstore_android.apps.googleusercontent.com,
    ///                bookstore_web.apps.googleusercontent.com
    pub audiences: Option<String>,
    /// URL of the provider's public key set to validate signature of the JWT. See
    /// [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata).
    /// Optional if the key set document:
    ///  - can be retrieved from
    ///    [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html
    ///    of the issuer.
    ///  - can be inferred from the email domain of the issuer (e.g. a Google service account).
    /// 
    /// Example: https://www.googleapis.com/oauth2/v1/certs
    #[serde(rename="jwksUri")]
    pub jwks_uri: Option<String>,
    /// The unique identifier of the auth provider. It will be referred to by
    /// `AuthRequirement.provider_id`.
    /// 
    /// Example: "bookstore_auth".
    pub id: Option<String>,
    /// Redirect URL if JWT token is required but no present or is expired.
    /// Implement authorizationUrl of securityDefinitions in OpenAPI spec.
    #[serde(rename="authorizationUrl")]
    pub authorization_url: Option<String>,
    /// Identifies the principal that issued the JWT. See
    /// https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.1
    /// Usually a URL or an email address.
    /// 
    /// Example: https://securetoken.google.com
    /// Example: 1234567-compute@developer.gserviceaccount.com
    pub issuer: Option<String>,
}

impl Part for AuthProvider {}


/// `Service` is the root object of Google service configuration schema. It
/// describes basic information about a service, such as the name and the
/// title, and delegates other aspects to sub-sections. Each sub-section is
/// either a proto message or a repeated proto message that configures a
/// specific aspect, such as auth. See each proto message definition for details.
/// 
/// Example:
/// 
///     type: google.api.Service
///     config_version: 3
///     name: calendar.googleapis.com
///     title: Google Calendar API
///     apis:
///     - name: google.calendar.v3.Calendar
///     authentication:
///       providers:
///       - id: google_calendar_auth
///         jwks_uri: https://www.googleapis.com/oauth2/v1/certs
///         issuer: https://securetoken.google.com
///       rules:
///       - selector: "*"
///         requirements:
///           provider_id: google_calendar_auth
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenancy units list services](struct.ServiceTenancyUnitListCall.html) (none)
/// * [tenancy units add project services](struct.ServiceTenancyUnitAddProjectCall.html) (none)
/// * [tenancy units create services](struct.ServiceTenancyUnitCreateCall.html) (none)
/// * [tenancy units remove project services](struct.ServiceTenancyUnitRemoveProjectCall.html) (none)
/// * [tenancy units delete services](struct.ServiceTenancyUnitDeleteCall.html) (none)
/// * [search services](struct.ServiceSearchCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// Configuration for the service control plane.
    pub control: Option<Control>,
    /// Defines the monitored resources used by this service. This is required
    /// by the Service.monitoring and Service.logging configurations.
    #[serde(rename="monitoredResources")]
    pub monitored_resources: Option<Vec<MonitoredResourceDescriptor>>,
    /// The Google project that owns this service.
    #[serde(rename="producerProjectId")]
    pub producer_project_id: Option<String>,
    /// HTTP configuration.
    pub http: Option<Http>,
    /// Defines the logs used by this service.
    pub logs: Option<Vec<LogDescriptor>>,
    /// A list of API interfaces exported by this service. Only the `name` field
    /// of the google.protobuf.Api needs to be provided by the configuration
    /// author, as the remaining fields will be derived from the IDL during the
    /// normalization process. It is an error to specify an API interface here
    /// which cannot be resolved against the associated IDL files.
    pub apis: Option<Vec<Api>>,
    /// Custom error configuration.
    #[serde(rename="customError")]
    pub custom_error: Option<CustomError>,
    /// Quota configuration.
    pub quota: Option<Quota>,
    /// API visibility configuration.
    pub visibility: Option<Visibility>,
    /// Defines the metrics used by this service.
    pub metrics: Option<Vec<MetricDescriptor>>,
    /// Logging configuration.
    pub logging: Option<Logging>,
    /// A list of all enum types included in this API service.  Enums
    /// referenced directly or indirectly by the `apis` are automatically
    /// included.  Enums which are not referenced but shall be included
    /// should be listed here by name. Example:
    /// 
    ///     enums:
    ///     - name: google.someapi.v1.SomeEnum
    pub enums: Option<Vec<Enum>>,
    /// A unique ID for a specific instance of this message, typically assigned
    /// by the client for tracking purpose. If empty, the server may choose to
    /// generate one instead.
    pub id: Option<String>,
    /// A list of all proto message types included in this API service.
    /// Types referenced directly or indirectly by the `apis` are
    /// automatically included.  Messages which are not referenced but
    /// shall be included, such as types used by the `google.protobuf.Any` type,
    /// should be listed here by name. Example:
    /// 
    ///     types:
    ///     - name: google.protobuf.Int32
    pub types: Option<Vec<Type>>,
    /// API backend configuration.
    pub backend: Option<Backend>,
    /// Configuration for network endpoints.  If this is empty, then an endpoint
    /// with the same name as the service is automatically generated to service all
    /// defined APIs.
    pub endpoints: Option<Vec<Endpoint>>,
    /// Monitoring configuration.
    pub monitoring: Option<Monitoring>,
    /// The DNS address at which this service is available,
    /// e.g. `calendar.googleapis.com`.
    pub name: Option<String>,
    /// Billing configuration.
    pub billing: Option<Billing>,
    /// System parameter configuration.
    #[serde(rename="systemParameters")]
    pub system_parameters: Option<SystemParameters>,
    /// Additional API documentation.
    pub documentation: Option<Documentation>,
    /// Output only. The source information for this configuration if available.
    #[serde(rename="sourceInfo")]
    pub source_info: Option<SourceInfo>,
    /// A list of all proto message types included in this API service.
    /// It serves similar purpose as [google.api.Service.types], except that
    /// these types are not needed by user-defined APIs. Therefore, they will not
    /// show up in the generated discovery doc. This field should only be used
    /// to define system APIs in ESF.
    #[serde(rename="systemTypes")]
    pub system_types: Option<Vec<Type>>,
    /// Auth configuration.
    pub authentication: Option<Authentication>,
    /// Context configuration.
    pub context: Option<Context>,
    /// Configuration controlling usage of this service.
    pub usage: Option<Usage>,
    /// The product title for this service.
    pub title: Option<String>,
    /// The semantic version of the service configuration. The config version
    /// affects the interpretation of the service configuration. For example,
    /// certain features are enabled by default for certain config versions.
    /// The latest config version is `3`.
    #[serde(rename="configVersion")]
    pub config_version: Option<u32>,
    /// Experimental configuration.
    pub experimental: Option<Experimental>,
}

impl Resource for Service {}


/// Enum type definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Enum {
    /// The source context.
    #[serde(rename="sourceContext")]
    pub source_context: Option<SourceContext>,
    /// Enum value definitions.
    pub enumvalue: Option<Vec<EnumValue>>,
    /// Protocol buffer options.
    pub options: Option<Vec<Option>>,
    /// Enum type name.
    pub name: Option<String>,
    /// The source syntax.
    pub syntax: Option<String>,
}

impl Part for Enum {}


/// Configuration of a specific billing destination (Currently only support
/// bill against consumer project).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BillingDestination {
    /// The monitored resource type. The type must be defined in
    /// Service.monitored_resources section.
    #[serde(rename="monitoredResource")]
    pub monitored_resource: Option<String>,
    /// Names of the metrics to report to this billing destination.
    /// Each name must be defined in Service.metrics section.
    pub metrics: Option<Vec<String>>,
}

impl Part for BillingDestination {}


/// Request to add a newly created and configured tenant project to tenancy
/// unit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenancy units add project services](struct.ServiceTenancyUnitAddProjectCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddTenantProjectRequest {
    /// Configuration of the new tenant project that will be added to tenancy unit
    /// resources.
    #[serde(rename="projectConfig")]
    pub project_config: Option<TenantProjectConfig>,
    /// Tag of the added project. Must be less than 128 characters. Required.
    pub tag: Option<String>,
}

impl RequestValue for AddTenantProjectRequest {}


/// Bind API methods to metrics. Binding a method to a metric causes that
/// metric's configured quota behaviors to apply to the method call.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricRule {
    /// Metrics to update when the selected methods are called, and the associated
    /// cost applied to each metric.
    /// 
    /// The key of the map is the metric name, and the values are the amount
    /// increased for the metric against which the quota limits are defined.
    /// The value must not be negative.
    #[serde(rename="metricCosts")]
    pub metric_costs: Option<HashMap<String, String>>,
    /// Selects the methods to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for MetricRule {}


/// Configuration of a specific logging destination (the producer project
/// or the consumer project).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoggingDestination {
    /// The monitored resource type. The type must be defined in the
    /// Service.monitored_resources section.
    #[serde(rename="monitoredResource")]
    pub monitored_resource: Option<String>,
    /// Names of the logs to be sent to this destination. Each name must
    /// be defined in the Service.logs section. If the log name is
    /// not a domain scoped name, it will be automatically prefixed with
    /// the service name followed by "/".
    pub logs: Option<Vec<String>>,
}

impl Part for LoggingDestination {}


/// This structure defines a tenant project to be added to the specified tenancy
/// unit and its initial configuration and properties. A project lien will be
/// created for the tenant project to prevent the tenant project from being
/// deleted accidentally. The lien will be deleted as part of tenant project
/// removal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TenantProjectConfig {
    /// Configuration for IAM service account on tenant project.
    #[serde(rename="serviceAccountConfig")]
    pub service_account_config: Option<ServiceAccountConfig>,
    /// Billing account properties.
    /// It may be specified explicitly, or created from the specified group
    /// during provisioning
    #[serde(rename="billingConfig")]
    pub billing_config: Option<BillingConfig>,
    /// Google Cloud API names of services that will be activated on this project
    /// during provisioning.  If any of these services can not be activated,
    /// addTenantProject method will fail.
    /// For example: 'compute.googleapis.com','cloudfunctions.googleapis.com'
    pub services: Option<Vec<String>>,
    /// Folder where project in this tenancy unit must be located
    /// This folder must have been previously created with proper
    /// permissions for the caller to create and configure a project in it.
    /// Valid folder resource names have the format `folders/{folder_number}`
    /// (for example, `folders/123456`).
    pub folder: Option<String>,
    /// Labels that will be applied to this project.
    pub labels: Option<HashMap<String, String>>,
    /// Describes ownership and policies for the new tenant project. Required.
    #[serde(rename="tenantProjectPolicy")]
    pub tenant_project_policy: Option<TenantProjectPolicy>,
}

impl Part for TenantProjectConfig {}


/// Response for the search query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search services](struct.ServiceSearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchTenancyUnitsResponse {
    /// Pagination token for large results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Tenancy Units matching the request.
    #[serde(rename="tenancyUnits")]
    pub tenancy_units: Option<Vec<TenancyUnit>>,
}

impl ResponseResult for SearchTenancyUnitsResponse {}


/// ### System parameter configuration
/// 
/// A system parameter is a special kind of parameter defined by the API
/// system, not by an individual API. It is typically mapped to an HTTP header
/// and/or a URL query parameter. This configuration specifies which methods
/// change the names of the system parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemParameters {
    /// Define system parameters.
    /// 
    /// The parameters defined here will override the default parameters
    /// implemented by the system. If this field is missing from the service
    /// config, default system parameters will be used. Default system parameters
    /// and names is implementation-dependent.
    /// 
    /// Example: define api key for all methods
    /// 
    ///     system_parameters
    ///       rules:
    ///         - selector: "*"
    ///           parameters:
    ///             - name: api_key
    ///               url_query_parameter: api_key
    /// 
    /// 
    /// Example: define 2 api key names for a specific method.
    /// 
    ///     system_parameters
    ///       rules:
    ///         - selector: "/ListShelves"
    ///           parameters:
    ///             - name: api_key
    ///               http_header: Api-Key1
    ///             - name: api_key
    ///               http_header: Api-Key2
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<SystemParameterRule>>,
}

impl Part for SystemParameters {}


/// Configuration for a custom authentication provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomAuthRequirements {
    /// A configuration string containing connection information for the
    /// authentication provider, typically formatted as a SmartService string
    /// (go/smartservice).
    pub provider: Option<String>,
}

impl Part for CustomAuthRequirements {}


/// Source information used to create a Service Config
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceInfo {
    /// All files used during config generation.
    #[serde(rename="sourceFiles")]
    pub source_files: Option<Vec<HashMap<String, String>>>,
}

impl Part for SourceInfo {}


/// Experimental service configuration. These configuration options can
/// only be used by whitelisted users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Experimental {
    /// Authorization configuration.
    pub authorization: Option<AuthorizationConfig>,
}

impl Part for Experimental {}


/// Selects and configures the service controller used by the service.  The
/// service controller handles features like abuse, quota, billing, logging,
/// monitoring, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Control {
    /// The service control environment to use. If empty, no control plane
    /// feature (like quota and billing) will be enabled.
    pub environment: Option<String>,
}

impl Part for Control {}


/// A context rule provides information about the context for an individual API
/// element.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContextRule {
    /// A list of full type names of provided contexts.
    pub provided: Option<Vec<String>>,
    /// Selects the methods to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
    /// A list of full type names of requested contexts.
    pub requested: Option<Vec<String>>,
}

impl Part for ContextRule {}


/// Describes billing configuration for new a Tenant Project
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BillingConfig {
    /// Name of the billing account.
    /// For example `billingAccounts/012345-567890-ABCDEF`.
    #[serde(rename="billingAccount")]
    pub billing_account: Option<String>,
}

impl Part for BillingConfig {}


/// A visibility rule provides visibility configuration for an individual API
/// element.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VisibilityRule {
    /// A comma-separated list of visibility labels that apply to the `selector`.
    /// Any of the listed labels can be used to grant the visibility.
    /// 
    /// If a rule has multiple labels, removing one of the labels but not all of
    /// them can break clients.
    /// 
    /// Example:
    /// 
    ///     visibility:
    ///       rules:
    ///       - selector: google.calendar.Calendar.EnhancedSearch
    ///         restriction: GOOGLE_INTERNAL, TRUSTED_TESTER
    /// 
    /// Removing GOOGLE_INTERNAL from this restriction will break clients that
    /// rely on this method and only had access to it through GOOGLE_INTERNAL.
    pub restriction: Option<String>,
    /// Selects methods, messages, fields, enums, etc. to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for VisibilityRule {}


/// Customize service error responses.  For example, list any service
/// specific protobuf types that can appear in error detail lists of
/// error responses.
/// 
/// Example:
/// 
///     custom_error:
///       types:
///       - google.foo.v1.CustomError
///       - google.foo.v1.AnotherError
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomError {
    /// The list of custom error rules that apply to individual API messages.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<CustomErrorRule>>,
    /// The list of custom error detail types, e.g. 'google.foo.v1.CustomError'.
    pub types: Option<Vec<String>>,
}

impl Part for CustomError {}


/// A documentation rule provides information about individual API elements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentationRule {
    /// Description of the selected API(s).
    pub description: Option<String>,
    /// Deprecation description of the selected element(s). It can be provided if an
    /// element is marked as `deprecated`.
    #[serde(rename="deprecationDescription")]
    pub deprecation_description: Option<String>,
    /// The selector is a comma-separated list of patterns. Each pattern is a
    /// qualified name of the element which may end in "*", indicating a wildcard.
    /// Wildcards are only allowed at the end and for a whole component of the
    /// qualified name, i.e. "foo.*" is ok, but not "foo.b*" or "foo.*.bar". To
    /// specify a default for all applicable elements, the whole pattern "*"
    /// is used.
    pub selector: Option<String>,
}

impl Part for DocumentationRule {}


/// Request to create a tenancy unit for a consumer of a service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenancy units create services](struct.ServiceTenancyUnitCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateTenancyUnitRequest {
    /// Optional producer provided identifier of the tenancy unit
    /// Must be no longer than 40 characters and preferably URI friendly
    /// If it is not provided, UID for the tenancy unit will be auto generated
    /// It must be unique across a service.
    /// If the tenancy unit already exists for the service and consumer pair,
    /// CreateTenancyUnit will return existing tenancy unit if provided identifier
    /// is identical or empty, otherwise the call will fail.
    #[serde(rename="tenancyUnitId")]
    pub tenancy_unit_id: Option<String>,
}

impl RequestValue for CreateTenancyUnitRequest {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete operations](struct.OperationDeleteCall.html) (none)
/// * [tenancy units add project services](struct.ServiceTenancyUnitAddProjectCall.html) (response)
/// * [list operations](struct.OperationListCall.html) (none)
/// * [tenancy units remove project services](struct.ServiceTenancyUnitRemoveProjectCall.html) (response)
/// * [tenancy units delete services](struct.ServiceTenancyUnitDeleteCall.html) (response)
/// * [cancel operations](struct.OperationCancelCall.html) (none)
/// * [get operations](struct.OperationGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, String>>,
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    pub done: Option<bool>,
    /// The normal response of the operation in case of success.  If the original
    /// method returns no data on success, such as `Delete`, the response is
    /// `google.protobuf.Empty`.  If the original method is standard
    /// `Get`/`Create`/`Update`, the response should be the resource.  For other
    /// methods, the response should have the type `XxxResponse`, where `Xxx`
    /// is the original method name.  For example, if the original method name
    /// is `TakeSnapshot()`, the inferred response type is
    /// `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, String>>,
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should have the format of `operations/some/unique/name`.
    pub name: Option<String>,
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<Status>,
}

impl Resource for Operation {}
impl ResponseResult for Operation {}


/// Configuration of authorization.
/// 
/// This section determines the authorization provider, if unspecified, then no
/// authorization check will be done.
/// 
/// Example:
/// 
///     experimental:
///       authorization:
///         provider: firebaserules.googleapis.com
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    /// The name of the authorization provider, such as
    /// firebaserules.googleapis.com.
    pub provider: Option<String>,
}

impl Part for AuthorizationConfig {}


/// OAuth scopes are a way to define data and permissions on data. For example,
/// there are scopes defined for "Read-only access to Google Calendar" and
/// "Access to Cloud Platform". Users can consent to a scope for an application,
/// giving it permission to access that data on their behalf.
/// 
/// OAuth scope specifications should be fairly coarse grained; a user will need
/// to see and understand the text description of what your scope means.
/// 
/// In most cases: use one or at most two OAuth scopes for an entire family of
/// products. If your product has multiple APIs, you should probably be sharing
/// the OAuth scope across all of those APIs.
/// 
/// When you need finer grained OAuth consent screens: talk with your product
/// management about how developers will use them in practice.
/// 
/// Please note that even though each of the canonical scopes is enough for a
/// request to be accepted and passed to the backend, a request can still fail
/// due to the backend requiring additional scopes or permissions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OAuthRequirements {
    /// The list of publicly documented OAuth scopes that are allowed access. An
    /// OAuth token containing any of these scopes will be accepted.
    /// 
    /// Example:
    /// 
    ///      canonical_scopes: https://www.googleapis.com/auth/calendar,
    ///                        https://www.googleapis.com/auth/calendar.read
    #[serde(rename="canonicalScopes")]
    pub canonical_scopes: Option<String>,
}

impl Part for OAuthRequirements {}


/// Define a system parameter rule mapping system parameter definitions to
/// methods.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemParameterRule {
    /// Define parameters. Multiple names may be defined for a parameter.
    /// For a given method call, only one of them should be used. If multiple
    /// names are used the behavior is implementation-dependent.
    /// If none of the specified names are present the behavior is
    /// parameter-dependent.
    pub parameters: Option<Vec<SystemParameter>>,
    /// Selects the methods to which this rule applies. Use '*' to indicate all
    /// methods in all APIs.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for SystemParameterRule {}


/// Configuration of a specific monitoring destination (the producer project
/// or the consumer project).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringDestination {
    /// The monitored resource type. The type must be defined in
    /// Service.monitored_resources section.
    #[serde(rename="monitoredResource")]
    pub monitored_resource: Option<String>,
    /// Names of the metrics to report to this monitoring destination.
    /// Each name must be defined in Service.metrics section.
    pub metrics: Option<Vec<String>>,
}

impl Part for MonitoringDestination {}


/// A custom pattern is used for defining custom HTTP verb.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomHttpPattern {
    /// The path matched by this custom verb.
    pub path: Option<String>,
    /// The name of this custom HTTP verb.
    pub kind: Option<String>,
}

impl Part for CustomHttpPattern {}


/// A single field of a message type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    /// The field type.
    pub kind: Option<String>,
    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    #[serde(rename="oneofIndex")]
    pub oneof_index: Option<i32>,
    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    #[serde(rename="typeUrl")]
    pub type_url: Option<String>,
    /// The field name.
    pub name: Option<String>,
    /// The string value of the default value of this field. Proto2 syntax only.
    #[serde(rename="defaultValue")]
    pub default_value: Option<String>,
    /// The field JSON name.
    #[serde(rename="jsonName")]
    pub json_name: Option<String>,
    /// The field number.
    pub number: Option<i32>,
    /// The field cardinality.
    pub cardinality: Option<String>,
    /// The protocol buffer options.
    pub options: Option<Vec<Option>>,
    /// Whether to use alternative packed wire representation.
    pub packed: Option<bool>,
}

impl Part for Field {}


/// A backend rule provides configuration for an individual API element.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackendRule {
    /// Minimum deadline in seconds needed for this method. Calls having deadline
    /// value lower than this will be rejected.
    #[serde(rename="minDeadline")]
    pub min_deadline: Option<f64>,
    /// The number of seconds to wait for a response from a request.  The default
    /// deadline for gRPC is infinite (no deadline) and HTTP requests is 5 seconds.
    pub deadline: Option<f64>,
    /// The address of the API backend.
    pub address: Option<String>,
    /// Selects the methods to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for BackendRule {}


/// Resource constituting the TenancyUnit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TenantResource {
    /// Status of tenant resource.
    pub status: Option<String>,
    /// Unique per single tenancy unit.
    pub tag: Option<String>,
    /// @OutputOnly Identifier of the tenant resource.
    /// For cloud projects it is in the form 'projects/{number}'.
    /// For example 'projects/123456'.
    pub resource: Option<String>,
}

impl Part for TenantResource {}


/// A protocol buffer message type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Type {
    /// The list of types appearing in `oneof` definitions in this type.
    pub oneofs: Option<Vec<String>>,
    /// The fully qualified message name.
    pub name: Option<String>,
    /// The list of fields.
    pub fields: Option<Vec<Field>>,
    /// The source context.
    #[serde(rename="sourceContext")]
    pub source_context: Option<SourceContext>,
    /// The protocol buffer options.
    pub options: Option<Vec<Option>>,
    /// The source syntax.
    pub syntax: Option<String>,
}

impl Part for Type {}


/// Representation of a tenancy unit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenancy units create services](struct.ServiceTenancyUnitCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TenancyUnit {
    /// Resources constituting the tenancy unit.
    #[serde(rename="tenantResources")]
    pub tenant_resources: Option<Vec<TenantResource>>,
    /// @OutputOnly Cloud resource One Platform Name of the consumer of this
    /// service. For example 'projects/123456'.
    pub consumer: Option<String>,
    /// @OutputOnly The time this tenancy unit was created.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// @OutputOnly Google Cloud API name of the service owning this tenancy unit.
    /// For example 'serviceconsumermanagement.googleapis.com'.
    pub service: Option<String>,
    /// Globally unique identifier of this tenancy unit
    /// "services/{service}/{collection id}/{resource id}/tenancyUnits/{unit}"
    pub name: Option<String>,
}

impl ResponseResult for TenancyUnit {}


/// `HttpRule` defines the mapping of an RPC method to one or more HTTP
/// REST API methods. The mapping specifies how different portions of the RPC
/// request message are mapped to URL path, URL query parameters, and
/// HTTP request body. The mapping is typically specified as an
/// `google.api.http` annotation on the RPC method,
/// see "google/api/annotations.proto" for details.
/// 
/// The mapping consists of a field specifying the path template and
/// method kind.  The path template can refer to fields in the request
/// message, as in the example below which describes a REST GET
/// operation on a resource collection of messages:
/// 
/// 
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http).get = "/v1/messages/{message_id}/{sub.subfield}";
///       }
///     }
///     message GetMessageRequest {
///       message SubMessage {
///         string subfield = 1;
///       }
///       string message_id = 1; // mapped to the URL
///       SubMessage sub = 2;    // `sub.subfield` is url-mapped
///     }
///     message Message {
///       string text = 1; // content of the resource
///     }
/// 
/// The same http annotation can alternatively be expressed inside the
/// `GRPC API Configuration` YAML file.
/// 
///     http:
///       rules:
///         - selector: <proto_package_name>.Messaging.GetMessage
///           get: /v1/messages/{message_id}/{sub.subfield}
/// 
/// This definition enables an automatic, bidrectional mapping of HTTP
/// JSON to RPC. Example:
/// 
/// HTTP | RPC
/// -----|-----
/// `GET /v1/messages/123456/foo`  | `GetMessage(message_id: "123456" sub: SubMessage(subfield: "foo"))`
/// 
/// In general, not only fields but also field paths can be referenced
/// from a path pattern. Fields mapped to the path pattern cannot be
/// repeated and must have a primitive (non-message) type.
/// 
/// Any fields in the request message which are not bound by the path
/// pattern automatically become (optional) HTTP query
/// parameters. Assume the following definition of the request message:
/// 
/// 
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http).get = "/v1/messages/{message_id}";
///       }
///     }
///     message GetMessageRequest {
///       message SubMessage {
///         string subfield = 1;
///       }
///       string message_id = 1; // mapped to the URL
///       int64 revision = 2;    // becomes a parameter
///       SubMessage sub = 3;    // `sub.subfield` becomes a parameter
///     }
/// 
/// 
/// This enables a HTTP JSON to RPC mapping as below:
/// 
/// HTTP | RPC
/// -----|-----
/// `GET /v1/messages/123456?revision=2&sub.subfield=foo` | `GetMessage(message_id: "123456" revision: 2 sub: SubMessage(subfield: "foo"))`
/// 
/// Note that fields which are mapped to HTTP parameters must have a
/// primitive type or a repeated primitive type. Message types are not
/// allowed. In the case of a repeated type, the parameter can be
/// repeated in the URL, as in `...?param=A&param=B`.
/// 
/// For HTTP method kinds which allow a request body, the `body` field
/// specifies the mapping. Consider a REST update method on the
/// message resource collection:
/// 
/// 
///     service Messaging {
///       rpc UpdateMessage(UpdateMessageRequest) returns (Message) {
///         option (google.api.http) = {
///           put: "/v1/messages/{message_id}"
///           body: "message"
///         };
///       }
///     }
///     message UpdateMessageRequest {
///       string message_id = 1; // mapped to the URL
///       Message message = 2;   // mapped to the body
///     }
/// 
/// 
/// The following HTTP JSON to RPC mapping is enabled, where the
/// representation of the JSON in the request body is determined by
/// protos JSON encoding:
/// 
/// HTTP | RPC
/// -----|-----
/// `PUT /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id: "123456" message { text: "Hi!" })`
/// 
/// The special name `*` can be used in the body mapping to define that
/// every field not bound by the path template should be mapped to the
/// request body.  This enables the following alternative definition of
/// the update method:
/// 
///     service Messaging {
///       rpc UpdateMessage(Message) returns (Message) {
///         option (google.api.http) = {
///           put: "/v1/messages/{message_id}"
///           body: "*"
///         };
///       }
///     }
///     message Message {
///       string message_id = 1;
///       string text = 2;
///     }
/// 
/// 
/// The following HTTP JSON to RPC mapping is enabled:
/// 
/// HTTP | RPC
/// -----|-----
/// `PUT /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id: "123456" text: "Hi!")`
/// 
/// Note that when using `*` in the body mapping, it is not possible to
/// have HTTP parameters, as all fields not bound by the path end in
/// the body. This makes this option more rarely used in practice of
/// defining REST APIs. The common usage of `*` is in custom methods
/// which don't use the URL at all for transferring data.
/// 
/// It is possible to define multiple HTTP methods for one RPC by using
/// the `additional_bindings` option. Example:
/// 
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http) = {
///           get: "/v1/messages/{message_id}"
///           additional_bindings {
///             get: "/v1/users/{user_id}/messages/{message_id}"
///           }
///         };
///       }
///     }
///     message GetMessageRequest {
///       string message_id = 1;
///       string user_id = 2;
///     }
/// 
/// 
/// This enables the following two alternative HTTP JSON to RPC
/// mappings:
/// 
/// HTTP | RPC
/// -----|-----
/// `GET /v1/messages/123456` | `GetMessage(message_id: "123456")`
/// `GET /v1/users/me/messages/123456` | `GetMessage(user_id: "me" message_id: "123456")`
/// 
/// # Rules for HTTP mapping
/// 
/// The rules for mapping HTTP path, query parameters, and body fields
/// to the request message are as follows:
/// 
/// 1. The `body` field specifies either `*` or a field path, or is
///    omitted. If omitted, it indicates there is no HTTP request body.
/// 2. Leaf fields (recursive expansion of nested messages in the
///    request) can be classified into three types:
///     (a) Matched in the URL template.
///     (b) Covered by body (if body is `*`, everything except (a) fields;
///         else everything under the body field)
///     (c) All other fields.
/// 3. URL query parameters found in the HTTP request are mapped to (c) fields.
/// 4. Any body sent with an HTTP request can contain only (b) fields.
/// 
/// The syntax of the path template is as follows:
/// 
///     Template = "/" Segments [ Verb ] ;
///     Segments = Segment { "/" Segment } ;
///     Segment  = "*" | "**" | LITERAL | Variable ;
///     Variable = "{" FieldPath [ "=" Segments ] "}" ;
///     FieldPath = IDENT { "." IDENT } ;
///     Verb     = ":" LITERAL ;
/// 
/// The syntax `*` matches a single path segment. The syntax `**` matches zero
/// or more path segments, which must be the last part of the path except the
/// `Verb`. The syntax `LITERAL` matches literal text in the path.
/// 
/// The syntax `Variable` matches part of the URL path as specified by its
/// template. A variable template must not contain other variables. If a variable
/// matches a single path segment, its template may be omitted, e.g. `{var}`
/// is equivalent to `{var=*}`.
/// 
/// If a variable contains exactly one path segment, such as `"{var}"` or
/// `"{var=*}"`, when such a variable is expanded into a URL path, all characters
/// except `[-_.~0-9a-zA-Z]` are percent-encoded. Such variables show up in the
/// Discovery Document as `{var}`.
/// 
/// If a variable contains one or more path segments, such as `"{var=foo/*}"`
/// or `"{var=**}"`, when such a variable is expanded into a URL path, all
/// characters except `[-_.~/0-9a-zA-Z]` are percent-encoded. Such variables
/// show up in the Discovery Document as `{+var}`.
/// 
/// NOTE: While the single segment variable matches the semantics of
/// [RFC 6570](https://tools.ietf.org/html/rfc6570) Section 3.2.2
/// Simple String Expansion, the multi segment variable **does not** match
/// RFC 6570 Reserved Expansion. The reason is that the Reserved Expansion
/// does not expand special characters like `?` and `#`, which would lead
/// to invalid URLs.
/// 
/// NOTE: the field paths in variables and in the `body` must not refer to
/// repeated fields or map fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRule {
    /// The name of the request field whose value is mapped to the HTTP body, or
    /// `*` for mapping all fields not captured by the path pattern to the HTTP
    /// body. NOTE: the referred field must not be a repeated field and must be
    /// present at the top-level of request message type.
    pub body: Option<String>,
    /// Selects methods to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
    /// Specifies the permission(s) required for an API element for the overall
    /// API request to succeed. It is typically used to mark request message fields
    /// that contain the name of the resource and indicates the permissions that
    /// will be checked on that resource.
    pub authorizations: Option<Vec<AuthorizationRule>>,
    /// Used for listing and getting information about resources.
    pub get: Option<String>,
    /// Use this only for Scotty Requests. Do not use this for bytestream methods.
    /// For media support, add instead [][google.bytestream.RestByteStream] as an
    /// API to your configuration.
    #[serde(rename="mediaDownload")]
    pub media_download: Option<MediaDownload>,
    /// Additional HTTP bindings for the selector. Nested bindings must
    /// not contain an `additional_bindings` field themselves (that is,
    /// the nesting may only be one level deep).
    #[serde(rename="additionalBindings")]
    pub additional_bindings: Option<Vec<HttpRule>>,
    /// DO NOT USE. This is an experimental field.
    /// 
    /// Optional. The REST collection name is by default derived from the URL
    /// pattern. If specified, this field overrides the default collection name.
    /// Example:
    /// 
    ///     rpc AddressesAggregatedList(AddressesAggregatedListRequest)
    ///         returns (AddressesAggregatedListResponse) {
    ///       option (google.api.http) = {
    ///         get: "/v1/projects/{project_id}/aggregated/addresses"
    ///         rest_collection: "projects.addresses"
    ///       };
    ///     }
    /// 
    /// This method has the automatically derived collection name
    /// "projects.aggregated". Because, semantically, this rpc is actually an
    /// operation on the "projects.addresses" collection, the `rest_collection`
    /// field is configured to override the derived collection name.
    #[serde(rename="restCollection")]
    pub rest_collection: Option<String>,
    /// The custom pattern is used for specifying an HTTP method that is not
    /// included in the `pattern` field, such as HEAD, or "*" to leave the
    /// HTTP method unspecified for this rule. The wild-card rule is useful
    /// for services that provide content to Web (HTML) clients.
    pub custom: Option<CustomHttpPattern>,
    /// The name of the response field whose value is mapped to the HTTP body of
    /// response. Other response fields are ignored. This field is optional. When
    /// not set, the response message will be used as HTTP body of response.
    /// NOTE: the referred field must be not a repeated field and must be present
    /// at the top-level of response message type.
    #[serde(rename="responseBody")]
    pub response_body: Option<String>,
    /// Use this only for Scotty Requests. Do not use this for media support using
    /// Bytestream, add instead
    /// [][google.bytestream.RestByteStream] as an API to your
    /// configuration for Bytestream methods.
    #[serde(rename="mediaUpload")]
    pub media_upload: Option<MediaUpload>,
    /// DO NOT USE. This is an experimental field.
    /// 
    /// Optional. The rest method name is by default derived from the URL
    /// pattern. If specified, this field overrides the default method name.
    /// Example:
    /// 
    ///     rpc CreateResource(CreateResourceRequest)
    ///         returns (CreateResourceResponse) {
    ///       option (google.api.http) = {
    ///         post: "/v1/resources",
    ///         body: "resource",
    ///         rest_method_name: "insert"
    ///       };
    ///     }
    /// 
    /// This method has the automatically derived rest method name
    /// "create", but for backwards compatibility with apiary, it is specified as
    /// insert.
    #[serde(rename="restMethodName")]
    pub rest_method_name: Option<String>,
    /// Used for updating a resource.
    pub put: Option<String>,
    /// Used for creating a resource.
    pub post: Option<String>,
    /// Used for updating a resource.
    pub patch: Option<String>,
    /// Used for deleting a resource.
    pub delete: Option<String>,
}

impl Part for HttpRule {}


/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Option {
    /// The option's name. For protobuf built-in options (options defined in
    /// descriptor.proto), this is the short name. For example, `"map_entry"`.
    /// For custom options, it should be the fully-qualified name. For example,
    /// `"google.api.http"`.
    pub name: Option<String>,
    /// The option's value packed in an Any message. If the value is a primitive,
    /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
    /// should be used. If the value is an enum, it should be stored as an int32
    /// value using the google.protobuf.Int32Value type.
    pub value: Option<HashMap<String, String>>,
}

impl Part for Option {}


/// Defines a metric type and its schema. Once a metric descriptor is created,
/// deleting or altering it stops data collection and makes the metric type's
/// existing data unusable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptor {
    /// A concise name for the metric, which can be displayed in user interfaces.
    /// Use sentence case without an ending period, for example "Request count".
    /// This field is optional but it is recommended to be set for any metrics
    /// associated with user-visible concepts, such as Quota.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// A detailed description of the metric, which can be used in documentation.
    pub description: Option<String>,
    /// Whether the metric records instantaneous values, changes to a value, etc.
    /// Some combinations of `metric_kind` and `value_type` might not be supported.
    #[serde(rename="metricKind")]
    pub metric_kind: Option<String>,
    /// Whether the measurement is an integer, a floating-point number, etc.
    /// Some combinations of `metric_kind` and `value_type` might not be supported.
    #[serde(rename="valueType")]
    pub value_type: Option<String>,
    /// The set of labels that can be used to describe a specific
    /// instance of this metric type. For example, the
    /// `appengine.googleapis.com/http/server/response_latencies` metric
    /// type has a label for the HTTP response code, `response_code`, so
    /// you can look at latencies for successful responses or just
    /// for responses that failed.
    pub labels: Option<Vec<LabelDescriptor>>,
    /// The metric type, including its DNS name prefix. The type is not
    /// URL-encoded.  All user-defined custom metric types have the DNS name
    /// `custom.googleapis.com`.  Metric types should use a natural hierarchical
    /// grouping. For example:
    /// 
    ///     "custom.googleapis.com/invoice/paid/amount"
    ///     "appengine.googleapis.com/http/server/response_latencies"
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The unit in which the metric value is reported. It is only applicable
    /// if the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The
    /// supported units are a subset of [The Unified Code for Units of
    /// Measure](http://unitsofmeasure.org/ucum.html) standard:
    /// 
    /// **Basic units (UNIT)**
    /// 
    /// * `bit`   bit
    /// * `By`    byte
    /// * `s`     second
    /// * `min`   minute
    /// * `h`     hour
    /// * `d`     day
    /// 
    /// **Prefixes (PREFIX)**
    /// 
    /// * `k`     kilo    (10**3)
    /// * `M`     mega    (10**6)
    /// * `G`     giga    (10**9)
    /// * `T`     tera    (10**12)
    /// * `P`     peta    (10**15)
    /// * `E`     exa     (10**18)
    /// * `Z`     zetta   (10**21)
    /// * `Y`     yotta   (10**24)
    /// * `m`     milli   (10**-3)
    /// * `u`     micro   (10**-6)
    /// * `n`     nano    (10**-9)
    /// * `p`     pico    (10**-12)
    /// * `f`     femto   (10**-15)
    /// * `a`     atto    (10**-18)
    /// * `z`     zepto   (10**-21)
    /// * `y`     yocto   (10**-24)
    /// * `Ki`    kibi    (2**10)
    /// * `Mi`    mebi    (2**20)
    /// * `Gi`    gibi    (2**30)
    /// * `Ti`    tebi    (2**40)
    /// 
    /// **Grammar**
    /// 
    /// The grammar includes the dimensionless unit `1`, such as `1/s`.
    /// 
    /// The grammar also includes these connectors:
    /// 
    /// * `/`    division (as an infix operator, e.g. `1/s`).
    /// * `.`    multiplication (as an infix operator, e.g. `GBy.d`)
    /// 
    /// The grammar for a unit is as follows:
    /// 
    ///     Expression = Component { "." Component } { "/" Component } ;
    /// 
    ///     Component = [ PREFIX ] UNIT [ Annotation ]
    ///               | Annotation
    ///               | "1"
    ///               ;
    /// 
    ///     Annotation = "{" NAME "}" ;
    /// 
    /// Notes:
    /// 
    /// * `Annotation` is just a comment if it follows a `UNIT` and is
    ///    equivalent to `1` if it is used alone. For examples,
    ///    `{requests}/s == 1/s`, `By{transmitted}/s == By/s`.
    /// * `NAME` is a sequence of non-blank printable ASCII characters not
    ///    containing '{' or '}'.
    pub unit: Option<String>,
    /// The resource name of the metric descriptor.
    pub name: Option<String>,
}

impl Part for MetricDescriptor {}


/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceContext {
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    #[serde(rename="fileName")]
    pub file_name: Option<String>,
}

impl Part for SourceContext {}


/// Defines the Media configuration for a service in case of an upload.
/// Use this only for Scotty Requests. Do not use this for media support using
/// Bytestream, add instead [][google.bytestream.RestByteStream] as an API to
/// your configuration for Bytestream methods.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaUpload {
    /// Whether to receive a notification for progress changes of media upload.
    #[serde(rename="progressNotification")]
    pub progress_notification: Option<bool>,
    /// Whether to receive a notification on the start of media upload.
    #[serde(rename="startNotification")]
    pub start_notification: Option<bool>,
    /// An array of mimetype patterns. Esf will only accept uploads that match one
    /// of the given patterns.
    #[serde(rename="mimeTypes")]
    pub mime_types: Option<Vec<String>>,
    /// A boolean that determines whether a notification for the completion of an
    /// upload should be sent to the backend. These notifications will not be seen
    /// by the client and will not consume quota.
    #[serde(rename="completeNotification")]
    pub complete_notification: Option<bool>,
    /// Whether upload is enabled.
    pub enabled: Option<bool>,
    /// Name of the Scotty dropzone to use for the current API.
    pub dropzone: Option<String>,
    /// Optional maximum acceptable size for an upload.
    /// The size is specified in bytes.
    #[serde(rename="maxSize")]
    pub max_size: Option<String>,
    /// DO NOT USE FIELDS BELOW THIS LINE UNTIL THIS WARNING IS REMOVED.
    /// 
    /// Specify name of the upload service if one is used for upload.
    #[serde(rename="uploadService")]
    pub upload_service: Option<String>,
}

impl Part for MediaUpload {}


/// Quota configuration helps to achieve fairness and budgeting in service
/// usage.
/// 
/// The quota configuration works this way:
/// - The service configuration defines a set of metrics.
/// - For API calls, the quota.metric_rules maps methods to metrics with
///   corresponding costs.
/// - The quota.limits defines limits on the metrics, which will be used for
///   quota checks at runtime.
/// 
/// An example quota configuration in yaml format:
/// 
///    quota:
/// 
///      - name: apiWriteQpsPerProject
///        metric: library.googleapis.com/write_calls
///        unit: "1/min/{project}"  # rate limit for consumer projects
///        values:
///          STANDARD: 10000
/// 
/// 
///      # The metric rules bind all methods to the read_calls metric,
///      # except for the UpdateBook and DeleteBook methods. These two methods
///      # are mapped to the write_calls metric, with the UpdateBook method
///      # consuming at twice rate as the DeleteBook method.
///      metric_rules:
///      - selector: "*"
///        metric_costs:
///          library.googleapis.com/read_calls: 1
///      - selector: google.example.library.v1.LibraryService.UpdateBook
///        metric_costs:
///          library.googleapis.com/write_calls: 2
///      - selector: google.example.library.v1.LibraryService.DeleteBook
///        metric_costs:
///          library.googleapis.com/write_calls: 1
/// 
///  Corresponding Metric definition:
/// 
///      metrics:
///      - name: library.googleapis.com/read_calls
///        display_name: Read requests
///        metric_kind: DELTA
///        value_type: INT64
/// 
///      - name: library.googleapis.com/write_calls
///        display_name: Write requests
///        metric_kind: DELTA
///        value_type: INT64
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Quota {
    /// List of `MetricRule` definitions, each one mapping a selected method to one
    /// or more metrics.
    #[serde(rename="metricRules")]
    pub metric_rules: Option<Vec<MetricRule>>,
    /// List of `QuotaLimit` definitions for the service.
    pub limits: Option<Vec<QuotaLimit>>,
}

impl Part for Quota {}


/// `Visibility` defines restrictions for the visibility of service
/// elements.  Restrictions are specified using visibility labels
/// (e.g., TRUSTED_TESTER) that are elsewhere linked to users and projects.
/// 
/// Users and projects can have access to more than one visibility label. The
/// effective visibility for multiple labels is the union of each label's
/// elements, plus any unrestricted elements.
/// 
/// If an element and its parents have no restrictions, visibility is
/// unconditionally granted.
/// 
/// Example:
/// 
///     visibility:
///       rules:
///       - selector: google.calendar.Calendar.EnhancedSearch
///         restriction: TRUSTED_TESTER
///       - selector: google.calendar.Calendar.Delegate
///         restriction: GOOGLE_INTERNAL
/// 
/// Here, all methods are publicly visible except for the restricted methods
/// EnhancedSearch and Delegate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Visibility {
    /// A list of visibility rules that apply to individual API elements.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<VisibilityRule>>,
}

impl Part for Visibility {}


/// A custom error rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomErrorRule {
    /// Mark this message as possible payload in error response.  Otherwise,
    /// objects of this type will be filtered when they appear in error payload.
    #[serde(rename="isErrorType")]
    pub is_error_type: Option<bool>,
    /// Selects messages to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for CustomErrorRule {}


/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:
/// 
/// - If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.
/// 
/// - Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.
/// 
/// - If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the root path if
///   specified.
/// 
/// Example of a simple mixin:
/// 
///     package google.acl.v1;
///     service AccessControl {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v1/{resource=**}:getAcl";
///       }
///     }
/// 
///     package google.storage.v2;
///     service Storage {
///       //       rpc GetAcl(GetAclRequest) returns (Acl);
/// 
///       // Get a data record.
///       rpc GetData(GetDataRequest) returns (Data) {
///         option (google.api.http).get = "/v2/{resource=**}";
///       }
///     }
/// 
/// Example of a mixin configuration:
/// 
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
/// 
/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inherting
/// documentation and annotations as follows:
/// 
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/{resource=**}:getAcl";
///       }
///       ...
///     }
/// 
/// Note how the version in the path pattern changed from `v1` to `v2`.
/// 
/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:
/// 
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///         root: acls
/// 
/// This implies the following inherited HTTP annotation:
/// 
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///       }
///       ...
///     }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Mixin {
    /// If non-empty specifies a path under which inherited HTTP paths
    /// are rooted.
    pub root: Option<String>,
    /// The fully qualified name of the interface which is included.
    pub name: Option<String>,
}

impl Part for Mixin {}


/// An object that describes the schema of a MonitoredResource object using a
/// type name and a set of labels.  For example, the monitored resource
/// descriptor for Google Compute Engine VM instances has a type of
/// `"gce_instance"` and specifies the use of the labels `"instance_id"` and
/// `"zone"` to identify particular VM instances.
/// 
/// Different APIs can support different monitored resource types. APIs generally
/// provide a `list` method that returns the monitored resource descriptors used
/// by the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitoredResourceDescriptor {
    /// Optional. A concise name for the monitored resource type that might be
    /// displayed in user interfaces. It should be a Title Cased Noun Phrase,
    /// without any article or other determiners. For example,
    /// `"Google Cloud SQL Database"`.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Required. A set of labels used to describe instances of this monitored
    /// resource type. For example, an individual Google Cloud SQL database is
    /// identified by values for the labels `"database_id"` and `"zone"`.
    pub labels: Option<Vec<LabelDescriptor>>,
    /// Required. The monitored resource type. For example, the type
    /// `"cloudsql_database"` represents databases in Google Cloud SQL.
    /// The maximum length of this value is 256 characters.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Optional. The resource name of the monitored resource descriptor:
    /// `"projects/{project_id}/monitoredResourceDescriptors/{type}"` where
    /// {type} is the value of the `type` field in this object and
    /// {project_id} is a project ID that provides API-specific context for
    /// accessing the type.  APIs that do not use project information can use the
    /// resource name format `"monitoredResourceDescriptors/{type}"`.
    pub name: Option<String>,
    /// Optional. A detailed description of the monitored resource type that might
    /// be used in documentation.
    pub description: Option<String>,
}

impl Part for MonitoredResourceDescriptor {}


/// Describes policy settings that need to be applied to a newly
/// created Tenant Project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TenantProjectPolicy {
    /// Additional policy bindings to be applied on the tenant
    /// project.
    /// At least one owner must be set in the bindings. Among the list of members
    /// as owners, at least one of them must be either `user` or `group` based.
    #[serde(rename="policyBindings")]
    pub policy_bindings: Option<Vec<PolicyBinding>>,
}

impl Part for TenantProjectPolicy {}


/// A description of a log type. Example in YAML format:
/// 
///     - name: library.googleapis.com/activity_history
///       description: The history of borrowing and returning library items.
///       display_name: Activity
///       labels:
///       - key: /customer_id
///         description: Identifier of a library customer
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LogDescriptor {
    /// The set of labels that are available to describe a specific log entry.
    /// Runtime requests that contain labels not specified here are
    /// considered invalid.
    pub labels: Option<Vec<LabelDescriptor>>,
    /// The human-readable name for this log. This information appears on
    /// the user interface and should be concise.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// A human-readable description of this log. This information appears in
    /// the documentation and can contain details.
    pub description: Option<String>,
    /// The name of the log. It must be less than 512 characters long and can
    /// include the following characters: upper- and lower-case alphanumeric
    /// characters [A-Za-z0-9], and punctuation characters including
    /// slash, underscore, hyphen, period [/_-.].
    pub name: Option<String>,
}

impl Part for LogDescriptor {}


/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// - Simple to use and understand for most users
/// - Flexible enough to meet unexpected needs
/// 
/// # Overview
/// 
/// The `Status` message contains three pieces of data: error code, error message,
/// and error details. The error code should be an enum value of
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


/// `Endpoint` describes a network endpoint that serves a set of APIs.
/// A service may expose any number of endpoints, and all endpoints share the
/// same service configuration, such as quota configuration and monitoring
/// configuration.
/// 
/// Example service configuration:
/// 
///     name: library-example.googleapis.com
///     endpoints:
///       # Below entry makes 'google.example.library.v1.Library'
///       # API be served from endpoint address library-example.googleapis.com.
///       # It also allows HTTP OPTIONS calls to be passed to the backend, for
///       # it to decide whether the subsequent cross-origin request is
///       # allowed to proceed.
///     - name: library-example.googleapis.com
///       allow_cors: true
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    /// Allowing
    /// [CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing), aka
    /// cross-domain traffic, would allow the backends served from this endpoint to
    /// receive and respond to HTTP OPTIONS requests. The response will be used by
    /// the browser to determine whether the subsequent cross-origin request is
    /// allowed to proceed.
    #[serde(rename="allowCors")]
    pub allow_cors: Option<bool>,
    /// DEPRECATED: This field is no longer supported. Instead of using aliases,
    /// please specify multiple google.api.Endpoint for each of the intended
    /// aliases.
    /// 
    /// Additional names that this endpoint will be hosted on.
    pub aliases: Option<Vec<String>>,
    /// The list of features enabled on this endpoint.
    pub features: Option<Vec<String>>,
    /// The canonical name of this endpoint.
    pub name: Option<String>,
    /// The specification of an Internet routable address of API frontend that will
    /// handle requests to this [API Endpoint](https://cloud.google.com/apis/design/glossary).
    /// It should be either a valid IPv4 address or a fully-qualified domain name.
    /// For example, "8.8.8.8" or "myservice.appspot.com".
    pub target: Option<String>,
}

impl Part for Endpoint {}


/// Logging configuration of the service.
/// 
/// The following example shows how to configure logs to be sent to the
/// producer and consumer projects. In the example, the `activity_history`
/// log is sent to both the producer and consumer projects, whereas the
/// `purchase_history` log is only sent to the producer project.
/// 
///     monitored_resources:
///     - type: library.googleapis.com/branch
///       labels:
///       - key: /city
///         description: The city where the library branch is located in.
///       - key: /name
///         description: The name of the branch.
///     logs:
///     - name: activity_history
///       labels:
///       - key: /customer_id
///     - name: purchase_history
///     logging:
///       producer_destinations:
///       - monitored_resource: library.googleapis.com/branch
///         logs:
///         - activity_history
///         - purchase_history
///       consumer_destinations:
///       - monitored_resource: library.googleapis.com/branch
///         logs:
///         - activity_history
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Logging {
    /// Logging configurations for sending logs to the producer project.
    /// There can be multiple producer destinations, each one must have a
    /// different monitored resource type. A log can be used in at most
    /// one producer destination.
    #[serde(rename="producerDestinations")]
    pub producer_destinations: Option<Vec<LoggingDestination>>,
    /// Logging configurations for sending logs to the consumer project.
    /// There can be multiple consumer destinations, each one must have a
    /// different monitored resource type. A log can be used in at most
    /// one consumer destination.
    #[serde(rename="consumerDestinations")]
    pub consumer_destinations: Option<Vec<LoggingDestination>>,
}

impl Part for Logging {}


/// `Documentation` provides the information for describing a service.
/// 
/// Example:
/// <pre><code>documentation:
///   summary: >
///     The Google Calendar API gives access
///     to most calendar features.
///   pages:
///   - name: Overview
///     content: &#40;== include google/foo/overview.md ==&#41;
///   - name: Tutorial
///     content: &#40;== include google/foo/tutorial.md ==&#41;
///     subpages;
///     - name: Java
///       content: &#40;== include google/foo/tutorial_java.md ==&#41;
///   rules:
///   - selector: google.calendar.Calendar.Get
///     description: >
///       ...
///   - selector: google.calendar.Calendar.Put
///     description: >
///       ...
/// </code></pre>
/// Documentation is provided in markdown syntax. In addition to
/// standard markdown features, definition lists, tables and fenced
/// code blocks are supported. Section headers can be provided and are
/// interpreted relative to the section nesting of the context where
/// a documentation fragment is embedded.
/// 
/// Documentation from the IDL is merged with documentation defined
/// via the config at normalization time, where documentation provided
/// by config rules overrides IDL provided.
/// 
/// A number of constructs specific to the API platform are supported
/// in documentation text.
/// 
/// In order to reference a proto element, the following
/// notation can be used:
/// <pre><code>&#91;fully.qualified.proto.name]&#91;]</code></pre>
/// To override the display text used for the link, this can be used:
/// <pre><code>&#91;display text]&#91;fully.qualified.proto.name]</code></pre>
/// Text can be excluded from doc using the following notation:
/// <pre><code>&#40;-- internal comment --&#41;</code></pre>
/// Comments can be made conditional using a visibility label. The below
/// text will be only rendered if the `BETA` label is available:
/// <pre><code>&#40;--BETA: comment for BETA users --&#41;</code></pre>
/// A few directives are available in documentation. Note that
/// directives must appear on a single line to be properly
/// identified. The `include` directive includes a markdown file from
/// an external source:
/// <pre><code>&#40;== include path/to/file ==&#41;</code></pre>
/// The `resource_for` directive marks a message to be the resource of
/// a collection in REST view. If it is not specified, tools attempt
/// to infer the resource from the operations in a collection:
/// <pre><code>&#40;== resource_for v1.shelves.books ==&#41;</code></pre>
/// The directive `suppress_warning` does not directly affect documentation
/// and is documented together with service config validation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Documentation {
    /// A list of documentation rules that apply to individual API elements.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<DocumentationRule>>,
    /// The URL to the root of documentation.
    #[serde(rename="documentationRootUrl")]
    pub documentation_root_url: Option<String>,
    /// A short summary of what the service does. Can only be provided by
    /// plain text.
    pub summary: Option<String>,
    /// The top level pages for the documentation set.
    pub pages: Option<Vec<Page>>,
    /// Declares a single overview page. For example:
    /// <pre><code>documentation:
    ///   summary: ...
    ///   overview: &#40;== include overview.md ==&#41;
    /// </code></pre>
    /// This is a shortcut for the following declaration (using pages style):
    /// <pre><code>documentation:
    ///   summary: ...
    ///   pages:
    ///   - name: Overview
    ///     content: &#40;== include overview.md ==&#41;
    /// </code></pre>
    /// Note: you cannot specify both `overview` field and `pages` field.
    pub overview: Option<String>,
}

impl Part for Documentation {}


/// User-defined authentication requirements, including support for
/// [JSON Web Token (JWT)](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthRequirement {
    /// id from authentication provider.
    /// 
    /// Example:
    /// 
    ///     provider_id: bookstore_auth
    #[serde(rename="providerId")]
    pub provider_id: Option<String>,
    /// NOTE: This will be deprecated soon, once AuthProvider.audiences is
    /// implemented and accepted in all the runtime components.
    /// 
    /// The list of JWT
    /// [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3).
    /// that are allowed to access. A JWT containing any of these audiences will
    /// be accepted. When this setting is absent, only JWTs with audience
    /// "https://Service_name/API_name"
    /// will be accepted. For example, if no audiences are in the setting,
    /// LibraryService API will only accept JWTs with the following audience
    /// "https://library-example.googleapis.com/google.example.library.v1.LibraryService".
    /// 
    /// Example:
    /// 
    ///     audiences: bookstore_android.apps.googleusercontent.com,
    ///                bookstore_web.apps.googleusercontent.com
    pub audiences: Option<String>,
}

impl Part for AuthRequirement {}


/// `Context` defines which contexts an API requests.
/// 
/// Example:
/// 
///     context:
///       rules:
///       - selector: "*"
///         requested:
///         - google.rpc.context.ProjectContext
///         - google.rpc.context.OriginContext
/// 
/// The above specifies that all methods in the API request
/// `google.rpc.context.ProjectContext` and
/// `google.rpc.context.OriginContext`.
/// 
/// Available context types are defined in package
/// `google.rpc.context`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    /// A list of RPC context rules that apply to individual API methods.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<ContextRule>>,
}

impl Part for Context {}


/// Authorization rule for API services.
/// 
/// It specifies the permission(s) required for an API element for the overall
/// API request to succeed. It is typically used to mark request message fields
/// that contain the name of the resource and indicates the permissions that
/// will be checked on that resource.
/// 
/// For example:
/// 
///     package google.storage.v1;
/// 
///     message CopyObjectRequest {
///       string source = 1 [
///         (google.api.authz).permissions = "storage.objects.get"];
/// 
///       string destination = 2 [
///         (google.api.authz).permissions =
///             "storage.objects.create,storage.objects.update"];
///     }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationRule {
    /// Selects the API elements to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
    /// The required permissions. The acceptable values vary depend on the
    /// authorization system used. For Google APIs, it should be a comma-separated
    /// Google IAM permission values. When multiple permissions are listed, the
    /// semantics is not defined by the system. Additional documentation must
    /// be provided manually.
    pub permissions: Option<String>,
}

impl Part for AuthorizationRule {}


/// Represents a documentation page. A page can contain subpages to represent
/// nested documentation set structure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    /// The Markdown content of the page. You can use <code>&#40;== include {path} ==&#41;</code>
    /// to include content from a Markdown file.
    pub content: Option<String>,
    /// Subpages of this page. The order of subpages specified here will be
    /// honored in the generated docset.
    pub subpages: Option<Vec<Page>>,
    /// The name of the page. It will be used as an identity of the page to
    /// generate URI of the page, text of the link to this page in navigation,
    /// etc. The full page name (start from the root page name to this page
    /// concatenated with `.`) can be used as reference to the page in your
    /// documentation. For example:
    /// <pre><code>pages:
    /// - name: Tutorial
    ///   content: &#40;== include tutorial.md ==&#41;
    ///   subpages:
    ///   - name: Java
    ///     content: &#40;== include tutorial_java.md ==&#41;
    /// </code></pre>
    /// You can reference `Java` page using Markdown reference link syntax:
    /// `Java`.
    pub name: Option<String>,
}

impl Part for Page {}


/// A description of a label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelDescriptor {
    /// The type of data that can be assigned to the label.
    #[serde(rename="valueType")]
    pub value_type: Option<String>,
    /// A human-readable description for the label.
    pub description: Option<String>,
    /// The label key.
    pub key: Option<String>,
}

impl Part for LabelDescriptor {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](struct.OperationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// A list of operations that matches the specified filter in the request.
    pub operations: Option<Vec<Operation>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl ResponseResult for ListOperationsResponse {}


/// Defines the Media configuration for a service in case of a download.
/// Use this only for Scotty Requests. Do not use this for media support using
/// Bytestream, add instead [][google.bytestream.RestByteStream] as an API to
/// your configuration for Bytestream methods.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaDownload {
    /// Optional maximum acceptable size for direct download.
    /// The size is specified in bytes.
    #[serde(rename="maxDirectDownloadSize")]
    pub max_direct_download_size: Option<String>,
    /// A boolean that determines if direct download from ESF should be used for
    /// download of this media.
    #[serde(rename="useDirectDownload")]
    pub use_direct_download: Option<bool>,
    /// Whether download is enabled.
    pub enabled: Option<bool>,
    /// A boolean that determines whether a notification for the completion of a
    /// download should be sent to the backend.
    #[serde(rename="completeNotification")]
    pub complete_notification: Option<bool>,
    /// Name of the Scotty dropzone to use for the current API.
    pub dropzone: Option<String>,
    /// DO NOT USE FIELDS BELOW THIS LINE UNTIL THIS WARNING IS REMOVED.
    /// 
    /// Specify name of the download service if one is used for download.
    #[serde(rename="downloadService")]
    pub download_service: Option<String>,
}

impl Part for MediaDownload {}


/// Usage configuration rules for the service.
/// 
/// NOTE: Under development.
/// 
/// 
/// Use this rule to configure unregistered calls for the service. Unregistered
/// calls are calls that do not contain consumer project identity.
/// (Example: calls that do not contain an API key).
/// By default, API methods do not allow unregistered calls, and each method call
/// must be identified by a consumer project identity. Use this rule to
/// allow/disallow unregistered calls.
/// 
/// Example of an API that wants to allow unregistered calls for entire service.
/// 
///     usage:
///       rules:
///       - selector: "*"
///         allow_unregistered_calls: true
/// 
/// Example of a method that wants to allow unregistered calls.
/// 
///     usage:
///       rules:
///       - selector: "google.example.library.v1.LibraryService.CreateBook"
///         allow_unregistered_calls: true
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageRule {
    /// Selects the methods to which this rule applies. Use '*' to indicate all
    /// methods in all APIs.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
    /// True, if the method should skip service control. If so, no control plane
    /// feature (like quota and billing) will be enabled.
    /// This flag is used by ESP to allow some Endpoints customers to bypass
    /// Google internal checks.
    #[serde(rename="skipServiceControl")]
    pub skip_service_control: Option<bool>,
    /// True, if the method allows unregistered calls; false otherwise.
    #[serde(rename="allowUnregisteredCalls")]
    pub allow_unregistered_calls: Option<bool>,
}

impl Part for UsageRule {}


/// Enum value definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnumValue {
    /// Enum value number.
    pub number: Option<i32>,
    /// Protocol buffer options.
    pub options: Option<Vec<Option>>,
    /// Enum value name.
    pub name: Option<String>,
}

impl Part for EnumValue {}


/// Api is a light-weight descriptor for an API Interface.
/// 
/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See https://cloud.google.com/apis/design/glossary for
/// detailed terminology.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Api {
    /// A version string for this interface. If specified, must have the form
    /// `major-version.minor-version`, as in `1.10`. If the minor version is
    /// omitted, it defaults to zero. If the entire version field is empty, the
    /// major version is derived from the package name, as outlined below. If the
    /// field is not empty, the version in the package name will be verified to be
    /// consistent with what is provided here.
    /// 
    /// The versioning schema uses [semantic
    /// versioning](http://semver.org) where the major version number
    /// indicates a breaking change and the minor version an additive,
    /// non-breaking change. Both version numbers are signals to users
    /// what to expect from different versions, and should be carefully
    /// chosen based on the product plan.
    /// 
    /// The major version is also reflected in the package name of the
    /// interface, which must end in `v<major-version>`, as in
    /// `google.feature.v1`. For major versions 0 and 1, the suffix can
    /// be omitted. Zero major versions must only be used for
    /// experimental, non-GA interfaces.
    /// 
    pub version: Option<String>,
    /// The methods of this interface, in unspecified order.
    pub methods: Option<Vec<Method>>,
    /// The fully qualified name of this interface, including package name
    /// followed by the interface's simple name.
    pub name: Option<String>,
    /// Source context for the protocol buffer service represented by this
    /// message.
    #[serde(rename="sourceContext")]
    pub source_context: Option<SourceContext>,
    /// Included interfaces. See Mixin.
    pub mixins: Option<Vec<Mixin>>,
    /// Any metadata attached to the interface.
    pub options: Option<Vec<Option>>,
    /// The source syntax of the service.
    pub syntax: Option<String>,
}

impl Part for Api {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](struct.OperationCancelCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl RequestValue for CancelOperationRequest {}


/// Authentication rules for the service.
/// 
/// By default, if a method has any authentication requirements, every request
/// must include a valid credential matching one of the requirements.
/// It's an error to include more than one kind of credential in a single
/// request.
/// 
/// If a method doesn't have any auth requirements, request credentials will be
/// ignored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationRule {
    /// The requirements for OAuth credentials.
    pub oauth: Option<OAuthRequirements>,
    /// Requirements for additional authentication providers.
    pub requirements: Option<Vec<AuthRequirement>>,
    /// Whether to allow requests without a credential. The credential can be
    /// an OAuth token, Google cookies (first-party auth) or EndUserCreds.
    /// 
    /// For requests without credentials, if the service control environment is
    /// specified, each incoming request **must** be associated with a service
    /// consumer. This can be done by passing an API key that belongs to a consumer
    /// project.
    #[serde(rename="allowWithoutCredential")]
    pub allow_without_credential: Option<bool>,
    /// Configuration for custom authentication.
    #[serde(rename="customAuth")]
    pub custom_auth: Option<CustomAuthRequirements>,
    /// Selects the methods to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for AuthenticationRule {}


/// `Backend` defines the backend configuration for a service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Backend {
    /// A list of API backend rules that apply to individual API methods.
    /// 
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    pub rules: Option<Vec<BackendRule>>,
}

impl Part for Backend {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the `ServiceConsumerManagement` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
}

impl<'a, C, A> MethodsBuilder for OperationMethods<'a, C, A> {}

impl<'a, C, A> OperationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is
    /// no longer interested in the operation result. It does not cancel the
    /// operation. If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn delete(&self, name: &str) -> OperationDeleteCall<'a, C, A> {
        OperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation.  The server
    /// makes a best effort to cancel the operation, but success is not
    /// guaranteed.  If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
    /// Operations.GetOperation or
    /// other methods to check whether the cancellation succeeded or whether the
    /// operation completed despite cancellation. On successful cancellation,
    /// the operation is not deleted; instead, it becomes an operation with
    /// an Operation.error value with a google.rpc.Status.code of 1,
    /// corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, request: CancelOperationRequest, name: &str) -> OperationCancelCall<'a, C, A> {
        OperationCancelCall {
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
    /// Lists operations that match the specified filter in the request. If the
    /// server doesn't support this method, it returns `UNIMPLEMENTED`.
    /// 
    /// NOTE: the `name` binding allows API services to override the binding
    /// to use different resource name schemes, such as `users/*/operations`. To
    /// override the binding, API services can add a binding such as
    /// `"/v1/{name=users/*}/operations"` to their service configuration.
    /// For backwards compatibility, the default name includes the operations
    /// collection id, however overriding users must ensure the name binding
    /// is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn list(&self, name: &str) -> OperationListCall<'a, C, A> {
        OperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, C, A> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *service* resources.
/// It is not used directly, but through the `ServiceConsumerManagement` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`, `tenancy_units_add_project(...)`, `tenancy_units_create(...)`, `tenancy_units_delete(...)`, `tenancy_units_list(...)` and `tenancy_units_remove_project(...)`
/// // to build up your call.
/// let rb = hub.services();
/// # }
/// ```
pub struct ServiceMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
}

impl<'a, C, A> MethodsBuilder for ServiceMethods<'a, C, A> {}

impl<'a, C, A> ServiceMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Find tenancy unit for a service and consumer.
    /// This method should not be used in producers' runtime path, e.g. finding
    /// the tenant project number when creating VMs. Producers should persist
    /// the tenant project information after the project is created.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Service and consumer. Required.
    ///              services/{service}/{collection id}/{resource id}
    ///              {collection id} is the cloud resource collection type representing the
    ///              service consumer, for example 'projects', or 'organizations'.
    ///              {resource id} is the consumer numeric id, such as project number: '123456'.
    ///              {service} the name of a service, for example 'service.googleapis.com'.
    pub fn tenancy_units_list(&self, parent: &str) -> ServiceTenancyUnitListCall<'a, C, A> {
        ServiceTenancyUnitListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add a new tenant project to the tenancy unit.
    /// If there are previously failed AddTenantProject calls, you might need to
    /// call RemoveTenantProject first to clean them before you can make another
    /// AddTenantProject with the same tag.
    /// Operation<response: Empty>.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Name of the tenancy unit.
    pub fn tenancy_units_add_project(&self, request: AddTenantProjectRequest, parent: &str) -> ServiceTenancyUnitAddProjectCall<'a, C, A> {
        ServiceTenancyUnitAddProjectCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a tenancy unit with no tenant resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - services/{service}/{collection id}/{resource id}
    ///              {collection id} is the cloud resource collection type representing the
    ///              service consumer, for example 'projects', or 'organizations'.
    ///              {resource id} is the consumer numeric id, such as project number: '123456'.
    ///              {service} the name of a service, for example 'service.googleapis.com'.
    ///              Enabled service binding using the new tenancy unit.
    pub fn tenancy_units_create(&self, request: CreateTenancyUnitRequest, parent: &str) -> ServiceTenancyUnitCreateCall<'a, C, A> {
        ServiceTenancyUnitCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes specified project resource identified by tenant resource tag.
    /// It will remove project lien with 'TenantManager' origin if that was added.
    /// It will then attempt to delete the project.
    /// If that operation fails, this method fails.
    /// Operation<response: Empty>.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the tenancy unit.
    ///            Such as 'services/service.googleapis.com/projects/12345/tenancyUnits/abcd'.
    pub fn tenancy_units_remove_project(&self, request: RemoveTenantProjectRequest, name: &str) -> ServiceTenancyUnitRemoveProjectCall<'a, C, A> {
        ServiceTenancyUnitRemoveProjectCall {
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
    /// Delete tenancy unit.  Before the tenancy unit is deleted, there should be
    /// no tenant resource in it.
    /// Operation<response: Empty>.
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the tenancy unit to be deleted.
    pub fn tenancy_units_delete(&self, name: &str) -> ServiceTenancyUnitDeleteCall<'a, C, A> {
        ServiceTenancyUnitDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search tenancy units for a service.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Service for which search is performed.
    ///              services/{service}
    ///              {service} the name of a service, for example 'service.googleapis.com'.
    pub fn search(&self, parent: &str) -> ServiceSearchCall<'a, C, A> {
        ServiceSearchCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Deletes a long-running operation. This method indicates that the client is
/// no longer interested in the operation result. It does not cancel the
/// operation. If the server doesn't support this method, it returns
/// `google.rpc.Code.UNIMPLEMENTED`.
///
/// A builder for the *delete* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().delete("name")
///              .doit();
/// # }
/// ```
pub struct OperationDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationDeleteCall<'a, C, A> {}

impl<'a, C, A> OperationDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.operations.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
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


    /// The name of the operation resource to be deleted.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OperationDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> OperationDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Starts asynchronous cancellation on a long-running operation.  The server
/// makes a best effort to cancel the operation, but success is not
/// guaranteed.  If the server doesn't support this method, it returns
/// `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
/// Operations.GetOperation or
/// other methods to check whether the cancellation succeeded or whether the
/// operation completed despite cancellation. On successful cancellation,
/// the operation is not deleted; instead, it becomes an operation with
/// an Operation.error value with a google.rpc.Status.code of 1,
/// corresponding to `Code.CANCELLED`.
///
/// A builder for the *cancel* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// use serviceconsumermanagement1::CancelOperationRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CancelOperationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().cancel(req, "name")
///              .doit();
/// # }
/// ```
pub struct OperationCancelCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _request: CancelOperationRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationCancelCall<'a, C, A> {}

impl<'a, C, A> OperationCancelCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.operations.cancel",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:cancel";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
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
    pub fn request(mut self, new_value: CancelOperationRequest) -> OperationCancelCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The name of the operation resource to be cancelled.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationCancelCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OperationCancelCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> OperationCancelCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationCancelCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists operations that match the specified filter in the request. If the
/// server doesn't support this method, it returns `UNIMPLEMENTED`.
/// 
/// NOTE: the `name` binding allows API services to override the binding
/// to use different resource name schemes, such as `users/*/operations`. To
/// override the binding, API services can add a binding such as
/// `"/v1/{name=users/*}/operations"` to their service configuration.
/// For backwards compatibility, the default name includes the operations
/// collection id, however overriding users must ensure the name binding
/// is the parent resource, without the operations collection id.
///
/// A builder for the *list* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().list("name")
///              .page_token("dolores")
///              .page_size(-61)
///              .filter("sadipscing")
///              .doit();
/// # }
/// ```
pub struct OperationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationListCall<'a, C, A> {}

impl<'a, C, A> OperationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListOperationsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.operations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "name", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
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


    /// The name of the operation's parent resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The standard list page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OperationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The standard list page size.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> OperationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The standard list filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> OperationListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OperationListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> OperationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the latest state of a long-running operation.  Clients can use this
/// method to poll the operation result at intervals as recommended by the API
/// service.
///
/// A builder for the *get* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().get("name")
///              .doit();
/// # }
/// ```
pub struct OperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationGetCall<'a, C, A> {}

impl<'a, C, A> OperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.operations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OperationGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> OperationGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Find tenancy unit for a service and consumer.
/// This method should not be used in producers' runtime path, e.g. finding
/// the tenant project number when creating VMs. Producers should persist
/// the tenant project information after the project is created.
///
/// A builder for the *tenancyUnits.list* method supported by a *service* resource.
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
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().tenancy_units_list("parent")
///              .page_token("no")
///              .page_size(-21)
///              .filter("justo")
///              .doit();
/// # }
/// ```
pub struct ServiceTenancyUnitListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceTenancyUnitListCall<'a, C, A> {}

impl<'a, C, A> ServiceTenancyUnitListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListTenancyUnitsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.services.tenancyUnits.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/tenancyUnits";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
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


    /// Service and consumer. Required.
    /// services/{service}/{collection id}/{resource id}
    /// {collection id} is the cloud resource collection type representing the
    /// service consumer, for example 'projects', or 'organizations'.
    /// {resource id} is the consumer numeric id, such as project number: '123456'.
    /// {service} the name of a service, for example 'service.googleapis.com'.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ServiceTenancyUnitListCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ServiceTenancyUnitListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results returned by this request.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ServiceTenancyUnitListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter expression over tenancy resources field. Optional.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> ServiceTenancyUnitListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceTenancyUnitListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceTenancyUnitListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceTenancyUnitListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Add a new tenant project to the tenancy unit.
/// If there are previously failed AddTenantProject calls, you might need to
/// call RemoveTenantProject first to clean them before you can make another
/// AddTenantProject with the same tag.
/// Operation<response: Empty>.
///
/// A builder for the *tenancyUnits.addProject* method supported by a *service* resource.
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
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// use serviceconsumermanagement1::AddTenantProjectRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AddTenantProjectRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().tenancy_units_add_project(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ServiceTenancyUnitAddProjectCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _request: AddTenantProjectRequest,
    _parent: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceTenancyUnitAddProjectCall<'a, C, A> {}

impl<'a, C, A> ServiceTenancyUnitAddProjectCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.services.tenancyUnits.addProject",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:addProject";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
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
    pub fn request(mut self, new_value: AddTenantProjectRequest) -> ServiceTenancyUnitAddProjectCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the tenancy unit.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ServiceTenancyUnitAddProjectCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceTenancyUnitAddProjectCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceTenancyUnitAddProjectCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceTenancyUnitAddProjectCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a tenancy unit with no tenant resources.
///
/// A builder for the *tenancyUnits.create* method supported by a *service* resource.
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
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// use serviceconsumermanagement1::CreateTenancyUnitRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateTenancyUnitRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().tenancy_units_create(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ServiceTenancyUnitCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _request: CreateTenancyUnitRequest,
    _parent: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceTenancyUnitCreateCall<'a, C, A> {}

impl<'a, C, A> ServiceTenancyUnitCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TenancyUnit)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.services.tenancyUnits.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/tenancyUnits";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
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
    pub fn request(mut self, new_value: CreateTenancyUnitRequest) -> ServiceTenancyUnitCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// services/{service}/{collection id}/{resource id}
    /// {collection id} is the cloud resource collection type representing the
    /// service consumer, for example 'projects', or 'organizations'.
    /// {resource id} is the consumer numeric id, such as project number: '123456'.
    /// {service} the name of a service, for example 'service.googleapis.com'.
    /// Enabled service binding using the new tenancy unit.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ServiceTenancyUnitCreateCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceTenancyUnitCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceTenancyUnitCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceTenancyUnitCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Removes specified project resource identified by tenant resource tag.
/// It will remove project lien with 'TenantManager' origin if that was added.
/// It will then attempt to delete the project.
/// If that operation fails, this method fails.
/// Operation<response: Empty>.
///
/// A builder for the *tenancyUnits.removeProject* method supported by a *service* resource.
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
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// use serviceconsumermanagement1::RemoveTenantProjectRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RemoveTenantProjectRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().tenancy_units_remove_project(req, "name")
///              .doit();
/// # }
/// ```
pub struct ServiceTenancyUnitRemoveProjectCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _request: RemoveTenantProjectRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceTenancyUnitRemoveProjectCall<'a, C, A> {}

impl<'a, C, A> ServiceTenancyUnitRemoveProjectCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.services.tenancyUnits.removeProject",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:removeProject";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
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
    pub fn request(mut self, new_value: RemoveTenantProjectRequest) -> ServiceTenancyUnitRemoveProjectCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the tenancy unit.
    /// Such as 'services/service.googleapis.com/projects/12345/tenancyUnits/abcd'.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ServiceTenancyUnitRemoveProjectCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceTenancyUnitRemoveProjectCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceTenancyUnitRemoveProjectCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceTenancyUnitRemoveProjectCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Delete tenancy unit.  Before the tenancy unit is deleted, there should be
/// no tenant resource in it.
/// Operation<response: Empty>.
///
/// A builder for the *tenancyUnits.delete* method supported by a *service* resource.
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
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().tenancy_units_delete("name")
///              .doit();
/// # }
/// ```
pub struct ServiceTenancyUnitDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceTenancyUnitDeleteCall<'a, C, A> {}

impl<'a, C, A> ServiceTenancyUnitDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.services.tenancyUnits.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
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

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
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


    /// Name of the tenancy unit to be deleted.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ServiceTenancyUnitDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceTenancyUnitDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceTenancyUnitDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceTenancyUnitDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Search tenancy units for a service.
///
/// A builder for the *search* method supported by a *service* resource.
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
/// # extern crate google_serviceconsumermanagement1 as serviceconsumermanagement1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use serviceconsumermanagement1::ServiceConsumerManagement;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceConsumerManagement::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().search("parent")
///              .query("et")
///              .page_token("duo")
///              .page_size(-32)
///              .doit();
/// # }
/// ```
pub struct ServiceSearchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceConsumerManagement<C, A>,
    _parent: String,
    _query: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceSearchCall<'a, C, A> {}

impl<'a, C, A> ServiceSearchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SearchTenancyUnitsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "serviceconsumermanagement.services.search",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._query {
            params.push(("query", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "parent", "query", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:search";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
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


    /// Service for which search is performed.
    /// services/{service}
    /// {service} the name of a service, for example 'service.googleapis.com'.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ServiceSearchCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Set a query `{expression}` for querying tenancy units. Your `{expression}`
    /// must be in the format: `field_name=literal_string`. The `field_name` is the
    /// name of the field you want to compare. Supported fields are
    /// `tenant_resources.tag` and`tenant_resources.resource`.
    /// 
    /// For example, to search tenancy units that contain at least one tenant
    /// resource with given tag 'xyz', use query `tenant_resources.tag=xyz`.
    /// To search tenancy units that contain at least one tenant resource with
    /// given resource name 'projects/123456', use query
    /// `tenant_resources.resource=projects/123456`.
    /// 
    /// Multiple expressions can be joined with `AND`s. Tenancy units must match
    /// all expressions to be included in the result set. For example,
    /// `tenant_resources.tag=xyz AND tenant_resources.resource=projects/123456`
    /// 
    /// Optional.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> ServiceSearchCall<'a, C, A> {
        self._query = Some(new_value.to_string());
        self
    }
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    /// 
    /// Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ServiceSearchCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results returned by this request. Currently, the
    /// default maximum is set to 1000. If page_size is not provided or provided a
    /// number larger than 1000, it will be automatically set to 1000.
    /// 
    /// Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ServiceSearchCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceSearchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceSearchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceSearchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


