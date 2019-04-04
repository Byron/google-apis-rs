// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Debugger* crate version *1.0.8+20190313*, where *20190313* is the exact revision of the *clouddebugger:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Cloud Debugger* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/debugger).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/clouddebugger2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CloudDebugger.html) ... 
//! 
//! * controller
//!  * [*debuggees breakpoints list*](struct.ControllerDebuggeeBreakpointListCall.html), [*debuggees breakpoints update*](struct.ControllerDebuggeeBreakpointUpdateCall.html) and [*debuggees register*](struct.ControllerDebuggeeRegisterCall.html)
//! * debugger
//!  * [*debuggees breakpoints delete*](struct.DebuggerDebuggeeBreakpointDeleteCall.html), [*debuggees breakpoints get*](struct.DebuggerDebuggeeBreakpointGetCall.html), [*debuggees breakpoints list*](struct.DebuggerDebuggeeBreakpointListCall.html), [*debuggees breakpoints set*](struct.DebuggerDebuggeeBreakpointSetCall.html) and [*debuggees list*](struct.DebuggerDebuggeeListCall.html)
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
//! * **[Hub](struct.CloudDebugger.html)**
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
//! let r = hub.debugger().debuggees_breakpoints_set(...).doit()
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
//! google-clouddebugger2 = "*"
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
//! extern crate google_clouddebugger2 as clouddebugger2;
//! use clouddebugger2::Breakpoint;
//! use clouddebugger2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use clouddebugger2::CloudDebugger;
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
//! let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Breakpoint::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.debugger().debuggees_breakpoints_set(req, "debuggeeId")
//!              .client_version("sed")
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
    /// Use Stackdriver Debugger
    CloudDebugger,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudDebugger => "https://www.googleapis.com/auth/cloud_debugger",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudDebugger
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all CloudDebugger related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_clouddebugger2 as clouddebugger2;
/// use clouddebugger2::Breakpoint;
/// use clouddebugger2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use clouddebugger2::CloudDebugger;
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
/// let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Breakpoint::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.debugger().debuggees_breakpoints_set(req, "debuggeeId")
///              .client_version("dolores")
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
pub struct CloudDebugger<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for CloudDebugger<C, A> {}

impl<'a, C, A> CloudDebugger<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CloudDebugger<C, A> {
        CloudDebugger {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://clouddebugger.googleapis.com/".to_string(),
            _root_url: "https://clouddebugger.googleapis.com/".to_string(),
        }
    }

    pub fn controller(&'a self) -> ControllerMethods<'a, C, A> {
        ControllerMethods { hub: &self }
    }
    pub fn debugger(&'a self) -> DebuggerMethods<'a, C, A> {
        DebuggerMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://clouddebugger.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://clouddebugger.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// An ExtendedSourceContext is a SourceContext combined with additional
/// details describing the context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedSourceContext {
    /// Labels with user defined metadata.
    pub labels: Option<HashMap<String, String>>,
    /// Any source context.
    pub context: Option<SourceContext>,
}

impl Part for ExtendedSourceContext {}


/// Response for listing breakpoints.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints list debugger](struct.DebuggerDebuggeeBreakpointListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBreakpointsResponse {
    /// A wait token that can be used in the next call to `list` (REST) or
    /// `ListBreakpoints` (RPC) to block until the list of breakpoints has changes.
    #[serde(rename="nextWaitToken")]
    pub next_wait_token: Option<String>,
    /// List of breakpoints matching the request.
    /// The fields `id` and `location` are guaranteed to be set on each breakpoint.
    /// The fields: `stack_frames`, `evaluated_expressions` and `variable_table`
    /// are cleared on each breakpoint regardless of its status.
    pub breakpoints: Option<Vec<Breakpoint>>,
}

impl ResponseResult for ListBreakpointsResponse {}


/// A CloudWorkspaceSourceContext denotes a workspace at a particular snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudWorkspaceSourceContext {
    /// The ID of the snapshot.
    /// An empty snapshot_id refers to the most recent snapshot.
    #[serde(rename="snapshotId")]
    pub snapshot_id: Option<String>,
    /// The ID of the workspace.
    #[serde(rename="workspaceId")]
    pub workspace_id: Option<CloudWorkspaceId>,
}

impl Part for CloudWorkspaceSourceContext {}


/// Represents a stack frame context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackFrame {
    /// Demangled function name at the call site.
    pub function: Option<String>,
    /// Set of arguments passed to this function.
    /// Note that this might not be populated for all stack frames.
    pub arguments: Option<Vec<Variable>>,
    /// Set of local variables at the stack frame location.
    /// Note that this might not be populated for all stack frames.
    pub locals: Option<Vec<Variable>>,
    /// Source location of the call site.
    pub location: Option<SourceLocation>,
}

impl Part for StackFrame {}


/// Response for listing debuggees.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees list debugger](struct.DebuggerDebuggeeListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDebuggeesResponse {
    /// List of debuggees accessible to the calling user.
    /// The fields `debuggee.id` and `description` are guaranteed to be set.
    /// The `description` field is a human readable field provided by agents and
    /// can be displayed to users.
    pub debuggees: Option<Vec<Debuggee>>,
}

impl ResponseResult for ListDebuggeesResponse {}


/// Request to register a debuggee.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees register controller](struct.ControllerDebuggeeRegisterCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegisterDebuggeeRequest {
    /// Debuggee information to register.
    /// The fields `project`, `uniquifier`, `description` and `agent_version`
    /// of the debuggee must be set.
    pub debuggee: Option<Debuggee>,
}

impl RequestValue for RegisterDebuggeeRequest {}


/// Response for updating an active breakpoint.
/// The message is defined to allow future extensions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints update controller](struct.ControllerDebuggeeBreakpointUpdateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateActiveBreakpointResponse { _never_set: Option<bool> }

impl ResponseResult for UpdateActiveBreakpointResponse {}


/// Selects a repo using a Google Cloud Platform project ID
/// (e.g. winged-cargo-31) and a repo name within that project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectRepoId {
    /// The ID of the project.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// The name of the repo. Leave empty for the default repo.
    #[serde(rename="repoName")]
    pub repo_name: Option<String>,
}

impl Part for ProjectRepoId {}


/// An alias to a repo revision.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AliasContext {
    /// The alias kind.
    pub kind: Option<String>,
    /// The alias name.
    pub name: Option<String>,
}

impl Part for AliasContext {}


/// Response for listing active breakpoints.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints list controller](struct.ControllerDebuggeeBreakpointListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListActiveBreakpointsResponse {
    /// If set to `true`, indicates that there is no change to the
    /// list of active breakpoints and the server-selected timeout has expired.
    /// The `breakpoints` field would be empty and should be ignored.
    #[serde(rename="waitExpired")]
    pub wait_expired: Option<bool>,
    /// A token that can be used in the next method call to block until
    /// the list of breakpoints changes.
    #[serde(rename="nextWaitToken")]
    pub next_wait_token: Option<String>,
    /// List of all active breakpoints.
    /// The fields `id` and `location` are guaranteed to be set on each breakpoint.
    pub breakpoints: Option<Vec<Breakpoint>>,
}

impl ResponseResult for ListActiveBreakpointsResponse {}


/// A CloudRepoSourceContext denotes a particular revision in a cloud
/// repo (a repo hosted by the Google Cloud Platform).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRepoSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    pub alias_context: Option<AliasContext>,
    /// A revision ID.
    #[serde(rename="revisionId")]
    pub revision_id: Option<String>,
    /// The name of an alias (branch, tag, etc.).
    #[serde(rename="aliasName")]
    pub alias_name: Option<String>,
    /// The ID of the repo.
    #[serde(rename="repoId")]
    pub repo_id: Option<RepoId>,
}

impl Part for CloudRepoSourceContext {}


/// A unique identifier for a cloud repo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepoId {
    /// A combination of a project ID and a repo name.
    #[serde(rename="projectRepoId")]
    pub project_repo_id: Option<ProjectRepoId>,
    /// A server-assigned, globally unique identifier.
    pub uid: Option<String>,
}

impl Part for RepoId {}


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
/// * [debuggees breakpoints delete debugger](struct.DebuggerDebuggeeBreakpointDeleteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// A SourceContext referring to a Gerrit project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GerritSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    pub alias_context: Option<AliasContext>,
    /// A revision (commit) ID.
    #[serde(rename="revisionId")]
    pub revision_id: Option<String>,
    /// The URI of a running Gerrit instance.
    #[serde(rename="hostUri")]
    pub host_uri: Option<String>,
    /// The name of an alias (branch, tag, etc.).
    #[serde(rename="aliasName")]
    pub alias_name: Option<String>,
    /// The full project name within the host. Projects may be nested, so
    /// "project/subproject" is a valid project name.
    /// The "repo name" is hostURI/project.
    #[serde(rename="gerritProject")]
    pub gerrit_project: Option<String>,
}

impl Part for GerritSourceContext {}


/// Response for registering a debuggee.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees register controller](struct.ControllerDebuggeeRegisterCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegisterDebuggeeResponse {
    /// Debuggee resource.
    /// The field `id` is guaranteed to be set (in addition to the echoed fields).
    /// If the field `is_disabled` is set to `true`, the agent should disable
    /// itself by removing all breakpoints and detaching from the application.
    /// It should however continue to poll `RegisterDebuggee` until reenabled.
    pub debuggee: Option<Debuggee>,
}

impl ResponseResult for RegisterDebuggeeResponse {}


/// A SourceContext is a reference to a tree of files. A SourceContext together
/// with a path point to a unique revision of a single file or directory.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceContext {
    /// A SourceContext referring to a snapshot in a cloud workspace.
    #[serde(rename="cloudWorkspace")]
    pub cloud_workspace: Option<CloudWorkspaceSourceContext>,
    /// A SourceContext referring to a revision in a cloud repo.
    #[serde(rename="cloudRepo")]
    pub cloud_repo: Option<CloudRepoSourceContext>,
    /// A SourceContext referring to any third party Git repo (e.g. GitHub).
    pub git: Option<GitSourceContext>,
    /// A SourceContext referring to a Gerrit project.
    pub gerrit: Option<GerritSourceContext>,
}

impl Part for SourceContext {}


/// Request to update an active breakpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints update controller](struct.ControllerDebuggeeBreakpointUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateActiveBreakpointRequest {
    /// Updated breakpoint information.
    /// The field `id` must be set.
    /// The agent must echo all Breakpoint specification fields in the update.
    pub breakpoint: Option<Breakpoint>,
}

impl RequestValue for UpdateActiveBreakpointRequest {}


/// Represents a location in the source code.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Column within a line. The first column in a line as the value `1`.
    /// Agents that do not support setting breakpoints on specific columns ignore
    /// this field.
    pub column: Option<i32>,
    /// Path to the source file within the source context of the target binary.
    pub path: Option<String>,
    /// Line inside the file. The first line in the file has the value `1`.
    pub line: Option<i32>,
}

impl Part for SourceLocation {}


/// Represents the debugged application. The application may include one or more
/// replicated processes executing the same code. Each of these processes is
/// attached with a debugger agent, carrying out the debugging commands.
/// Agents attached to the same debuggee identify themselves as such by using
/// exactly the same Debuggee message value when registering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Debuggee {
    /// Human readable message to be displayed to the user about this debuggee.
    /// Absence of this field indicates no status. The message can be either
    /// informational or an error status.
    pub status: Option<StatusMessage>,
    /// Human readable description of the debuggee.
    /// Including a human-readable project name, environment name and version
    /// information is recommended.
    pub description: Option<String>,
    /// If set to `true`, indicates that the agent should disable itself and
    /// detach from the debuggee.
    #[serde(rename="isDisabled")]
    pub is_disabled: Option<bool>,
    /// A set of custom debuggee properties, populated by the agent, to be
    /// displayed to the user.
    pub labels: Option<HashMap<String, String>>,
    /// Uniquifier to further distinguish the application.
    /// It is possible that different applications might have identical values in
    /// the debuggee message, thus, incorrectly identified as a single application
    /// by the Controller service. This field adds salt to further distinguish the
    /// application. Agents should consider seeding this field with value that
    /// identifies the code, binary, configuration and environment.
    pub uniquifier: Option<String>,
    /// Project the debuggee is associated with.
    /// Use project number or id when registering a Google Cloud Platform project.
    pub project: Option<String>,
    /// References to the locations and revisions of the source code used in the
    /// deployed application.
    #[serde(rename="sourceContexts")]
    pub source_contexts: Option<Vec<SourceContext>>,
    /// References to the locations and revisions of the source code used in the
    /// deployed application.
    #[serde(rename="extSourceContexts")]
    pub ext_source_contexts: Option<Vec<ExtendedSourceContext>>,
    /// Version ID of the agent.
    /// Schema: `domain/language-platform/vmajor.minor` (for example
    /// `google.com/java-gcp/v1.1`).
    #[serde(rename="agentVersion")]
    pub agent_version: Option<String>,
    /// If set to `true`, indicates that Controller service does not detect any
    /// activity from the debuggee agents and the application is possibly stopped.
    #[serde(rename="isInactive")]
    pub is_inactive: Option<bool>,
    /// Unique identifier for the debuggee generated by the controller service.
    pub id: Option<String>,
}

impl Part for Debuggee {}


/// Represents a message with parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FormatMessage {
    /// Optional parameters to be embedded into the message.
    pub parameters: Option<Vec<String>>,
    /// Format template for the message. The `format` uses placeholders `$0`,
    /// `$1`, etc. to reference parameters. `$$` can be used to denote the `$`
    /// character.
    /// 
    /// Examples:
    /// 
    /// *   `Failed to load '$0' which helps debug $1 the first time it
    ///     is loaded.  Again, $0 is very important.`
    /// *   `Please pay $$10 to use $0 instead of $1.`
    pub format: Option<String>,
}

impl Part for FormatMessage {}


/// A GitSourceContext denotes a particular revision in a third party Git
/// repository (e.g. GitHub).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitSourceContext {
    /// Git repository URL.
    pub url: Option<String>,
    /// Git commit hash.
    /// required.
    #[serde(rename="revisionId")]
    pub revision_id: Option<String>,
}

impl Part for GitSourceContext {}


/// A CloudWorkspaceId is a unique identifier for a cloud workspace.
/// A cloud workspace is a place associated with a repo where modified files
/// can be stored before they are committed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudWorkspaceId {
    /// The unique name of the workspace within the repo.  This is the name
    /// chosen by the client in the Source API's CreateWorkspace method.
    pub name: Option<String>,
    /// The ID of the repo containing the workspace.
    #[serde(rename="repoId")]
    pub repo_id: Option<RepoId>,
}

impl Part for CloudWorkspaceId {}


/// Represents the breakpoint specification, status and results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints set debugger](struct.DebuggerDebuggeeBreakpointSetCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Breakpoint {
    /// Breakpoint status.
    /// 
    /// The status includes an error flag and a human readable message.
    /// This field is usually unset. The message can be either
    /// informational or an error message. Regardless, clients should always
    /// display the text message back to the user.
    /// 
    /// Error status indicates complete failure of the breakpoint.
    /// 
    /// Example (non-final state): `Still loading symbols...`
    /// 
    /// Examples (final state):
    /// 
    /// *   `Invalid line number` referring to location
    /// *   `Field f not found in class C` referring to condition
    pub status: Option<StatusMessage>,
    /// The `variable_table` exists to aid with computation, memory and network
    /// traffic optimization.  It enables storing a variable once and reference
    /// it from multiple variables, including variables stored in the
    /// `variable_table` itself.
    /// For example, the same `this` object, which may appear at many levels of
    /// the stack, can have all of its data stored once in this table.  The
    /// stack frame variables then would hold only a reference to it.
    /// 
    /// The variable `var_table_index` field is an index into this repeated field.
    /// The stored objects are nameless and get their name from the referencing
    /// variable. The effective variable is a merge of the referencing variable
    /// and the referenced variable.
    #[serde(rename="variableTable")]
    pub variable_table: Option<Vec<Variable>>,
    /// Time this breakpoint was finalized as seen by the server in seconds
    /// resolution.
    #[serde(rename="finalTime")]
    pub final_time: Option<String>,
    /// Indicates the severity of the log. Only relevant when action is `LOG`.
    #[serde(rename="logLevel")]
    pub log_level: Option<String>,
    /// A set of custom breakpoint properties, populated by the agent, to be
    /// displayed to the user.
    pub labels: Option<HashMap<String, String>>,
    /// E-mail address of the user that created this breakpoint
    #[serde(rename="userEmail")]
    pub user_email: Option<String>,
    /// The stack at breakpoint time, where stack_frames[0] represents the most
    /// recently entered function.
    #[serde(rename="stackFrames")]
    pub stack_frames: Option<Vec<StackFrame>>,
    /// List of read-only expressions to evaluate at the breakpoint location.
    /// The expressions are composed using expressions in the programming language
    /// at the source location. If the breakpoint action is `LOG`, the evaluated
    /// expressions are included in log statements.
    pub expressions: Option<Vec<String>>,
    /// Values of evaluated expressions at breakpoint time.
    /// The evaluated expressions appear in exactly the same order they
    /// are listed in the `expressions` field.
    /// The `name` field holds the original expression text, the `value` or
    /// `members` field holds the result of the evaluated expression.
    /// If the expression cannot be evaluated, the `status` inside the `Variable`
    /// will indicate an error and contain the error text.
    #[serde(rename="evaluatedExpressions")]
    pub evaluated_expressions: Option<Vec<Variable>>,
    /// Time this breakpoint was created by the server in seconds resolution.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// Condition that triggers the breakpoint.
    /// The condition is a compound boolean expression composed using expressions
    /// in a programming language at the source location.
    pub condition: Option<String>,
    /// Only relevant when action is `LOG`. Defines the message to log when
    /// the breakpoint hits. The message may include parameter placeholders `$0`,
    /// `$1`, etc. These placeholders are replaced with the evaluated value
    /// of the appropriate expression. Expressions not referenced in
    /// `log_message_format` are not logged.
    /// 
    /// Example: `Message received, id = $0, count = $1` with
    /// `expressions` = `[ message.id, message.count ]`.
    #[serde(rename="logMessageFormat")]
    pub log_message_format: Option<String>,
    /// Breakpoint identifier, unique in the scope of the debuggee.
    pub id: Option<String>,
    /// Breakpoint source location.
    pub location: Option<SourceLocation>,
    /// Action that the agent should perform when the code at the
    /// breakpoint location is hit.
    pub action: Option<String>,
    /// When true, indicates that this is a final result and the
    /// breakpoint state will not change from here on.
    #[serde(rename="isFinalState")]
    pub is_final_state: Option<bool>,
}

impl RequestValue for Breakpoint {}


/// Represents a variable or an argument possibly of a compound object type.
/// Note how the following variables are represented:
/// 
/// 1) A simple variable:
/// 
///     int x = 5
/// 
///     { name: "x", value: "5", type: "int" }  // Captured variable
/// 
/// 2) A compound object:
/// 
///     struct T {
///         int m1;
///         int m2;
///     };
///     T x = { 3, 7 };
/// 
///     {  // Captured variable
///         name: "x",
///         type: "T",
///         members { name: "m1", value: "3", type: "int" },
///         members { name: "m2", value: "7", type: "int" }
///     }
/// 
/// 3) A pointer where the pointee was captured:
/// 
///     T x = { 3, 7 };
///     T* p = &x;
/// 
///     {   // Captured variable
///         name: "p",
///         type: "T*",
///         value: "0x00500500",
///         members { name: "m1", value: "3", type: "int" },
///         members { name: "m2", value: "7", type: "int" }
///     }
/// 
/// 4) A pointer where the pointee was not captured:
/// 
///     T* p = new T;
/// 
///     {   // Captured variable
///         name: "p",
///         type: "T*",
///         value: "0x00400400"
///         status { is_error: true, description { format: "unavailable" } }
///     }
/// 
/// The status should describe the reason for the missing value,
/// such as `<optimized out>`, `<inaccessible>`, `<pointers limit reached>`.
/// 
/// Note that a null pointer should not have members.
/// 
/// 5) An unnamed value:
/// 
///     int* p = new int(7);
/// 
///     {   // Captured variable
///         name: "p",
///         value: "0x00500500",
///         type: "int*",
///         members { value: "7", type: "int" } }
/// 
/// 6) An unnamed pointer where the pointee was not captured:
/// 
///     int* p = new int(7);
///     int** pp = &p;
/// 
///     {  // Captured variable
///         name: "pp",
///         value: "0x00500500",
///         type: "int**",
///         members {
///             value: "0x00400400",
///             type: "int*"
///             status {
///                 is_error: true,
///                 description: { format: "unavailable" } }
///             }
///         }
///     }
/// 
/// To optimize computation, memory and network traffic, variables that
/// repeat in the output multiple times can be stored once in a shared
/// variable table and be referenced using the `var_table_index` field.  The
/// variables stored in the shared table are nameless and are essentially
/// a partition of the complete variable. To reconstruct the complete
/// variable, merge the referencing variable with the referenced variable.
/// 
/// When using the shared variable table, the following variables:
/// 
///     T x = { 3, 7 };
///     T* p = &x;
///     T& r = x;
/// 
///     { name: "x", var_table_index: 3, type: "T" }  // Captured variables
///     { name: "p", value "0x00500500", type="T*", var_table_index: 3 }
///     { name: "r", type="T&", var_table_index: 3 }
/// 
///     {  // Shared variable table entry #3:
///         members { name: "m1", value: "3", type: "int" },
///         members { name: "m2", value: "7", type: "int" }
///     }
/// 
/// Note that the pointer address is stored with the referencing variable
/// and not with the referenced variable. This allows the referenced variable
/// to be shared between pointers and references.
/// 
/// The type field is optional. The debugger agent may or may not support it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    /// Status associated with the variable. This field will usually stay
    /// unset. A status of a single variable only applies to that variable or
    /// expression. The rest of breakpoint data still remains valid. Variables
    /// might be reported in error state even when breakpoint is not in final
    /// state.
    /// 
    /// The message may refer to variable name with `refers_to` set to
    /// `VARIABLE_NAME`. Alternatively `refers_to` will be set to `VARIABLE_VALUE`.
    /// In either case variable value and members will be unset.
    /// 
    /// Example of error message applied to name: `Invalid expression syntax`.
    /// 
    /// Example of information message applied to value: `Not captured`.
    /// 
    /// Examples of error message applied to value:
    /// 
    /// *   `Malformed string`,
    /// *   `Field f not found in class C`
    /// *   `Null pointer dereference`
    pub status: Option<StatusMessage>,
    /// Name of the variable, if any.
    pub name: Option<String>,
    /// Reference to a variable in the shared variable table. More than
    /// one variable can reference the same variable in the table. The
    /// `var_table_index` field is an index into `variable_table` in Breakpoint.
    #[serde(rename="varTableIndex")]
    pub var_table_index: Option<i32>,
    /// Variable type (e.g. `MyClass`). If the variable is split with
    /// `var_table_index`, `type` goes next to `value`. The interpretation of
    /// a type is agent specific. It is recommended to include the dynamic type
    /// rather than a static type of an object.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Members contained or pointed to by the variable.
    pub members: Option<Vec<Variable>>,
    /// Simple value of the variable.
    pub value: Option<String>,
}

impl Part for Variable {}


/// Response for setting a breakpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints set debugger](struct.DebuggerDebuggeeBreakpointSetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetBreakpointResponse {
    /// Breakpoint resource.
    /// The field `id` is guaranteed to be set (in addition to the echoed fileds).
    pub breakpoint: Option<Breakpoint>,
}

impl ResponseResult for SetBreakpointResponse {}


/// Represents a contextual status message.
/// The message can indicate an error or informational status, and refer to
/// specific parts of the containing object.
/// For example, the `Breakpoint.status` field can indicate an error referring
/// to the `BREAKPOINT_SOURCE_LOCATION` with the message `Location not found`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatusMessage {
    /// Status message text.
    pub description: Option<FormatMessage>,
    /// Reference to which the message applies.
    #[serde(rename="refersTo")]
    pub refers_to: Option<String>,
    /// Distinguishes errors from informational messages.
    #[serde(rename="isError")]
    pub is_error: Option<bool>,
}

impl Part for StatusMessage {}


/// Response for getting breakpoint information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints get debugger](struct.DebuggerDebuggeeBreakpointGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetBreakpointResponse {
    /// Complete breakpoint state.
    /// The fields `id` and `location` are guaranteed to be set.
    pub breakpoint: Option<Breakpoint>,
}

impl ResponseResult for GetBreakpointResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *controller* resources.
/// It is not used directly, but through the `CloudDebugger` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_clouddebugger2 as clouddebugger2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use clouddebugger2::CloudDebugger;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `debuggees_breakpoints_list(...)`, `debuggees_breakpoints_update(...)` and `debuggees_register(...)`
/// // to build up your call.
/// let rb = hub.controller();
/// # }
/// ```
pub struct ControllerMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
}

impl<'a, C, A> MethodsBuilder for ControllerMethods<'a, C, A> {}

impl<'a, C, A> ControllerMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the breakpoint state or mutable fields.
    /// The entire Breakpoint message must be sent back to the controller service.
    /// 
    /// Updates to active breakpoint fields are only allowed if the new value
    /// does not change the breakpoint specification. Updates to the `location`,
    /// `condition` and `expressions` fields should not alter the breakpoint
    /// semantics. These may only make changes such as canonicalizing a value
    /// or snapping the location to the correct line of code.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `debuggeeId` - Identifies the debuggee being debugged.
    /// * `id` - Breakpoint identifier, unique in the scope of the debuggee.
    pub fn debuggees_breakpoints_update(&self, request: UpdateActiveBreakpointRequest, debuggee_id: &str, id: &str) -> ControllerDebuggeeBreakpointUpdateCall<'a, C, A> {
        ControllerDebuggeeBreakpointUpdateCall {
            hub: self.hub,
            _request: request,
            _debuggee_id: debuggee_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Registers the debuggee with the controller service.
    /// 
    /// All agents attached to the same application must call this method with
    /// exactly the same request content to get back the same stable `debuggee_id`.
    /// Agents should call this method again whenever `google.rpc.Code.NOT_FOUND`
    /// is returned from any controller method.
    /// 
    /// This protocol allows the controller service to disable debuggees, recover
    /// from data loss, or change the `debuggee_id` format. Agents must handle
    /// `debuggee_id` value changing upon re-registration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn debuggees_register(&self, request: RegisterDebuggeeRequest) -> ControllerDebuggeeRegisterCall<'a, C, A> {
        ControllerDebuggeeRegisterCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of all active breakpoints for the debuggee.
    /// 
    /// The breakpoint specification (`location`, `condition`, and `expressions`
    /// fields) is semantically immutable, although the field values may
    /// change. For example, an agent may update the location line number
    /// to reflect the actual line where the breakpoint was set, but this
    /// doesn't change the breakpoint semantics.
    /// 
    /// This means that an agent does not need to check if a breakpoint has changed
    /// when it encounters the same breakpoint on a successive call.
    /// Moreover, an agent should remember the breakpoints that are completed
    /// until the controller removes them from the active list to avoid
    /// setting those breakpoints again.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - Identifies the debuggee.
    pub fn debuggees_breakpoints_list(&self, debuggee_id: &str) -> ControllerDebuggeeBreakpointListCall<'a, C, A> {
        ControllerDebuggeeBreakpointListCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _wait_token: Default::default(),
            _success_on_timeout: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *debugger* resources.
/// It is not used directly, but through the `CloudDebugger` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_clouddebugger2 as clouddebugger2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use clouddebugger2::CloudDebugger;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `debuggees_breakpoints_delete(...)`, `debuggees_breakpoints_get(...)`, `debuggees_breakpoints_list(...)`, `debuggees_breakpoints_set(...)` and `debuggees_list(...)`
/// // to build up your call.
/// let rb = hub.debugger();
/// # }
/// ```
pub struct DebuggerMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
}

impl<'a, C, A> MethodsBuilder for DebuggerMethods<'a, C, A> {}

impl<'a, C, A> DebuggerMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets breakpoint information.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - ID of the debuggee whose breakpoint to get.
    /// * `breakpointId` - ID of the breakpoint to get.
    pub fn debuggees_breakpoints_get(&self, debuggee_id: &str, breakpoint_id: &str) -> DebuggerDebuggeeBreakpointGetCall<'a, C, A> {
        DebuggerDebuggeeBreakpointGetCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _breakpoint_id: breakpoint_id.to_string(),
            _client_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the breakpoint from the debuggee.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - ID of the debuggee whose breakpoint to delete.
    /// * `breakpointId` - ID of the breakpoint to delete.
    pub fn debuggees_breakpoints_delete(&self, debuggee_id: &str, breakpoint_id: &str) -> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A> {
        DebuggerDebuggeeBreakpointDeleteCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _breakpoint_id: breakpoint_id.to_string(),
            _client_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the debuggees that the user has access to.
    pub fn debuggees_list(&self) -> DebuggerDebuggeeListCall<'a, C, A> {
        DebuggerDebuggeeListCall {
            hub: self.hub,
            _project: Default::default(),
            _include_inactive: Default::default(),
            _client_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the breakpoint to the debuggee.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `debuggeeId` - ID of the debuggee where the breakpoint is to be set.
    pub fn debuggees_breakpoints_set(&self, request: Breakpoint, debuggee_id: &str) -> DebuggerDebuggeeBreakpointSetCall<'a, C, A> {
        DebuggerDebuggeeBreakpointSetCall {
            hub: self.hub,
            _request: request,
            _debuggee_id: debuggee_id.to_string(),
            _client_version: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all breakpoints for the debuggee.
    /// 
    /// # Arguments
    ///
    /// * `debuggeeId` - ID of the debuggee whose breakpoints to list.
    pub fn debuggees_breakpoints_list(&self, debuggee_id: &str) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        DebuggerDebuggeeBreakpointListCall {
            hub: self.hub,
            _debuggee_id: debuggee_id.to_string(),
            _wait_token: Default::default(),
            _strip_results: Default::default(),
            _include_inactive: Default::default(),
            _include_all_users: Default::default(),
            _client_version: Default::default(),
            _action_value: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Updates the breakpoint state or mutable fields.
/// The entire Breakpoint message must be sent back to the controller service.
/// 
/// Updates to active breakpoint fields are only allowed if the new value
/// does not change the breakpoint specification. Updates to the `location`,
/// `condition` and `expressions` fields should not alter the breakpoint
/// semantics. These may only make changes such as canonicalizing a value
/// or snapping the location to the correct line of code.
///
/// A builder for the *debuggees.breakpoints.update* method supported by a *controller* resource.
/// It is not used directly, but through a `ControllerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// use clouddebugger2::UpdateActiveBreakpointRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UpdateActiveBreakpointRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.controller().debuggees_breakpoints_update(req, "debuggeeId", "id")
///              .doit();
/// # }
/// ```
pub struct ControllerDebuggeeBreakpointUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _request: UpdateActiveBreakpointRequest,
    _debuggee_id: String,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ControllerDebuggeeBreakpointUpdateCall<'a, C, A> {}

impl<'a, C, A> ControllerDebuggeeBreakpointUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UpdateActiveBreakpointResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.controller.debuggees.breakpoints.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("debuggeeId", self._debuggee_id.to_string()));
        params.push(("id", self._id.to_string()));
        for &field in ["alt", "debuggeeId", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/controller/debuggees/{debuggeeId}/breakpoints/{id}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{debuggeeId}", "debuggeeId"), ("{id}", "id")].iter() {
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
            for param_name in ["id", "debuggeeId"].iter() {
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
    pub fn request(mut self, new_value: UpdateActiveBreakpointRequest) -> ControllerDebuggeeBreakpointUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Identifies the debuggee being debugged.
    ///
    /// Sets the *debuggee id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn debuggee_id(mut self, new_value: &str) -> ControllerDebuggeeBreakpointUpdateCall<'a, C, A> {
        self._debuggee_id = new_value.to_string();
        self
    }
    /// Breakpoint identifier, unique in the scope of the debuggee.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> ControllerDebuggeeBreakpointUpdateCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ControllerDebuggeeBreakpointUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ControllerDebuggeeBreakpointUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ControllerDebuggeeBreakpointUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Registers the debuggee with the controller service.
/// 
/// All agents attached to the same application must call this method with
/// exactly the same request content to get back the same stable `debuggee_id`.
/// Agents should call this method again whenever `google.rpc.Code.NOT_FOUND`
/// is returned from any controller method.
/// 
/// This protocol allows the controller service to disable debuggees, recover
/// from data loss, or change the `debuggee_id` format. Agents must handle
/// `debuggee_id` value changing upon re-registration.
///
/// A builder for the *debuggees.register* method supported by a *controller* resource.
/// It is not used directly, but through a `ControllerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// use clouddebugger2::RegisterDebuggeeRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RegisterDebuggeeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.controller().debuggees_register(req)
///              .doit();
/// # }
/// ```
pub struct ControllerDebuggeeRegisterCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _request: RegisterDebuggeeRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ControllerDebuggeeRegisterCall<'a, C, A> {}

impl<'a, C, A> ControllerDebuggeeRegisterCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RegisterDebuggeeResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.controller.debuggees.register",
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

        let mut url = self.hub._base_url.clone() + "v2/controller/debuggees/register";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: RegisterDebuggeeRequest) -> ControllerDebuggeeRegisterCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ControllerDebuggeeRegisterCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ControllerDebuggeeRegisterCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ControllerDebuggeeRegisterCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the list of all active breakpoints for the debuggee.
/// 
/// The breakpoint specification (`location`, `condition`, and `expressions`
/// fields) is semantically immutable, although the field values may
/// change. For example, an agent may update the location line number
/// to reflect the actual line where the breakpoint was set, but this
/// doesn't change the breakpoint semantics.
/// 
/// This means that an agent does not need to check if a breakpoint has changed
/// when it encounters the same breakpoint on a successive call.
/// Moreover, an agent should remember the breakpoints that are completed
/// until the controller removes them from the active list to avoid
/// setting those breakpoints again.
///
/// A builder for the *debuggees.breakpoints.list* method supported by a *controller* resource.
/// It is not used directly, but through a `ControllerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.controller().debuggees_breakpoints_list("debuggeeId")
///              .wait_token("justo")
///              .success_on_timeout(true)
///              .doit();
/// # }
/// ```
pub struct ControllerDebuggeeBreakpointListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _debuggee_id: String,
    _wait_token: Option<String>,
    _success_on_timeout: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ControllerDebuggeeBreakpointListCall<'a, C, A> {}

impl<'a, C, A> ControllerDebuggeeBreakpointListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListActiveBreakpointsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.controller.debuggees.breakpoints.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("debuggeeId", self._debuggee_id.to_string()));
        if let Some(value) = self._wait_token {
            params.push(("waitToken", value.to_string()));
        }
        if let Some(value) = self._success_on_timeout {
            params.push(("successOnTimeout", value.to_string()));
        }
        for &field in ["alt", "debuggeeId", "waitToken", "successOnTimeout"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/controller/debuggees/{debuggeeId}/breakpoints";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{debuggeeId}", "debuggeeId")].iter() {
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
            for param_name in ["debuggeeId"].iter() {
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


    /// Identifies the debuggee.
    ///
    /// Sets the *debuggee id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn debuggee_id(mut self, new_value: &str) -> ControllerDebuggeeBreakpointListCall<'a, C, A> {
        self._debuggee_id = new_value.to_string();
        self
    }
    /// A token that, if specified, blocks the method call until the list
    /// of active breakpoints has changed, or a server-selected timeout has
    /// expired. The value should be set from the `next_wait_token` field in
    /// the last response. The initial value should be set to `"init"`.
    ///
    /// Sets the *wait token* query property to the given value.
    pub fn wait_token(mut self, new_value: &str) -> ControllerDebuggeeBreakpointListCall<'a, C, A> {
        self._wait_token = Some(new_value.to_string());
        self
    }
    /// If set to `true` (recommended), returns `google.rpc.Code.OK` status and
    /// sets the `wait_expired` response field to `true` when the server-selected
    /// timeout has expired.
    /// 
    /// If set to `false` (deprecated), returns `google.rpc.Code.ABORTED` status
    /// when the server-selected timeout has expired.
    ///
    /// Sets the *success on timeout* query property to the given value.
    pub fn success_on_timeout(mut self, new_value: bool) -> ControllerDebuggeeBreakpointListCall<'a, C, A> {
        self._success_on_timeout = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ControllerDebuggeeBreakpointListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ControllerDebuggeeBreakpointListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ControllerDebuggeeBreakpointListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets breakpoint information.
///
/// A builder for the *debuggees.breakpoints.get* method supported by a *debugger* resource.
/// It is not used directly, but through a `DebuggerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.debugger().debuggees_breakpoints_get("debuggeeId", "breakpointId")
///              .client_version("sea")
///              .doit();
/// # }
/// ```
pub struct DebuggerDebuggeeBreakpointGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _debuggee_id: String,
    _breakpoint_id: String,
    _client_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DebuggerDebuggeeBreakpointGetCall<'a, C, A> {}

impl<'a, C, A> DebuggerDebuggeeBreakpointGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetBreakpointResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.debugger.debuggees.breakpoints.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("debuggeeId", self._debuggee_id.to_string()));
        params.push(("breakpointId", self._breakpoint_id.to_string()));
        if let Some(value) = self._client_version {
            params.push(("clientVersion", value.to_string()));
        }
        for &field in ["alt", "debuggeeId", "breakpointId", "clientVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/debugger/debuggees/{debuggeeId}/breakpoints/{breakpointId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{debuggeeId}", "debuggeeId"), ("{breakpointId}", "breakpointId")].iter() {
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
            for param_name in ["breakpointId", "debuggeeId"].iter() {
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


    /// ID of the debuggee whose breakpoint to get.
    ///
    /// Sets the *debuggee id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn debuggee_id(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointGetCall<'a, C, A> {
        self._debuggee_id = new_value.to_string();
        self
    }
    /// ID of the breakpoint to get.
    ///
    /// Sets the *breakpoint id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn breakpoint_id(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointGetCall<'a, C, A> {
        self._breakpoint_id = new_value.to_string();
        self
    }
    /// The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    ///
    /// Sets the *client version* query property to the given value.
    pub fn client_version(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointGetCall<'a, C, A> {
        self._client_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DebuggerDebuggeeBreakpointGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DebuggerDebuggeeBreakpointGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DebuggerDebuggeeBreakpointGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the breakpoint from the debuggee.
///
/// A builder for the *debuggees.breakpoints.delete* method supported by a *debugger* resource.
/// It is not used directly, but through a `DebuggerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.debugger().debuggees_breakpoints_delete("debuggeeId", "breakpointId")
///              .client_version("gubergren")
///              .doit();
/// # }
/// ```
pub struct DebuggerDebuggeeBreakpointDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _debuggee_id: String,
    _breakpoint_id: String,
    _client_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DebuggerDebuggeeBreakpointDeleteCall<'a, C, A> {}

impl<'a, C, A> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.debugger.debuggees.breakpoints.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("debuggeeId", self._debuggee_id.to_string()));
        params.push(("breakpointId", self._breakpoint_id.to_string()));
        if let Some(value) = self._client_version {
            params.push(("clientVersion", value.to_string()));
        }
        for &field in ["alt", "debuggeeId", "breakpointId", "clientVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/debugger/debuggees/{debuggeeId}/breakpoints/{breakpointId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{debuggeeId}", "debuggeeId"), ("{breakpointId}", "breakpointId")].iter() {
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
            for param_name in ["breakpointId", "debuggeeId"].iter() {
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


    /// ID of the debuggee whose breakpoint to delete.
    ///
    /// Sets the *debuggee id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn debuggee_id(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A> {
        self._debuggee_id = new_value.to_string();
        self
    }
    /// ID of the breakpoint to delete.
    ///
    /// Sets the *breakpoint id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn breakpoint_id(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A> {
        self._breakpoint_id = new_value.to_string();
        self
    }
    /// The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    ///
    /// Sets the *client version* query property to the given value.
    pub fn client_version(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A> {
        self._client_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DebuggerDebuggeeBreakpointDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all the debuggees that the user has access to.
///
/// A builder for the *debuggees.list* method supported by a *debugger* resource.
/// It is not used directly, but through a `DebuggerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.debugger().debuggees_list()
///              .project("sadipscing")
///              .include_inactive(true)
///              .client_version("ea")
///              .doit();
/// # }
/// ```
pub struct DebuggerDebuggeeListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _project: Option<String>,
    _include_inactive: Option<bool>,
    _client_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DebuggerDebuggeeListCall<'a, C, A> {}

impl<'a, C, A> DebuggerDebuggeeListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListDebuggeesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.debugger.debuggees.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._project {
            params.push(("project", value.to_string()));
        }
        if let Some(value) = self._include_inactive {
            params.push(("includeInactive", value.to_string()));
        }
        if let Some(value) = self._client_version {
            params.push(("clientVersion", value.to_string()));
        }
        for &field in ["alt", "project", "includeInactive", "clientVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/debugger/debuggees";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Project number of a Google Cloud project whose debuggees to list.
    ///
    /// Sets the *project* query property to the given value.
    pub fn project(mut self, new_value: &str) -> DebuggerDebuggeeListCall<'a, C, A> {
        self._project = Some(new_value.to_string());
        self
    }
    /// When set to `true`, the result includes all debuggees. Otherwise, the
    /// result includes only debuggees that are active.
    ///
    /// Sets the *include inactive* query property to the given value.
    pub fn include_inactive(mut self, new_value: bool) -> DebuggerDebuggeeListCall<'a, C, A> {
        self._include_inactive = Some(new_value);
        self
    }
    /// The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    ///
    /// Sets the *client version* query property to the given value.
    pub fn client_version(mut self, new_value: &str) -> DebuggerDebuggeeListCall<'a, C, A> {
        self._client_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DebuggerDebuggeeListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DebuggerDebuggeeListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DebuggerDebuggeeListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets the breakpoint to the debuggee.
///
/// A builder for the *debuggees.breakpoints.set* method supported by a *debugger* resource.
/// It is not used directly, but through a `DebuggerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// use clouddebugger2::Breakpoint;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Breakpoint::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.debugger().debuggees_breakpoints_set(req, "debuggeeId")
///              .client_version("justo")
///              .doit();
/// # }
/// ```
pub struct DebuggerDebuggeeBreakpointSetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _request: Breakpoint,
    _debuggee_id: String,
    _client_version: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DebuggerDebuggeeBreakpointSetCall<'a, C, A> {}

impl<'a, C, A> DebuggerDebuggeeBreakpointSetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SetBreakpointResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.debugger.debuggees.breakpoints.set",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("debuggeeId", self._debuggee_id.to_string()));
        if let Some(value) = self._client_version {
            params.push(("clientVersion", value.to_string()));
        }
        for &field in ["alt", "debuggeeId", "clientVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/debugger/debuggees/{debuggeeId}/breakpoints/set";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{debuggeeId}", "debuggeeId")].iter() {
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
            for param_name in ["debuggeeId"].iter() {
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
    pub fn request(mut self, new_value: Breakpoint) -> DebuggerDebuggeeBreakpointSetCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// ID of the debuggee where the breakpoint is to be set.
    ///
    /// Sets the *debuggee id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn debuggee_id(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointSetCall<'a, C, A> {
        self._debuggee_id = new_value.to_string();
        self
    }
    /// The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    ///
    /// Sets the *client version* query property to the given value.
    pub fn client_version(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointSetCall<'a, C, A> {
        self._client_version = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DebuggerDebuggeeBreakpointSetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DebuggerDebuggeeBreakpointSetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DebuggerDebuggeeBreakpointSetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all breakpoints for the debuggee.
///
/// A builder for the *debuggees.breakpoints.list* method supported by a *debugger* resource.
/// It is not used directly, but through a `DebuggerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_clouddebugger2 as clouddebugger2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use clouddebugger2::CloudDebugger;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudDebugger::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.debugger().debuggees_breakpoints_list("debuggeeId")
///              .wait_token("et")
///              .strip_results(true)
///              .include_inactive(true)
///              .include_all_users(false)
///              .client_version("Lorem")
///              .action_value("et")
///              .doit();
/// # }
/// ```
pub struct DebuggerDebuggeeBreakpointListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudDebugger<C, A>,
    _debuggee_id: String,
    _wait_token: Option<String>,
    _strip_results: Option<bool>,
    _include_inactive: Option<bool>,
    _include_all_users: Option<bool>,
    _client_version: Option<String>,
    _action_value: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DebuggerDebuggeeBreakpointListCall<'a, C, A> {}

impl<'a, C, A> DebuggerDebuggeeBreakpointListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListBreakpointsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "clouddebugger.debugger.debuggees.breakpoints.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("debuggeeId", self._debuggee_id.to_string()));
        if let Some(value) = self._wait_token {
            params.push(("waitToken", value.to_string()));
        }
        if let Some(value) = self._strip_results {
            params.push(("stripResults", value.to_string()));
        }
        if let Some(value) = self._include_inactive {
            params.push(("includeInactive", value.to_string()));
        }
        if let Some(value) = self._include_all_users {
            params.push(("includeAllUsers", value.to_string()));
        }
        if let Some(value) = self._client_version {
            params.push(("clientVersion", value.to_string()));
        }
        if let Some(value) = self._action_value {
            params.push(("action.value", value.to_string()));
        }
        for &field in ["alt", "debuggeeId", "waitToken", "stripResults", "includeInactive", "includeAllUsers", "clientVersion", "action.value"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v2/debugger/debuggees/{debuggeeId}/breakpoints";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{debuggeeId}", "debuggeeId")].iter() {
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
            for param_name in ["debuggeeId"].iter() {
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


    /// ID of the debuggee whose breakpoints to list.
    ///
    /// Sets the *debuggee id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn debuggee_id(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        self._debuggee_id = new_value.to_string();
        self
    }
    /// A wait token that, if specified, blocks the call until the breakpoints
    /// list has changed, or a server selected timeout has expired.  The value
    /// should be set from the last response. The error code
    /// `google.rpc.Code.ABORTED` (RPC) is returned on wait timeout, which
    /// should be called again with the same `wait_token`.
    ///
    /// Sets the *wait token* query property to the given value.
    pub fn wait_token(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        self._wait_token = Some(new_value.to_string());
        self
    }
    /// This field is deprecated. The following fields are always stripped out of
    /// the result: `stack_frames`, `evaluated_expressions` and `variable_table`.
    ///
    /// Sets the *strip results* query property to the given value.
    pub fn strip_results(mut self, new_value: bool) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        self._strip_results = Some(new_value);
        self
    }
    /// When set to `true`, the response includes active and inactive
    /// breakpoints. Otherwise, it includes only active breakpoints.
    ///
    /// Sets the *include inactive* query property to the given value.
    pub fn include_inactive(mut self, new_value: bool) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        self._include_inactive = Some(new_value);
        self
    }
    /// When set to `true`, the response includes the list of breakpoints set by
    /// any user. Otherwise, it includes only breakpoints set by the caller.
    ///
    /// Sets the *include all users* query property to the given value.
    pub fn include_all_users(mut self, new_value: bool) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        self._include_all_users = Some(new_value);
        self
    }
    /// The client version making the call.
    /// Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).
    ///
    /// Sets the *client version* query property to the given value.
    pub fn client_version(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        self._client_version = Some(new_value.to_string());
        self
    }
    /// Only breakpoints with the specified action will pass the filter.
    ///
    /// Sets the *action.value* query property to the given value.
    pub fn action_value(mut self, new_value: &str) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
        self._action_value = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DebuggerDebuggeeBreakpointListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DebuggerDebuggeeBreakpointListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DebuggerDebuggeeBreakpointListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


