// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Resource Manager* crate version *1.0.12+20190701*, where *20190701* is the exact revision of the *cloudresourcemanager:v1beta1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.12*.
//! 
//! Everything else about the *Cloud Resource Manager* *v1_beta1* API can be found at the
//! [official documentation site](https://cloud.google.com/resource-manager).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/cloudresourcemanager1_beta1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CloudResourceManager.html) ... 
//! 
//! * [organizations](struct.Organization.html)
//!  * [*get*](struct.OrganizationGetCall.html), [*get iam policy*](struct.OrganizationGetIamPolicyCall.html), [*list*](struct.OrganizationListCall.html), [*set iam policy*](struct.OrganizationSetIamPolicyCall.html), [*test iam permissions*](struct.OrganizationTestIamPermissionCall.html) and [*update*](struct.OrganizationUpdateCall.html)
//! * [projects](struct.Project.html)
//!  * [*create*](struct.ProjectCreateCall.html), [*delete*](struct.ProjectDeleteCall.html), [*get*](struct.ProjectGetCall.html), [*get ancestry*](struct.ProjectGetAncestryCall.html), [*get iam policy*](struct.ProjectGetIamPolicyCall.html), [*list*](struct.ProjectListCall.html), [*set iam policy*](struct.ProjectSetIamPolicyCall.html), [*test iam permissions*](struct.ProjectTestIamPermissionCall.html), [*undelete*](struct.ProjectUndeleteCall.html) and [*update*](struct.ProjectUpdateCall.html)
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
//! * **[Hub](struct.CloudResourceManager.html)**
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
//! let r = hub.projects().test_iam_permissions(...).doit()
//! let r = hub.projects().undelete(...).doit()
//! let r = hub.projects().set_iam_policy(...).doit()
//! let r = hub.projects().get(...).doit()
//! let r = hub.projects().get_ancestry(...).doit()
//! let r = hub.projects().update(...).doit()
//! let r = hub.projects().get_iam_policy(...).doit()
//! let r = hub.projects().delete(...).doit()
//! let r = hub.projects().create(...).doit()
//! let r = hub.projects().list(...).doit()
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
//! google-cloudresourcemanager1_beta1 = "*"
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
//! extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
//! use cloudresourcemanager1_beta1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use cloudresourcemanager1_beta1::CloudResourceManager;
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
//! let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().list()
//!              .page_token("sed")
//!              .page_size(-85)
//!              .filter("dolores")
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

    /// View your data across Google Cloud Platform services
    CloudPlatformReadOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
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

/// Central instance to access all CloudResourceManager related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudresourcemanager1_beta1::CloudResourceManager;
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
/// let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().list()
///              .page_token("kasd")
///              .page_size(-22)
///              .filter("takimata")
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
pub struct CloudResourceManager<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for CloudResourceManager<C, A> {}

impl<'a, C, A> CloudResourceManager<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CloudResourceManager<C, A> {
        CloudResourceManager {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.12".to_string(),
            _base_url: "https://cloudresourcemanager.googleapis.com/".to_string(),
            _root_url: "https://cloudresourcemanager.googleapis.com/".to_string(),
        }
    }

    pub fn organizations(&'a self) -> OrganizationMethods<'a, C, A> {
        OrganizationMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.12`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://cloudresourcemanager.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://cloudresourcemanager.googleapis.com/`.
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


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy projects](struct.ProjectGetIamPolicyCall.html) (request)
/// * [get iam policy organizations](struct.OrganizationGetIamPolicyCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to
    /// `GetIamPolicy`. This field is only used by Cloud IAM.
    pub options: Option<GetPolicyOptions>,
}

impl RequestValue for GetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions organizations](struct.OrganizationTestIamPermissionCall.html) (request)
/// * [test iam permissions projects](struct.ProjectTestIamPermissionCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with
    /// wildcards (such as '*' or 'storage.*') are not allowed. For more
    /// information see
    /// [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    pub permissions: Option<Vec<String>>,
}

impl RequestValue for TestIamPermissionsRequest {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy projects](struct.ProjectSetIamPolicyCall.html) (request)
/// * [set iam policy organizations](struct.OrganizationSetIamPolicyCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of
    /// the policy is limited to a few 10s of KB. An empty policy is a
    /// valid policy but certain Cloud Platform services (such as Projects)
    /// might reject them.
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only
    /// the fields in the mask will be modified. If no mask is provided, the
    /// following default mask is used:
    /// paths: "bindings, etag"
    /// This field is only used by Cloud IAM.
    #[serde(rename="updateMask")]
    pub update_mask: Option<String>,
}

impl RequestValue for SetIamPolicyRequest {}


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


/// The response returned from the `ListOrganizations` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list organizations](struct.OrganizationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOrganizationsResponse {
    /// A pagination token to be used to retrieve the next page of results. If the
    /// result is too large to fit within the page size specified in the request,
    /// this field will be set with a token that can be used to fetch the next page
    /// of results. If this field is empty, it indicates that this response
    /// contains the last page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of Organizations that matched the list query, possibly paginated.
    pub organizations: Option<Vec<Organization>>,
}

impl ResponseResult for ListOrganizationsResponse {}


/// The request sent to the
/// GetAncestry
/// method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get ancestry projects](struct.ProjectGetAncestryCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetAncestryRequest { _never_set: Option<bool> }

impl RequestValue for GetAncestryRequest {}


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
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy projects](struct.ProjectSetIamPolicyCall.html) (response)
/// * [set iam policy organizations](struct.OrganizationSetIamPolicyCall.html) (response)
/// * [get iam policy projects](struct.ProjectGetIamPolicyCall.html) (response)
/// * [get iam policy organizations](struct.OrganizationGetIamPolicyCall.html) (response)
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

impl ResponseResult for Policy {}


/// The root node in the resource hierarchy to which a particular entity's
/// (e.g., company) resources belong.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy organizations](struct.OrganizationSetIamPolicyCall.html) (none)
/// * [get iam policy organizations](struct.OrganizationGetIamPolicyCall.html) (none)
/// * [test iam permissions organizations](struct.OrganizationTestIamPermissionCall.html) (none)
/// * [get organizations](struct.OrganizationGetCall.html) (response)
/// * [update organizations](struct.OrganizationUpdateCall.html) (request|response)
/// * [list organizations](struct.OrganizationListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    /// The owner of this Organization. The owner should be specified on
    /// creation. Once set, it cannot be changed.
    /// This field is required.
    pub owner: Option<OrganizationOwner>,
    /// A human-readable string that refers to the Organization in the
    /// GCP Console UI. This string is set by the server and cannot be
    /// changed. The string will be set to the primary domain (for example,
    /// "google.com") of the G Suite customer that owns the organization.
    /// @OutputOnly
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Output Only. The resource name of the organization. This is the
    /// organization's relative path in the API. Its format is
    /// "organizations/[organization_id]". For example, "organizations/1234".
    pub name: Option<String>,
    /// An immutable id for the Organization that is assigned on creation. This
    /// should be omitted when creating a new Organization.
    /// This field is read-only.
    #[serde(rename="organizationId")]
    pub organization_id: Option<String>,
    /// Timestamp when the Organization was created. Assigned by the server.
    /// @OutputOnly
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// The organization's current lifecycle state. Assigned by the server.
    /// @OutputOnly
    #[serde(rename="lifecycleState")]
    pub lifecycle_state: Option<String>,
}

impl RequestValue for Organization {}
impl Resource for Organization {}
impl ResponseResult for Organization {}


/// Identifying information for a single ancestor of a project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Ancestor {
    /// Resource id of the ancestor.
    #[serde(rename="resourceId")]
    pub resource_id: Option<ResourceId>,
}

impl Part for Ancestor {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete projects](struct.ProjectDeleteCall.html) (response)
/// * [undelete projects](struct.ProjectUndeleteCall.html) (response)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}


/// The request sent to the UndeleteProject
/// method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [undelete projects](struct.ProjectUndeleteCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteProjectRequest { _never_set: Option<bool> }

impl RequestValue for UndeleteProjectRequest {}


/// A Project is a high-level Google Cloud Platform entity.  It is a
/// container for ACLs, APIs, App Engine Apps, VMs, and other
/// Google Cloud Platform resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions projects](struct.ProjectTestIamPermissionCall.html) (none)
/// * [undelete projects](struct.ProjectUndeleteCall.html) (none)
/// * [set iam policy projects](struct.ProjectSetIamPolicyCall.html) (none)
/// * [get projects](struct.ProjectGetCall.html) (response)
/// * [get ancestry projects](struct.ProjectGetAncestryCall.html) (none)
/// * [update projects](struct.ProjectUpdateCall.html) (request|response)
/// * [get iam policy projects](struct.ProjectGetIamPolicyCall.html) (none)
/// * [delete projects](struct.ProjectDeleteCall.html) (none)
/// * [create projects](struct.ProjectCreateCall.html) (request|response)
/// * [list projects](struct.ProjectListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    /// The optional user-assigned display name of the Project.
    /// When present it must be between 4 to 30 characters.
    /// Allowed characters are: lowercase and uppercase letters, numbers,
    /// hyphen, single-quote, double-quote, space, and exclamation point.
    /// 
    /// Example: <code>My Project</code>
    /// Read-write.
    pub name: Option<String>,
    /// An optional reference to a parent Resource.
    /// 
    /// Supported parent types include "organization" and "folder". Once set, the
    /// parent cannot be cleared. The `parent` can be set on creation or using the
    /// `UpdateProject` method; the end user must have the
    /// `resourcemanager.projects.create` permission on the parent.
    /// 
    /// Read-write.
    pub parent: Option<ResourceId>,
    /// The unique, user-assigned ID of the Project.
    /// It must be 6 to 30 lowercase letters, digits, or hyphens.
    /// It must start with a letter.
    /// Trailing hyphens are prohibited.
    /// 
    /// Example: <code>tokyo-rain-123</code>
    /// Read-only after creation.
    #[serde(rename="projectId")]
    pub project_id: Option<String>,
    /// The labels associated with this Project.
    /// 
    /// Label keys must be between 1 and 63 characters long and must conform
    /// to the following regular expression: \[a-z\](\[-a-z0-9\]*\[a-z0-9\])?.
    /// 
    /// Label values must be between 0 and 63 characters long and must conform
    /// to the regular expression (\[a-z\](\[-a-z0-9\]*\[a-z0-9\])?)?. A label
    /// value can be empty.
    /// 
    /// No more than 256 labels can be associated with a given resource.
    /// 
    /// Clients should store labels in a representation such as JSON that does not
    /// depend on specific characters being disallowed.
    /// 
    /// Example: <code>"environment" : "dev"</code>
    /// Read-write.
    pub labels: Option<HashMap<String, String>>,
    /// The number uniquely identifying the project.
    /// 
    /// Example: <code>415104041262</code>
    /// Read-only.
    #[serde(rename="projectNumber")]
    pub project_number: Option<String>,
    /// Creation time.
    /// 
    /// Read-only.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// The Project lifecycle state.
    /// 
    /// Read-only.
    #[serde(rename="lifecycleState")]
    pub lifecycle_state: Option<String>,
}

impl RequestValue for Project {}
impl Resource for Project {}
impl ResponseResult for Project {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions organizations](struct.OrganizationTestIamPermissionCall.html) (response)
/// * [test iam permissions projects](struct.ProjectTestIamPermissionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is
    /// allowed.
    pub permissions: Option<Vec<String>>,
}

impl ResponseResult for TestIamPermissionsResponse {}


/// A page of the response received from the
/// ListProjects
/// method.
/// 
/// A paginated response where more pages are available has
/// `next_page_token` set. This token can be used in a subsequent request to
/// retrieve the next request page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list projects](struct.ProjectListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProjectsResponse {
    /// Pagination token.
    /// 
    /// If the result set is too large to fit in a single response, this token
    /// is returned. It encodes the position of the current result cursor.
    /// Feeding this value into a new list request with the `page_token` parameter
    /// gives the next page of the results.
    /// 
    /// When `next_page_token` is not filled in, there is no next page and
    /// the list returned is the last page in the result set.
    /// 
    /// Pagination tokens have a limited lifetime.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of Projects that matched the list filter. This list can
    /// be paginated.
    pub projects: Option<Vec<Project>>,
}

impl ResponseResult for ListProjectsResponse {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The policy format version to be returned.
    /// Acceptable values are 0 and 1.
    /// If the value is 0, or the field is omitted, policy format version 1 will be
    /// returned.
    #[serde(rename="requestedPolicyVersion")]
    pub requested_policy_version: Option<i32>,
}

impl Part for GetPolicyOptions {}


/// A container to reference an id for any resource type. A `resource` in Google
/// Cloud Platform is a generic term for something you (a developer) may want to
/// interact with through one of our API's. Some examples are an App Engine app,
/// a Compute Engine instance, a Cloud SQL database, and so on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceId {
    /// Required field representing the resource type this id is for.
    /// At present, the valid types are "project", "folder", and "organization".
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Required field for the type-specific id. This should correspond to the id
    /// used in the type-specific API's.
    pub id: Option<String>,
}

impl Part for ResourceId {}


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


/// Response from the GetAncestry method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get ancestry projects](struct.ProjectGetAncestryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetAncestryResponse {
    /// Ancestors are ordered from bottom to top of the resource hierarchy. The
    /// first ancestor is the project itself, followed by the project's parent,
    /// etc.
    pub ancestor: Option<Vec<Ancestor>>,
}

impl ResponseResult for GetAncestryResponse {}


/// The entity that owns an Organization. The lifetime of the Organization and
/// all of its descendants are bound to the `OrganizationOwner`. If the
/// `OrganizationOwner` is deleted, the Organization and all its descendants will
/// be deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationOwner {
    /// The G Suite customer id used in the Directory API.
    #[serde(rename="directoryCustomerId")]
    pub directory_customer_id: Option<String>,
}

impl Part for OrganizationOwner {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the `CloudResourceManager` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `get_iam_policy(...)`, `list(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
}

impl<'a, C, A> MethodsBuilder for OrganizationMethods<'a, C, A> {}

impl<'a, C, A> OrganizationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on an Organization resource. Replaces any
    /// existing policy. The `resource` field should be the organization's resource
    /// name, e.g. "organizations/123".
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> OrganizationSetIamPolicyCall<'a, C, A> {
        OrganizationSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for an Organization resource. May be empty
    /// if no such policy or resource exists. The `resource` field should be the
    /// organization's resource name, e.g. "organizations/123".
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> OrganizationGetIamPolicyCall<'a, C, A> {
        OrganizationGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified Organization.
    /// The `resource` field should be the organization's resource name,
    /// e.g. "organizations/123".
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> OrganizationTestIamPermissionCall<'a, C, A> {
        OrganizationTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches an Organization resource identified by the specified resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the Organization to fetch. This is the organization's
    ///            relative path in the API, formatted as "organizations/[organizationId]".
    ///            For example, "organizations/1234".
    pub fn get(&self, name: &str) -> OrganizationGetCall<'a, C, A> {
        OrganizationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _organization_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an Organization resource identified by the specified resource name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output Only. The resource name of the organization. This is the
    ///            organization's relative path in the API. Its format is
    ///            "organizations/[organization_id]". For example, "organizations/1234".
    pub fn update(&self, request: Organization, name: &str) -> OrganizationUpdateCall<'a, C, A> {
        OrganizationUpdateCall {
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
    /// Lists Organization resources that are visible to the user and satisfy
    /// the specified filter. This method returns Organizations in an unspecified
    /// order. New Organizations do not necessarily appear at the end of the list.
    pub fn list(&self) -> OrganizationListCall<'a, C, A> {
        OrganizationListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `CloudResourceManager` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `get_ancestry(...)`, `get_iam_policy(...)`, `list(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)`, `undelete(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified Project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectTestIamPermissionCall<'a, C, A> {
        ProjectTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores the Project identified by the specified
    /// `project_id` (for example, `my-project-123`).
    /// You can only use this method for a Project that has a lifecycle state of
    /// DELETE_REQUESTED.
    /// After deletion starts, the Project cannot be restored.
    /// 
    /// The caller must have modify permissions for this Project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The project ID (for example, `foo-bar-123`).
    ///                 Required.
    pub fn undelete(&self, request: UndeleteProjectRequest, project_id: &str) -> ProjectUndeleteCall<'a, C, A> {
        ProjectUndeleteCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Project resource.
    /// 
    /// Initially, the Project resource is owned by its creator exclusively.
    /// The creator can later grant permission to others to read or update the
    /// Project.
    /// 
    /// Several APIs are activated automatically for the Project, including
    /// Google Cloud Storage. The parent is identified by a specified
    /// ResourceId, which must include both an ID and a type, such as
    /// project, folder, or organization.
    /// 
    /// This method does not associate the new project with a billing account.
    /// You can set or update the billing account associated with a project using
    /// the [`projects.updateBillingInfo`]
    /// (/billing/reference/rest/v1/projects/updateBillingInfo) method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Project) -> ProjectCreateCall<'a, C, A> {
        ProjectCreateCall {
            hub: self.hub,
            _request: request,
            _use_legacy_stack: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the IAM access control policy for the specified Project. Overwrites
    /// any existing policy.
    /// 
    /// The following constraints apply when using `setIamPolicy()`:
    /// 
    /// + Project does not support `allUsers` and `allAuthenticatedUsers` as
    /// `members` in a `Binding` of a `Policy`.
    /// 
    /// + The owner role can be granted only to `user` and `serviceAccount`.
    /// 
    /// + Service accounts can be made owners of a project directly
    /// without any restrictions. However, to be added as an owner, a user must be
    /// invited via Cloud Platform console and must accept the invitation.
    /// 
    /// + A user cannot be granted the owner role using `setIamPolicy()`. The user
    /// must be granted the owner role using the Cloud Platform Console and must
    /// explicitly accept the invitation.
    /// 
    /// + Invitations to grant the owner role cannot be sent using
    /// `setIamPolicy()`; they must be sent only using the Cloud Platform Console.
    /// 
    /// + Membership changes that leave the project without any owners that have
    /// accepted the Terms of Service (ToS) will be rejected.
    /// 
    /// + If the project is not part of an organization, there must be at least
    /// one owner who has accepted the Terms of Service (ToS) agreement in the
    /// policy. Calling `setIamPolicy()` to remove the last ToS-accepted owner
    /// from the policy will fail. This restriction also applies to legacy
    /// projects that no longer have owners who have accepted the ToS. Edits to
    /// IAM policies will be rejected until the lack of a ToS-accepting owner is
    /// rectified.
    /// 
    /// + This method will replace the existing policy, and cannot be used to
    /// append additional IAM settings.
    /// 
    /// Note: Removing service accounts from policies or changing their roles
    /// can render services completely inoperable. It is important to understand
    /// how the service account is being used before removing or updating its
    /// roles.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectSetIamPolicyCall<'a, C, A> {
        ProjectSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the Project identified by the specified
    /// `project_id` (for example, `my-project-123`).
    /// 
    /// The caller must have read permissions for this Project.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - The Project ID (for example, `my-project-123`).
    ///                 Required.
    pub fn get(&self, project_id: &str) -> ProjectGetCall<'a, C, A> {
        ProjectGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a list of ancestors in the resource hierarchy for the Project
    /// identified by the specified `project_id` (for example, `my-project-123`).
    /// 
    /// The caller must have read permissions for this Project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The Project ID (for example, `my-project-123`).
    ///                 Required.
    pub fn get_ancestry(&self, request: GetAncestryRequest, project_id: &str) -> ProjectGetAncestryCall<'a, C, A> {
        ProjectGetAncestryCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the attributes of the Project identified by the specified
    /// `project_id` (for example, `my-project-123`).
    /// 
    /// The caller must have modify permissions for this Project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The project ID (for example, `my-project-123`).
    ///                 Required.
    pub fn update(&self, request: Project, project_id: &str) -> ProjectUpdateCall<'a, C, A> {
        ProjectUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the IAM access control policy for the specified Project.
    /// Permission is denied if the policy or the resource does not exist.
    /// 
    /// For additional information about resource structure and identification,
    /// see [Resource Names](/apis/design/resource_names).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectGetIamPolicyCall<'a, C, A> {
        ProjectGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks the Project identified by the specified
    /// `project_id` (for example, `my-project-123`) for deletion.
    /// This method will only affect the Project if it has a lifecycle state of
    /// ACTIVE.
    /// 
    /// This method changes the Project's lifecycle state from
    /// ACTIVE
    /// to DELETE_REQUESTED.
    /// The deletion starts at an unspecified time, at which point the project is
    /// no longer accessible.
    /// 
    /// Until the deletion completes, you can check the lifecycle state
    /// checked by retrieving the Project with GetProject,
    /// and the Project remains visible to ListProjects.
    /// However, you cannot update the project.
    /// 
    /// After the deletion completes, the Project is not retrievable by
    /// the  GetProject
    /// and ListProjects
    /// methods.
    /// 
    /// The caller must have modify permissions for this Project.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - The Project ID (for example, `foo-bar-123`).
    ///                 Required.
    pub fn delete(&self, project_id: &str) -> ProjectDeleteCall<'a, C, A> {
        ProjectDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Projects that the caller has the `resourcemanager.projects.get`
    /// permission on and satisfy the specified filter.
    /// 
    /// This method returns Projects in an unspecified order.
    /// This method is eventually consistent with project mutations; this means
    /// that a newly created project may not appear in the results or recent
    /// updates to an existing project may not be reflected in the results. To
    /// retrieve the latest state of a project, use the
    /// GetProject method.
    /// 
    /// NOTE: If the request filter contains a `parent.type` and `parent.id` and
    /// the caller has the `resourcemanager.projects.list` permission on the
    /// parent, the results will be drawn from an alternate index which provides
    /// more consistent results. In future versions of this API, this List method
    /// will be split into List and Search to properly capture the behavorial
    /// difference.
    pub fn list(&self) -> ProjectListCall<'a, C, A> {
        ProjectListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Sets the access control policy on an Organization resource. Replaces any
/// existing policy. The `resource` field should be the organization's resource
/// name, e.g. "organizations/123".
///
/// A builder for the *setIamPolicy* method supported by a *organization* resource.
/// It is not used directly, but through a `OrganizationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::SetIamPolicyRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SetIamPolicyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.organizations().set_iam_policy(req, "resource")
///              .doit();
/// # }
/// ```
pub struct OrganizationSetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: SetIamPolicyRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrganizationSetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> OrganizationSetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.organizations.setIamPolicy",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resource", self._resource.to_string()));
        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+resource}:setIamPolicy";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
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
            for param_name in ["resource"].iter() {
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
    pub fn request(mut self, new_value: SetIamPolicyRequest) -> OrganizationSetIamPolicyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy is being specified.
    /// See the operation documentation for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> OrganizationSetIamPolicyCall<'a, C, A> {
        self._resource = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> OrganizationSetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OrganizationSetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OrganizationSetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the access control policy for an Organization resource. May be empty
/// if no such policy or resource exists. The `resource` field should be the
/// organization's resource name, e.g. "organizations/123".
///
/// A builder for the *getIamPolicy* method supported by a *organization* resource.
/// It is not used directly, but through a `OrganizationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::GetIamPolicyRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIamPolicyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.organizations().get_iam_policy(req, "resource")
///              .doit();
/// # }
/// ```
pub struct OrganizationGetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: GetIamPolicyRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrganizationGetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> OrganizationGetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.organizations.getIamPolicy",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resource", self._resource.to_string()));
        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+resource}:getIamPolicy";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
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
            for param_name in ["resource"].iter() {
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
    pub fn request(mut self, new_value: GetIamPolicyRequest) -> OrganizationGetIamPolicyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy is being requested.
    /// See the operation documentation for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> OrganizationGetIamPolicyCall<'a, C, A> {
        self._resource = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> OrganizationGetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OrganizationGetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OrganizationGetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns permissions that a caller has on the specified Organization.
/// The `resource` field should be the organization's resource name,
/// e.g. "organizations/123".
///
/// A builder for the *testIamPermissions* method supported by a *organization* resource.
/// It is not used directly, but through a `OrganizationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::TestIamPermissionsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestIamPermissionsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.organizations().test_iam_permissions(req, "resource")
///              .doit();
/// # }
/// ```
pub struct OrganizationTestIamPermissionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: TestIamPermissionsRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrganizationTestIamPermissionCall<'a, C, A> {}

impl<'a, C, A> OrganizationTestIamPermissionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TestIamPermissionsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.organizations.testIamPermissions",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resource", self._resource.to_string()));
        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+resource}:testIamPermissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
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
            for param_name in ["resource"].iter() {
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
    pub fn request(mut self, new_value: TestIamPermissionsRequest) -> OrganizationTestIamPermissionCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy detail is being requested.
    /// See the operation documentation for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> OrganizationTestIamPermissionCall<'a, C, A> {
        self._resource = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> OrganizationTestIamPermissionCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OrganizationTestIamPermissionCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OrganizationTestIamPermissionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Fetches an Organization resource identified by the specified resource name.
///
/// A builder for the *get* method supported by a *organization* resource.
/// It is not used directly, but through a `OrganizationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.organizations().get("name")
///              .organization_id("sea")
///              .doit();
/// # }
/// ```
pub struct OrganizationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _name: String,
    _organization_id: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrganizationGetCall<'a, C, A> {}

impl<'a, C, A> OrganizationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Organization)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.organizations.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._organization_id {
            params.push(("organizationId", value.to_string()));
        }
        for &field in ["alt", "name", "organizationId"].iter() {
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


    /// The resource name of the Organization to fetch. This is the organization's
    /// relative path in the API, formatted as "organizations/[organizationId]".
    /// For example, "organizations/1234".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OrganizationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The id of the Organization resource to fetch.
    /// This field is deprecated and will be removed in v1. Use name instead.
    ///
    /// Sets the *organization id* query property to the given value.
    pub fn organization_id(mut self, new_value: &str) -> OrganizationGetCall<'a, C, A> {
        self._organization_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> OrganizationGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OrganizationGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OrganizationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an Organization resource identified by the specified resource name.
///
/// A builder for the *update* method supported by a *organization* resource.
/// It is not used directly, but through a `OrganizationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::Organization;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Organization::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.organizations().update(req, "name")
///              .doit();
/// # }
/// ```
pub struct OrganizationUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: Organization,
    _name: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrganizationUpdateCall<'a, C, A> {}

impl<'a, C, A> OrganizationUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Organization)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.organizations.update",
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
    pub fn request(mut self, new_value: Organization) -> OrganizationUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Output Only. The resource name of the organization. This is the
    /// organization's relative path in the API. Its format is
    /// "organizations/[organization_id]". For example, "organizations/1234".
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OrganizationUpdateCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> OrganizationUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OrganizationUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OrganizationUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists Organization resources that are visible to the user and satisfy
/// the specified filter. This method returns Organizations in an unspecified
/// order. New Organizations do not necessarily appear at the end of the list.
///
/// A builder for the *list* method supported by a *organization* resource.
/// It is not used directly, but through a `OrganizationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.organizations().list()
///              .page_token("dolores")
///              .page_size(-61)
///              .filter("sadipscing")
///              .doit();
/// # }
/// ```
pub struct OrganizationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OrganizationListCall<'a, C, A> {}

impl<'a, C, A> OrganizationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListOrganizationsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.organizations.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/organizations";
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


    /// A pagination token returned from a previous call to `ListOrganizations`
    /// that indicates from where listing should continue.
    /// This field is optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OrganizationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of Organizations to return in the response.
    /// This field is optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> OrganizationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// An optional query string used to filter the Organizations to return in
    /// the response. Filter rules are case-insensitive.
    /// 
    /// 
    /// Organizations may be filtered by `owner.directoryCustomerId` or by
    /// `domain`, where the domain is a G Suite domain, for example:
    /// 
    /// * Filter `owner.directorycustomerid:123456789` returns Organization
    /// resources with `owner.directory_customer_id` equal to `123456789`.
    /// * Filter `domain:google.com` returns Organization resources corresponding
    /// to the domain `google.com`.
    /// 
    /// This field is optional.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> OrganizationListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> OrganizationListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OrganizationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OrganizationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns permissions that a caller has on the specified Project.
///
/// A builder for the *testIamPermissions* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::TestIamPermissionsRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestIamPermissionsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_iam_permissions(req, "resource")
///              .doit();
/// # }
/// ```
pub struct ProjectTestIamPermissionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: TestIamPermissionsRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectTestIamPermissionCall<'a, C, A> {}

impl<'a, C, A> ProjectTestIamPermissionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TestIamPermissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.testIamPermissions",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resource", self._resource.to_string()));
        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{resource}:testIamPermissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{resource}", "resource")].iter() {
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
            for param_name in ["resource"].iter() {
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
    pub fn request(mut self, new_value: TestIamPermissionsRequest) -> ProjectTestIamPermissionCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy detail is being requested.
    /// See the operation documentation for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> ProjectTestIamPermissionCall<'a, C, A> {
        self._resource = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectTestIamPermissionCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestIamPermissionCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectTestIamPermissionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Restores the Project identified by the specified
/// `project_id` (for example, `my-project-123`).
/// You can only use this method for a Project that has a lifecycle state of
/// DELETE_REQUESTED.
/// After deletion starts, the Project cannot be restored.
/// 
/// The caller must have modify permissions for this Project.
///
/// A builder for the *undelete* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::UndeleteProjectRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UndeleteProjectRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().undelete(req, "projectId")
///              .doit();
/// # }
/// ```
pub struct ProjectUndeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: UndeleteProjectRequest,
    _project_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectUndeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectUndeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.undelete",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{projectId}:undelete";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: UndeleteProjectRequest) -> ProjectUndeleteCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project ID (for example, `foo-bar-123`).
    /// 
    /// Required.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectUndeleteCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectUndeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectUndeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectUndeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a Project resource.
/// 
/// Initially, the Project resource is owned by its creator exclusively.
/// The creator can later grant permission to others to read or update the
/// Project.
/// 
/// Several APIs are activated automatically for the Project, including
/// Google Cloud Storage. The parent is identified by a specified
/// ResourceId, which must include both an ID and a type, such as
/// project, folder, or organization.
/// 
/// This method does not associate the new project with a billing account.
/// You can set or update the billing account associated with a project using
/// the [`projects.updateBillingInfo`]
/// (/billing/reference/rest/v1/projects/updateBillingInfo) method.
///
/// A builder for the *create* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::Project;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Project::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().create(req)
///              .use_legacy_stack(false)
///              .doit();
/// # }
/// ```
pub struct ProjectCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: Project,
    _use_legacy_stack: Option<bool>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectCreateCall<'a, C, A> {}

impl<'a, C, A> ProjectCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Project)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._use_legacy_stack {
            params.push(("useLegacyStack", value.to_string()));
        }
        for &field in ["alt", "useLegacyStack"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects";
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
    pub fn request(mut self, new_value: Project) -> ProjectCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// A safety hatch to opt out of the new reliable project creation process.
    ///
    /// Sets the *use legacy stack* query property to the given value.
    pub fn use_legacy_stack(mut self, new_value: bool) -> ProjectCreateCall<'a, C, A> {
        self._use_legacy_stack = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets the IAM access control policy for the specified Project. Overwrites
/// any existing policy.
/// 
/// The following constraints apply when using `setIamPolicy()`:
/// 
/// + Project does not support `allUsers` and `allAuthenticatedUsers` as
/// `members` in a `Binding` of a `Policy`.
/// 
/// + The owner role can be granted only to `user` and `serviceAccount`.
/// 
/// + Service accounts can be made owners of a project directly
/// without any restrictions. However, to be added as an owner, a user must be
/// invited via Cloud Platform console and must accept the invitation.
/// 
/// + A user cannot be granted the owner role using `setIamPolicy()`. The user
/// must be granted the owner role using the Cloud Platform Console and must
/// explicitly accept the invitation.
/// 
/// + Invitations to grant the owner role cannot be sent using
/// `setIamPolicy()`; they must be sent only using the Cloud Platform Console.
/// 
/// + Membership changes that leave the project without any owners that have
/// accepted the Terms of Service (ToS) will be rejected.
/// 
/// + If the project is not part of an organization, there must be at least
/// one owner who has accepted the Terms of Service (ToS) agreement in the
/// policy. Calling `setIamPolicy()` to remove the last ToS-accepted owner
/// from the policy will fail. This restriction also applies to legacy
/// projects that no longer have owners who have accepted the ToS. Edits to
/// IAM policies will be rejected until the lack of a ToS-accepting owner is
/// rectified.
/// 
/// + This method will replace the existing policy, and cannot be used to
/// append additional IAM settings.
/// 
/// Note: Removing service accounts from policies or changing their roles
/// can render services completely inoperable. It is important to understand
/// how the service account is being used before removing or updating its
/// roles.
///
/// A builder for the *setIamPolicy* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::SetIamPolicyRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SetIamPolicyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().set_iam_policy(req, "resource")
///              .doit();
/// # }
/// ```
pub struct ProjectSetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: SetIamPolicyRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectSetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> ProjectSetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.setIamPolicy",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resource", self._resource.to_string()));
        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{resource}:setIamPolicy";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{resource}", "resource")].iter() {
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
            for param_name in ["resource"].iter() {
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
    pub fn request(mut self, new_value: SetIamPolicyRequest) -> ProjectSetIamPolicyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy is being specified.
    /// See the operation documentation for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> ProjectSetIamPolicyCall<'a, C, A> {
        self._resource = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectSetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectSetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves the Project identified by the specified
/// `project_id` (for example, `my-project-123`).
/// 
/// The caller must have read permissions for this Project.
///
/// A builder for the *get* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get("projectId")
///              .doit();
/// # }
/// ```
pub struct ProjectGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _project_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectGetCall<'a, C, A> {}

impl<'a, C, A> ProjectGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Project)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{projectId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// The Project ID (for example, `my-project-123`).
    /// 
    /// Required.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectGetCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a list of ancestors in the resource hierarchy for the Project
/// identified by the specified `project_id` (for example, `my-project-123`).
/// 
/// The caller must have read permissions for this Project.
///
/// A builder for the *getAncestry* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::GetAncestryRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetAncestryRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get_ancestry(req, "projectId")
///              .doit();
/// # }
/// ```
pub struct ProjectGetAncestryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: GetAncestryRequest,
    _project_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectGetAncestryCall<'a, C, A> {}

impl<'a, C, A> ProjectGetAncestryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetAncestryResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.getAncestry",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{projectId}:getAncestry";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: GetAncestryRequest) -> ProjectGetAncestryCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The Project ID (for example, `my-project-123`).
    /// 
    /// Required.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectGetAncestryCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectGetAncestryCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetAncestryCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectGetAncestryCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the attributes of the Project identified by the specified
/// `project_id` (for example, `my-project-123`).
/// 
/// The caller must have modify permissions for this Project.
///
/// A builder for the *update* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::Project;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Project::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().update(req, "projectId")
///              .doit();
/// # }
/// ```
pub struct ProjectUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: Project,
    _project_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectUpdateCall<'a, C, A> {}

impl<'a, C, A> ProjectUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Project)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{projectId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Project) -> ProjectUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project ID (for example, `my-project-123`).
    /// 
    /// Required.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectUpdateCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the IAM access control policy for the specified Project.
/// Permission is denied if the policy or the resource does not exist.
/// 
/// For additional information about resource structure and identification,
/// see [Resource Names](/apis/design/resource_names).
///
/// A builder for the *getIamPolicy* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// use cloudresourcemanager1_beta1::GetIamPolicyRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIamPolicyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get_iam_policy(req, "resource")
///              .doit();
/// # }
/// ```
pub struct ProjectGetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _request: GetIamPolicyRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectGetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> ProjectGetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.getIamPolicy",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("resource", self._resource.to_string()));
        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{resource}:getIamPolicy";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{resource}", "resource")].iter() {
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
            for param_name in ["resource"].iter() {
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
    pub fn request(mut self, new_value: GetIamPolicyRequest) -> ProjectGetIamPolicyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy is being requested.
    /// See the operation documentation for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> ProjectGetIamPolicyCall<'a, C, A> {
        self._resource = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectGetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectGetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Marks the Project identified by the specified
/// `project_id` (for example, `my-project-123`) for deletion.
/// This method will only affect the Project if it has a lifecycle state of
/// ACTIVE.
/// 
/// This method changes the Project's lifecycle state from
/// ACTIVE
/// to DELETE_REQUESTED.
/// The deletion starts at an unspecified time, at which point the project is
/// no longer accessible.
/// 
/// Until the deletion completes, you can check the lifecycle state
/// checked by retrieving the Project with GetProject,
/// and the Project remains visible to ListProjects.
/// However, you cannot update the project.
/// 
/// After the deletion completes, the Project is not retrievable by
/// the  GetProject
/// and ListProjects
/// methods.
/// 
/// The caller must have modify permissions for this Project.
///
/// A builder for the *delete* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().delete("projectId")
///              .doit();
/// # }
/// ```
pub struct ProjectDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _project_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectDeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects/{projectId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// The Project ID (for example, `foo-bar-123`).
    /// 
    /// Required.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectDeleteCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists Projects that the caller has the `resourcemanager.projects.get`
/// permission on and satisfy the specified filter.
/// 
/// This method returns Projects in an unspecified order.
/// This method is eventually consistent with project mutations; this means
/// that a newly created project may not appear in the results or recent
/// updates to an existing project may not be reflected in the results. To
/// retrieve the latest state of a project, use the
/// GetProject method.
/// 
/// NOTE: If the request filter contains a `parent.type` and `parent.id` and
/// the caller has the `resourcemanager.projects.list` permission on the
/// parent, the results will be drawn from an alternate index which provides
/// more consistent results. In future versions of this API, this List method
/// will be split into List and Search to properly capture the behavorial
/// difference.
///
/// A builder for the *list* method supported by a *project* resource.
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
/// # extern crate google_cloudresourcemanager1_beta1 as cloudresourcemanager1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudresourcemanager1_beta1::CloudResourceManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudResourceManager::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().list()
///              .page_token("Lorem")
///              .page_size(-21)
///              .filter("duo")
///              .doit();
/// # }
/// ```
pub struct ProjectListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudResourceManager<C, A>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectListCall<'a, C, A> {}

impl<'a, C, A> ProjectListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListProjectsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudresourcemanager.projects.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/projects";
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


    /// A pagination token returned from a previous call to ListProjects
    /// that indicates from where listing should continue.
    /// 
    /// Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of Projects to return in the response.
    /// The server can return fewer Projects than requested.
    /// If unspecified, server picks an appropriate default.
    /// 
    /// Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// An expression for filtering the results of the request.  Filter rules are
    /// case insensitive. The fields eligible for filtering are:
    /// 
    /// + `name`
    /// + `id`
    /// + `labels.<key>` (where *key* is the name of a label)
    /// + `parent.type`
    /// + `parent.id`
    /// 
    /// Some examples of using labels as filters:
    /// 
    /// | Filter           | Description                                         |
    /// |------------------|-----------------------------------------------------|
    /// | name:how*        | The project's name starts with "how".               |
    /// | name:Howl        | The project's name is `Howl` or `howl`.             |
    /// | name:HOWL        | Equivalent to above.                                |
    /// | NAME:howl        | Equivalent to above.                                |
    /// | labels.color:*   | The project has the label `color`.                  |
    /// | labels.color:red | The project's label `color` has the value `red`.    |
    /// | labels.color:red&nbsp;labels.size:big |The project's label `color` has
    ///   the value `red` and its label `size` has the value `big`.              |
    /// 
    /// If no filter is specified, the call will return projects for which the user
    /// has the `resourcemanager.projects.get` permission.
    /// 
    /// NOTE: To perform a by-parent query (eg., what projects are directly in a
    /// Folder), the caller must have the `resourcemanager.projects.list`
    /// permission on the parent and the filter must contain both a `parent.type`
    /// and a `parent.id` restriction
    /// (example: "parent.type:folder parent.id:123"). In this case an alternate
    /// search index is used which provides more consistent results.
    /// 
    /// Optional.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> ProjectListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ProjectListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


