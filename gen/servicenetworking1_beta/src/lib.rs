// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Service Networking* crate version *1.0.7+20181011*, where *20181011* is the exact revision of the *servicenetworking:v1beta* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.7*.
//! 
//! Everything else about the *Service Networking* *v1_beta* API can be found at the
//! [official documentation site](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/servicenetworking1_beta).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.ServiceNetworking.html) ... 
//! 
//! * [operations](struct.Operation.html)
//!  * [*get*](struct.OperationGetCall.html)
//! * [services](struct.Service.html)
//!  * [*add subnetwork*](struct.ServiceAddSubnetworkCall.html), [*connections create*](struct.ServiceConnectionCreateCall.html) and [*connections list*](struct.ServiceConnectionListCall.html)
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
//! * **[Hub](struct.ServiceNetworking.html)**
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
//! let r = hub.services().connections_create(...).doit()
//! let r = hub.services().connections_list(...).doit()
//! let r = hub.services().add_subnetwork(...).doit()
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
//! google-servicenetworking1_beta = "*"
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
//! extern crate google_servicenetworking1_beta as servicenetworking1_beta;
//! use servicenetworking1_beta::Connection;
//! use servicenetworking1_beta::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use servicenetworking1_beta::ServiceNetworking;
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
//! let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Connection::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.services().connections_create(req, "parent")
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
//! All structures provided by this library are made to be [encodable](trait.RequestValue.html) and 
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

/// Central instance to access all ServiceNetworking related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_servicenetworking1_beta as servicenetworking1_beta;
/// use servicenetworking1_beta::Connection;
/// use servicenetworking1_beta::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use servicenetworking1_beta::ServiceNetworking;
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
/// let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Connection::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().connections_create(req, "parent")
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
pub struct ServiceNetworking<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for ServiceNetworking<C, A> {}

impl<'a, C, A> ServiceNetworking<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> ServiceNetworking<C, A> {
        ServiceNetworking {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.7".to_string(),
            _base_url: "https://servicenetworking.googleapis.com/".to_string(),
            _root_url: "https://servicenetworking.googleapis.com/".to_string(),
        }
    }

    pub fn operations(&'a self) -> OperationMethods<'a, C, A> {
        OperationMethods { hub: &self }
    }
    pub fn services(&'a self) -> ServiceMethods<'a, C, A> {
        ServiceMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.7`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://servicenetworking.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://servicenetworking.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
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


/// ListConnectionsResponse is the response to list peering states for the
/// given service and consumer project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [connections list services](struct.ServiceConnectionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectionsResponse {
    /// The list of Connections.
    pub connections: Option<Vec<Connection>>,
}

impl ResponseResult for ListConnectionsResponse {}


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


/// Request to create a subnetwork in a previously peered service network.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add subnetwork services](struct.ServiceAddSubnetworkCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSubnetworkRequest {
    /// Optional. Description of the subnetwork.
    pub description: Option<String>,
    /// Required. The prefix length of the IP range.
    /// Use usual CIDR range notation.
    /// For example, '30' to provision subnet with x.x.x.x/30 CIDR range.
    /// Actual range will be determined using allocated range for the consumer
    /// peered network and returned in the result.
    #[serde(rename="ipPrefixLength")]
    pub ip_prefix_length: Option<i32>,
    /// Required. Network name in the consumer project.   This network must have been
    /// already peered with a shared VPC network using CreateConnection
    /// method.
    /// Must be in a form 'projects/{project}/global/networks/{network}'.
    /// {project} is a project number, as in '12345'
    /// {network} is network name.
    #[serde(rename="consumerNetwork")]
    pub consumer_network: Option<String>,
    /// Required. Cloud [region](/compute/docs/reference/rest/v1/regions) for the new
    /// subnetwork.
    pub region: Option<String>,
    /// Required. Resource representing service consumer. It may be different from
    /// the project number in consumer network parameter in case of that network
    /// being a shared VPC network. In that case, Service Networking will validate
    /// that this resource belongs to that shared VPC.
    /// For example 'projects/123456'.
    pub consumer: Option<String>,
    /// Required. Name for the new subnetwork.
    /// Must be a legal [subnetwork](compute/docs/reference/rest/v1/subnetworks)
    /// name.
    pub subnetwork: Option<String>,
    /// Optional. List of members that will be granted 'compute.networkUser' role
    /// on the newly added subnetwork.
    #[serde(rename="subnetworkUsers")]
    pub subnetwork_users: Option<Vec<String>>,
}

impl RequestValue for AddSubnetworkRequest {}


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


/// # gRPC Transcoding
/// 
/// gRPC Transcoding is a feature for mapping between a gRPC method and one or
/// more HTTP REST endpoints. It allows developers to build a single API service
/// that supports both gRPC APIs and REST APIs. Many systems, including [Google
/// APIs](https://github.com/googleapis/googleapis),
/// [Cloud Endpoints](https://cloud.google.com/endpoints), [gRPC
/// Gateway](https://github.com/grpc-ecosystem/grpc-gateway),
/// and [Envoy](https://github.com/envoyproxy/envoy) proxy support this feature
/// and use it for large scale production services.
/// 
/// `HttpRule` defines the schema of the gRPC/REST mapping. The mapping specifies
/// how different portions of the gRPC request message are mapped to the URL
/// path, URL query parameters, and HTTP request body. It also controls how the
/// gRPC response message is mapped to the HTTP response body. `HttpRule` is
/// typically specified as an `google.api.http` annotation on the gRPC method.
/// 
/// Each mapping specifies a URL path template and an HTTP method. The path
/// template may refer to one or more fields in the gRPC request message, as long
/// as each field is a non-repeated field with a primitive (non-message) type.
/// The path template controls how fields of the request message are mapped to
/// the URL path.
/// 
/// Example:
/// 
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http) = {
///             get: "/v1/{name=messages/*}"
///         };
///       }
///     }
///     message GetMessageRequest {
///       string name = 1; // Mapped to URL path.
///     }
///     message Message {
///       string text = 1; // The resource content.
///     }
/// 
/// This enables an HTTP REST to gRPC mapping as below:
/// 
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456`  | `GetMessage(name: "messages/123456")`
/// 
/// Any fields in the request message which are not bound by the path template
/// automatically become HTTP query parameters if there is no HTTP request body.
/// For example:
/// 
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http) = {
///             get:"/v1/messages/{message_id}"
///         };
///       }
///     }
///     message GetMessageRequest {
///       message SubMessage {
///         string subfield = 1;
///       }
///       string message_id = 1; // Mapped to URL path.
///       int64 revision = 2;    // Mapped to URL query parameter `revision`.
///       SubMessage sub = 3;    // Mapped to URL query parameter `sub.subfield`.
///     }
/// 
/// This enables a HTTP JSON to RPC mapping as below:
/// 
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456?revision=2&sub.subfield=foo` | `GetMessage(message_id: "123456" revision: 2 sub: SubMessage(subfield: "foo"))`
/// 
/// Note that fields which are mapped to URL query parameters must have a
/// primitive type or a repeated primitive type or a non-repeated message type.
/// In the case of a repeated type, the parameter can be repeated in the URL
/// as `...?param=A&param=B`. In the case of a message type, each field of the
/// message is mapped to a separate parameter, such as
/// `...?foo.a=A&foo.b=B&foo.c=C`.
/// 
/// For HTTP methods that allow a request body, the `body` field
/// specifies the mapping. Consider a REST update method on the
/// message resource collection:
/// 
///     service Messaging {
///       rpc UpdateMessage(UpdateMessageRequest) returns (Message) {
///         option (google.api.http) = {
///           patch: "/v1/messages/{message_id}"
///           body: "message"
///         };
///       }
///     }
///     message UpdateMessageRequest {
///       string message_id = 1; // mapped to the URL
///       Message message = 2;   // mapped to the body
///     }
/// 
/// The following HTTP JSON to RPC mapping is enabled, where the
/// representation of the JSON in the request body is determined by
/// protos JSON encoding:
/// 
/// HTTP | gRPC
/// -----|-----
/// `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id: "123456" message { text: "Hi!" })`
/// 
/// The special name `*` can be used in the body mapping to define that
/// every field not bound by the path template should be mapped to the
/// request body.  This enables the following alternative definition of
/// the update method:
/// 
///     service Messaging {
///       rpc UpdateMessage(Message) returns (Message) {
///         option (google.api.http) = {
///           patch: "/v1/messages/{message_id}"
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
/// HTTP | gRPC
/// -----|-----
/// `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id: "123456" text: "Hi!")`
/// 
/// Note that when using `*` in the body mapping, it is not possible to
/// have HTTP parameters, as all fields not bound by the path end in
/// the body. This makes this option more rarely used in practice when
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
/// This enables the following two alternative HTTP JSON to RPC mappings:
/// 
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456` | `GetMessage(message_id: "123456")`
/// `GET /v1/users/me/messages/123456` | `GetMessage(user_id: "me" message_id: "123456")`
/// 
/// ## Rules for HTTP mapping
/// 
/// 1. Leaf request fields (recursive expansion nested messages in the request
///    message) are classified into three categories:
///    - Fields referred by the path template. They are passed via the URL path.
///    - Fields referred by the HttpRule.body. They are passed via the HTTP
///      request body.
///    - All other fields are passed via the URL query parameters, and the
///      parameter name is the field path in the request message. A repeated
///      field can be represented as multiple query parameters under the same
///      name.
///  2. If HttpRule.body is "*", there is no URL query parameter, all fields
///     are passed via URL path and HTTP request body.
///  3. If HttpRule.body is omitted, there is no HTTP request body, all
///     fields are passed via URL path and URL query parameters.
/// 
/// ### Path template syntax
/// 
///     Template = "/" Segments [ Verb ] ;
///     Segments = Segment { "/" Segment } ;
///     Segment  = "*" | "**" | LITERAL | Variable ;
///     Variable = "{" FieldPath [ "=" Segments ] "}" ;
///     FieldPath = IDENT { "." IDENT } ;
///     Verb     = ":" LITERAL ;
/// 
/// The syntax `*` matches a single URL path segment. The syntax `**` matches
/// zero or more URL path segments, which must be the last part of the URL path
/// except the `Verb`.
/// 
/// The syntax `Variable` matches part of the URL path as specified by its
/// template. A variable template must not contain other variables. If a variable
/// matches a single path segment, its template may be omitted, e.g. `{var}`
/// is equivalent to `{var=*}`.
/// 
/// The syntax `LITERAL` matches literal text in the URL path. If the `LITERAL`
/// contains any reserved character, such characters should be percent-encoded
/// before the matching.
/// 
/// If a variable contains exactly one path segment, such as `"{var}"` or
/// `"{var=*}"`, when such a variable is expanded into a URL path on the client
/// side, all characters except `[-_.~0-9a-zA-Z]` are percent-encoded. The
/// server side does the reverse decoding. Such variables show up in the
/// [Discovery Document](https://developers.google.com/discovery/v1/reference/apis)
/// as `{var}`.
/// 
/// If a variable contains multiple path segments, such as `"{var=foo/*}"`
/// or `"{var=**}"`, when such a variable is expanded into a URL path on the
/// client side, all characters except `[-_.~/0-9a-zA-Z]` are percent-encoded.
/// The server side does the reverse decoding, except "%2F" and "%2f" are left
/// unchanged. Such variables show up in the
/// [Discovery Document](https://developers.google.com/discovery/v1/reference/apis)
/// as `{+var}`.
/// 
/// ## Using gRPC API Service Configuration
/// 
/// gRPC API Service Configuration (service config) is a configuration language
/// for configuring a gRPC service to become a user-facing product. The
/// service config is simply the YAML representation of the `google.api.Service`
/// proto message.
/// 
/// As an alternative to annotating your proto file, you can configure gRPC
/// transcoding in your service config YAML files. You do this by specifying a
/// `HttpRule` that maps the gRPC method to a REST endpoint, achieving the same
/// effect as the proto annotation. This can be particularly useful if you
/// have a proto that is reused in multiple services. Note that any transcoding
/// specified in the service config will override any matching transcoding
/// configuration in the proto.
/// 
/// Example:
/// 
///     http:
///       rules:
///         # Selects a gRPC method and applies HttpRule to it.
///         - selector: example.v1.Messaging.GetMessage
///           get: /v1/messages/{message_id}/{sub.subfield}
/// 
/// ## Special notes
/// 
/// When gRPC Transcoding is used to map a gRPC to JSON REST endpoints, the
/// proto to JSON conversion must follow the [proto3
/// specification](https://developers.google.com/protocol-buffers/docs/proto3#json).
/// 
/// While the single segment variable follows the semantics of
/// [RFC 6570](https://tools.ietf.org/html/rfc6570) Section 3.2.2 Simple String
/// Expansion, the multi segment variable **does not** follow RFC 6570 Section
/// 3.2.3 Reserved Expansion. The reason is that the Reserved Expansion
/// does not expand special characters like `?` and `#`, which would lead
/// to invalid URLs. As the result, gRPC Transcoding uses a custom encoding
/// for multi segment variables.
/// 
/// The path variables **must not** refer to any repeated or mapped field,
/// because client libraries are not capable of handling such variable expansion.
/// 
/// The path variables **must not** capture the leading "/" character. The reason
/// is that the most common use case "{var}" does not capture the leading "/"
/// character. For consistency, all path variables must share the same behavior.
/// 
/// Repeated message fields must not be mapped to URL query parameters, because
/// no client library can support such complicated mapping.
/// 
/// If an API needs to use a JSON array for request or response body, it can map
/// the request or response body to a repeated field. However, some gRPC
/// Transcoding implementations may not support this feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpRule {
    /// The name of the request field whose value is mapped to the HTTP request
    /// body, or `*` for mapping all request fields not captured by the path
    /// pattern to the HTTP body, or omitted for not having any HTTP request body.
    /// 
    /// NOTE: the referred field must be present at the top-level of the request
    /// message type.
    pub body: Option<String>,
    /// The custom pattern is used for specifying an HTTP method that is not
    /// included in the `pattern` field, such as HEAD, or "*" to leave the
    /// HTTP method unspecified for this rule. The wild-card rule is useful
    /// for services that provide content to Web (HTML) clients.
    pub custom: Option<CustomHttpPattern>,
    /// Maps to HTTP GET. Used for listing and getting information about
    /// resources.
    pub get: Option<String>,
    /// Additional HTTP bindings for the selector. Nested bindings must
    /// not contain an `additional_bindings` field themselves (that is,
    /// the nesting may only be one level deep).
    #[serde(rename="additionalBindings")]
    pub additional_bindings: Option<Vec<HttpRule>>,
    /// Selects a method to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
    /// Optional. The name of the response field whose value is mapped to the HTTP
    /// response body. When omitted, the entire response message will be used
    /// as the HTTP response body.
    /// 
    /// NOTE: The referred field must be present at the top-level of the response
    /// message type.
    #[serde(rename="responseBody")]
    pub response_body: Option<String>,
    /// Maps to HTTP PUT. Used for replacing a resource.
    pub put: Option<String>,
    /// Maps to HTTP POST. Used for creating a resource or performing an action.
    pub post: Option<String>,
    /// Maps to HTTP PATCH. Used for updating a resource.
    pub patch: Option<String>,
    /// Maps to HTTP DELETE. Used for deleting a resource.
    pub delete: Option<String>,
}

impl Part for HttpRule {}


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
    /// Optional. Metadata which can be used to guide usage of the metric.
    pub metadata: Option<MetricDescriptorMetadata>,
    /// The metric type, including its DNS name prefix. The type is not
    /// URL-encoded.  All user-defined metric types have the DNS name
    /// `custom.googleapis.com` or `external.googleapis.com`.  Metric types should
    /// use a natural hierarchical grouping. For example:
    /// 
    ///     "custom.googleapis.com/invoice/paid/amount"
    ///     "external.googleapis.com/prometheus/up"
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
    /// The grammar also includes these connectors:
    /// 
    /// * `/`    division (as an infix operator, e.g. `1/s`).
    /// * `.`    multiplication (as an infix operator, e.g. `GBy.d`)
    /// 
    /// The grammar for a unit is as follows:
    /// 
    ///     Expression = Component { "." Component } { "/" Component } ;
    /// 
    ///     Component = ( [ PREFIX ] UNIT | "%" ) [ Annotation ]
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
    /// * `1` represents dimensionless value 1, such as in `1/s`.
    /// * `%` represents dimensionless value 1/100, and annotates values giving
    ///    a percentage.
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
/// * [connections create services](struct.ServiceConnectionCreateCall.html) (none)
/// * [connections list services](struct.ServiceConnectionListCall.html) (none)
/// * [add subnetwork services](struct.ServiceAddSubnetworkCall.html) (none)
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
    /// Additional API documentation.
    pub documentation: Option<Documentation>,
    /// Custom error configuration.
    #[serde(rename="customError")]
    pub custom_error: Option<CustomError>,
    /// Quota configuration.
    pub quota: Option<Quota>,
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
    /// generate one instead. Must be no longer than 60 characters.
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
    /// The product title for this service.
    pub title: Option<String>,
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


/// Additional annotations that can be used to guide the usage of a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricDescriptorMetadata {
    /// The sampling period of metric data points. For metrics which are written
    /// periodically, consecutive data points are stored at this time interval,
    /// excluding data loss due to errors. Metrics with a higher granularity have
    /// a smaller sampling period.
    #[serde(rename="samplePeriod")]
    pub sample_period: Option<String>,
    /// The delay of data points caused by ingestion. Data points older than this
    /// age are guaranteed to be ingested and available to be read, excluding
    /// data loss due to errors.
    #[serde(rename="ingestDelay")]
    pub ingest_delay: Option<String>,
    /// The launch stage of the metric definition.
    #[serde(rename="launchStage")]
    pub launch_stage: Option<String>,
}

impl Part for MetricDescriptorMetadata {}


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
///      limits:
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


/// Configuration of a specific billing destination (Currently only support
/// bill against consumer project).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BillingDestination {
    /// Names of the metrics to report to this billing destination.
    /// Each name must be defined in Service.metrics section.
    pub metrics: Option<Vec<String>>,
    /// The monitored resource type. The type must be defined in
    /// Service.monitored_resources section.
    #[serde(rename="monitoredResource")]
    pub monitored_resource: Option<String>,
}

impl Part for BillingDestination {}


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
    /// The name of the log. It must be less than 512 characters long and can
    /// include the following characters: upper- and lower-case alphanumeric
    /// characters [A-Za-z0-9], and punctuation characters including
    /// slash, underscore, hyphen, period [/_-.].
    pub name: Option<String>,
    /// A human-readable description of this log. This information appears in
    /// the documentation and can contain details.
    pub description: Option<String>,
}

impl Part for LogDescriptor {}


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
/// 
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
/// This also provides mechanism to whitelist any protobuf message extension that
/// can be sent in grpc metadata using “x-goog-ext-<extension_id>-bin” and
/// “x-goog-ext-<extension_id>-jspb” format. For example, list any service
/// specific protobuf types that can appear in grpc metadata as follows in your
/// yaml file:
/// 
/// Example:
/// 
///     context:
///       rules:
///        - selector: "google.example.library.v1.LibraryService.CreateBook"
///          allowed_request_extensions:
///          - google.foo.v1.NewExtension
///          allowed_response_extensions:
///          - google.foo.v1.NewExtension
/// 
/// You can also specify extension ID instead of fully qualified extension name
/// here.
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
    /// If true, the selected method should skip service control and the control
    /// plane features, such as quota and billing, will not be available.
    /// This flag is used by Google Cloud Endpoints to bypass checks for internal
    /// methods, such as service health check methods.
    #[serde(rename="skipServiceControl")]
    pub skip_service_control: Option<bool>,
    /// If true, the selected method allows unregistered calls, e.g. calls
    /// that don't identify any user or application.
    #[serde(rename="allowUnregisteredCalls")]
    pub allow_unregistered_calls: Option<bool>,
    /// Selects the methods to which this rule applies. Use '*' to indicate all
    /// methods in all APIs.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for UsageRule {}


/// A context rule provides information about the context for an individual API
/// element.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContextRule {
    /// A list of full type names of provided contexts.
    pub provided: Option<Vec<String>>,
    /// A list of full type names or extension IDs of extensions allowed in grpc
    /// side channel from backend to client.
    #[serde(rename="allowedResponseExtensions")]
    pub allowed_response_extensions: Option<Vec<String>>,
    /// A list of full type names or extension IDs of extensions allowed in grpc
    /// side channel from client to backend.
    #[serde(rename="allowedRequestExtensions")]
    pub allowed_request_extensions: Option<Vec<String>>,
    /// A list of full type names of requested contexts.
    pub requested: Option<Vec<String>>,
    /// Selects the methods to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for ContextRule {}


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


/// Message returning the created service connection.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [connections create services](struct.ServiceConnectionCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    /// Output only.
    /// Name of the peering connection that is created by the peering service.
    pub peering: Option<String>,
    /// Named IP address range(s) of PEERING type allocated for this service
    /// provider.
    /// Note that invoking this method with a different range when connection is
    /// already established will not modify already provisioned service
    /// producer subnetworks.
    #[serde(rename="reservedPeeringRanges")]
    pub reserved_peering_ranges: Option<Vec<String>>,
    /// Name of VPC network connected with service producer network.
    /// Must be in a form 'projects/{project}/global/networks/{network}'.
    /// {project} is a project number, as in '12345'
    /// {network} is a network name.
    pub network: Option<String>,
}

impl RequestValue for Connection {}


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


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [connections create services](struct.ServiceConnectionCreateCall.html) (response)
/// * [get operations](struct.OperationGetCall.html) (response)
/// * [add subnetwork services](struct.ServiceAddSubnetworkCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<Status>,
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
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, String>>,
}

impl Resource for Operation {}
impl ResponseResult for Operation {}


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
    /// If true, the service accepts API keys without any other credential.
    #[serde(rename="allowWithoutCredential")]
    pub allow_without_credential: Option<bool>,
    /// Selects the methods to which this rule applies.
    /// 
    /// Refer to selector for syntax details.
    pub selector: Option<String>,
}

impl Part for AuthenticationRule {}


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

/// A builder providing access to all methods supported on *service* resources.
/// It is not used directly, but through the `ServiceNetworking` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_servicenetworking1_beta as servicenetworking1_beta;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use servicenetworking1_beta::ServiceNetworking;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_subnetwork(...)`, `connections_create(...)` and `connections_list(...)`
/// // to build up your call.
/// let rb = hub.services();
/// # }
/// ```
pub struct ServiceMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceNetworking<C, A>,
}

impl<'a, C, A> MethodsBuilder for ServiceMethods<'a, C, A> {}

impl<'a, C, A> ServiceMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// To connect service to a VPC network peering connection
    /// must be established prior to service provisioning.
    /// This method must be invoked by the consumer VPC network administrator
    /// It will establish a permanent peering connection with a shared
    /// network created in the service producer organization and register a
    /// allocated IP range(s) to be used for service subnetwork provisioning.
    /// This connection will be used for all supported services in the service
    /// producer organization, so it only needs to be invoked once.
    /// Operation<response: Connection>.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Provider peering service that is managing peering connectivity for a
    ///              service provider organization.
    ///              For Google services that support this functionality it is
    ///              'services/servicenetworking.googleapis.com'.
    pub fn connections_create(&self, request: Connection, parent: &str) -> ServiceConnectionCreateCall<'a, C, A> {
        ServiceConnectionCreateCall {
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
    /// Service consumers use this method to list configured peering connection for
    /// the given service and consumer network.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Provider peering service that is managing peering connectivity for a
    ///              service provider organization.
    ///              For Google services that support this functionality it is
    ///              'services/servicenetworking.googleapis.com'.
    pub fn connections_list(&self, parent: &str) -> ServiceConnectionListCall<'a, C, A> {
        ServiceConnectionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _network: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Service producers use this method to provision a new subnet in
    /// peered service shared VPC network.
    /// It will validate previously provided allocated ranges, find
    /// non-conflicting sub-range of requested size (expressed in
    /// number of leading bits of ipv4 network mask, as in CIDR range
    /// notation). It will then create a subnetwork in the request
    /// region. The subsequent call will try to reuse the
    /// subnetwork previously created if subnetwork name, region and
    /// prefix length of the IP range match.
    /// Operation<response: Subnetwork>
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. This is a 'tenant' project in the service producer organization.
    ///              services/{service}/{collection-id}/{resource-id}
    ///              {collection id} is the cloud resource collection type representing the
    ///              tenant project. Only 'projects' are currently supported.
    ///              {resource id} is the tenant project numeric id: '123456'.
    ///              {service} the name of the peering service, for example
    ///              'service-peering.example.com'. This service must be activated.
    ///              in the consumer project.
    pub fn add_subnetwork(&self, request: AddSubnetworkRequest, parent: &str) -> ServiceAddSubnetworkCall<'a, C, A> {
        ServiceAddSubnetworkCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the `ServiceNetworking` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_servicenetworking1_beta as servicenetworking1_beta;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use servicenetworking1_beta::ServiceNetworking;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceNetworking<C, A>,
}

impl<'a, C, A> MethodsBuilder for OperationMethods<'a, C, A> {}

impl<'a, C, A> OperationMethods<'a, C, A> {
    
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





// ###################
// CallBuilders   ###
// #################

/// To connect service to a VPC network peering connection
/// must be established prior to service provisioning.
/// This method must be invoked by the consumer VPC network administrator
/// It will establish a permanent peering connection with a shared
/// network created in the service producer organization and register a
/// allocated IP range(s) to be used for service subnetwork provisioning.
/// This connection will be used for all supported services in the service
/// producer organization, so it only needs to be invoked once.
/// Operation<response: Connection>.
///
/// A builder for the *connections.create* method supported by a *service* resource.
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
/// # extern crate google_servicenetworking1_beta as servicenetworking1_beta;
/// use servicenetworking1_beta::Connection;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use servicenetworking1_beta::ServiceNetworking;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Connection::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().connections_create(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ServiceConnectionCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceNetworking<C, A>,
    _request: Connection,
    _parent: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceConnectionCreateCall<'a, C, A> {}

impl<'a, C, A> ServiceConnectionCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "servicenetworking.services.connections.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "v1beta/{+parent}/connections";
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
    pub fn request(mut self, new_value: Connection) -> ServiceConnectionCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Provider peering service that is managing peering connectivity for a
    /// service provider organization.
    /// For Google services that support this functionality it is
    /// 'services/servicenetworking.googleapis.com'.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ServiceConnectionCreateCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceConnectionCreateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceConnectionCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceConnectionCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Service consumers use this method to list configured peering connection for
/// the given service and consumer network.
///
/// A builder for the *connections.list* method supported by a *service* resource.
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
/// # extern crate google_servicenetworking1_beta as servicenetworking1_beta;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use servicenetworking1_beta::ServiceNetworking;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().connections_list("parent")
///              .network("dolores")
///              .doit();
/// # }
/// ```
pub struct ServiceConnectionListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceNetworking<C, A>,
    _parent: String,
    _network: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceConnectionListCall<'a, C, A> {}

impl<'a, C, A> ServiceConnectionListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListConnectionsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "servicenetworking.services.connections.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._network {
            params.push(("network", value.to_string()));
        }
        for &field in ["alt", "parent", "network"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta/{+parent}/connections";
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


    /// Provider peering service that is managing peering connectivity for a
    /// service provider organization.
    /// For Google services that support this functionality it is
    /// 'services/servicenetworking.googleapis.com'.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ServiceConnectionListCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Network name in the consumer project.   This network must have been
    /// already peered with a shared VPC network using CreateConnection
    /// method.
    /// Must be in a form 'projects/{project}/global/networks/{network}'.
    /// {project} is a project number, as in '12345'
    /// {network} is network name.
    ///
    /// Sets the *network* query property to the given value.
    pub fn network(mut self, new_value: &str) -> ServiceConnectionListCall<'a, C, A> {
        self._network = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceConnectionListCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceConnectionListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceConnectionListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Service producers use this method to provision a new subnet in
/// peered service shared VPC network.
/// It will validate previously provided allocated ranges, find
/// non-conflicting sub-range of requested size (expressed in
/// number of leading bits of ipv4 network mask, as in CIDR range
/// notation). It will then create a subnetwork in the request
/// region. The subsequent call will try to reuse the
/// subnetwork previously created if subnetwork name, region and
/// prefix length of the IP range match.
/// Operation<response: Subnetwork>
///
/// A builder for the *addSubnetwork* method supported by a *service* resource.
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
/// # extern crate google_servicenetworking1_beta as servicenetworking1_beta;
/// use servicenetworking1_beta::AddSubnetworkRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use servicenetworking1_beta::ServiceNetworking;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AddSubnetworkRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.services().add_subnetwork(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ServiceAddSubnetworkCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceNetworking<C, A>,
    _request: AddSubnetworkRequest,
    _parent: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ServiceAddSubnetworkCall<'a, C, A> {}

impl<'a, C, A> ServiceAddSubnetworkCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "servicenetworking.services.addSubnetwork",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "v1beta/{+parent}:addSubnetwork";
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
    pub fn request(mut self, new_value: AddSubnetworkRequest) -> ServiceAddSubnetworkCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required. This is a 'tenant' project in the service producer organization.
    /// services/{service}/{collection-id}/{resource-id}
    /// {collection id} is the cloud resource collection type representing the
    /// tenant project. Only 'projects' are currently supported.
    /// {resource id} is the tenant project numeric id: '123456'.
    /// {service} the name of the peering service, for example
    /// 'service-peering.example.com'. This service must be activated.
    /// in the consumer project.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ServiceAddSubnetworkCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ServiceAddSubnetworkCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ServiceAddSubnetworkCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ServiceAddSubnetworkCall<'a, C, A>
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
/// # extern crate google_servicenetworking1_beta as servicenetworking1_beta;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use servicenetworking1_beta::ServiceNetworking;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ServiceNetworking::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().get("name")
///              .doit();
/// # }
/// ```
pub struct OperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ServiceNetworking<C, A>,
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
        dlg.begin(MethodInfo { id: "servicenetworking.operations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "v1beta/{+name}";
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
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


