// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *manager* crate version *0.1.0+20140915*, where *20140915* is the exact revision of the *manager:v1beta2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.0*.
//! 
//! Everything else about the *manager* *v1_beta2* API can be found at the
//! [official documentation site](https://developers.google.com/deployment-manager/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/manager1_beta2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Manager.html) ... 
//! 
//! * [deployments](struct.Deployment.html)
//!  * [*delete*](struct.DeploymentDeleteCall.html), [*get*](struct.DeploymentGetCall.html), [*insert*](struct.DeploymentInsertCall.html) and [*list*](struct.DeploymentListCall.html)
//! * [templates](struct.Template.html)
//!  * [*delete*](struct.TemplateDeleteCall.html), [*get*](struct.TemplateGetCall.html), [*insert*](struct.TemplateInsertCall.html) and [*list*](struct.TemplateListCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Manager.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
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
//! let r = hub.deployments().delete(...).doit()
//! let r = hub.deployments().get(...).doit()
//! let r = hub.deployments().insert(...).doit()
//! let r = hub.deployments().list(...).doit()
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
//! google-manager1_beta2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-manager1_beta2" as manager1_beta2;
//! use manager1_beta2::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use manager1_beta2::Manager;
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
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Manager::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.deployments().list("projectId", "region")
//!              .page_token("kasd")
//!              .max_results(-22)
//!              .doit();
//! 
//! match result {
//!     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!     Result::MissingToken => println!("OAuth2: Missing Token"),
//!     Result::Cancelled => println!("Operation cancelled by user"),
//!     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     Result::Success(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downlods
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
//! [decodable](trait.ResponseResult.html) via json. Optionals are used to indicate that partial requests are responses are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifyable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(core,io,thread_sleep)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, ResourceMethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your Google Compute Engine resources
    Compute,

    /// View and manage your applications deployed on Google App Engine
    AppengineAdmin,

    /// View your Google Cloud Platform management resources and deployment status information
    NdevCloudmanReadonly,

    /// View and manage your Google Cloud Platform management resources and deployment status information
    NdevCloudman,

    /// Manage your data in Google Cloud Storage
    DevstorageReadWrite,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            Scope::Compute => "https://www.googleapis.com/auth/compute",
            Scope::AppengineAdmin => "https://www.googleapis.com/auth/appengine.admin",
            Scope::NdevCloudmanReadonly => "https://www.googleapis.com/auth/ndev.cloudman.readonly",
            Scope::NdevCloudman => "https://www.googleapis.com/auth/ndev.cloudman",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::NdevCloudmanReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Manager related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-manager1_beta2" as manager1_beta2;
/// use manager1_beta2::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use manager1_beta2::Manager;
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
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Manager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.deployments().list("projectId", "region")
///              .page_token("amet.")
///              .max_results(-81)
///              .doit();
/// 
/// match result {
///     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
///     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///     Result::MissingToken => println!("OAuth2: Missing Token"),
///     Result::Cancelled => println!("Operation cancelled by user"),
///     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     Result::Success(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Manager<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Manager<C, NC, A> {}

impl<'a, C, NC, A> Manager<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Manager<C, NC, A> {
        Manager {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.0".to_string(),
            _m: PhantomData
        }
    }

    pub fn deployments(&'a self) -> DeploymentMethods<'a, C, NC, A> {
        DeploymentMethods { hub: &self }
    }
    pub fn templates(&'a self) -> TemplateMethods<'a, C, NC, A> {
        TemplateMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingModule {
    /// no description provided    
    #[serde(alias="maxNumReplicas")]
    pub max_num_replicas: i32,
    /// no description provided    
    pub description: String,
    /// no description provided    
    #[serde(alias="targetModule")]
    pub target_module: String,
    /// target_utilization should be in range [0,1].    
    #[serde(alias="targetUtilization")]
    pub target_utilization: f64,
    /// no description provided    
    #[serde(alias="signalType")]
    pub signal_type: String,
    /// no description provided    
    #[serde(alias="minNumReplicas")]
    pub min_num_replicas: i32,
    /// no description provided    
    #[serde(alias="coolDownPeriodSec")]
    pub cool_down_period_sec: i32,
}

impl Part for AutoscalingModule {}


/// A Firewall resource
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallModule {
    /// The description of the firewall (optional)    
    pub description: String,
    /// Target Tags to apply this firewall to, see the GCE Spec for details on syntax    
    #[serde(alias="targetTags")]
    pub target_tags: Vec<String>,
    /// The allowed ports or port ranges.    
    pub allowed: Vec<AllowedRule>,
    /// Source Tags to apply this firewall to, see the GCE Spec for details on syntax    
    #[serde(alias="sourceTags")]
    pub source_tags: Vec<String>,
    /// Source IP ranges to apply this firewall to, see the GCE Spec for details on syntax    
    #[serde(alias="sourceRanges")]
    pub source_ranges: Vec<String>,
    /// The NetworkModule to which this firewall should apply. If not specified, or if specified as 'default', this firewall will be applied to the 'default' network.    
    pub network: String,
}

impl Part for FirewallModule {}


/// A Compute Engine network accessConfig. Identical to the accessConfig on corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessConfig {
    /// Type of this access configuration file. (Currently only ONE_TO_ONE_NAT is legal.)    
    #[serde(alias="type")]
    pub type_: String,
    /// Name of this access configuration.    
    pub name: String,
    /// An external IP address associated with this instance.    
    #[serde(alias="natIp")]
    pub nat_ip: String,
}

impl Part for AccessConfig {}


/// A module in a configuration. A module represents a single homogeneous, possibly replicated task.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    /// no description provided    
    #[serde(alias="lbModule")]
    pub lb_module: LbModule,
    /// no description provided    
    #[serde(alias="replicaPoolModule")]
    pub replica_pool_module: ReplicaPoolModule,
    /// no description provided    
    #[serde(alias="firewallModule")]
    pub firewall_module: FirewallModule,
    /// no description provided    
    #[serde(alias="healthCheckModule")]
    pub health_check_module: HealthCheckModule,
    /// no description provided    
    #[serde(alias="autoscalingModule")]
    pub autoscaling_module: AutoscalingModule,
    /// no description provided    
    #[serde(alias="networkModule")]
    pub network_module: NetworkModule,
    /// The type of this module. Valid values ("AUTOSCALING", "FIREWALL", "HEALTH_CHECK", "LOAD_BALANCING", "NETWORK", "REPLICA_POOL")    
    #[serde(alias="type")]
    pub type_: String,
}

impl Part for Module {}


/// A Template represents a complete configuration for a Deployment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete templates](struct.TemplateDeleteCall.html) (none)
/// * [get templates](struct.TemplateGetCall.html) (response)
/// * [list templates](struct.TemplateListCall.html) (none)
/// * [insert templates](struct.TemplateInsertCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    /// A list of modules for this Template.    
    pub modules: Option<HashMap<String, Module>>,
    /// A user-supplied description of this Template.    
    pub description: Option<String>,
    /// Action definitions for use in Module intents in this Template.    
    pub actions: Option<HashMap<String, Action>>,
    /// Name of this Template. The name must conform to the expression: [a-zA-Z0-9-_]{1,64}    
    pub name: Option<String>,
}

impl RequestValue for Template {}
impl Resource for Template {}
impl ResponseResult for Template {}


/// [Output Only] The current state of a replica or module.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeployState {
    /// [Output Only] The status of the deployment. Possible values include: 
    /// - UNKNOWN
    /// - DEPLOYING
    /// - DEPLOYED
    /// - DEPLOYMENT_FAILED
    /// - DELETING
    /// - DELETED
    /// - DELETE_FAILED
    pub status: String,
    /// [Output Only] Human readable details about the current state.    
    pub details: String,
}

impl Part for DeployState {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallModuleStatus {
    /// [Output Only] The URL of the corresponding Firewall resource.    
    #[serde(alias="firewallUrl")]
    pub firewall_url: String,
}

impl Part for FirewallModuleStatus {}


/// A Compute Engine metadata entry. Identical to the metadata on the corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// A list of metadata items.    
    pub items: Vec<MetadataItem>,
    /// The fingerprint of the metadata.    
    #[serde(alias="fingerPrint")]
    pub finger_print: String,
}

impl Part for Metadata {}


/// [Output Only] Aggregate status for a module.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModuleStatus {
    /// [Output Only] The status of the HealthCheckModule, set for type HEALTH_CHECK.    
    #[serde(alias="healthCheckModuleStatus")]
    pub health_check_module_status: HealthCheckModuleStatus,
    /// [Output Only] The status of the NetworkModule, set for type NETWORK.    
    #[serde(alias="networkModuleStatus")]
    pub network_module_status: NetworkModuleStatus,
    /// [Output Only] The status of the ReplicaPoolModule, set for type VM.    
    #[serde(alias="replicaPoolModuleStatus")]
    pub replica_pool_module_status: ReplicaPoolModuleStatus,
    /// [Output Only] The current state of the module.    
    pub state: DeployState,
    /// [Output Only] The status of the AutoscalingModule, set for type AUTOSCALING.    
    #[serde(alias="autoscalingModuleStatus")]
    pub autoscaling_module_status: AutoscalingModuleStatus,
    /// [Output Only] The status of the FirewallModule, set for type FIREWALL.    
    #[serde(alias="firewallModuleStatus")]
    pub firewall_module_status: FirewallModuleStatus,
    /// [Output Only] The type of the module.    
    #[serde(alias="type")]
    pub type_: String,
    /// [Output Only] The status of the LbModule, set for type LOAD_BALANCING.    
    #[serde(alias="lbModuleStatus")]
    pub lb_module_status: LbModuleStatus,
}

impl Part for ModuleStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkModuleStatus {
    /// [Output Only] The URL of the corresponding Network resource.    
    #[serde(alias="networkUrl")]
    pub network_url: String,
}

impl Part for NetworkModuleStatus {}


/// An environment variable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvVariable {
    /// Whether this variable is hidden or visible.    
    pub hidden: bool,
    /// Value of the environment variable.    
    pub value: String,
}

impl Part for EnvVariable {}


/// A pre-existing persistent disk that will be attached to every Replica in the Pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExistingDisk {
    /// The fully-qualified URL of the Persistent Disk resource. It must be in the same zone as the Pool.    
    pub source: String,
    /// Optional. How the disk will be attached to the Replica.    
    pub attachment: DiskAttachment,
}

impl Part for ExistingDisk {}


/// Configuration information for a ReplicaPools resource. Specifying an item within will determine the ReplicaPools API version used for a ReplicaPoolModule. Only one may be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolParams {
    /// ReplicaPoolParams specifications for use with ReplicaPools v1beta1.    
    pub v1beta1: ReplicaPoolParamsV1Beta1,
}

impl Part for ReplicaPoolParams {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list templates](struct.TemplateListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TemplatesListResponse {
    /// no description provided    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// no description provided    
    pub resources: Vec<Template>,
}

impl ResponseResult for TemplatesListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list deployments](struct.DeploymentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DeploymentsListResponse {
    /// no description provided    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// no description provided    
    pub resources: Vec<Deployment>,
}

impl ResponseResult for DeploymentsListResponse {}


/// A deployment represents a physical instantiation of a Template.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete deployments](struct.DeploymentDeleteCall.html) (none)
/// * [get deployments](struct.DeploymentGetCall.html) (response)
/// * [insert deployments](struct.DeploymentInsertCall.html) (request|response)
/// * [list deployments](struct.DeploymentListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    /// [Output Only] Current status of this deployment.    
    pub state: Option<DeployState>,
    /// A user-supplied description of this Deployment.    
    pub description: Option<String>,
    /// The name of the Template on which this deployment is based.    
    #[serde(alias="templateName")]
    pub template_name: Option<String>,
    /// The set of parameter overrides to apply to the corresponding Template before deploying.    
    pub overrides: Option<Vec<ParamOverride>>,
    /// [Output Only] The time when this deployment was created.    
    #[serde(alias="creationDate")]
    pub creation_date: Option<String>,
    /// [Output Only] List of status for the modules in this deployment.    
    pub modules: Option<HashMap<String, ModuleStatus>>,
    /// Name of this deployment. The name must conform to the following regular expression: [a-zA-Z0-9-_]{1,64}    
    pub name: Option<String>,
}

impl RequestValue for Deployment {}
impl Resource for Deployment {}
impl ResponseResult for Deployment {}


/// Initialization parameters for creating a new disk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewDiskInitializeParams {
    /// The size of the created disk in gigabytes.    
    #[serde(alias="diskSizeGb")]
    pub disk_size_gb: String,
    /// The fully-qualified URL of a source image to use to create this disk.    
    #[serde(alias="sourceImage")]
    pub source_image: String,
    /// Name of the disk type resource describing which disk type to use to create the disk. For example 'pd-ssd' or 'pd-standard'. Default is 'pd-standard'    
    #[serde(alias="diskType")]
    pub disk_type: String,
}

impl Part for NewDiskInitializeParams {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkModule {
    /// Required; The range of internal addresses that are legal on this network. This range is a CIDR specification, for example: 192.168.0.0/16.    
    #[serde(alias="IPv4Range")]
    pub i_pv4_range: String,
    /// An optional address that is used for default routing to other networks. This must be within the range specified by IPv4Range, and is typicall the first usable address in that range. If not specified, the default value is the first usable address in IPv4Range.    
    #[serde(alias="gatewayIPv4")]
    pub gateway_i_pv4: String,
    /// The description of the network.    
    pub description: String,
}

impl Part for NetworkModule {}


/// Configuration information for a ReplicaPools v1beta1 API resource. Directly maps to ReplicaPool InitTemplate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolParamsV1Beta1 {
    /// A list of Disk resources to create and attach to each Replica in the Pool. Currently, you can only define one disk and it must be a root persistent disk. Note that Replica Pool will create a root persistent disk for each replica.    
    #[serde(alias="disksToCreate")]
    pub disks_to_create: Vec<NewDisk>,
    /// Whether these replicas should be restarted if they experience a failure. The default value is true.    
    #[serde(alias="autoRestart")]
    pub auto_restart: bool,
    /// Enables IP Forwarding    
    #[serde(alias="canIpForward")]
    pub can_ip_forward: bool,
    /// An optional textual description of the resource.    
    pub description: String,
    /// The zone for this ReplicaPool.    
    pub zone: String,
    /// A list of tags to apply to the Google Compute Engine instance to identify resources.    
    pub tags: Tag,
    /// no description provided    
    #[serde(alias="onHostMaintenance")]
    pub on_host_maintenance: String,
    /// A list of existing Persistent Disk resources to attach to each replica in the pool. Each disk will be attached in read-only mode to every replica.    
    #[serde(alias="disksToAttach")]
    pub disks_to_attach: Vec<ExistingDisk>,
    /// The machine type for this instance. Either a complete URL, or the resource name (e.g. n1-standard-1).    
    #[serde(alias="machineType")]
    pub machine_type: String,
    /// The base name for instances within this ReplicaPool.    
    #[serde(alias="baseInstanceName")]
    pub base_instance_name: String,
    /// Name of the Action to be run during initialization of a ReplicaPoolModule.    
    #[serde(alias="initAction")]
    pub init_action: String,
    /// The metadata key/value pairs assigned to this instance.    
    pub metadata: Metadata,
    /// A list of Service Accounts to enable for this instance.    
    #[serde(alias="serviceAccounts")]
    pub service_accounts: Vec<ServiceAccount>,
    /// A list of network interfaces for the instance. Currently only one interface is supported by Google Compute Engine.    
    #[serde(alias="networkInterfaces")]
    pub network_interfaces: Vec<NetworkInterface>,
}

impl Part for ReplicaPoolParamsV1Beta1 {}


/// A specification for overriding parameters in a Template that corresponds to the Deployment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParamOverride {
    /// A JSON Path expression that specifies which parameter should be overridden.    
    pub path: String,
    /// The new value to assign to the overridden parameter.    
    pub value: String,
}

impl Part for ParamOverride {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolModuleStatus {
    /// [Output Only] The URL of the Resource Group associated with this ReplicaPool.    
    #[serde(alias="resourceViewUrl")]
    pub resource_view_url: String,
    /// [Output Only] The URL of the associated ReplicaPool resource.    
    #[serde(alias="replicaPoolUrl")]
    pub replica_pool_url: String,
}

impl Part for ReplicaPoolModuleStatus {}


/// A Compute Engine service account, identical to the Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// List of OAuth2 scopes to obtain for the service account.    
    pub scopes: Vec<String>,
    /// Service account email address.    
    pub email: String,
}

impl Part for ServiceAccount {}


/// A Compute Engine NetworkInterface resource. Identical to the NetworkInterface on the corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// An array of configurations for this interface. This specifies how this interface is configured to interact with other network services    
    #[serde(alias="accessConfigs")]
    pub access_configs: Vec<AccessConfig>,
    /// An optional IPV4 internal network address to assign to the instance for this network interface.    
    #[serde(alias="networkIp")]
    pub network_ip: String,
    /// Name of the interface.    
    pub name: String,
    /// The name of the NetworkModule to which this interface applies. If not specified, or specified as 'default', this will use the 'default' network.    
    pub network: String,
}

impl Part for NetworkInterface {}


/// An Action encapsulates a set of commands as a single runnable module with additional information needed during run-time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    /// The timeout in milliseconds for this action to run.    
    #[serde(alias="timeoutMs")]
    pub timeout_ms: i32,
    /// A list of commands to run sequentially for this action.    
    pub commands: Vec<String>,
}

impl Part for Action {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingModuleStatus {
    /// [Output Only] The URL of the corresponding Autoscaling configuration.    
    #[serde(alias="autoscalingConfigUrl")]
    pub autoscaling_config_url: String,
}

impl Part for AutoscalingModuleStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LbModuleStatus {
    /// [Output Only] The URL of the corresponding ForwardingRule in GCE.    
    #[serde(alias="forwardingRuleUrl")]
    pub forwarding_rule_url: String,
    /// [Output Only] The URL of the corresponding TargetPool resource in GCE.    
    #[serde(alias="targetPoolUrl")]
    pub target_pool_url: String,
}

impl Part for LbModuleStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LbModule {
    /// no description provided    
    pub description: String,
    /// no description provided    
    #[serde(alias="healthChecks")]
    pub health_checks: Vec<String>,
    /// no description provided    
    #[serde(alias="portRange")]
    pub port_range: String,
    /// no description provided    
    #[serde(alias="targetModules")]
    pub target_modules: Vec<String>,
    /// no description provided    
    #[serde(alias="ipAddress")]
    pub ip_address: String,
    /// no description provided    
    #[serde(alias="ipProtocol")]
    pub ip_protocol: String,
    /// no description provided    
    #[serde(alias="sessionAffinity")]
    pub session_affinity: String,
}

impl Part for LbModule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HealthCheckModuleStatus {
    /// [Output Only] The HealthCheck URL.    
    #[serde(alias="healthCheckUrl")]
    pub health_check_url: String,
}

impl Part for HealthCheckModuleStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HealthCheckModule {
    /// no description provided    
    pub description: String,
    /// no description provided    
    #[serde(alias="timeoutSec")]
    pub timeout_sec: i32,
    /// no description provided    
    #[serde(alias="checkIntervalSec")]
    pub check_interval_sec: i32,
    /// no description provided    
    #[serde(alias="unhealthyThreshold")]
    pub unhealthy_threshold: i32,
    /// no description provided    
    #[serde(alias="healthyThreshold")]
    pub healthy_threshold: i32,
    /// no description provided    
    pub host: String,
    /// no description provided    
    pub path: String,
    /// no description provided    
    pub port: i32,
}

impl Part for HealthCheckModule {}


/// A Persistent Disk resource that will be created and attached to each Replica in the Pool. Each Replica will have a unique persistent disk that is created and attached to that Replica.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewDisk {
    /// If true, indicates that this is the root persistent disk.    
    pub boot: bool,
    /// Create the new disk using these parameters. The name of the disk will be <instance_name>-<five_random_charactersgt;.    
    #[serde(alias="initializeParams")]
    pub initialize_params: NewDiskInitializeParams,
    /// How the disk will be attached to the Replica.    
    pub attachment: DiskAttachment,
    /// If true, then this disk will be deleted when the instance is deleted.    
    #[serde(alias="autoDelete")]
    pub auto_delete: bool,
}

impl Part for NewDisk {}


/// An allowed port resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AllowedRule {
    /// ?tcp?, ?udp? or ?icmp?    
    #[serde(alias="IPProtocol")]
    pub ip_protocol: String,
    /// List of ports or port ranges (Example inputs include: ["22"], [?33?, "12345-12349"].    
    pub ports: Vec<String>,
}

impl Part for AllowedRule {}


/// A Compute Engine Instance tag, identical to the tags on the corresponding Compute Engine Instance resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    /// Items contained in this tag.    
    pub items: Vec<String>,
    /// The fingerprint of the tag.    
    #[serde(alias="fingerPrint")]
    pub finger_print: String,
}

impl Part for Tag {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolModule {
    /// The Health Checks to configure for the ReplicaPoolModule    
    #[serde(alias="healthChecks")]
    pub health_checks: Vec<String>,
    /// Information for a ReplicaPoolModule.    
    #[serde(alias="replicaPoolParams")]
    pub replica_pool_params: ReplicaPoolParams,
    /// [Output Only] The name of the Resource View associated with a ReplicaPoolModule. This field will be generated by the service.    
    #[serde(alias="resourceView")]
    pub resource_view: String,
    /// Number of replicas in this module.    
    #[serde(alias="numReplicas")]
    pub num_replicas: i32,
    /// A list of environment variables.    
    #[serde(alias="envVariables")]
    pub env_variables: HashMap<String, EnvVariable>,
}

impl Part for ReplicaPoolModule {}


/// A Compute Engine metadata item, defined as a key:value pair. Identical to the metadata on the corresponding Compute Engine resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataItem {
    /// A metadata key.    
    pub key: String,
    /// A metadata value.    
    pub value: String,
}

impl Part for MetadataItem {}


/// How to attach a disk to a Replica.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiskAttachment {
    /// The device name of this disk.    
    #[serde(alias="deviceName")]
    pub device_name: String,
    /// A zero-based index to assign to this disk, where 0 is reserved for the boot disk. If not specified, this is assigned by the server.    
    pub index: u32,
}

impl Part for DiskAttachment {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *template* resources.
/// It is not used directly, but through the `Manager` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-manager1_beta2" as manager1_beta2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use manager1_beta2::Manager;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Manager::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.templates();
/// # }
/// ```
pub struct TemplateMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for TemplateMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> TemplateMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn list(&self, project_id: &str) -> TemplateListCall<'a, C, NC, A> {
        TemplateListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn delete(&self, project_id: &str, template_name: &str) -> TemplateDeleteCall<'a, C, NC, A> {
        TemplateDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _template_name: template_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn insert(&self, request: &Template, project_id: &str) -> TemplateInsertCall<'a, C, NC, A> {
        TemplateInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn get(&self, project_id: &str, template_name: &str) -> TemplateGetCall<'a, C, NC, A> {
        TemplateGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _template_name: template_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *deployment* resources.
/// It is not used directly, but through the `Manager` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-manager1_beta2" as manager1_beta2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use manager1_beta2::Manager;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Manager::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.deployments();
/// # }
/// ```
pub struct DeploymentMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for DeploymentMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> DeploymentMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn insert(&self, request: &Deployment, project_id: &str, region: &str) -> DeploymentInsertCall<'a, C, NC, A> {
        DeploymentInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn get(&self, project_id: &str, region: &str, deployment_name: &str) -> DeploymentGetCall<'a, C, NC, A> {
        DeploymentGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _deployment_name: deployment_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn list(&self, project_id: &str, region: &str) -> DeploymentListCall<'a, C, NC, A> {
        DeploymentListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///     
    pub fn delete(&self, project_id: &str, region: &str, deployment_name: &str) -> DeploymentDeleteCall<'a, C, NC, A> {
        DeploymentDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _region: region.to_string(),
            _deployment_name: deployment_name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// 
///
/// A builder for the *list* method supported by a *template* resource.
/// It is not used directly, but through a `TemplateMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.templates().list("projectId")
///              .page_token("sea")
///              .max_results(-90)
///              .doit();
/// # }
/// ```
pub struct TemplateListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _project_id: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TemplateListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TemplateListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TemplatesListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.templates.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "projectId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/templates".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::NdevCloudmanReadonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> TemplateListCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// Specifies a nextPageToken returned by a previous list request. This token can be used to request the next page of results from a previous list request.    
    pub fn page_token(mut self, new_value: &str) -> TemplateListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum count of results to be returned. Acceptable values are 0 to 100, inclusive. (Default: 50)    
    pub fn max_results(mut self, new_value: i32) -> TemplateListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TemplateListCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TemplateListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TemplateListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *delete* method supported by a *template* resource.
/// It is not used directly, but through a `TemplateMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.templates().delete("projectId", "templateName")
///              .doit();
/// # }
/// ```
pub struct TemplateDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _project_id: String,
    _template_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TemplateDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TemplateDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.templates.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("templateName", self._template_name.to_string()));
        for &field in ["projectId", "templateName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/templates/{templateName}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppengineAdmin.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{templateName}", "templateName")].iter() {
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
            for param_name in ["projectId", "templateName"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> TemplateDeleteCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *template name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn template_name(mut self, new_value: &str) -> TemplateDeleteCall<'a, C, NC, A> {
        self._template_name = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TemplateDeleteCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TemplateDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TemplateDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *insert* method supported by a *template* resource.
/// It is not used directly, but through a `TemplateMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// use manager1_beta2::Template;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Template = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.templates().insert(&req, "projectId")
///              .doit();
/// # }
/// ```
pub struct TemplateInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _request: Template,
    _project_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TemplateInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TemplateInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Template)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.templates.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/templates".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppengineAdmin.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
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
            for param_name in ["projectId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Template) -> TemplateInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> TemplateInsertCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TemplateInsertCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TemplateInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TemplateInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *get* method supported by a *template* resource.
/// It is not used directly, but through a `TemplateMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.templates().get("projectId", "templateName")
///              .doit();
/// # }
/// ```
pub struct TemplateGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _project_id: String,
    _template_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TemplateGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TemplateGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Template)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.templates.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("templateName", self._template_name.to_string()));
        for &field in ["alt", "projectId", "templateName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/templates/{templateName}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::NdevCloudmanReadonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{templateName}", "templateName")].iter() {
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
            for param_name in ["projectId", "templateName"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> TemplateGetCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *template name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn template_name(mut self, new_value: &str) -> TemplateGetCall<'a, C, NC, A> {
        self._template_name = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TemplateGetCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TemplateGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TemplateGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *insert* method supported by a *deployment* resource.
/// It is not used directly, but through a `DeploymentMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// use manager1_beta2::Deployment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Deployment = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.deployments().insert(&req, "projectId", "region")
///              .doit();
/// # }
/// ```
pub struct DeploymentInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _request: Deployment,
    _project_id: String,
    _region: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DeploymentInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DeploymentInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Deployment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.deployments.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("region", self._region.to_string()));
        for &field in ["alt", "projectId", "region"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/regions/{region}/deployments".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppengineAdmin.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{region}", "region")].iter() {
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
            for param_name in ["projectId", "region"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Deployment) -> DeploymentInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> DeploymentInsertCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *region* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn region(mut self, new_value: &str) -> DeploymentInsertCall<'a, C, NC, A> {
        self._region = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DeploymentInsertCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DeploymentInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DeploymentInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *get* method supported by a *deployment* resource.
/// It is not used directly, but through a `DeploymentMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.deployments().get("projectId", "region", "deploymentName")
///              .doit();
/// # }
/// ```
pub struct DeploymentGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _project_id: String,
    _region: String,
    _deployment_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DeploymentGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DeploymentGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Deployment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.deployments.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("region", self._region.to_string()));
        params.push(("deploymentName", self._deployment_name.to_string()));
        for &field in ["alt", "projectId", "region", "deploymentName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/regions/{region}/deployments/{deploymentName}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::NdevCloudmanReadonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{region}", "region"), ("{deploymentName}", "deploymentName")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["projectId", "region", "deploymentName"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> DeploymentGetCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *region* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn region(mut self, new_value: &str) -> DeploymentGetCall<'a, C, NC, A> {
        self._region = new_value.to_string();
        self
    }
    /// Sets the *deployment name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn deployment_name(mut self, new_value: &str) -> DeploymentGetCall<'a, C, NC, A> {
        self._deployment_name = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DeploymentGetCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DeploymentGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DeploymentGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *list* method supported by a *deployment* resource.
/// It is not used directly, but through a `DeploymentMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.deployments().list("projectId", "region")
///              .page_token("Lorem")
///              .max_results(-21)
///              .doit();
/// # }
/// ```
pub struct DeploymentListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _project_id: String,
    _region: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DeploymentListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DeploymentListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DeploymentsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.deployments.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("region", self._region.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "projectId", "region", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/regions/{region}/deployments".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::NdevCloudmanReadonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{region}", "region")].iter() {
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
            for param_name in ["projectId", "region"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> DeploymentListCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *region* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn region(mut self, new_value: &str) -> DeploymentListCall<'a, C, NC, A> {
        self._region = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// Specifies a nextPageToken returned by a previous list request. This token can be used to request the next page of results from a previous list request.    
    pub fn page_token(mut self, new_value: &str) -> DeploymentListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum count of results to be returned. Acceptable values are 0 to 100, inclusive. (Default: 50)    
    pub fn max_results(mut self, new_value: i32) -> DeploymentListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DeploymentListCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DeploymentListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DeploymentListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// 
///
/// A builder for the *delete* method supported by a *deployment* resource.
/// It is not used directly, but through a `DeploymentMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-manager1_beta2" as manager1_beta2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manager1_beta2::Manager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Manager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.deployments().delete("projectId", "region", "deploymentName")
///              .doit();
/// # }
/// ```
pub struct DeploymentDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Manager<C, NC, A>,
    _project_id: String,
    _region: String,
    _deployment_name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DeploymentDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DeploymentDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manager.deployments.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("projectId", self._project_id.to_string()));
        params.push(("region", self._region.to_string()));
        params.push(("deploymentName", self._deployment_name.to_string()));
        for &field in ["projectId", "region", "deploymentName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/manager/v1beta2/projects/{projectId}/regions/{region}/deployments/{deploymentName}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppengineAdmin.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{region}", "region"), ("{deploymentName}", "deploymentName")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["projectId", "region", "deploymentName"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
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
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn project_id(mut self, new_value: &str) -> DeploymentDeleteCall<'a, C, NC, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// Sets the *region* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn region(mut self, new_value: &str) -> DeploymentDeleteCall<'a, C, NC, A> {
        self._region = new_value.to_string();
        self
    }
    /// Sets the *deployment name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn deployment_name(mut self, new_value: &str) -> DeploymentDeleteCall<'a, C, NC, A> {
        self._deployment_name = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DeploymentDeleteCall<'a, C, NC, A> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DeploymentDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> DeploymentDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


