// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Drive Activity* crate version *1.0.10+20190702*, where *20190702* is the exact revision of the *driveactivity:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.10*.
//! 
//! Everything else about the *Drive Activity* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/drive/activity/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/driveactivity2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.DriveActivityHub.html) ... 
//! 
//! * activity
//!  * [*query*](struct.ActivityQueryCall.html)
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
//! * **[Hub](struct.DriveActivityHub.html)**
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
//! let r = hub.activity().query(...).doit()
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
//! google-driveactivity2 = "*"
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
//! extern crate google_driveactivity2 as driveactivity2;
//! use driveactivity2::QueryDriveActivityRequest;
//! use driveactivity2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use driveactivity2::DriveActivityHub;
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
//! let mut hub = DriveActivityHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = QueryDriveActivityRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.activity().query(req)
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
    /// View and add to the activity record of files in your Google Drive
    DriveActivity,

    /// View the activity record of files in your Google Drive
    DriveActivityReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DriveActivity => "https://www.googleapis.com/auth/drive.activity",
            Scope::DriveActivityReadonly => "https://www.googleapis.com/auth/drive.activity.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DriveActivityReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all DriveActivityHub related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_driveactivity2 as driveactivity2;
/// use driveactivity2::QueryDriveActivityRequest;
/// use driveactivity2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use driveactivity2::DriveActivityHub;
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
/// let mut hub = DriveActivityHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryDriveActivityRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activity().query(req)
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
pub struct DriveActivityHub<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for DriveActivityHub<C, A> {}

impl<'a, C, A> DriveActivityHub<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> DriveActivityHub<C, A> {
        DriveActivityHub {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.10".to_string(),
            _base_url: "https://driveactivity.googleapis.com/".to_string(),
            _root_url: "https://driveactivity.googleapis.com/".to_string(),
        }
    }

    pub fn activity(&'a self) -> ActivityMethods<'a, C, A> {
        ActivityMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.10`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://driveactivity.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://driveactivity.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Information about an impersonation, where an admin acts on behalf of an end
/// user. Information about the acting admin is not currently available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Impersonation {
    /// The impersonated user.
    #[serde(rename="impersonatedUser")]
    pub impersonated_user: Option<User>,
}

impl Part for Impersonation {}


/// Information about a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// The email address of the group.
    pub email: Option<String>,
    /// The title of the group.
    pub title: Option<String>,
}

impl Part for Group {}


/// A change of the permission setting on an item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionChange {
    /// The set of permissions removed by this change.
    #[serde(rename="removedPermissions")]
    pub removed_permissions: Option<Vec<Permission>>,
    /// The set of permissions added by this change.
    #[serde(rename="addedPermissions")]
    pub added_permissions: Option<Vec<Permission>>,
}

impl Part for PermissionChange {}


/// An object was created.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Create {
    /// If present, indicates the object was newly created (e.g. as a blank
    /// document), not derived from a Drive object or external object.
    pub new: Option<New>,
    /// If present, indicates the object was created by copying an existing Drive
    /// object.
    pub copy: Option<Copy>,
    /// If present, indicates the object originated externally and was uploaded
    /// to Drive.
    pub upload: Option<Upload>,
}

impl Part for Create {}


/// Information about time ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    /// The end of the time range.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// The start of the time range.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
}

impl Part for TimeRange {}


/// Activity in applications other than Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationReference {
    /// The reference type corresponding to this event.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for ApplicationReference {}


/// The actor of a Drive activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    /// An account acting on behalf of another.
    pub impersonation: Option<Impersonation>,
    /// An administrator.
    pub administrator: Option<Administrator>,
    /// A non-user actor (i.e. system triggered).
    pub system: Option<SystemEvent>,
    /// An anonymous user.
    pub anonymous: Option<AnonymousUser>,
    /// An end user.
    pub user: Option<User>,
}

impl Part for Actor {}


/// Information about restriction policy changes to a feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestrictionChange {
    /// The feature which had a change in restriction policy.
    pub feature: Option<String>,
    /// The restriction in place after the change.
    #[serde(rename="newRestriction")]
    pub new_restriction: Option<String>,
}

impl Part for RestrictionChange {}


/// Represents any user (including a logged out user).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Anyone { _never_set: Option<bool> }

impl Part for Anyone {}


/// A user about whom nothing is currently known.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnknownUser { _never_set: Option<bool> }

impl Part for UnknownUser {}


/// An object was created from scratch.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct New { _never_set: Option<bool> }

impl Part for New {}


/// A lightweight reference to a shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveReference {
    /// The resource name of the shared drive. The format is
    /// "COLLECTION_ID/DRIVE_ID". Clients should not assume a specific collection
    /// ID for this resource name.
    pub name: Option<String>,
    /// The title of the shared drive.
    pub title: Option<String>,
}

impl Part for DriveReference {}


/// This item is deprecated; please see `DriveFolder` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    /// This field is deprecated; please see `DriveFolder.type` instead.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for Folder {}


/// An object was uploaded into Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Upload { _never_set: Option<bool> }

impl Part for Upload {}


/// A user whose account has since been deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeletedUser { _never_set: Option<bool> }

impl Part for DeletedUser {}


/// This item is deprecated; please see `Drive` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDrive {
    /// This field is deprecated; please see `Drive.root` instead.
    pub root: Option<DriveItem>,
    /// This field is deprecated; please see `Drive.name` instead.
    pub name: Option<String>,
    /// This field is deprecated; please see `Drive.title` instead.
    pub title: Option<String>,
}

impl Part for TeamDrive {}


/// Empty message representing an administrator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Administrator { _never_set: Option<bool> }

impl Part for Administrator {}


/// Empty message representing an anonymous user or indicating the authenticated
/// user should be anonymized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnonymousUser { _never_set: Option<bool> }

impl Part for AnonymousUser {}


/// The permission setting of an object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// If set, this permission applies to anyone, even logged out users.
    pub anyone: Option<Anyone>,
    /// The domain to whom this permission applies.
    pub domain: Option<Domain>,
    /// The group to whom this permission applies.
    pub group: Option<Group>,
    /// If true, the item can be discovered (e.g. in the user's "Shared with me"
    /// collection) without needing a link to the item.
    #[serde(rename="allowDiscovery")]
    pub allow_discovery: Option<bool>,
    /// Indicates the
    /// <a href="/drive/web/manage-sharing#roles">Google Drive permissions
    /// role</a>. The role determines a user's ability to read, write, and
    /// comment on items.
    pub role: Option<String>,
    /// The user to whom this permission applies.
    pub user: Option<User>,
}

impl Part for Permission {}


/// A comment with an assignment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Assignment {
    /// The sub-type of this event.
    pub subtype: Option<String>,
}

impl Part for Assignment {}


/// Event triggered by system operations instead of end users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    /// The type of the system event that may triggered activity.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for SystemEvent {}


/// A Drive item which is a file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFile { _never_set: Option<bool> }

impl Part for DriveFile {}


/// Data describing the type and additional information of an action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActionDetail {
    /// A change about comments was made.
    pub comment: Option<Comment>,
    /// A change happened in data leak prevention status.
    #[serde(rename="dlpChange")]
    pub dlp_change: Option<DataLeakPreventionChange>,
    /// An object was referenced in an application outside of Drive/Docs.
    pub reference: Option<ApplicationReference>,
    /// The permission on an object was changed.
    #[serde(rename="permissionChange")]
    pub permission_change: Option<PermissionChange>,
    /// An object was edited.
    pub edit: Option<Edit>,
    /// Settings were changed.
    #[serde(rename="settingsChange")]
    pub settings_change: Option<SettingsChange>,
    /// An object was created.
    pub create: Option<Create>,
    /// An object was moved.
    #[serde(rename="move")]
    pub move_: Option<Move>,
    /// An object was renamed.
    pub rename: Option<Rename>,
    /// A deleted object was restored.
    pub restore: Option<Restore>,
    /// An object was deleted.
    pub delete: Option<Delete>,
}

impl Part for ActionDetail {}


/// A Drive item, such as a file or folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveItem {
    /// The MIME type of the Drive item.  See
    /// https://developers.google.com/drive/v3/web/mime-types.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// The target Drive item. The format is "items/ITEM_ID".
    pub name: Option<String>,
    /// The title of the Drive item.
    pub title: Option<String>,
    /// The Drive item is a folder.
    #[serde(rename="driveFolder")]
    pub drive_folder: Option<DriveFolder>,
    /// The Drive item is a file.
    #[serde(rename="driveFile")]
    pub drive_file: Option<DriveFile>,
    /// This field is deprecated; please use the `driveFile` field instead.
    pub file: Option<File>,
    /// Information about the owner of this Drive item.
    pub owner: Option<Owner>,
    /// This field is deprecated; please use the `driveFolder` field instead.
    pub folder: Option<Folder>,
}

impl Part for DriveItem {}


/// A regular posted comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    /// The sub-type of this event.
    pub subtype: Option<String>,
}

impl Part for Post {}


/// An object was created by copying an existing object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Copy {
    /// The the original object.
    #[serde(rename="originalObject")]
    pub original_object: Option<TargetReference>,
}

impl Part for Copy {}


/// A lightweight reference to a Drive item, such as a file or folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveItemReference {
    /// The Drive item is a file.
    #[serde(rename="driveFile")]
    pub drive_file: Option<DriveFile>,
    /// The target Drive item. The format is "items/ITEM_ID".
    pub name: Option<String>,
    /// This field is deprecated; please use the `driveFile` field instead.
    pub file: Option<File>,
    /// The title of the Drive item.
    pub title: Option<String>,
    /// This field is deprecated; please use the `driveFolder` field instead.
    pub folder: Option<Folder>,
    /// The Drive item is a folder.
    #[serde(rename="driveFolder")]
    pub drive_folder: Option<DriveFolder>,
}

impl Part for DriveItemReference {}


/// A change in the object's data leak prevention status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataLeakPreventionChange {
    /// The type of Data Leak Prevention (DLP) change.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for DataLeakPreventionChange {}


/// A change about comments on an object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// A change on an assignment.
    pub assignment: Option<Assignment>,
    /// A change on a regular posted comment.
    pub post: Option<Post>,
    /// Users who are mentioned in this comment.
    #[serde(rename="mentionedUsers")]
    pub mentioned_users: Option<Vec<User>>,
    /// A change on a suggestion.
    pub suggestion: Option<Suggestion>,
}

impl Part for Comment {}


/// Information about the target of activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// This field is deprecated; please use the `drive` field instead.
    #[serde(rename="teamDrive")]
    pub team_drive: Option<TeamDrive>,
    /// The target is a comment on a Drive file.
    #[serde(rename="fileComment")]
    pub file_comment: Option<FileComment>,
    /// The target is a Drive item.
    #[serde(rename="driveItem")]
    pub drive_item: Option<DriveItem>,
    /// The target is a shared drive.
    pub drive: Option<Drive>,
}

impl Part for Target {}


/// An empty message indicating an object was edited.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Edit { _never_set: Option<bool> }

impl Part for Edit {}


/// A strategy which does no consolidation of individual activities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NoConsolidation { _never_set: Option<bool> }

impl Part for NoConsolidation {}


/// A lightweight reference to the target of activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetReference {
    /// This field is deprecated; please use the `drive` field instead.
    #[serde(rename="teamDrive")]
    pub team_drive: Option<TeamDriveReference>,
    /// The target is a Drive item.
    #[serde(rename="driveItem")]
    pub drive_item: Option<DriveItemReference>,
    /// The target is a shared drive.
    pub drive: Option<DriveReference>,
}

impl Part for TargetReference {}


/// Information about a shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Drive {
    /// The root of this shared drive.
    pub root: Option<DriveItem>,
    /// The resource name of the shared drive. The format is
    /// "COLLECTION_ID/DRIVE_ID". Clients should not assume a specific collection
    /// ID for this resource name.
    pub name: Option<String>,
    /// The title of the shared drive.
    pub title: Option<String>,
}

impl Part for Drive {}


/// A comment on a file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileComment {
    /// The comment in the discussion thread. This identifier is an opaque string
    /// compatible with the Drive API; see
    /// https://developers.google.com/drive/v3/reference/comments/get
    #[serde(rename="legacyCommentId")]
    pub legacy_comment_id: Option<String>,
    /// The discussion thread to which the comment was added. This identifier is an
    /// opaque string compatible with the Drive API and references the first
    /// comment in a discussion; see
    /// https://developers.google.com/drive/v3/reference/comments/get
    #[serde(rename="legacyDiscussionId")]
    pub legacy_discussion_id: Option<String>,
    /// The Drive item containing this comment.
    pub parent: Option<DriveItem>,
    /// The link to the discussion thread containing this comment, for example,
    /// "https://docs.google.com/DOCUMENT_ID/edit?disco=THREAD_ID".
    #[serde(rename="linkToDiscussion")]
    pub link_to_discussion: Option<String>,
}

impl Part for FileComment {}


/// A suggestion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Suggestion {
    /// The sub-type of this event.
    pub subtype: Option<String>,
}

impl Part for Suggestion {}


/// Information about the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    /// The action occurred at this specific time.
    pub timestamp: Option<String>,
    /// The action occurred over this time range.
    #[serde(rename="timeRange")]
    pub time_range: Option<TimeRange>,
    /// The type and detailed information about the action.
    pub detail: Option<ActionDetail>,
    /// The actor responsible for this action (or empty if all actors are
    /// responsible).
    pub actor: Option<Actor>,
    /// The target this action affects (or empty if affecting all targets). This
    /// represents the state of the target immediately after this action occurred.
    pub target: Option<Target>,
}

impl Part for Action {}


/// A single Drive activity comprising one or more Actions by one or more
/// Actors on one or more Targets. Some Action groupings occur spontaneously,
/// such as moving an item into a shared folder triggering a permission change.
/// Other groupings of related Actions, such as multiple Actors editing one item
/// or moving multiple files into a new folder, are controlled by the selection
/// of a ConsolidationStrategy in the QueryDriveActivityRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveActivity {
    /// Key information about the primary action for this activity. This is either
    /// representative, or the most important, of all actions in the activity,
    /// according to the ConsolidationStrategy in the request.
    #[serde(rename="primaryActionDetail")]
    pub primary_action_detail: Option<ActionDetail>,
    /// The activity occurred at this specific time.
    pub timestamp: Option<String>,
    /// The activity occurred over this time range.
    #[serde(rename="timeRange")]
    pub time_range: Option<TimeRange>,
    /// All actor(s) responsible for the activity.
    pub actors: Option<Vec<Actor>>,
    /// Details on all actions in this activity.
    pub actions: Option<Vec<Action>>,
    /// All Google Drive objects this activity is about (e.g. file, folder, drive).
    /// This represents the state of the target immediately after the actions
    /// occurred.
    pub targets: Option<Vec<Target>>,
}

impl Part for DriveActivity {}


/// A Drive item which is a folder.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveFolder {
    /// The type of Drive folder.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for DriveFolder {}


/// An object was deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Delete {
    /// The type of delete action taken.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for Delete {}


/// An object was renamed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rename {
    /// The new title of the drive object.
    #[serde(rename="newTitle")]
    pub new_title: Option<String>,
    /// The previous title of the drive object.
    #[serde(rename="oldTitle")]
    pub old_title: Option<String>,
}

impl Part for Rename {}


/// A deleted object was restored.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Restore {
    /// The type of restore action taken.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for Restore {}


/// Information about a domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Domain {
    /// The name of the domain, e.g. "google.com".
    pub name: Option<String>,
    /// An opaque string used to identify this domain.
    #[serde(rename="legacyId")]
    pub legacy_id: Option<String>,
}

impl Part for Domain {}


/// The request message for querying Drive activity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query activity](struct.ActivityQueryCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryDriveActivityRequest {
    /// The filtering for items returned from this query request. The format of the
    /// filter string is a sequence of expressions, joined by an optional "AND",
    /// where each expression is of the form "field operator value".
    /// 
    /// Supported fields:
    /// 
    /// * <tt>time</tt>: Uses numerical operators on date values either in
    ///   terms of milliseconds since Jan 1, 1970 or in RFC 3339 format.
    ///   Examples:
    ///   
    ///   * <tt>time > 1452409200000 AND time <= 1492812924310</tt>
    ///   * <tt>time >= "2016-01-10T01:02:03-05:00"</tt>
    /// * <tt>detail.action_detail_case</tt>: Uses the "has" operator (:) and
    ///   either a singular value or a list of allowed action types enclosed in
    ///   parentheses.
    ///   Examples:
    ///   
    ///   * <tt>detail.action_detail_case: RENAME</tt>
    ///   * <tt>detail.action_detail_case:(CREATE UPLOAD)</tt>
    ///   * <tt>-detail.action_detail_case:MOVE</tt>
    pub filter: Option<String>,
    /// The next_page_token value returned from a previous QueryDriveActivity
    /// request, if any.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
    /// Details on how to consolidate related actions that make up the activity. If
    /// not set, then related actions will not be consolidated.
    #[serde(rename="consolidationStrategy")]
    pub consolidation_strategy: Option<ConsolidationStrategy>,
    /// The requested number of activity to return. If not set, a default value
    /// will be used.
    #[serde(rename="pageSize")]
    pub page_size: Option<i32>,
    /// Return activities for this Drive folder and all children and descendants.
    /// The format is "items/ITEM_ID".
    #[serde(rename="ancestorName")]
    pub ancestor_name: Option<String>,
    /// Return activities for this Drive item. The format is
    /// "items/ITEM_ID".
    #[serde(rename="itemName")]
    pub item_name: Option<String>,
}

impl RequestValue for QueryDriveActivityRequest {}


/// A known user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KnownUser {
    /// The identifier for this user that can be used with the People API to get
    /// more information. The format is "people/ACCOUNT_ID". See
    /// https://developers.google.com/people/.
    #[serde(rename="personName")]
    pub person_name: Option<String>,
    /// True if this is the user making the request.
    #[serde(rename="isCurrentUser")]
    pub is_current_user: Option<bool>,
}

impl Part for KnownUser {}


/// An object was moved.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    /// The removed parent object(s).
    #[serde(rename="removedParents")]
    pub removed_parents: Option<Vec<TargetReference>>,
    /// The added parent object(s).
    #[serde(rename="addedParents")]
    pub added_parents: Option<Vec<TargetReference>>,
}

impl Part for Move {}


/// This item is deprecated; please see `DriveReference` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveReference {
    /// This field is deprecated; please see `DriveReference.name` instead.
    pub name: Option<String>,
    /// This field is deprecated; please see `DriveReference.title` instead.
    pub title: Option<String>,
}

impl Part for TeamDriveReference {}


/// How the individual activities are consolidated. A set of activities may be
/// consolidated into one combined activity if they are related in some way, such
/// as one actor performing the same action on multiple targets, or multiple
/// actors performing the same action on a single target. The strategy defines
/// the rules for which activities are related.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsolidationStrategy {
    /// The individual activities are not consolidated.
    pub none: Option<NoConsolidation>,
    /// The individual activities are consolidated using the legacy strategy.
    pub legacy: Option<Legacy>,
}

impl Part for ConsolidationStrategy {}


/// A strategy which consolidates activities using the grouping rules from the
/// legacy V1 Activity API. Similar actions occurring within a window of time
/// can be grouped across multiple targets (such as moving a set of files at
/// once) or multiple actors (such as several users editing the same item).
/// Grouping rules for this strategy are specific to each type of action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Legacy { _never_set: Option<bool> }

impl Part for Legacy {}


/// Response message for querying Drive activity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query activity](struct.ActivityQueryCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryDriveActivityResponse {
    /// Token to retrieve the next page of results, or
    /// empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// List of activity requested.
    pub activities: Option<Vec<DriveActivity>>,
}

impl ResponseResult for QueryDriveActivityResponse {}


/// Information about an end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// A user about whom nothing is currently known.
    #[serde(rename="unknownUser")]
    pub unknown_user: Option<UnknownUser>,
    /// A user whose account has since been deleted.
    #[serde(rename="deletedUser")]
    pub deleted_user: Option<DeletedUser>,
    /// A known user.
    #[serde(rename="knownUser")]
    pub known_user: Option<KnownUser>,
}

impl Part for User {}


/// This item is deprecated; please see `DriveFile` instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File { _never_set: Option<bool> }

impl Part for File {}


/// Information about settings changes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SettingsChange {
    /// The set of changes made to restrictions.
    #[serde(rename="restrictionChanges")]
    pub restriction_changes: Option<Vec<RestrictionChange>>,
}

impl Part for SettingsChange {}


/// Information about the owner of a Drive item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    /// This field is deprecated; please use the `drive` field instead.
    #[serde(rename="teamDrive")]
    pub team_drive: Option<TeamDriveReference>,
    /// The domain of the Drive item owner.
    pub domain: Option<Domain>,
    /// The drive that owns the item.
    pub drive: Option<DriveReference>,
    /// The user that owns the Drive item.
    pub user: Option<User>,
}

impl Part for Owner {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the `DriveActivityHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_driveactivity2 as driveactivity2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use driveactivity2::DriveActivityHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveActivityHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `query(...)`
/// // to build up your call.
/// let rb = hub.activity();
/// # }
/// ```
pub struct ActivityMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveActivityHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for ActivityMethods<'a, C, A> {}

impl<'a, C, A> ActivityMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Query past activity in Google Drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn query(&self, request: QueryDriveActivityRequest) -> ActivityQueryCall<'a, C, A> {
        ActivityQueryCall {
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

/// Query past activity in Google Drive.
///
/// A builder for the *query* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_driveactivity2 as driveactivity2;
/// use driveactivity2::QueryDriveActivityRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use driveactivity2::DriveActivityHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveActivityHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryDriveActivityRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activity().query(req)
///              .doit();
/// # }
/// ```
pub struct ActivityQueryCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveActivityHub<C, A>,
    _request: QueryDriveActivityRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActivityQueryCall<'a, C, A> {}

impl<'a, C, A> ActivityQueryCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, QueryDriveActivityResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "driveactivity.activity.query",
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

        let mut url = self.hub._base_url.clone() + "v2/activity:query";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveActivity.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: QueryDriveActivityRequest) -> ActivityQueryCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityQueryCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ActivityQueryCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveActivity`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ActivityQueryCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


