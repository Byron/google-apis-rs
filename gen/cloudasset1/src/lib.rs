// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Asset* crate version *1.0.11+20190625*, where *20190625* is the exact revision of the *cloudasset:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.11*.
//! 
//! Everything else about the *Cloud Asset* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/quickstart-cloud-asset-inventory).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/cloudasset1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CloudAsset.html) ... 
//! 
//! * [operations](struct.Operation.html)
//!  * [*get*](struct.OperationGetCall.html)
//! 
//! Other activities are ...
//! 
//! * [batch get assets history](struct.MethodBatchGetAssetsHistoryCall.html)
//! * [export assets](struct.MethodExportAssetCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.CloudAsset.html)**
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
//! let r = hub.methods().export_assets(...).doit()
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
//! google-cloudasset1 = "*"
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
//! extern crate google_cloudasset1 as cloudasset1;
//! use cloudasset1::ExportAssetsRequest;
//! use cloudasset1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use cloudasset1::CloudAsset;
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
//! let mut hub = CloudAsset::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = ExportAssetsRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.methods().export_assets(req, "parent")
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

/// Central instance to access all CloudAsset related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudasset1 as cloudasset1;
/// use cloudasset1::ExportAssetsRequest;
/// use cloudasset1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudasset1::CloudAsset;
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
/// let mut hub = CloudAsset::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ExportAssetsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().export_assets(req, "parent")
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
pub struct CloudAsset<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for CloudAsset<C, A> {}

impl<'a, C, A> CloudAsset<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CloudAsset<C, A> {
        CloudAsset {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.11".to_string(),
            _base_url: "https://cloudasset.googleapis.com/".to_string(),
            _root_url: "https://cloudasset.googleapis.com/".to_string(),
        }
    }

    pub fn methods(&'a self) -> MethodMethods<'a, C, A> {
        MethodMethods { hub: &self }
    }
    pub fn operations(&'a self) -> OperationMethods<'a, C, A> {
        OperationMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.11`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://cloudasset.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://cloudasset.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Provides the configuration for logging a type of permissions.
/// Example:
/// 
/// ````text
/// {
///   "audit_log_configs": [
///     {
///       "log_type": "DATA_READ",
///       "exempted_members": [
///         "user:foo@gmail.com"
///       ]
///     },
///     {
///       "log_type": "DATA_WRITE",
///     }
///   ]
/// }
/// ````
/// 
/// This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting
/// foo@gmail.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of
    /// permission.
    /// Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    pub log_type: Option<String>,
}

impl Part for AuditLogConfig {}


/// Ignores policies set above this resource and restores the
/// `constraint_default` enforcement behavior of the specific `Constraint` at
/// this resource.
/// 
/// Suppose that `constraint_default` is set to `ALLOW` for the
/// `Constraint` `constraints/serviceuser.services`. Suppose that organization
/// foo.com sets a `Policy` at their Organization resource node that restricts
/// the allowed service activations to deny all service activations. They
/// could then set a `Policy` with the `policy_type` `restore_default` on
/// several experimental projects, restoring the `constraint_default`
/// enforcement of the `Constraint` for only those projects, allowing those
/// projects to have all services activated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1RestoreDefault { _never_set: Option<bool> }

impl Part for GoogleCloudOrgpolicyV1RestoreDefault {}


/// Representation of a cloud resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceType {
    /// The API version. Example: "v1".
    pub version: Option<String>,
    /// The REST URL for accessing the resource. An HTTP GET operation using this
    /// URL returns the resource itself.
    /// Example:
    /// `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`.
    /// It will be left unspecified for resources without a REST API.
    #[serde(rename="resourceUrl")]
    pub resource_url: Option<String>,
    /// The full name of the immediate parent of this resource. See
    /// [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    /// 
    /// For GCP assets, it is the parent resource defined in the [Cloud IAM policy
    /// hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy).
    /// For example:
    /// `"//cloudresourcemanager.googleapis.com/projects/my_project_123"`.
    /// 
    /// For third-party assets, it is up to the users to define.
    pub parent: Option<String>,
    /// The URL of the discovery document containing the resource's JSON schema.
    /// For example:
    /// `"https://www.googleapis.com/discovery/v1/apis/compute/v1/rest"`.
    /// It will be left unspecified for resources without a discovery-based API,
    /// such as Cloud Bigtable.
    #[serde(rename="discoveryDocumentUri")]
    pub discovery_document_uri: Option<String>,
    /// The content of the resource, in which some sensitive fields are scrubbed
    /// away and may not be present.
    pub data: Option<HashMap<String, String>>,
    /// The JSON schema name listed in the discovery document.
    /// Example: "Project". It will be left unspecified for resources (such as
    /// Cloud Bigtable) without a discovery-based API.
    #[serde(rename="discoveryName")]
    pub discovery_name: Option<String>,
}

impl Part for ResourceType {}


/// Export asset request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [export assets](struct.MethodExportAssetCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportAssetsRequest {
    /// A list of asset types of which to take a snapshot for. For example:
    /// "compute.googleapis.com/Disk". If specified, only matching assets will be
    /// returned. See [Introduction to Cloud Asset
    /// Inventory](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview)
    /// for all supported asset types.
    #[serde(rename="assetTypes")]
    pub asset_types: Option<Vec<String>>,
    /// Asset content type. If not specified, no content but the asset name will be
    /// returned.
    #[serde(rename="contentType")]
    pub content_type: Option<String>,
    /// Required. Output configuration indicating where the results will be output
    /// to. All results will be in newline delimited JSON format.
    #[serde(rename="outputConfig")]
    pub output_config: Option<OutputConfig>,
    /// Timestamp to take an asset snapshot. This can only be set to a timestamp
    /// between 2018-10-02 UTC (inclusive) and the current time. If not specified,
    /// the current time will be used. Due to delays in resource data collection
    /// and indexing, there is a volatile window during which running the same
    /// query may get different results.
    #[serde(rename="readTime")]
    pub read_time: Option<String>,
}

impl RequestValue for ExportAssetsRequest {}


/// Represents an expression text. Example:
/// 
/// ````text
/// title: "User account presence"
/// description: "Determines whether the request has a user account"
/// expression: "size(request.user) > 0"
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// An optional description of the expression. This is a longer text which
    /// describes the expression, e.g. when hovered over it in a UI.
    pub description: Option<String>,
    /// Textual representation of an expression in
    /// Common Expression Language syntax.
    /// 
    /// The application context of the containing message determines which
    /// well-known feature set of CEL is supported.
    pub expression: Option<String>,
    /// An optional string indicating the location of the expression for error
    /// reporting, e.g. a file name and a position in the file.
    pub location: Option<String>,
    /// An optional title for the expression, i.e. a short string describing
    /// its purpose. This can be used e.g. in UIs which allow to enter the
    /// expression.
    pub title: Option<String>,
}

impl Part for Expr {}


/// Temporal asset. In addition to the asset, the temporal asset includes the
/// status of the asset and valid from and to time of it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TemporalAsset {
    /// If the asset is deleted or not.
    pub deleted: Option<bool>,
    /// The time window when the asset data and state was observed.
    pub window: Option<TimeWindow>,
    /// Asset.
    pub asset: Option<Asset>,
}

impl Part for TemporalAsset {}


/// Cloud asset. This includes all Google Cloud Platform resources,
/// Cloud IAM policies, and other non-GCP assets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    /// Representation of the actual Cloud IAM policy set on a cloud resource. For
    /// each resource, there must be at most one Cloud IAM policy set on it.
    #[serde(rename="iamPolicy")]
    pub iam_policy: Option<Policy>,
    /// Type of the asset. Example: "compute.googleapis.com/Disk".
    #[serde(rename="assetType")]
    pub asset_type: Option<String>,
    /// Representation of the resource.
    pub resource: Option<ResourceType>,
    /// The full name of the asset. For example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    pub name: Option<String>,
    /// Representation of the Cloud Organization Policy set on an asset. For each
    /// asset, there could be multiple Organization policies with different
    /// constraints.
    #[serde(rename="orgPolicy")]
    pub org_policy: Option<Vec<GoogleCloudOrgpolicyV1Policy>>,
}

impl Part for Asset {}


/// Output configuration for export assets destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Destination on Cloud Storage.
    #[serde(rename="gcsDestination")]
    pub gcs_destination: Option<GcsDestination>,
}

impl Part for OutputConfig {}


/// Defines an Identity and Access Management (IAM) policy. It is used to
/// specify access control policies for Cloud Platform resources.
/// 
/// A `Policy` consists of a list of `bindings`. A `binding` binds a list of
/// `members` to a `role`, where the members can be user accounts, Google groups,
/// Google domains, and service accounts. A `role` is a named list of permissions
/// defined by IAM.
/// 
/// **JSON Example**
/// 
/// ````text
/// {
///   "bindings": [
///     {
///       "role": "roles/owner",
///       "members": [
///         "user:mike@example.com",
///         "group:admins@example.com",
///         "domain:google.com",
///         "serviceAccount:my-other-app@appspot.gserviceaccount.com"
///       ]
///     },
///     {
///       "role": "roles/viewer",
///       "members": ["user:sean@example.com"]
///     }
///   ]
/// }
/// ````
/// 
/// **YAML Example**
/// 
/// ````text
/// bindings:
/// - members:
///   - user:mike@example.com
///   - group:admins@example.com
///   - domain:google.com
///   - serviceAccount:my-other-app@appspot.gserviceaccount.com
///   role: roles/owner
/// - members:
///   - user:sean@example.com
///   role: roles/viewer
/// ````
/// 
/// For a description of IAM and its features, see the
/// [IAM developer's guide](https://cloud.google.com/iam/docs).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a policy from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform policy updates in order to avoid race
    /// conditions: An `etag` is returned in the response to `getIamPolicy`, and
    /// systems are expected to put that etag in the request to `setIamPolicy` to
    /// ensure that their change will be applied to the same version of the policy.
    /// 
    /// If no `etag` is provided in the call to `setIamPolicy`, then the existing
    /// policy is overwritten blindly.
    pub etag: Option<String>,
    /// Associates a list of `members` to a `role`.
    /// `bindings` with no members will result in an error.
    pub bindings: Option<Vec<Binding>>,
    /// Deprecated.
    pub version: Option<i32>,
}

impl Part for Policy {}


/// Used in `policy_type` to specify how `list_policy` behaves at this
/// resource.
/// 
/// `ListPolicy` can define specific values and subtrees of Cloud Resource
/// Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that
/// are allowed or denied by setting the `allowed_values` and `denied_values`
/// fields. This is achieved by using the `under:` and optional `is:` prefixes.
/// The `under:` prefix is used to denote resource subtree values.
/// The `is:` prefix is used to denote specific values, and is required only
/// if the value contains a ":". Values prefixed with "is:" are treated the
/// same as values with no prefix.
/// Ancestry subtrees must be in one of the following formats:
/// - “projects/<project-id>”, e.g. “projects/tokyo-rain-123”
/// - “folders/<folder-id>”, e.g. “folders/1234”
/// - “organizations/<organization-id>”, e.g. “organizations/1234”
/// The `supports_under` field of the associated `Constraint`  defines whether
/// ancestry prefixes can be used. You can set `allowed_values` and
/// `denied_values` in the same `Policy` if `all_values` is
/// `ALL_VALUES_UNSPECIFIED`. `ALLOW` or `DENY` are used to allow or deny all
/// values. If `all_values` is set to either `ALLOW` or `DENY`,
/// `allowed_values` and `denied_values` must be unset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1ListPolicy {
    /// The policy all_values state.
    #[serde(rename="allValues")]
    pub all_values: Option<String>,
    /// List of values allowed  at this resource. Can only be set if `all_values`
    /// is set to `ALL_VALUES_UNSPECIFIED`.
    #[serde(rename="allowedValues")]
    pub allowed_values: Option<Vec<String>>,
    /// Determines the inheritance behavior for this `Policy`.
    /// 
    /// By default, a `ListPolicy` set at a resource supercedes any `Policy` set
    /// anywhere up the resource hierarchy. However, if `inherit_from_parent` is
    /// set to `true`, then the values from the effective `Policy` of the parent
    /// resource are inherited, meaning the values set in this `Policy` are
    /// added to the values inherited up the hierarchy.
    /// 
    /// Setting `Policy` hierarchies that inherit both allowed values and denied
    /// values isn't recommended in most circumstances to keep the configuration
    /// simple and understandable. However, it is possible to set a `Policy` with
    /// `allowed_values` set that inherits a `Policy` with `denied_values` set.
    /// In this case, the values that are allowed must be in `allowed_values` and
    /// not present in `denied_values`.
    /// 
    /// For example, suppose you have a `Constraint`
    /// `constraints/serviceuser.services`, which has a `constraint_type` of
    /// `list_constraint`, and with `constraint_default` set to `ALLOW`.
    /// Suppose that at the Organization level, a `Policy` is applied that
    /// restricts the allowed API activations to {`E1`, `E2`}. Then, if a
    /// `Policy` is applied to a project below the Organization that has
    /// `inherit_from_parent` set to `false` and field all_values set to DENY,
    /// then an attempt to activate any API will be denied.
    /// 
    /// The following examples demonstrate different possible layerings for
    /// `projects/bar` parented by `organizations/foo`:
    /// 
    /// Example 1 (no inherited values):
    /// `organizations/foo` has a `Policy` with values:
    /// {allowed_values: “E1” allowed_values:”E2”}
    /// `projects/bar` has `inherit_from_parent` `false` and values:
    /// {allowed_values: "E3" allowed_values: "E4"}
    /// The accepted values at `organizations/foo` are `E1`, `E2`.
    /// The accepted values at `projects/bar` are `E3`, and `E4`.
    /// 
    /// Example 2 (inherited values):
    /// `organizations/foo` has a `Policy` with values:
    /// {allowed_values: “E1” allowed_values:”E2”}
    /// `projects/bar` has a `Policy` with values:
    /// {value: “E3” value: ”E4” inherit_from_parent: true}
    /// The accepted values at `organizations/foo` are `E1`, `E2`.
    /// The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`.
    /// 
    /// Example 3 (inheriting both allowed and denied values):
    /// `organizations/foo` has a `Policy` with values:
    /// {allowed_values: "E1" allowed_values: "E2"}
    /// `projects/bar` has a `Policy` with:
    /// {denied_values: "E1"}
    /// The accepted values at `organizations/foo` are `E1`, `E2`.
    /// The value accepted at `projects/bar` is `E2`.
    /// 
    /// Example 4 (RestoreDefault):
    /// `organizations/foo` has a `Policy` with values:
    /// {allowed_values: “E1” allowed_values:”E2”}
    /// `projects/bar` has a `Policy` with values:
    /// {RestoreDefault: {}}
    /// The accepted values at `organizations/foo` are `E1`, `E2`.
    /// The accepted values at `projects/bar` are either all or none depending on
    /// the value of `constraint_default` (if `ALLOW`, all; if
    /// `DENY`, none).
    /// 
    /// Example 5 (no policy inherits parent policy):
    /// `organizations/foo` has no `Policy` set.
    /// `projects/bar` has no `Policy` set.
    /// The accepted values at both levels are either all or none depending on
    /// the value of `constraint_default` (if `ALLOW`, all; if
    /// `DENY`, none).
    /// 
    /// Example 6 (ListConstraint allowing all):
    /// `organizations/foo` has a `Policy` with values:
    /// {allowed_values: “E1” allowed_values: ”E2”}
    /// `projects/bar` has a `Policy` with:
    /// {all: ALLOW}
    /// The accepted values at `organizations/foo` are `E1`, E2`. Any value is accepted at `projects/bar`.
    /// 
    /// Example 7 (ListConstraint allowing none):
    /// `organizations/foo` has a `Policy` with values:
    /// {allowed_values: “E1” allowed_values: ”E2”}
    /// `projects/bar` has a `Policy` with:
    /// {all: DENY}
    /// The accepted values at `organizations/foo` are `E1`, E2`. No value is accepted at `projects/bar`.
    /// 
    /// Example 10 (allowed and denied subtrees of Resource Manager hierarchy):
    /// Given the following resource hierarchy
    /// O1->{F1, F2}; F1->{P1}; F2->{P2, P3},
    /// `organizations/foo` has a `Policy` with values:
    /// {allowed_values: "under:organizations/O1"}
    /// `projects/bar` has a `Policy` with:
    /// {allowed_values: "under:projects/P3"}
    /// {denied_values: "under:folders/F2"}
    /// The accepted values at `organizations/foo` are `organizations/O1`,
    /// `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`,
    /// `projects/P3`.
    /// The accepted values at `projects/bar` are `organizations/O1`,
    /// `folders/F1`, `projects/P1`.
    #[serde(rename="inheritFromParent")]
    pub inherit_from_parent: Option<bool>,
    /// Optional. The Google Cloud Console will try to default to a configuration
    /// that matches the value specified in this `Policy`. If `suggested_value`
    /// is not set, it will inherit the value specified higher in the hierarchy,
    /// unless `inherit_from_parent` is `false`.
    #[serde(rename="suggestedValue")]
    pub suggested_value: Option<String>,
    /// List of values denied at this resource. Can only be set if `all_values`
    /// is set to `ALL_VALUES_UNSPECIFIED`.
    #[serde(rename="deniedValues")]
    pub denied_values: Option<Vec<String>>,
}

impl Part for GoogleCloudOrgpolicyV1ListPolicy {}


/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
/// 
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
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


/// Used in `policy_type` to specify how `boolean_policy` will behave at this
/// resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1BooleanPolicy {
    /// If `true`, then the `Policy` is enforced. If `false`, then any
    /// configuration is acceptable.
    /// 
    /// Suppose you have a `Constraint`
    /// `constraints/compute.disableSerialPortAccess` with `constraint_default`
    /// set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following
    /// behavior:
    /// 
    /// * If the `Policy` at this resource has enforced set to `false`, serial
    ///   port connection attempts will be allowed.
    /// * If the `Policy` at this resource has enforced set to `true`, serial
    ///   port connection attempts will be refused.
    /// * If the `Policy` at this resource is `RestoreDefault`, serial port
    ///   connection attempts will be allowed.
    /// * If no `Policy` is set at this resource or anywhere higher in the
    ///   resource hierarchy, serial port connection attempts will be allowed.
    /// * If no `Policy` is set at this resource, but one exists higher in the
    ///   resource hierarchy, the behavior is as if the`Policy` were set at
    ///   this resource.
    /// 
    /// The following examples demonstrate the different possible layerings:
    /// 
    /// Example 1 (nearest `Constraint` wins):
    /// `organizations/foo` has a `Policy` with:
    /// {enforced: false}
    /// `projects/bar` has no `Policy` set.
    /// The constraint at `projects/bar` and `organizations/foo` will not be
    /// enforced.
    /// 
    /// Example 2 (enforcement gets replaced):
    /// `organizations/foo` has a `Policy` with:
    /// {enforced: false}
    /// `projects/bar` has a `Policy` with:
    /// {enforced: true}
    /// The constraint at `organizations/foo` is not enforced.
    /// The constraint at `projects/bar` is enforced.
    /// 
    /// Example 3 (RestoreDefault):
    /// `organizations/foo` has a `Policy` with:
    /// {enforced: true}
    /// `projects/bar` has a `Policy` with:
    /// {RestoreDefault: {}}
    /// The constraint at `organizations/foo` is enforced.
    /// The constraint at `projects/bar` is not enforced, because
    /// `constraint_default` for the `Constraint` is `ALLOW`.
    pub enforced: Option<bool>,
}

impl Part for GoogleCloudOrgpolicyV1BooleanPolicy {}


/// Defines a Cloud Organization `Policy` which is used to specify `Constraints`
/// for configurations of Cloud Platform resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV1Policy {
    /// The time stamp the `Policy` was previously updated. This is set by the
    /// server, not specified by the caller, and represents the last time a call to
    /// `SetOrgPolicy` was made for that `Policy`. Any value set by the client will
    /// be ignored.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// Version of the `Policy`. Default version is 0;
    pub version: Option<i32>,
    /// An opaque tag indicating the current version of the `Policy`, used for
    /// concurrency control.
    /// 
    /// When the `Policy` is returned from either a `GetPolicy` or a
    /// `ListOrgPolicy` request, this `etag` indicates the version of the current
    /// `Policy` to use when executing a read-modify-write loop.
    /// 
    /// When the `Policy` is returned from a `GetEffectivePolicy` request, the
    /// `etag` will be unset.
    /// 
    /// When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value
    /// that was returned from a `GetOrgPolicy` request as part of a
    /// read-modify-write loop for concurrency control. Not setting the `etag`in a
    /// `SetOrgPolicy` request will result in an unconditional write of the
    /// `Policy`.
    pub etag: Option<String>,
    /// The name of the `Constraint` the `Policy` is configuring, for example,
    /// `constraints/serviceuser.services`.
    /// 
    /// Immutable after creation.
    pub constraint: Option<String>,
    /// Restores the default behavior of the constraint; independent of
    /// `Constraint` type.
    #[serde(rename="restoreDefault")]
    pub restore_default: Option<GoogleCloudOrgpolicyV1RestoreDefault>,
    /// List of values either allowed or disallowed.
    #[serde(rename="listPolicy")]
    pub list_policy: Option<GoogleCloudOrgpolicyV1ListPolicy>,
    /// For boolean `Constraints`, whether to enforce the `Constraint` or not.
    #[serde(rename="booleanPolicy")]
    pub boolean_policy: Option<GoogleCloudOrgpolicyV1BooleanPolicy>,
}

impl Part for GoogleCloudOrgpolicyV1Policy {}


/// Associates `members` with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// Role that is assigned to `members`.
    /// For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    pub role: Option<String>,
    /// The condition that is associated with this binding.
    /// NOTE: An unsatisfied condition will not allow user access via current
    /// binding. Different bindings, including their conditions, are examined
    /// independently.
    pub condition: Option<Expr>,
    /// Specifies the identities requesting access for a Cloud Platform resource.
    /// `members` can have the following values:
    /// 
    /// * `allUsers`: A special identifier that represents anyone who is
    ///    on the internet; with or without a Google account.
    /// 
    /// * `allAuthenticatedUsers`: A special identifier that represents anyone
    ///    who is authenticated with a Google account or a service account.
    /// 
    /// * `user:{emailid}`: An email address that represents a specific Google
    ///    account. For example, `alice@gmail.com` .
    /// 
    /// 
    /// * `serviceAccount:{emailid}`: An email address that represents a service
    ///    account. For example, `my-other-app@appspot.gserviceaccount.com`.
    /// 
    /// * `group:{emailid}`: An email address that represents a Google group.
    ///    For example, `admins@example.com`.
    /// 
    /// 
    /// * `domain:{domain}`: The G Suite domain (primary) that represents all the
    ///    users of that domain. For example, `google.com` or `example.com`.
    /// 
    /// 
    pub members: Option<Vec<String>>,
}

impl Part for Binding {}


/// Batch get assets history response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch get assets history](struct.MethodBatchGetAssetsHistoryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetAssetsHistoryResponse {
    /// A list of assets with valid time windows.
    pub assets: Option<Vec<TemporalAsset>>,
}

impl ResponseResult for BatchGetAssetsHistoryResponse {}


/// Specifies the audit configuration for a service.
/// The configuration determines which permission types are logged, and what
/// identities, if any, are exempted from logging.
/// An AuditConfig must have one or more AuditLogConfigs.
/// 
/// If there are AuditConfigs for both `allServices` and a specific service,
/// the union of the two AuditConfigs is used for that service: the log_types
/// specified in each AuditConfig are enabled, and the exempted_members in each
/// AuditLogConfig are exempted.
/// 
/// Example Policy with multiple AuditConfigs:
/// 
/// ````text
/// {
///   "audit_configs": [
///     {
///       "service": "allServices"
///       "audit_log_configs": [
///         {
///           "log_type": "DATA_READ",
///           "exempted_members": [
///             "user:foo@gmail.com"
///           ]
///         },
///         {
///           "log_type": "DATA_WRITE",
///         },
///         {
///           "log_type": "ADMIN_READ",
///         }
///       ]
///     },
///     {
///       "service": "fooservice.googleapis.com"
///       "audit_log_configs": [
///         {
///           "log_type": "DATA_READ",
///         },
///         {
///           "log_type": "DATA_WRITE",
///           "exempted_members": [
///             "user:bar@gmail.com"
///           ]
///         }
///       ]
///     }
///   ]
/// }
/// ````
/// 
/// For fooservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ
/// logging. It also exempts foo@gmail.com from DATA_READ logging, and
/// bar@gmail.com from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging.
    /// For example, `storage.googleapis.com`, `cloudsql.googleapis.com`.
    /// `allServices` is a special value that covers all services.
    pub service: Option<String>,
}

impl Part for AuditConfig {}


/// A time window of (start_time, end_time].
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeWindow {
    /// End time of the time window (inclusive).
    /// Current timestamp if not specified.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// Start time of the time window (exclusive).
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
}

impl Part for TimeWindow {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [export assets](struct.MethodExportAssetCall.html) (response)
/// * [get operations](struct.OperationGetCall.html) (response)
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
    /// `name` should be a resource name ending with `operations/{unique_id}`.
    pub name: Option<String>,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, String>>,
}

impl Resource for Operation {}
impl ResponseResult for Operation {}


/// A Cloud Storage location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsDestination {
    /// The uri prefix of all generated Cloud Storage objects. For example:
    /// "gs://bucket_name/object_name_prefix". Each object uri is in format:
    /// "gs://bucket_name/object_name_prefix/<asset type>/<shard number> and only
    /// contains assets for that type. <shard number> starts from 0. For example:
    /// "gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0" is
    /// the first shard of output objects containing all
    /// compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be
    /// returned if file with the same name "gs://bucket_name/object_name_prefix"
    /// already exists.
    #[serde(rename="uriPrefix")]
    pub uri_prefix: Option<String>,
    /// The uri of the Cloud Storage object. It's the same uri that is used by
    /// gsutil. For example: "gs://bucket_name/object_name". See [Viewing and
    /// Editing Object
    /// Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)
    /// for more information.
    pub uri: Option<String>,
}

impl Part for GcsDestination {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the `CloudAsset` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudasset1::CloudAsset;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudAsset::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudAsset<C, A>,
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



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `CloudAsset` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudasset1::CloudAsset;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudAsset::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get_assets_history(...)` and `export_assets(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudAsset<C, A>,
}

impl<'a, C, A> MethodsBuilder for MethodMethods<'a, C, A> {}

impl<'a, C, A> MethodMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch gets the update history of assets that overlap a time window.
    /// For RESOURCE content, this API outputs history with asset in both
    /// non-delete or deleted status.
    /// For IAM_POLICY content, this API outputs history when the asset and its
    /// attached IAM POLICY both exist. This can create gaps in the output history.
    /// If a specified asset does not exist, this API returns an INVALID_ARGUMENT
    /// error.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative name of the root asset. It can only be an
    ///              organization number (such as "organizations/123"), a project ID (such as
    ///              "projects/my-project-id")", or a project number (such as "projects/12345").
    pub fn batch_get_assets_history(&self, parent: &str) -> MethodBatchGetAssetsHistoryCall<'a, C, A> {
        MethodBatchGetAssetsHistoryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time_window_start_time: Default::default(),
            _read_time_window_end_time: Default::default(),
            _content_type: Default::default(),
            _asset_names: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports assets with time and resource types to a given Cloud Storage
    /// location. The output format is newline-delimited JSON.
    /// This API implements the google.longrunning.Operation API allowing you
    /// to keep track of the export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative name of the root asset. This can only be an
    ///              organization number (such as "organizations/123"), a project ID (such as
    ///              "projects/my-project-id"), or a project number (such as "projects/12345"),
    ///              or a folder number (such as "folders/123").
    pub fn export_assets(&self, request: ExportAssetsRequest, parent: &str) -> MethodExportAssetCall<'a, C, A> {
        MethodExportAssetCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

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
/// # extern crate google_cloudasset1 as cloudasset1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudasset1::CloudAsset;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudAsset::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().get("name")
///              .doit();
/// # }
/// ```
pub struct OperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudAsset<C, A>,
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
        dlg.begin(MethodInfo { id: "cloudasset.operations.get",
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


/// Batch gets the update history of assets that overlap a time window.
/// For RESOURCE content, this API outputs history with asset in both
/// non-delete or deleted status.
/// For IAM_POLICY content, this API outputs history when the asset and its
/// attached IAM POLICY both exist. This can create gaps in the output history.
/// If a specified asset does not exist, this API returns an INVALID_ARGUMENT
/// error.
///
/// A builder for the *batchGetAssetsHistory* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudasset1 as cloudasset1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudasset1::CloudAsset;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudAsset::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().batch_get_assets_history("parent")
///              .read_time_window_start_time("dolores")
///              .read_time_window_end_time("kasd")
///              .content_type("accusam")
///              .add_asset_names("takimata")
///              .doit();
/// # }
/// ```
pub struct MethodBatchGetAssetsHistoryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudAsset<C, A>,
    _parent: String,
    _read_time_window_start_time: Option<String>,
    _read_time_window_end_time: Option<String>,
    _content_type: Option<String>,
    _asset_names: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MethodBatchGetAssetsHistoryCall<'a, C, A> {}

impl<'a, C, A> MethodBatchGetAssetsHistoryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchGetAssetsHistoryResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudasset.batchGetAssetsHistory",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._read_time_window_start_time {
            params.push(("readTimeWindow.startTime", value.to_string()));
        }
        if let Some(value) = self._read_time_window_end_time {
            params.push(("readTimeWindow.endTime", value.to_string()));
        }
        if let Some(value) = self._content_type {
            params.push(("contentType", value.to_string()));
        }
        if self._asset_names.len() > 0 {
            for f in self._asset_names.iter() {
                params.push(("assetNames", f.to_string()));
            }
        }
        for &field in ["alt", "parent", "readTimeWindow.startTime", "readTimeWindow.endTime", "contentType", "assetNames"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:batchGetAssetsHistory";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
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


    /// Required. The relative name of the root asset. It can only be an
    /// organization number (such as "organizations/123"), a project ID (such as
    /// "projects/my-project-id")", or a project number (such as "projects/12345").
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> MethodBatchGetAssetsHistoryCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Start time of the time window (exclusive).
    ///
    /// Sets the *read time window.start time* query property to the given value.
    pub fn read_time_window_start_time(mut self, new_value: &str) -> MethodBatchGetAssetsHistoryCall<'a, C, A> {
        self._read_time_window_start_time = Some(new_value.to_string());
        self
    }
    /// End time of the time window (inclusive).
    /// Current timestamp if not specified.
    ///
    /// Sets the *read time window.end time* query property to the given value.
    pub fn read_time_window_end_time(mut self, new_value: &str) -> MethodBatchGetAssetsHistoryCall<'a, C, A> {
        self._read_time_window_end_time = Some(new_value.to_string());
        self
    }
    /// Required. The content type.
    ///
    /// Sets the *content type* query property to the given value.
    pub fn content_type(mut self, new_value: &str) -> MethodBatchGetAssetsHistoryCall<'a, C, A> {
        self._content_type = Some(new_value.to_string());
        self
    }
    /// A list of the full names of the assets. For example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// and [Resource Name
    /// Format](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/resource-name-format)
    /// for more info.
    /// 
    /// The request becomes a no-op if the asset name list is empty, and the max
    /// size of the asset name list is 100 in one request.
    ///
    /// Append the given value to the *asset names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_asset_names(mut self, new_value: &str) -> MethodBatchGetAssetsHistoryCall<'a, C, A> {
        self._asset_names.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodBatchGetAssetsHistoryCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodBatchGetAssetsHistoryCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodBatchGetAssetsHistoryCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Exports assets with time and resource types to a given Cloud Storage
/// location. The output format is newline-delimited JSON.
/// This API implements the google.longrunning.Operation API allowing you
/// to keep track of the export.
///
/// A builder for the *exportAssets* method.
/// It is not used directly, but through a `MethodMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudasset1 as cloudasset1;
/// use cloudasset1::ExportAssetsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudasset1::CloudAsset;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudAsset::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ExportAssetsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().export_assets(req, "parent")
///              .doit();
/// # }
/// ```
pub struct MethodExportAssetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudAsset<C, A>,
    _request: ExportAssetsRequest,
    _parent: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MethodExportAssetCall<'a, C, A> {}

impl<'a, C, A> MethodExportAssetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "cloudasset.exportAssets",
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

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:exportAssets";
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
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
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
    pub fn request(mut self, new_value: ExportAssetsRequest) -> MethodExportAssetCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Required. The relative name of the root asset. This can only be an
    /// organization number (such as "organizations/123"), a project ID (such as
    /// "projects/my-project-id"), or a project number (such as "projects/12345"),
    /// or a folder number (such as "folders/123").
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> MethodExportAssetCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodExportAssetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodExportAssetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> MethodExportAssetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


