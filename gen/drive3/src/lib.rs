// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *drive* crate version *1.0.11+20190620*, where *20190620* is the exact revision of the *drive:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.11*.
//! 
//! Everything else about the *drive* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/drive/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/drive3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.DriveHub.html) ... 
//! 
//! * [about](struct.About.html)
//!  * [*get*](struct.AboutGetCall.html)
//! * [changes](struct.Change.html)
//!  * [*get start page token*](struct.ChangeGetStartPageTokenCall.html), [*list*](struct.ChangeListCall.html) and [*watch*](struct.ChangeWatchCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * [comments](struct.Comment.html)
//!  * [*create*](struct.CommentCreateCall.html), [*delete*](struct.CommentDeleteCall.html), [*get*](struct.CommentGetCall.html), [*list*](struct.CommentListCall.html) and [*update*](struct.CommentUpdateCall.html)
//! * [drives](struct.Drive.html)
//!  * [*create*](struct.DriveCreateCall.html), [*delete*](struct.DriveDeleteCall.html), [*get*](struct.DriveGetCall.html), [*hide*](struct.DriveHideCall.html), [*list*](struct.DriveListCall.html), [*unhide*](struct.DriveUnhideCall.html) and [*update*](struct.DriveUpdateCall.html)
//! * [files](struct.File.html)
//!  * [*copy*](struct.FileCopyCall.html), [*create*](struct.FileCreateCall.html), [*delete*](struct.FileDeleteCall.html), [*empty trash*](struct.FileEmptyTrashCall.html), [*export*](struct.FileExportCall.html), [*generate ids*](struct.FileGenerateIdCall.html), [*get*](struct.FileGetCall.html), [*list*](struct.FileListCall.html), [*update*](struct.FileUpdateCall.html) and [*watch*](struct.FileWatchCall.html)
//! * [permissions](struct.Permission.html)
//!  * [*create*](struct.PermissionCreateCall.html), [*delete*](struct.PermissionDeleteCall.html), [*get*](struct.PermissionGetCall.html), [*list*](struct.PermissionListCall.html) and [*update*](struct.PermissionUpdateCall.html)
//! * [replies](struct.Reply.html)
//!  * [*create*](struct.ReplyCreateCall.html), [*delete*](struct.ReplyDeleteCall.html), [*get*](struct.ReplyGetCall.html), [*list*](struct.ReplyListCall.html) and [*update*](struct.ReplyUpdateCall.html)
//! * [revisions](struct.Revision.html)
//!  * [*delete*](struct.RevisionDeleteCall.html), [*get*](struct.RevisionGetCall.html), [*list*](struct.RevisionListCall.html) and [*update*](struct.RevisionUpdateCall.html)
//! * teamdrives
//!  * [*create*](struct.TeamdriveCreateCall.html), [*delete*](struct.TeamdriveDeleteCall.html), [*get*](struct.TeamdriveGetCall.html), [*list*](struct.TeamdriveListCall.html) and [*update*](struct.TeamdriveUpdateCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*update files*](struct.FileUpdateCall.html)
//! * [*create files*](struct.FileCreateCall.html)
//! 
//! Download supported by ...
//! 
//! * [*watch files*](struct.FileWatchCall.html)
//! * [*export files*](struct.FileExportCall.html)
//! * [*get revisions*](struct.RevisionGetCall.html)
//! * [*get files*](struct.FileGetCall.html)
//! 
//! Subscription supported by ...
//! 
//! * [*watch files*](struct.FileWatchCall.html)
//! * [*list changes*](struct.ChangeListCall.html)
//! * [*create files*](struct.FileCreateCall.html)
//! * [*get files*](struct.FileGetCall.html)
//! * [*watch changes*](struct.ChangeWatchCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.DriveHub.html)**
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
//! let r = hub.files().watch(...).doit()
//! let r = hub.files().empty_trash(...).doit()
//! let r = hub.files().generate_ids(...).doit()
//! let r = hub.files().copy(...).doit()
//! let r = hub.files().create(...).doit()
//! let r = hub.files().list(...).doit()
//! let r = hub.files().delete(...).doit()
//! let r = hub.files().update(...).doit()
//! let r = hub.files().get(...).doit()
//! let r = hub.files().export(...).doit()
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
//! google-drive3 = "*"
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
//! extern crate google_drive3 as drive3;
//! use drive3::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use drive3::DriveHub;
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
//! let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.files().list()
//!              .team_drive_id("sea")
//!              .supports_team_drives(false)
//!              .supports_all_drives(true)
//!              .spaces("gubergren")
//!              .q("sadipscing")
//!              .page_token("aliquyam")
//!              .page_size(-66)
//!              .order_by("no")
//!              .include_team_drive_items(true)
//!              .include_items_from_all_drives(true)
//!              .drive_id("et")
//!              .corpus("et")
//!              .corpora("diam")
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
    /// See, edit, create, and delete all of your Google Drive files
    Full,

    /// View the photos, videos and albums in your Google Photos
    PhotoReadonly,

    /// View and manage metadata of files in your Google Drive
    Metadata,

    /// View metadata for files in your Google Drive
    MetadataReadonly,

    /// View and manage Google Drive files and folders that you have opened or created with this app
    File,

    /// Modify your Google Apps Script scripts' behavior
    Script,

    /// See and download all your Google Drive files
    Readonly,

    /// View and manage its own configuration data in your Google Drive
    Appdata,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/drive",
            Scope::PhotoReadonly => "https://www.googleapis.com/auth/drive.photos.readonly",
            Scope::Metadata => "https://www.googleapis.com/auth/drive.metadata",
            Scope::MetadataReadonly => "https://www.googleapis.com/auth/drive.metadata.readonly",
            Scope::File => "https://www.googleapis.com/auth/drive.file",
            Scope::Script => "https://www.googleapis.com/auth/drive.scripts",
            Scope::Readonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::Appdata => "https://www.googleapis.com/auth/drive.appdata",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::PhotoReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all DriveHub related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// use drive3::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
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
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().list()
///              .team_drive_id("ipsum")
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .spaces("duo")
///              .q("aliquyam")
///              .page_token("sea")
///              .page_size(-55)
///              .order_by("eos")
///              .include_team_drive_items(false)
///              .include_items_from_all_drives(false)
///              .drive_id("dolor")
///              .corpus("eirmod")
///              .corpora("elitr")
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
pub struct DriveHub<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for DriveHub<C, A> {}

impl<'a, C, A> DriveHub<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> DriveHub<C, A> {
        DriveHub {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.11".to_string(),
            _base_url: "https://www.googleapis.com/drive/v3/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn about(&'a self) -> AboutMethods<'a, C, A> {
        AboutMethods { hub: &self }
    }
    pub fn changes(&'a self) -> ChangeMethods<'a, C, A> {
        ChangeMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, C, A> {
        ChannelMethods { hub: &self }
    }
    pub fn comments(&'a self) -> CommentMethods<'a, C, A> {
        CommentMethods { hub: &self }
    }
    pub fn drives(&'a self) -> DriveMethods<'a, C, A> {
        DriveMethods { hub: &self }
    }
    pub fn files(&'a self) -> FileMethods<'a, C, A> {
        FileMethods { hub: &self }
    }
    pub fn permissions(&'a self) -> PermissionMethods<'a, C, A> {
        PermissionMethods { hub: &self }
    }
    pub fn replies(&'a self) -> ReplyMethods<'a, C, A> {
        ReplyMethods { hub: &self }
    }
    pub fn revisions(&'a self) -> RevisionMethods<'a, C, A> {
        RevisionMethods { hub: &self }
    }
    pub fn teamdrives(&'a self) -> TeamdriveMethods<'a, C, A> {
        TeamdriveMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.11`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/drive/v3/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// A comment on a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update comments](struct.CommentUpdateCall.html) (request|response)
/// * [create comments](struct.CommentCreateCall.html) (request|response)
/// * [get comments](struct.CommentGetCall.html) (response)
/// * [list comments](struct.CommentListCall.html) (none)
/// * [delete comments](struct.CommentDeleteCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// Whether the comment has been resolved by one of its replies.
    pub resolved: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#comment".
    pub kind: Option<String>,
    /// The last time the comment or any of its replies was modified (RFC 3339 date-time).
    #[serde(rename="modifiedTime")]
    pub modified_time: Option<String>,
    /// The user who created the comment.
    pub author: Option<User>,
    /// Whether the comment has been deleted. A deleted comment has no content.
    pub deleted: Option<bool>,
    /// The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
    #[serde(rename="quotedFileContent")]
    pub quoted_file_content: Option<CommentQuotedFileContent>,
    /// The content of the comment with HTML formatting.
    #[serde(rename="htmlContent")]
    pub html_content: Option<String>,
    /// A region of the document represented as a JSON string. See anchor documentation for details on how to define and interpret anchor properties.
    pub anchor: Option<String>,
    /// The plain text content of the comment. This field is used for setting the content, while htmlContent should be displayed.
    pub content: Option<String>,
    /// The full list of replies to the comment in chronological order.
    pub replies: Option<Vec<Reply>>,
    /// The time at which the comment was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    pub created_time: Option<String>,
    /// The ID of the comment.
    pub id: Option<String>,
}

impl RequestValue for Comment {}
impl Resource for Comment {}
impl ResponseResult for Comment {}


/// Information about the user, the user's Drive, and system capabilities.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get about](struct.AboutGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct About {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#about".
    pub kind: Option<String>,
    /// A map of source MIME type to possible targets for all supported exports.
    #[serde(rename="exportFormats")]
    pub export_formats: Option<HashMap<String, Vec<String>>>,
    /// Deprecated - use driveThemes instead.
    #[serde(rename="teamDriveThemes")]
    pub team_drive_themes: Option<Vec<AboutTeamDriveThemes>>,
    /// The authenticated user.
    pub user: Option<User>,
    /// A map of maximum import sizes by MIME type, in bytes.
    #[serde(rename="maxImportSizes")]
    pub max_import_sizes: Option<HashMap<String, String>>,
    /// The user's storage quota limits and usage. All fields are measured in bytes.
    #[serde(rename="storageQuota")]
    pub storage_quota: Option<AboutStorageQuota>,
    /// The currently supported folder colors as RGB hex strings.
    #[serde(rename="folderColorPalette")]
    pub folder_color_palette: Option<Vec<String>>,
    /// A list of themes that are supported for shared drives.
    #[serde(rename="driveThemes")]
    pub drive_themes: Option<Vec<AboutDriveThemes>>,
    /// Whether the user can create shared drives.
    #[serde(rename="canCreateDrives")]
    pub can_create_drives: Option<bool>,
    /// Deprecated - use canCreateDrives instead.
    #[serde(rename="canCreateTeamDrives")]
    pub can_create_team_drives: Option<bool>,
    /// A map of source MIME type to possible targets for all supported imports.
    #[serde(rename="importFormats")]
    pub import_formats: Option<HashMap<String, Vec<String>>>,
    /// The maximum upload size in bytes.
    #[serde(rename="maxUploadSize")]
    pub max_upload_size: Option<String>,
    /// Whether the user has installed the requesting app.
    #[serde(rename="appInstalled")]
    pub app_installed: Option<bool>,
}

impl ResponseResult for About {}


/// Deprecated - use permissionDetails instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionTeamDrivePermissionDetails {
    /// Deprecated - use permissionDetails/inherited instead.
    pub inherited: Option<bool>,
    /// Deprecated - use permissionDetails/permissionType instead.
    #[serde(rename="teamDrivePermissionType")]
    pub team_drive_permission_type: Option<String>,
    /// Deprecated - use permissionDetails/role instead.
    pub role: Option<String>,
    /// Deprecated - use permissionDetails/inheritedFrom instead.
    #[serde(rename="inheritedFrom")]
    pub inherited_from: Option<String>,
}

impl NestedType for PermissionTeamDrivePermissionDetails {}
impl Part for PermissionTeamDrivePermissionDetails {}


/// A set of restrictions that apply to this shared drive or items inside this shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveRestrictions {
    /// Whether administrative privileges on this shared drive are required to modify restrictions.
    #[serde(rename="adminManagedRestrictions")]
    pub admin_managed_restrictions: Option<bool>,
    /// Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this shared drive.
    #[serde(rename="copyRequiresWriterPermission")]
    pub copy_requires_writer_permission: Option<bool>,
    /// Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive.
    #[serde(rename="domainUsersOnly")]
    pub domain_users_only: Option<bool>,
    /// Whether access to items inside this shared drive is restricted to its members.
    #[serde(rename="driveMembersOnly")]
    pub drive_members_only: Option<bool>,
}

impl NestedType for DriveRestrictions {}
impl Part for DriveRestrictions {}


/// Deprecated - use driveThemes instead.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutTeamDriveThemes {
    /// Deprecated - use driveThemes/colorRgb instead.
    #[serde(rename="colorRgb")]
    pub color_rgb: Option<String>,
    /// Deprecated - use driveThemes/backgroundImageLink instead.
    #[serde(rename="backgroundImageLink")]
    pub background_image_link: Option<String>,
    /// Deprecated - use driveThemes/id instead.
    pub id: Option<String>,
}

impl NestedType for AboutTeamDriveThemes {}
impl Part for AboutTeamDriveThemes {}


/// Capabilities the current user has on this Team Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveCapabilities {
    /// Whether the current user can read the revisions resource of files in this Team Drive.
    #[serde(rename="canReadRevisions")]
    pub can_read_revisions: Option<bool>,
    /// Whether the current user can copy files in this Team Drive.
    #[serde(rename="canCopy")]
    pub can_copy: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this Team Drive.
    #[serde(rename="canChangeCopyRequiresWriterPermissionRestriction")]
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /// Whether the current user can trash children from folders in this Team Drive.
    #[serde(rename="canTrashChildren")]
    pub can_trash_children: Option<bool>,
    /// Whether the current user can change the domainUsersOnly restriction of this Team Drive.
    #[serde(rename="canChangeDomainUsersOnlyRestriction")]
    pub can_change_domain_users_only_restriction: Option<bool>,
    /// Whether the current user can delete this Team Drive. Attempting to delete the Team Drive may still fail if there are untrashed items inside the Team Drive.
    #[serde(rename="canDeleteTeamDrive")]
    pub can_delete_team_drive: Option<bool>,
    /// Whether the current user can rename this Team Drive.
    #[serde(rename="canRenameTeamDrive")]
    pub can_rename_team_drive: Option<bool>,
    /// Whether the current user can comment on files in this Team Drive.
    #[serde(rename="canComment")]
    pub can_comment: Option<bool>,
    /// Whether the current user can list the children of folders in this Team Drive.
    #[serde(rename="canListChildren")]
    pub can_list_children: Option<bool>,
    /// Whether the current user can rename files or folders in this Team Drive.
    #[serde(rename="canRename")]
    pub can_rename: Option<bool>,
    /// Whether the current user can delete children from folders in this Team Drive.
    #[serde(rename="canDeleteChildren")]
    pub can_delete_children: Option<bool>,
    /// Whether the current user can add children to folders in this Team Drive.
    #[serde(rename="canAddChildren")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can share files or folders in this Team Drive.
    #[serde(rename="canShare")]
    pub can_share: Option<bool>,
    /// Whether the current user can add members to this Team Drive or remove them or change their role.
    #[serde(rename="canManageMembers")]
    pub can_manage_members: Option<bool>,
    /// Whether the current user can download files in this Team Drive.
    #[serde(rename="canDownload")]
    pub can_download: Option<bool>,
    /// Whether the current user can change the teamMembersOnly restriction of this Team Drive.
    #[serde(rename="canChangeTeamMembersOnlyRestriction")]
    pub can_change_team_members_only_restriction: Option<bool>,
    /// Whether the current user can change the background of this Team Drive.
    #[serde(rename="canChangeTeamDriveBackground")]
    pub can_change_team_drive_background: Option<bool>,
    /// Deprecated - use canDeleteChildren or canTrashChildren instead.
    #[serde(rename="canRemoveChildren")]
    pub can_remove_children: Option<bool>,
    /// Whether the current user can edit files in this Team Drive
    #[serde(rename="canEdit")]
    pub can_edit: Option<bool>,
}

impl NestedType for TeamDriveCapabilities {}
impl Part for TeamDriveCapabilities {}


/// An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveBackgroundImageFile {
    /// The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
    pub width: Option<f32>,
    /// The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.
    #[serde(rename="yCoordinate")]
    pub y_coordinate: Option<f32>,
    /// The ID of an image file in Drive to use for the background image.
    pub id: Option<String>,
    /// The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
    #[serde(rename="xCoordinate")]
    pub x_coordinate: Option<f32>,
}

impl NestedType for TeamDriveBackgroundImageFile {}
impl Part for TeamDriveBackgroundImageFile {}


/// Additional metadata about video media. This may not be available immediately upon upload.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileVideoMediaMetadata {
    /// The width of the video in pixels.
    pub width: Option<i32>,
    /// The duration of the video in milliseconds.
    #[serde(rename="durationMillis")]
    pub duration_millis: Option<String>,
    /// The height of the video in pixels.
    pub height: Option<i32>,
}

impl NestedType for FileVideoMediaMetadata {}
impl Part for FileVideoMediaMetadata {}


/// A list of changes for a user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list changes](struct.ChangeListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChangeList {
    /// The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#changeList".
    pub kind: Option<String>,
    /// The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    pub changes: Option<Vec<Change>>,
    /// The starting page token for future changes. This will be present only if the end of the current changes list has been reached.
    #[serde(rename="newStartPageToken")]
    pub new_start_page_token: Option<String>,
}

impl ResponseResult for ChangeList {}


/// A list of themes that are supported for shared drives.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutDriveThemes {
    /// The color of this theme as an RGB hex string.
    #[serde(rename="colorRgb")]
    pub color_rgb: Option<String>,
    /// A link to this theme's background image.
    #[serde(rename="backgroundImageLink")]
    pub background_image_link: Option<String>,
    /// The ID of the theme.
    pub id: Option<String>,
}

impl NestedType for AboutDriveThemes {}
impl Part for AboutDriveThemes {}


/// A list of replies to a comment on a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list replies](struct.ReplyListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplyList {
    /// The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#replyList".
    pub kind: Option<String>,
    /// The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    pub replies: Option<Vec<Reply>>,
}

impl ResponseResult for ReplyList {}


/// A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileContentHintsThumbnail {
    /// The MIME type of the thumbnail.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5).
    pub image: Option<String>,
}

impl NestedType for FileContentHintsThumbnail {}
impl Part for FileContentHintsThumbnail {}


/// Representation of a shared drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list drives](struct.DriveListCall.html) (none)
/// * [unhide drives](struct.DriveUnhideCall.html) (response)
/// * [hide drives](struct.DriveHideCall.html) (response)
/// * [create drives](struct.DriveCreateCall.html) (request|response)
/// * [delete drives](struct.DriveDeleteCall.html) (none)
/// * [update drives](struct.DriveUpdateCall.html) (request|response)
/// * [get drives](struct.DriveGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Drive {
    /// A set of restrictions that apply to this shared drive or items inside this shared drive.
    pub restrictions: Option<DriveRestrictions>,
    /// The color of this shared drive as an RGB hex string. It can only be set on a drive.drives.update request that does not set themeId.
    #[serde(rename="colorRgb")]
    pub color_rgb: Option<String>,
    /// A short-lived link to this shared drive's background image.
    #[serde(rename="backgroundImageLink")]
    pub background_image_link: Option<String>,
    /// The name of this shared drive.
    pub name: Option<String>,
    /// The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
    #[serde(rename="themeId")]
    pub theme_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#drive".
    pub kind: Option<String>,
    /// Capabilities the current user has on this shared drive.
    pub capabilities: Option<DriveCapabilities>,
    /// An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
    #[serde(rename="backgroundImageFile")]
    pub background_image_file: Option<DriveBackgroundImageFile>,
    /// The time at which the shared drive was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    pub created_time: Option<String>,
    /// Whether the shared drive is hidden from default view.
    pub hidden: Option<bool>,
    /// The ID of this shared drive which is also the ID of the top level folder of this shared drive.
    pub id: Option<String>,
}

impl RequestValue for Drive {}
impl Resource for Drive {}
impl ResponseResult for Drive {}


/// A list of shared drives.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list drives](struct.DriveListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveList {
    /// The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#driveList".
    pub kind: Option<String>,
    /// The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    pub drives: Option<Vec<Drive>>,
}

impl ResponseResult for DriveList {}


/// Additional metadata about image media, if available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileImageMediaMetadata {
    /// The exposure bias of the photo (APEX value).
    #[serde(rename="exposureBias")]
    pub exposure_bias: Option<f32>,
    /// The length of the exposure, in seconds.
    #[serde(rename="exposureTime")]
    pub exposure_time: Option<f32>,
    /// The smallest f-number of the lens at the focal length used to create the photo (APEX value).
    #[serde(rename="maxApertureValue")]
    pub max_aperture_value: Option<f32>,
    /// The color space of the photo.
    #[serde(rename="colorSpace")]
    pub color_space: Option<String>,
    /// The height of the image in pixels.
    pub height: Option<i32>,
    /// The lens used to create the photo.
    pub lens: Option<String>,
    /// The aperture used to create the photo (f-number).
    pub aperture: Option<f32>,
    /// The rotation in clockwise degrees from the image's original orientation.
    pub rotation: Option<i32>,
    /// The white balance mode used to create the photo.
    #[serde(rename="whiteBalance")]
    pub white_balance: Option<String>,
    /// The model of the camera used to create the photo.
    #[serde(rename="cameraModel")]
    pub camera_model: Option<String>,
    /// Whether a flash was used to create the photo.
    #[serde(rename="flashUsed")]
    pub flash_used: Option<bool>,
    /// The make of the camera used to create the photo.
    #[serde(rename="cameraMake")]
    pub camera_make: Option<String>,
    /// The focal length used to create the photo, in millimeters.
    #[serde(rename="focalLength")]
    pub focal_length: Option<f32>,
    /// The exposure mode used to create the photo.
    #[serde(rename="exposureMode")]
    pub exposure_mode: Option<String>,
    /// The distance to the subject of the photo, in meters.
    #[serde(rename="subjectDistance")]
    pub subject_distance: Option<i32>,
    /// The width of the image in pixels.
    pub width: Option<i32>,
    /// The metering mode used to create the photo.
    #[serde(rename="meteringMode")]
    pub metering_mode: Option<String>,
    /// Geographic location information stored in the image.
    pub location: Option<FileImageMediaMetadataLocation>,
    /// The date and time the photo was taken (EXIF DateTime).
    pub time: Option<String>,
    /// The ISO speed used to create the photo.
    #[serde(rename="isoSpeed")]
    pub iso_speed: Option<i32>,
    /// The type of sensor used to create the photo.
    pub sensor: Option<String>,
}

impl NestedType for FileImageMediaMetadata {}
impl Part for FileImageMediaMetadata {}


/// A list of revisions of a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list revisions](struct.RevisionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevisionList {
    /// The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#revisionList".
    pub kind: Option<String>,
    /// The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    pub revisions: Option<Vec<Revision>>,
}

impl ResponseResult for RevisionList {}


/// A reply to a comment on a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create replies](struct.ReplyCreateCall.html) (request|response)
/// * [get replies](struct.ReplyGetCall.html) (response)
/// * [update replies](struct.ReplyUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Reply {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#reply".
    pub kind: Option<String>,
    /// The last time the reply was modified (RFC 3339 date-time).
    #[serde(rename="modifiedTime")]
    pub modified_time: Option<String>,
    /// The user who created the reply.
    pub author: Option<User>,
    /// Whether the reply has been deleted. A deleted reply has no content.
    pub deleted: Option<bool>,
    /// The content of the reply with HTML formatting.
    #[serde(rename="htmlContent")]
    pub html_content: Option<String>,
    /// The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified.
    pub content: Option<String>,
    /// The action the reply performed to the parent comment. Valid values are:  
    /// - resolve 
    /// - reopen
    pub action: Option<String>,
    /// The time at which the reply was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    pub created_time: Option<String>,
    /// The ID of the reply.
    pub id: Option<String>,
}

impl RequestValue for Reply {}
impl ResponseResult for Reply {}


/// A list of generated file IDs which can be provided in create requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [generate ids files](struct.FileGenerateIdCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedIds {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#generatedIds".
    pub kind: Option<String>,
    /// The IDs generated for the requesting user in the specified space.
    pub ids: Option<Vec<String>>,
    /// The type of file that can be created with these IDs.
    pub space: Option<String>,
}

impl ResponseResult for GeneratedIds {}


/// The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentQuotedFileContent {
    /// The MIME type of the quoted content.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// The quoted content itself. This is interpreted as plain text if set through the API.
    pub value: Option<String>,
}

impl NestedType for CommentQuotedFileContent {}
impl Part for CommentQuotedFileContent {}


/// An notification channel used to watch for resource changes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [watch changes](struct.ChangeWatchCall.html) (request|response)
/// * [watch files](struct.FileWatchCall.html) (request|response)
/// * [stop channels](struct.ChannelStopCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// A version-specific identifier for the watched resource.
    #[serde(rename="resourceUri")]
    pub resource_uri: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".
    pub kind: Option<String>,
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
    #[serde(rename="resourceId")]
    pub resource_id: Option<String>,
    /// A UUID or similar unique string that identifies this channel.
    pub id: Option<String>,
    /// An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
    pub token: Option<String>,
    /// Additional parameters controlling delivery channel behavior. Optional.
    pub params: Option<HashMap<String, String>>,
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    pub expiration: Option<String>,
    /// The address where notifications are delivered for this channel.
    pub address: Option<String>,
    /// The type of delivery mechanism used for this channel.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    pub payload: Option<bool>,
}

impl RequestValue for Channel {}
impl Resource for Channel {}
impl ResponseResult for Channel {}


/// The metadata for a revision to a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete revisions](struct.RevisionDeleteCall.html) (none)
/// * [get revisions](struct.RevisionGetCall.html) (response)
/// * [list revisions](struct.RevisionListCall.html) (none)
/// * [update revisions](struct.RevisionUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Revision {
    /// The MIME type of the revision.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#revision".
    pub kind: Option<String>,
    /// The last time the revision was modified (RFC 3339 date-time).
    #[serde(rename="modifiedTime")]
    pub modified_time: Option<String>,
    /// The last user to modify this revision.
    #[serde(rename="lastModifyingUser")]
    pub last_modifying_user: Option<User>,
    /// Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file.
    /// This field is only applicable to files with binary content in Drive.
    #[serde(rename="keepForever")]
    pub keep_forever: Option<bool>,
    /// Whether subsequent revisions will be automatically republished. This is only applicable to Google Docs.
    #[serde(rename="publishAuto")]
    pub publish_auto: Option<bool>,
    /// Whether this revision is published outside the domain. This is only applicable to Google Docs.
    #[serde(rename="publishedOutsideDomain")]
    pub published_outside_domain: Option<bool>,
    /// The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive.
    #[serde(rename="md5Checksum")]
    pub md5_checksum: Option<String>,
    /// Links for exporting Google Docs to specific formats.
    #[serde(rename="exportLinks")]
    pub export_links: Option<HashMap<String, String>>,
    /// Whether this revision is published. This is only applicable to Google Docs.
    pub published: Option<bool>,
    /// The ID of the revision.
    pub id: Option<String>,
    /// The original filename used to create this revision. This is only applicable to files with binary content in Drive.
    #[serde(rename="originalFilename")]
    pub original_filename: Option<String>,
    /// The size of the revision's content in bytes. This is only applicable to files with binary content in Drive.
    pub size: Option<String>,
}

impl RequestValue for Revision {}
impl Resource for Revision {}
impl ResponseResult for Revision {}


/// A list of comments on a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list comments](struct.CommentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentList {
    /// The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#commentList".
    pub kind: Option<String>,
    /// The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    pub comments: Option<Vec<Comment>>,
}

impl ResponseResult for CommentList {}


/// Additional information about the content of the file. These fields are never populated in responses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileContentHints {
    /// Text to be indexed for the file to improve fullText queries. This is limited to 128KB in length and may contain HTML elements.
    #[serde(rename="indexableText")]
    pub indexable_text: Option<String>,
    /// A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
    pub thumbnail: Option<FileContentHintsThumbnail>,
}

impl NestedType for FileContentHints {}
impl Part for FileContentHints {}


/// Capabilities the current user has on this shared drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveCapabilities {
    /// Whether the current user can read the revisions resource of files in this shared drive.
    #[serde(rename="canReadRevisions")]
    pub can_read_revisions: Option<bool>,
    /// Whether the current user can copy files in this shared drive.
    #[serde(rename="canCopy")]
    pub can_copy: Option<bool>,
    /// Whether the current user can delete this shared drive. Attempting to delete the shared drive may still fail if there are untrashed items inside the shared drive.
    #[serde(rename="canDeleteDrive")]
    pub can_delete_drive: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this shared drive.
    #[serde(rename="canChangeCopyRequiresWriterPermissionRestriction")]
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /// Whether the current user can trash children from folders in this shared drive.
    #[serde(rename="canTrashChildren")]
    pub can_trash_children: Option<bool>,
    /// Whether the current user can change the driveMembersOnly restriction of this shared drive.
    #[serde(rename="canChangeDriveMembersOnlyRestriction")]
    pub can_change_drive_members_only_restriction: Option<bool>,
    /// Whether the current user can change the background of this shared drive.
    #[serde(rename="canChangeDriveBackground")]
    pub can_change_drive_background: Option<bool>,
    /// Whether the current user can comment on files in this shared drive.
    #[serde(rename="canComment")]
    pub can_comment: Option<bool>,
    /// Whether the current user can delete children from folders in this shared drive.
    #[serde(rename="canDeleteChildren")]
    pub can_delete_children: Option<bool>,
    /// Whether the current user can list the children of folders in this shared drive.
    #[serde(rename="canListChildren")]
    pub can_list_children: Option<bool>,
    /// Whether the current user can rename files or folders in this shared drive.
    #[serde(rename="canRename")]
    pub can_rename: Option<bool>,
    /// Whether the current user can rename this shared drive.
    #[serde(rename="canRenameDrive")]
    pub can_rename_drive: Option<bool>,
    /// Whether the current user can add children to folders in this shared drive.
    #[serde(rename="canAddChildren")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can share files or folders in this shared drive.
    #[serde(rename="canShare")]
    pub can_share: Option<bool>,
    /// Whether the current user can add members to this shared drive or remove them or change their role.
    #[serde(rename="canManageMembers")]
    pub can_manage_members: Option<bool>,
    /// Whether the current user can download files in this shared drive.
    #[serde(rename="canDownload")]
    pub can_download: Option<bool>,
    /// Whether the current user can change the domainUsersOnly restriction of this shared drive.
    #[serde(rename="canChangeDomainUsersOnlyRestriction")]
    pub can_change_domain_users_only_restriction: Option<bool>,
    /// Whether the current user can edit files in this shared drive
    #[serde(rename="canEdit")]
    pub can_edit: Option<bool>,
}

impl NestedType for DriveCapabilities {}
impl Part for DriveCapabilities {}


/// A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create permissions](struct.PermissionCreateCall.html) (request|response)
/// * [list permissions](struct.PermissionListCall.html) (none)
/// * [get permissions](struct.PermissionGetCall.html) (response)
/// * [delete permissions](struct.PermissionDeleteCall.html) (none)
/// * [update permissions](struct.PermissionUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// The domain to which this permission refers.
    pub domain: Option<String>,
    /// A displayable name for users, groups or domains.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone.
    #[serde(rename="allowFileDiscovery")]
    pub allow_file_discovery: Option<bool>,
    /// Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions.
    pub deleted: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#permission".
    pub kind: Option<String>,
    /// The email address of the user or group to which this permission refers.
    #[serde(rename="emailAddress")]
    pub email_address: Option<String>,
    /// A link to the user's profile photo, if available.
    #[serde(rename="photoLink")]
    pub photo_link: Option<String>,
    /// Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items.
    #[serde(rename="permissionDetails")]
    pub permission_details: Option<Vec<PermissionPermissionDetails>>,
    /// Deprecated - use permissionDetails instead.
    #[serde(rename="teamDrivePermissionDetails")]
    pub team_drive_permission_details: Option<Vec<PermissionTeamDrivePermissionDetails>>,
    /// The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:  
    /// - They can only be set on user and group permissions 
    /// - The time must be in the future 
    /// - The time cannot be more than a year in the future
    #[serde(rename="expirationTime")]
    pub expiration_time: Option<String>,
    /// The role granted by this permission. While new values may be supported in the future, the following are currently allowed:  
    /// - owner 
    /// - organizer 
    /// - fileOrganizer 
    /// - writer 
    /// - commenter 
    /// - reader
    pub role: Option<String>,
    /// The type of the grantee. Valid values are:  
    /// - user 
    /// - group 
    /// - domain 
    /// - anyone
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId.
    pub id: Option<String>,
}

impl RequestValue for Permission {}
impl Resource for Permission {}
impl ResponseResult for Permission {}


/// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileCapabilities {
    /// Whether the current user can move this item outside of this drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added.
    #[serde(rename="canMoveItemOutOfDrive")]
    pub can_move_item_out_of_drive: Option<bool>,
    /// Whether the current user can restore this file from trash.
    #[serde(rename="canUntrash")]
    pub can_untrash: Option<bool>,
    /// Whether the current user can copy this file. For an item in a shared drive, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder.
    #[serde(rename="canCopy")]
    pub can_copy: Option<bool>,
    /// Whether the current user can move this item within this shared drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added. Only populated for items in shared drives.
    #[serde(rename="canMoveItemWithinDrive")]
    pub can_move_item_within_drive: Option<bool>,
    /// Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read.
    #[serde(rename="canReadRevisions")]
    pub can_read_revisions: Option<bool>,
    /// Deprecated - use canMoveItemOutOfDrive instead.
    #[serde(rename="canMoveItemIntoTeamDrive")]
    pub can_move_item_into_team_drive: Option<bool>,
    /// Deprecated - use canMoveItemWithinDrive instead.
    #[serde(rename="canMoveItemWithinTeamDrive")]
    pub can_move_item_within_team_drive: Option<bool>,
    /// Deprecated - use canMoveItemOutOfDrive instead.
    #[serde(rename="canMoveItemOutOfTeamDrive")]
    pub can_move_item_out_of_team_drive: Option<bool>,
    /// Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canDeleteChildren")]
    pub can_delete_children: Option<bool>,
    /// Whether the current user can change the copyRequiresWriterPermission restriction of this file.
    #[serde(rename="canChangeCopyRequiresWriterPermission")]
    pub can_change_copy_requires_writer_permission: Option<bool>,
    /// Whether the current user can download this file.
    #[serde(rename="canDownload")]
    pub can_download: Option<bool>,
    /// Whether the current user can edit this file.
    #[serde(rename="canEdit")]
    pub can_edit: Option<bool>,
    /// Deprecated - use canMoveChildrenWithinDrive instead.
    #[serde(rename="canMoveChildrenWithinTeamDrive")]
    pub can_move_children_within_team_drive: Option<bool>,
    /// Whether the current user can comment on this file.
    #[serde(rename="canComment")]
    pub can_comment: Option<bool>,
    /// Whether the current user can list the children of this folder. This is always false when the item is not a folder.
    #[serde(rename="canListChildren")]
    pub can_list_children: Option<bool>,
    /// Whether the current user can rename this file.
    #[serde(rename="canRename")]
    pub can_rename: Option<bool>,
    /// Whether the current user can move this file to trash.
    #[serde(rename="canTrash")]
    pub can_trash: Option<bool>,
    /// Whether the current user can delete this file.
    #[serde(rename="canDelete")]
    pub can_delete: Option<bool>,
    /// Whether the current user can read the shared drive to which this file belongs. Only populated for items in shared drives.
    #[serde(rename="canReadDrive")]
    pub can_read_drive: Option<bool>,
    /// Deprecated - use canMoveItemWithinDrive or canMoveItemOutOfDrive instead.
    #[serde(rename="canMoveTeamDriveItem")]
    pub can_move_team_drive_item: Option<bool>,
    /// Whether the current user can add children to this folder. This is always false when the item is not a folder.
    #[serde(rename="canAddChildren")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can modify the sharing settings for this file.
    #[serde(rename="canShare")]
    pub can_share: Option<bool>,
    /// Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canTrashChildren")]
    pub can_trash_children: Option<bool>,
    /// Deprecated
    #[serde(rename="canChangeViewersCanCopyContent")]
    pub can_change_viewers_can_copy_content: Option<bool>,
    /// Whether the current user can move children of this folder outside of the shared drive. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canMoveChildrenOutOfDrive")]
    pub can_move_children_out_of_drive: Option<bool>,
    /// Deprecated - use canMoveChildrenOutOfDrive instead.
    #[serde(rename="canMoveChildrenOutOfTeamDrive")]
    pub can_move_children_out_of_team_drive: Option<bool>,
    /// Whether the current user can remove children from this folder. This is always false when the item is not a folder. For a folder in a shared drive, use canDeleteChildren or canTrashChildren instead.
    #[serde(rename="canRemoveChildren")]
    pub can_remove_children: Option<bool>,
    /// Deprecated - use canReadDrive instead.
    #[serde(rename="canReadTeamDrive")]
    pub can_read_team_drive: Option<bool>,
    /// Whether the current user can move children of this folder within the shared drive. This is false when the item is not a folder. Only populated for items in shared drives.
    #[serde(rename="canMoveChildrenWithinDrive")]
    pub can_move_children_within_drive: Option<bool>,
}

impl NestedType for FileCapabilities {}
impl Part for FileCapabilities {}


/// A set of restrictions that apply to this Team Drive or items inside this Team Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveRestrictions {
    /// Whether administrative privileges on this Team Drive are required to modify restrictions.
    #[serde(rename="adminManagedRestrictions")]
    pub admin_managed_restrictions: Option<bool>,
    /// Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive.
    #[serde(rename="copyRequiresWriterPermission")]
    pub copy_requires_writer_permission: Option<bool>,
    /// Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive.
    #[serde(rename="domainUsersOnly")]
    pub domain_users_only: Option<bool>,
    /// Whether access to items inside this Team Drive is restricted to members of this Team Drive.
    #[serde(rename="teamMembersOnly")]
    pub team_members_only: Option<bool>,
}

impl NestedType for TeamDriveRestrictions {}
impl Part for TeamDriveRestrictions {}


/// The user's storage quota limits and usage. All fields are measured in bytes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AboutStorageQuota {
    /// The total usage across all services.
    pub usage: Option<String>,
    /// The usage by all files in Google Drive.
    #[serde(rename="usageInDrive")]
    pub usage_in_drive: Option<String>,
    /// The usage by trashed files in Google Drive.
    #[serde(rename="usageInDriveTrash")]
    pub usage_in_drive_trash: Option<String>,
    /// The usage limit, if applicable. This will not be present if the user has unlimited storage.
    pub limit: Option<String>,
}

impl NestedType for AboutStorageQuota {}
impl Part for AboutStorageQuota {}


/// A list of files.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list files](struct.FileListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileList {
    /// The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    pub files: Option<Vec<File>>,
    /// Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the "allDrives" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as "user" or "drive".
    #[serde(rename="incompleteSearch")]
    pub incomplete_search: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#fileList".
    pub kind: Option<String>,
    /// The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl ResponseResult for FileList {}


/// An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DriveBackgroundImageFile {
    /// The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
    pub width: Option<f32>,
    /// The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.
    #[serde(rename="yCoordinate")]
    pub y_coordinate: Option<f32>,
    /// The ID of an image file in Google Drive to use for the background image.
    pub id: Option<String>,
    /// The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
    #[serde(rename="xCoordinate")]
    pub x_coordinate: Option<f32>,
}

impl NestedType for DriveBackgroundImageFile {}
impl Part for DriveBackgroundImageFile {}


/// Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionPermissionDetails {
    /// Whether this permission is inherited. This field is always populated. This is an output-only field.
    pub inherited: Option<bool>,
    /// The permission type for this user. While new values may be added in future, the following are currently possible:  
    /// - file 
    /// - member
    #[serde(rename="permissionType")]
    pub permission_type: Option<String>,
    /// The primary role for this user. While new values may be added in the future, the following are currently possible:  
    /// - organizer 
    /// - fileOrganizer 
    /// - writer 
    /// - commenter 
    /// - reader
    pub role: Option<String>,
    /// The ID of the item from which this permission is inherited. This is an output-only field and is only populated for members of the shared drive.
    #[serde(rename="inheritedFrom")]
    pub inherited_from: Option<String>,
}

impl NestedType for PermissionPermissionDetails {}
impl Part for PermissionPermissionDetails {}


/// Deprecated: use the drive collection instead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get teamdrives](struct.TeamdriveGetCall.html) (response)
/// * [create teamdrives](struct.TeamdriveCreateCall.html) (request|response)
/// * [update teamdrives](struct.TeamdriveUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDrive {
    /// A set of restrictions that apply to this Team Drive or items inside this Team Drive.
    pub restrictions: Option<TeamDriveRestrictions>,
    /// The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId.
    #[serde(rename="colorRgb")]
    pub color_rgb: Option<String>,
    /// A short-lived link to this Team Drive's background image.
    #[serde(rename="backgroundImageLink")]
    pub background_image_link: Option<String>,
    /// The name of this Team Drive.
    pub name: Option<String>,
    /// The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile.
    #[serde(rename="themeId")]
    pub theme_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#teamDrive".
    pub kind: Option<String>,
    /// Capabilities the current user has on this Team Drive.
    pub capabilities: Option<TeamDriveCapabilities>,
    /// An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
    #[serde(rename="backgroundImageFile")]
    pub background_image_file: Option<TeamDriveBackgroundImageFile>,
    /// The time at which the Team Drive was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    pub created_time: Option<String>,
    /// The ID of this Team Drive which is also the ID of the top level folder of this Team Drive.
    pub id: Option<String>,
}

impl RequestValue for TeamDrive {}
impl Resource for TeamDrive {}
impl ResponseResult for TeamDrive {}


/// Information about a Drive user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Whether this user is the requesting user.
    pub me: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#user".
    pub kind: Option<String>,
    /// A plain text displayable name for this user.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// A link to the user's profile photo, if available.
    #[serde(rename="photoLink")]
    pub photo_link: Option<String>,
    /// The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
    #[serde(rename="emailAddress")]
    pub email_address: Option<String>,
    /// The user's ID as visible in Permission resources.
    #[serde(rename="permissionId")]
    pub permission_id: Option<String>,
}

impl Part for User {}


/// The metadata for a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [watch files](struct.FileWatchCall.html) (none)
/// * [empty trash files](struct.FileEmptyTrashCall.html) (none)
/// * [generate ids files](struct.FileGenerateIdCall.html) (none)
/// * [copy files](struct.FileCopyCall.html) (request|response)
/// * [create files](struct.FileCreateCall.html) (request|response)
/// * [list files](struct.FileListCall.html) (none)
/// * [delete files](struct.FileDeleteCall.html) (none)
/// * [update files](struct.FileUpdateCall.html) (request|response)
/// * [get files](struct.FileGetCall.html) (response)
/// * [export files](struct.FileExportCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    /// Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field.
    #[serde(rename="hasThumbnail")]
    pub has_thumbnail: Option<bool>,
    /// The MIME type of the file.
    /// Google Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded.
    /// If a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// The last time the file was modified by the user (RFC 3339 date-time).
    #[serde(rename="modifiedByMeTime")]
    pub modified_by_me_time: Option<String>,
    /// A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content.
    #[serde(rename="thumbnailLink")]
    pub thumbnail_link: Option<String>,
    /// The thumbnail version for use in thumbnail cache invalidation.
    #[serde(rename="thumbnailVersion")]
    pub thumbnail_version: Option<String>,
    /// Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder.
    #[serde(rename="explicitlyTrashed")]
    pub explicitly_trashed: Option<bool>,
    /// Whether the file was created or opened by the requesting app.
    #[serde(rename="isAppAuthorized")]
    pub is_app_authorized: Option<bool>,
    /// Whether users with only writer permission can modify the file's permissions. Not populated for items in shared drives.
    #[serde(rename="writersCanShare")]
    pub writers_can_share: Option<bool>,
    /// Whether the user owns the file. Not populated for items in shared drives.
    #[serde(rename="ownedByMe")]
    pub owned_by_me: Option<bool>,
    /// The last time the file was viewed by the user (RFC 3339 date-time).
    #[serde(rename="viewedByMeTime")]
    pub viewed_by_me_time: Option<String>,
    /// The ID of the file.
    pub id: Option<String>,
    /// The user who shared the file with the requesting user, if applicable.
    #[serde(rename="sharingUser")]
    pub sharing_user: Option<User>,
    /// The size of the file's content in bytes. This is only applicable to files with binary content in Google Drive.
    pub size: Option<String>,
    /// Additional metadata about video media. This may not be available immediately upon upload.
    #[serde(rename="videoMediaMetadata")]
    pub video_media_metadata: Option<FileVideoMediaMetadata>,
    /// The last user to modify the file.
    #[serde(rename="lastModifyingUser")]
    pub last_modifying_user: Option<User>,
    /// The color for a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource.
    /// If an unsupported color is specified, the closest color in the palette will be used instead.
    #[serde(rename="folderColorRgb")]
    pub folder_color_rgb: Option<String>,
    /// A collection of arbitrary key-value pairs which are private to the requesting app.
    /// Entries with null values are cleared in update and copy requests.
    #[serde(rename="appProperties")]
    pub app_properties: Option<HashMap<String, String>>,
    /// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
    pub capabilities: Option<FileCapabilities>,
    /// A collection of arbitrary key-value pairs which are visible to all apps.
    /// Entries with null values are cleared in update and copy requests.
    pub properties: Option<HashMap<String, String>>,
    /// A link for opening the file in a relevant Google editor or viewer in a browser.
    #[serde(rename="webViewLink")]
    pub web_view_link: Option<String>,
    /// A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user.
    pub version: Option<String>,
    /// The IDs of the parent folders which contain the file.
    /// If not specified as part of a create request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list.
    pub parents: Option<Vec<String>>,
    /// The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive.
    #[serde(rename="md5Checksum")]
    pub md5_checksum: Option<String>,
    /// Links for exporting Google Docs to specific formats.
    #[serde(rename="exportLinks")]
    pub export_links: Option<HashMap<String, String>>,
    /// Whether the file has been shared. Not populated for items in shared drives.
    pub shared: Option<bool>,
    /// Whether the options to copy, print, or download this file, should be disabled for readers and commenters.
    #[serde(rename="copyRequiresWriterPermission")]
    pub copy_requires_writer_permission: Option<bool>,
    /// The full file extension extracted from the name field. May contain multiple concatenated extensions, such as "tar.gz". This is only available for files with binary content in Google Drive.
    /// This is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension.
    #[serde(rename="fullFileExtension")]
    pub full_file_extension: Option<String>,
    /// The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Google Drive.
    #[serde(rename="originalFilename")]
    pub original_filename: Option<String>,
    /// Additional metadata about image media, if available.
    #[serde(rename="imageMediaMetadata")]
    pub image_media_metadata: Option<FileImageMediaMetadata>,
    /// A short description of the file.
    pub description: Option<String>,
    /// The last time the file was modified by anyone (RFC 3339 date-time).
    /// Note that setting modifiedTime will also update modifiedByMeTime for the user.
    #[serde(rename="modifiedTime")]
    pub modified_time: Option<String>,
    /// Whether the file has been viewed by this user.
    #[serde(rename="viewedByMe")]
    pub viewed_by_me: Option<bool>,
    /// Whether the file has been modified by this user.
    #[serde(rename="modifiedByMe")]
    pub modified_by_me: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#file".
    pub kind: Option<String>,
    /// The time at which the file was created (RFC 3339 date-time).
    #[serde(rename="createdTime")]
    pub created_time: Option<String>,
    /// The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled.
    #[serde(rename="quotaBytesUsed")]
    pub quota_bytes_used: Option<String>,
    /// Deprecated - use driveId instead.
    #[serde(rename="teamDriveId")]
    pub team_drive_id: Option<String>,
    /// The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.
    #[serde(rename="trashedTime")]
    pub trashed_time: Option<String>,
    /// The time at which the file was shared with the user, if applicable (RFC 3339 date-time).
    #[serde(rename="sharedWithMeTime")]
    pub shared_with_me_time: Option<String>,
    /// A static, unauthenticated link to the file's icon.
    #[serde(rename="iconLink")]
    pub icon_link: Option<String>,
    /// Deprecated - use copyRequiresWriterPermission instead.
    #[serde(rename="viewersCanCopyContent")]
    pub viewers_can_copy_content: Option<bool>,
    /// The owners of the file. Currently, only certain legacy files may have more than one owner. Not populated for items in shared drives.
    pub owners: Option<Vec<User>>,
    /// The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the name is constant.
    pub name: Option<String>,
    /// A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive.
    #[serde(rename="webContentLink")]
    pub web_content_link: Option<String>,
    /// If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives.
    #[serde(rename="trashingUser")]
    pub trashing_user: Option<User>,
    /// ID of the shared drive the file resides in. Only populated for items in shared drives.
    #[serde(rename="driveId")]
    pub drive_id: Option<String>,
    /// The list of spaces which contain the file. The currently supported values are 'drive', 'appDataFolder' and 'photos'.
    pub spaces: Option<Vec<String>>,
    /// List of permission IDs for users with access to this file.
    #[serde(rename="permissionIds")]
    pub permission_ids: Option<Vec<String>>,
    /// Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, and other users cannot see files in the owner's trash.
    pub trashed: Option<bool>,
    /// Additional information about the content of the file. These fields are never populated in responses.
    #[serde(rename="contentHints")]
    pub content_hints: Option<FileContentHints>,
    /// The final component of fullFileExtension. This is only available for files with binary content in Google Drive.
    #[serde(rename="fileExtension")]
    pub file_extension: Option<String>,
    /// Whether any users are granted file access directly on this file. This field is only populated for shared drive files.
    #[serde(rename="hasAugmentedPermissions")]
    pub has_augmented_permissions: Option<bool>,
    /// Whether the user has starred the file.
    pub starred: Option<bool>,
    /// The ID of the file's head revision. This is currently only available for files with binary content in Google Drive.
    #[serde(rename="headRevisionId")]
    pub head_revision_id: Option<String>,
    /// The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives.
    pub permissions: Option<Vec<Permission>>,
}

impl RequestValue for File {}
impl Resource for File {}
impl ResponseResult for File {}


/// Geographic location information stored in the image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileImageMediaMetadataLocation {
    /// The latitude stored in the image.
    pub latitude: Option<f64>,
    /// The altitude stored in the image.
    pub altitude: Option<f64>,
    /// The longitude stored in the image.
    pub longitude: Option<f64>,
}

impl NestedType for FileImageMediaMetadataLocation {}
impl Part for FileImageMediaMetadataLocation {}


/// A list of permissions for a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list permissions](struct.PermissionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionList {
    /// The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#permissionList".
    pub kind: Option<String>,
    /// The list of permissions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    pub permissions: Option<Vec<Permission>>,
}

impl ResponseResult for PermissionList {}


/// A list of Team Drives.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list teamdrives](struct.TeamdriveListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TeamDriveList {
    /// The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#teamDriveList".
    pub kind: Option<String>,
    /// The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename="teamDrives")]
    pub team_drives: Option<Vec<TeamDrive>>,
}

impl ResponseResult for TeamDriveList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get start page token changes](struct.ChangeGetStartPageTokenCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartPageToken {
    /// Identifies what kind of resource this is. Value: the fixed string "drive#startPageToken".
    pub kind: Option<String>,
    /// The starting page token for listing changes.
    #[serde(rename="startPageToken")]
    pub start_page_token: Option<String>,
}

impl ResponseResult for StartPageToken {}


/// A change to a file or shared drive.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list changes](struct.ChangeListCall.html) (none)
/// * [get start page token changes](struct.ChangeGetStartPageTokenCall.html) (none)
/// * [watch changes](struct.ChangeWatchCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Change {
    /// Deprecated - use drive instead.
    #[serde(rename="teamDrive")]
    pub team_drive: Option<TeamDrive>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#change".
    pub kind: Option<String>,
    /// The type of the change. Possible values are file and drive.
    #[serde(rename="changeType")]
    pub change_type: Option<String>,
    /// The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted.
    pub drive: Option<Drive>,
    /// The ID of the shared drive associated with this change.
    #[serde(rename="driveId")]
    pub drive_id: Option<String>,
    /// The updated state of the file. Present if the type is file and the file has not been removed from this list of changes.
    pub file: Option<File>,
    /// The time of this change (RFC 3339 date-time).
    pub time: Option<String>,
    /// Deprecated - use driveId instead.
    #[serde(rename="teamDriveId")]
    pub team_drive_id: Option<String>,
    /// Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access.
    pub removed: Option<bool>,
    /// Deprecated - use changeType instead.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The ID of the file which has changed.
    #[serde(rename="fileId")]
    pub file_id: Option<String>,
}

impl Resource for Change {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *file* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `copy(...)`, `create(...)`, `delete(...)`, `empty_trash(...)`, `export(...)`, `generate_ids(...)`, `get(...)`, `list(...)`, `update(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.files();
/// # }
/// ```
pub struct FileMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for FileMethods<'a, C, A> {}

impl<'a, C, A> FileMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Subscribes to changes to a file
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn watch(&self, request: Channel, file_id: &str) -> FileWatchCall<'a, C, A> {
        FileWatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports a Google Doc to the requested MIME type and returns the exported content. Please note that the exported content is limited to 10MB.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `mimeType` - The MIME type of the format requested for this export.
    pub fn export(&self, file_id: &str, mime_type: &str) -> FileExportCall<'a, C, A> {
        FileExportCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _mime_type: mime_type.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn delete(&self, file_id: &str) -> FileDeleteCall<'a, C, A> {
        FileDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a copy of a file and applies any requested updates with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn copy(&self, request: File, file_id: &str) -> FileCopyCall<'a, C, A> {
        FileCopyCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _ocr_language: Default::default(),
            _keep_revision_forever: Default::default(),
            _ignore_default_visibility: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes all of the user's trashed files.
    pub fn empty_trash(&self) -> FileEmptyTrashCall<'a, C, A> {
        FileEmptyTrashCall {
            hub: self.hub,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists or searches files.
    pub fn list(&self) -> FileListCall<'a, C, A> {
        FileListCall {
            hub: self.hub,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _spaces: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _drive_id: Default::default(),
            _corpus: Default::default(),
            _corpora: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a set of file IDs which can be provided in create or copy requests.
    pub fn generate_ids(&self) -> FileGenerateIdCall<'a, C, A> {
        FileGenerateIdCall {
            hub: self.hub,
            _space: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: File) -> FileCreateCall<'a, C, A> {
        FileCreateCall {
            hub: self.hub,
            _request: request,
            _use_content_as_indexable_text: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _ocr_language: Default::default(),
            _keep_revision_forever: Default::default(),
            _ignore_default_visibility: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a file's metadata and/or content with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn update(&self, request: File, file_id: &str) -> FileUpdateCall<'a, C, A> {
        FileUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _use_content_as_indexable_text: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _remove_parents: Default::default(),
            _ocr_language: Default::default(),
            _keep_revision_forever: Default::default(),
            _add_parents: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a file's metadata or content by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn get(&self, file_id: &str) -> FileGetCall<'a, C, A> {
        FileGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *teamdrive* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.teamdrives();
/// # }
/// ```
pub struct TeamdriveMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for TeamdriveMethods<'a, C, A> {}

impl<'a, C, A> TeamdriveMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.get instead.
    /// 
    /// # Arguments
    ///
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn get(&self, team_drive_id: &str) -> TeamdriveGetCall<'a, C, A> {
        TeamdriveGetCall {
            hub: self.hub,
            _team_drive_id: team_drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.create instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `requestId` - An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
    pub fn create(&self, request: TeamDrive, request_id: &str) -> TeamdriveCreateCall<'a, C, A> {
        TeamdriveCreateCall {
            hub: self.hub,
            _request: request,
            _request_id: request_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.list instead.
    pub fn list(&self) -> TeamdriveListCall<'a, C, A> {
        TeamdriveListCall {
            hub: self.hub,
            _use_domain_admin_access: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.delete instead.
    /// 
    /// # Arguments
    ///
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn delete(&self, team_drive_id: &str) -> TeamdriveDeleteCall<'a, C, A> {
        TeamdriveDeleteCall {
            hub: self.hub,
            _team_drive_id: team_drive_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.update instead
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn update(&self, request: TeamDrive, team_drive_id: &str) -> TeamdriveUpdateCall<'a, C, A> {
        TeamdriveUpdateCall {
            hub: self.hub,
            _request: request,
            _team_drive_id: team_drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *drive* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `hide(...)`, `list(...)`, `unhide(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.drives();
/// # }
/// ```
pub struct DriveMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for DriveMethods<'a, C, A> {}

impl<'a, C, A> DriveMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `requestId` - An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned.
    pub fn create(&self, request: Drive, request_id: &str) -> DriveCreateCall<'a, C, A> {
        DriveCreateCall {
            hub: self.hub,
            _request: request,
            _request_id: request_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the metadate for a shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `driveId` - The ID of the shared drive.
    pub fn update(&self, request: Drive, drive_id: &str) -> DriveUpdateCall<'a, C, A> {
        DriveUpdateCall {
            hub: self.hub,
            _request: request,
            _drive_id: drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores a shared drive to the default view.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn unhide(&self, drive_id: &str) -> DriveUnhideCall<'a, C, A> {
        DriveUnhideCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn delete(&self, drive_id: &str) -> DriveDeleteCall<'a, C, A> {
        DriveDeleteCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the user's shared drives.
    pub fn list(&self) -> DriveListCall<'a, C, A> {
        DriveListCall {
            hub: self.hub,
            _use_domain_admin_access: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a shared drive's metadata by ID.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn get(&self, drive_id: &str) -> DriveGetCall<'a, C, A> {
        DriveGetCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Hides a shared drive from the default view.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn hide(&self, drive_id: &str) -> DriveHideCall<'a, C, A> {
        DriveHideCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for CommentMethods<'a, C, A> {}

impl<'a, C, A> CommentMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a comment.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn delete(&self, file_id: &str, comment_id: &str) -> CommentDeleteCall<'a, C, A> {
        CommentDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a comment by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn get(&self, file_id: &str, comment_id: &str) -> CommentGetCall<'a, C, A> {
        CommentGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a comment with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn update(&self, request: Comment, file_id: &str, comment_id: &str) -> CommentUpdateCall<'a, C, A> {
        CommentUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new comment on a file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn create(&self, request: Comment, file_id: &str) -> CommentCreateCall<'a, C, A> {
        CommentCreateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's comments.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> CommentListCall<'a, C, A> {
        CommentListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _start_modified_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for ChannelMethods<'a, C, A> {}

impl<'a, C, A> ChannelMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop watching resources through this channel
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn stop(&self, request: Channel) -> ChannelStopCall<'a, C, A> {
        ChannelStopCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *reply* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.replies();
/// # }
/// ```
pub struct ReplyMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for ReplyMethods<'a, C, A> {}

impl<'a, C, A> ReplyMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new reply to a comment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn create(&self, request: Reply, file_id: &str, comment_id: &str) -> ReplyCreateCall<'a, C, A> {
        ReplyCreateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a comment's replies.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn list(&self, file_id: &str, comment_id: &str) -> ReplyListCall<'a, C, A> {
        ReplyListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a reply by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn get(&self, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyGetCall<'a, C, A> {
        ReplyGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a reply.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn delete(&self, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyDeleteCall<'a, C, A> {
        ReplyDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a reply with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn update(&self, request: Reply, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyUpdateCall<'a, C, A> {
        ReplyUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *about* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.about();
/// # }
/// ```
pub struct AboutMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for AboutMethods<'a, C, A> {}

impl<'a, C, A> AboutMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about the user, the user's Drive, and system capabilities.
    pub fn get(&self) -> AboutGetCall<'a, C, A> {
        AboutGetCall {
            hub: self.hub,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *permission* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.permissions();
/// # }
/// ```
pub struct PermissionMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for PermissionMethods<'a, C, A> {}

impl<'a, C, A> PermissionMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a permission.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file or shared drive.
    /// * `permissionId` - The ID of the permission.
    pub fn delete(&self, file_id: &str, permission_id: &str) -> PermissionDeleteCall<'a, C, A> {
        PermissionDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a permission with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file or shared drive.
    /// * `permissionId` - The ID of the permission.
    pub fn update(&self, request: Permission, file_id: &str, permission_id: &str) -> PermissionUpdateCall<'a, C, A> {
        PermissionUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _transfer_ownership: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _remove_expiration: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's or shared drive's permissions.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file or shared drive.
    pub fn list(&self, file_id: &str) -> PermissionListCall<'a, C, A> {
        PermissionListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a permission by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `permissionId` - The ID of the permission.
    pub fn get(&self, file_id: &str, permission_id: &str) -> PermissionGetCall<'a, C, A> {
        PermissionGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a permission for a file or shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file or shared drive.
    pub fn create(&self, request: Permission, file_id: &str) -> PermissionCreateCall<'a, C, A> {
        PermissionCreateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _transfer_ownership: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _send_notification_email: Default::default(),
            _email_message: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *change* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_start_page_token(...)`, `list(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.changes();
/// # }
/// ```
pub struct ChangeMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for ChangeMethods<'a, C, A> {}

impl<'a, C, A> ChangeMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the changes for a user or shared drive.
    /// 
    /// # Arguments
    ///
    /// * `pageToken` - The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    pub fn list(&self, page_token: &str) -> ChangeListCall<'a, C, A> {
        ChangeListCall {
            hub: self.hub,
            _page_token: page_token.to_string(),
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _spaces: Default::default(),
            _restrict_to_my_drive: Default::default(),
            _page_size: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_removed: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _include_corpus_removals: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the starting pageToken for listing future changes.
    pub fn get_start_page_token(&self) -> ChangeGetStartPageTokenCall<'a, C, A> {
        ChangeGetStartPageTokenCall {
            hub: self.hub,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Subscribes to changes for a user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `pageToken` - The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    pub fn watch(&self, request: Channel, page_token: &str) -> ChangeWatchCall<'a, C, A> {
        ChangeWatchCall {
            hub: self.hub,
            _request: request,
            _page_token: page_token.to_string(),
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _spaces: Default::default(),
            _restrict_to_my_drive: Default::default(),
            _page_size: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_removed: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _include_corpus_removals: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *revision* resources.
/// It is not used directly, but through the `DriveHub` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_drive3 as drive3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use drive3::DriveHub;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.revisions();
/// # }
/// ```
pub struct RevisionMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
}

impl<'a, C, A> MethodsBuilder for RevisionMethods<'a, C, A> {}

impl<'a, C, A> RevisionMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a revision with patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn update(&self, request: Revision, file_id: &str, revision_id: &str) -> RevisionUpdateCall<'a, C, A> {
        RevisionUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a revision's metadata or content by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn get(&self, file_id: &str, revision_id: &str) -> RevisionGetCall<'a, C, A> {
        RevisionGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn delete(&self, file_id: &str, revision_id: &str) -> RevisionDeleteCall<'a, C, A> {
        RevisionDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's revisions.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> RevisionListCall<'a, C, A> {
        RevisionListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
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

/// Subscribes to changes to a file
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Channel` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *watch* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().watch(req, "fileId")
///              .supports_team_drives(false)
///              .supports_all_drives(true)
///              .acknowledge_abuse(true)
///              .doit();
/// # }
/// ```
pub struct FileWatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Channel,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _acknowledge_abuse: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileWatchCall<'a, C, A> {}

impl<'a, C, A> FileWatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.watch",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._acknowledge_abuse {
            params.push(("acknowledgeAbuse", value.to_string()));
        }
        for &field in ["fileId", "supportsTeamDrives", "supportsAllDrives", "acknowledgeAbuse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let (json_field_missing, enable_resource_parsing) = {
            let mut enable = true;
            let mut field_present = true;
            for &(name, ref value) in params.iter() {
                if name == "alt" {
                    field_present = false;
                    if <String as AsRef<str>>::as_ref(&value) != "json" {
                        enable = false;
                    }
                    break;
                }
            }
            (field_present, enable)
        };
        if json_field_missing {
            params.push(("alt", "json".to_string()));
        }

        let mut url = self.hub._base_url.clone() + "files/{fileId}/watch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    } else { (res, Default::default()) };

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
    pub fn request(mut self, new_value: Channel) -> FileWatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileWatchCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileWatchCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileWatchCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    ///
    /// Sets the *acknowledge abuse* query property to the given value.
    pub fn acknowledge_abuse(mut self, new_value: bool) -> FileWatchCall<'a, C, A> {
        self._acknowledge_abuse = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileWatchCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileWatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileWatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Exports a Google Doc to the requested MIME type and returns the exported content. Please note that the exported content is limited to 10MB.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
///
/// A builder for the *export* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().export("fileId", "mimeType")
///              .doit();
/// # }
/// ```
pub struct FileExportCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _mime_type: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileExportCall<'a, C, A> {}

impl<'a, C, A> FileExportCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.export",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("mimeType", self._mime_type.to_string()));
        for &field in ["fileId", "mimeType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/export";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileExportCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The MIME type of the format requested for this export.
    ///
    /// Sets the *mime type* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn mime_type(mut self, new_value: &str) -> FileExportCall<'a, C, A> {
        self._mime_type = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileExportCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileExportCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileExportCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.
///
/// A builder for the *delete* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().delete("fileId")
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .doit();
/// # }
/// ```
pub struct FileDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileDeleteCall<'a, C, A> {}

impl<'a, C, A> FileDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        for &field in ["fileId", "supportsTeamDrives", "supportsAllDrives"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileDeleteCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileDeleteCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileDeleteCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a copy of a file and applies any requested updates with patch semantics.
///
/// A builder for the *copy* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::File;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = File::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().copy(req, "fileId")
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .ocr_language("et")
///              .keep_revision_forever(true)
///              .ignore_default_visibility(false)
///              .doit();
/// # }
/// ```
pub struct FileCopyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: File,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _ocr_language: Option<String>,
    _keep_revision_forever: Option<bool>,
    _ignore_default_visibility: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileCopyCall<'a, C, A> {}

impl<'a, C, A> FileCopyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, File)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.copy",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._ocr_language {
            params.push(("ocrLanguage", value.to_string()));
        }
        if let Some(value) = self._keep_revision_forever {
            params.push(("keepRevisionForever", value.to_string()));
        }
        if let Some(value) = self._ignore_default_visibility {
            params.push(("ignoreDefaultVisibility", value.to_string()));
        }
        for &field in ["alt", "fileId", "supportsTeamDrives", "supportsAllDrives", "ocrLanguage", "keepRevisionForever", "ignoreDefaultVisibility"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/copy";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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
    pub fn request(mut self, new_value: File) -> FileCopyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileCopyCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileCopyCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileCopyCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    ///
    /// Sets the *ocr language* query property to the given value.
    pub fn ocr_language(mut self, new_value: &str) -> FileCopyCall<'a, C, A> {
        self._ocr_language = Some(new_value.to_string());
        self
    }
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive.
    ///
    /// Sets the *keep revision forever* query property to the given value.
    pub fn keep_revision_forever(mut self, new_value: bool) -> FileCopyCall<'a, C, A> {
        self._keep_revision_forever = Some(new_value);
        self
    }
    /// Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
    ///
    /// Sets the *ignore default visibility* query property to the given value.
    pub fn ignore_default_visibility(mut self, new_value: bool) -> FileCopyCall<'a, C, A> {
        self._ignore_default_visibility = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileCopyCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileCopyCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileCopyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes all of the user's trashed files.
///
/// A builder for the *emptyTrash* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().empty_trash()
///              .doit();
/// # }
/// ```
pub struct FileEmptyTrashCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileEmptyTrashCall<'a, C, A> {}

impl<'a, C, A> FileEmptyTrashCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.emptyTrash",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(1 + self._additional_params.len());
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/trash";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileEmptyTrashCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileEmptyTrashCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileEmptyTrashCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists or searches files.
///
/// A builder for the *list* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().list()
///              .team_drive_id("et")
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .spaces("consetetur")
///              .q("ut")
///              .page_token("ea")
///              .page_size(-80)
///              .order_by("dolor")
///              .include_team_drive_items(true)
///              .include_items_from_all_drives(true)
///              .drive_id("et")
///              .corpus("consetetur")
///              .corpora("amet.")
///              .doit();
/// # }
/// ```
pub struct FileListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _spaces: Option<String>,
    _q: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _include_team_drive_items: Option<bool>,
    _include_items_from_all_drives: Option<bool>,
    _drive_id: Option<String>,
    _corpus: Option<String>,
    _corpora: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileListCall<'a, C, A> {}

impl<'a, C, A> FileListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, FileList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._spaces {
            params.push(("spaces", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._include_team_drive_items {
            params.push(("includeTeamDriveItems", value.to_string()));
        }
        if let Some(value) = self._include_items_from_all_drives {
            params.push(("includeItemsFromAllDrives", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        if let Some(value) = self._corpus {
            params.push(("corpus", value.to_string()));
        }
        if let Some(value) = self._corpora {
            params.push(("corpora", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "spaces", "q", "pageToken", "pageSize", "orderBy", "includeTeamDriveItems", "includeItemsFromAllDrives", "driveId", "corpus", "corpora"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
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


    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileListCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileListCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of spaces to query within the corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    ///
    /// Sets the *spaces* query property to the given value.
    pub fn spaces(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._spaces = Some(new_value.to_string());
        self
    }
    /// A query for filtering the file results. See the "Search for Files" guide for supported syntax.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> FileListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// A comma-separated list of sort keys. Valid keys are 'createdTime', 'folder', 'modifiedByMeTime', 'modifiedTime', 'name', 'name_natural', 'quotaBytesUsed', 'recency', 'sharedWithMeTime', 'starred', and 'viewedByMeTime'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Deprecated use includeItemsFromAllDrives instead.
    ///
    /// Sets the *include team drive items* query property to the given value.
    pub fn include_team_drive_items(mut self, new_value: bool) -> FileListCall<'a, C, A> {
        self._include_team_drive_items = Some(new_value);
        self
    }
    /// Deprecated - Whether both My Drive and shared drive items should be included in results. This parameter will only be effective until June 1, 2020. Afterwards shared drive items will be included in the results.
    ///
    /// Sets the *include items from all drives* query property to the given value.
    pub fn include_items_from_all_drives(mut self, new_value: bool) -> FileListCall<'a, C, A> {
        self._include_items_from_all_drives = Some(new_value);
        self
    }
    /// ID of the shared drive to search.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The source of files to list. Deprecated: use 'corpora' instead.
    ///
    /// Sets the *corpus* query property to the given value.
    pub fn corpus(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._corpus = Some(new_value.to_string());
        self
    }
    /// Bodies of items (files/documents) to which the query applies. Supported bodies are 'user', 'domain', 'drive' and 'allDrives'. Prefer 'user' or 'drive' to 'allDrives' for efficiency.
    ///
    /// Sets the *corpora* query property to the given value.
    pub fn corpora(mut self, new_value: &str) -> FileListCall<'a, C, A> {
        self._corpora = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generates a set of file IDs which can be provided in create or copy requests.
///
/// A builder for the *generateIds* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().generate_ids()
///              .space("voluptua.")
///              .count(-56)
///              .doit();
/// # }
/// ```
pub struct FileGenerateIdCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _space: Option<String>,
    _count: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileGenerateIdCall<'a, C, A> {}

impl<'a, C, A> FileGenerateIdCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GeneratedIds)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.generateIds",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._space {
            params.push(("space", value.to_string()));
        }
        if let Some(value) = self._count {
            params.push(("count", value.to_string()));
        }
        for &field in ["alt", "space", "count"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/generateIds";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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


    /// The space in which the IDs can be used to create new files. Supported values are 'drive' and 'appDataFolder'.
    ///
    /// Sets the *space* query property to the given value.
    pub fn space(mut self, new_value: &str) -> FileGenerateIdCall<'a, C, A> {
        self._space = Some(new_value.to_string());
        self
    }
    /// The number of IDs to return.
    ///
    /// Sets the *count* query property to the given value.
    pub fn count(mut self, new_value: i32) -> FileGenerateIdCall<'a, C, A> {
        self._count = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileGenerateIdCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileGenerateIdCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileGenerateIdCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new file.
///
/// A builder for the *create* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::File;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = File::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().create(req)
///              .use_content_as_indexable_text(true)
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .ocr_language("vero")
///              .keep_revision_forever(true)
///              .ignore_default_visibility(false)
///              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct FileCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: File,
    _use_content_as_indexable_text: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _ocr_language: Option<String>,
    _keep_revision_forever: Option<bool>,
    _ignore_default_visibility: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileCreateCall<'a, C, A> {}

impl<'a, C, A> FileCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::client::Response, File)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._use_content_as_indexable_text {
            params.push(("useContentAsIndexableText", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._ocr_language {
            params.push(("ocrLanguage", value.to_string()));
        }
        if let Some(value) = self._keep_revision_forever {
            params.push(("keepRevisionForever", value.to_string()));
        }
        if let Some(value) = self._ignore_default_visibility {
            params.push(("ignoreDefaultVisibility", value.to_string()));
        }
        for &field in ["alt", "useContentAsIndexableText", "supportsTeamDrives", "supportsAllDrives", "ocrLanguage", "keepRevisionForever", "ignoreDefaultVisibility"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let (mut url, upload_type) =
            if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/drive/v3/files", "multipart")
            } else if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/drive/v3/files", "resumable")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

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
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let url = upload_url.as_ref().and_then(|s| Some(hyper::Url::parse(s).unwrap())).unwrap();
                    hyper::client::Response::new(url, Box::new(cmn::DummyNetworkStream)).and_then(|mut res| {
                        res.status = hyper::status::StatusCode::Ok;
                        res.headers.set(Location(upload_url.as_ref().unwrap().clone()));
                        Ok(res)
                    })
                } else {
                    let mut mp_reader: MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, ContentType(mime_type))
                        },
                        _ => (&mut request_value_reader as &mut io::Read, ContentType(json_mime_type.clone())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                        .header(UserAgent(self.hub._user_agent.clone()))
                        .header(auth_header.clone())
                        .header(content_type)
                        .body(&mut body_reader);
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req = req.header(cmn::XUploadContentType(reader_mime_type.clone()));
                    }
    
                    dlg.pre_request();
                    req.send()
                }
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
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url_str = &res.headers.get::<Location>().expect("Location header is part of protocol").0;
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status.is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(Error::Failure(res))
                                }
                            }
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

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 5120GB
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, File)>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 5120GB
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, File)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: File) -> FileCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Whether to use the uploaded content as indexable text.
    ///
    /// Sets the *use content as indexable text* query property to the given value.
    pub fn use_content_as_indexable_text(mut self, new_value: bool) -> FileCreateCall<'a, C, A> {
        self._use_content_as_indexable_text = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileCreateCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileCreateCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    ///
    /// Sets the *ocr language* query property to the given value.
    pub fn ocr_language(mut self, new_value: &str) -> FileCreateCall<'a, C, A> {
        self._ocr_language = Some(new_value.to_string());
        self
    }
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive.
    ///
    /// Sets the *keep revision forever* query property to the given value.
    pub fn keep_revision_forever(mut self, new_value: bool) -> FileCreateCall<'a, C, A> {
        self._keep_revision_forever = Some(new_value);
        self
    }
    /// Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
    ///
    /// Sets the *ignore default visibility* query property to the given value.
    pub fn ignore_default_visibility(mut self, new_value: bool) -> FileCreateCall<'a, C, A> {
        self._ignore_default_visibility = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileCreateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a file's metadata and/or content with patch semantics.
///
/// A builder for the *update* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::File;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = File::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().update(req, "fileId")
///              .use_content_as_indexable_text(true)
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .remove_parents("invidunt")
///              .ocr_language("consetetur")
///              .keep_revision_forever(false)
///              .add_parents("duo")
///              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct FileUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: File,
    _file_id: String,
    _use_content_as_indexable_text: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _remove_parents: Option<String>,
    _ocr_language: Option<String>,
    _keep_revision_forever: Option<bool>,
    _add_parents: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileUpdateCall<'a, C, A> {}

impl<'a, C, A> FileUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::client::Response, File)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.update",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._use_content_as_indexable_text {
            params.push(("useContentAsIndexableText", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._remove_parents {
            params.push(("removeParents", value.to_string()));
        }
        if let Some(value) = self._ocr_language {
            params.push(("ocrLanguage", value.to_string()));
        }
        if let Some(value) = self._keep_revision_forever {
            params.push(("keepRevisionForever", value.to_string()));
        }
        if let Some(value) = self._add_parents {
            params.push(("addParents", value.to_string()));
        }
        for &field in ["alt", "fileId", "useContentAsIndexableText", "supportsTeamDrives", "supportsAllDrives", "removeParents", "ocrLanguage", "keepRevisionForever", "addParents"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let (mut url, upload_type) =
            if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/drive/v3/files/{fileId}", "multipart")
            } else if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/drive/v3/files/{fileId}", "resumable")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

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
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let url = upload_url.as_ref().and_then(|s| Some(hyper::Url::parse(s).unwrap())).unwrap();
                    hyper::client::Response::new(url, Box::new(cmn::DummyNetworkStream)).and_then(|mut res| {
                        res.status = hyper::status::StatusCode::Ok;
                        res.headers.set(Location(upload_url.as_ref().unwrap().clone()));
                        Ok(res)
                    })
                } else {
                    let mut mp_reader: MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, ContentType(mime_type))
                        },
                        _ => (&mut request_value_reader as &mut io::Read, ContentType(json_mime_type.clone())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                        .header(UserAgent(self.hub._user_agent.clone()))
                        .header(auth_header.clone())
                        .header(content_type)
                        .body(&mut body_reader);
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req = req.header(cmn::XUploadContentType(reader_mime_type.clone()));
                    }
    
                    dlg.pre_request();
                    req.send()
                }
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
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 5497558138880 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 5497558138880))
                        }
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url_str = &res.headers.get::<Location>().expect("Location header is part of protocol").0;
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status.is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(Error::Failure(res))
                                }
                            }
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

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 5120GB
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, File)>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 5120GB
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, File)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: File) -> FileUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileUpdateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// Whether to use the uploaded content as indexable text.
    ///
    /// Sets the *use content as indexable text* query property to the given value.
    pub fn use_content_as_indexable_text(mut self, new_value: bool) -> FileUpdateCall<'a, C, A> {
        self._use_content_as_indexable_text = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileUpdateCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileUpdateCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of parent IDs to remove.
    ///
    /// Sets the *remove parents* query property to the given value.
    pub fn remove_parents(mut self, new_value: &str) -> FileUpdateCall<'a, C, A> {
        self._remove_parents = Some(new_value.to_string());
        self
    }
    /// A language hint for OCR processing during image import (ISO 639-1 code).
    ///
    /// Sets the *ocr language* query property to the given value.
    pub fn ocr_language(mut self, new_value: &str) -> FileUpdateCall<'a, C, A> {
        self._ocr_language = Some(new_value.to_string());
        self
    }
    /// Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive.
    ///
    /// Sets the *keep revision forever* query property to the given value.
    pub fn keep_revision_forever(mut self, new_value: bool) -> FileUpdateCall<'a, C, A> {
        self._keep_revision_forever = Some(new_value);
        self
    }
    /// A comma-separated list of parent IDs to add.
    ///
    /// Sets the *add parents* query property to the given value.
    pub fn add_parents(mut self, new_value: &str) -> FileUpdateCall<'a, C, A> {
        self._add_parents = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a file's metadata or content by ID.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `File` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *get* method supported by a *file* resource.
/// It is not used directly, but through a `FileMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().get("fileId")
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .acknowledge_abuse(true)
///              .doit();
/// # }
/// ```
pub struct FileGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _acknowledge_abuse: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for FileGetCall<'a, C, A> {}

impl<'a, C, A> FileGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, File)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.files.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._acknowledge_abuse {
            params.push(("acknowledgeAbuse", value.to_string()));
        }
        for &field in ["fileId", "supportsTeamDrives", "supportsAllDrives", "acknowledgeAbuse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let (json_field_missing, enable_resource_parsing) = {
            let mut enable = true;
            let mut field_present = true;
            for &(name, ref value) in params.iter() {
                if name == "alt" {
                    field_present = false;
                    if <String as AsRef<str>>::as_ref(&value) != "json" {
                        enable = false;
                    }
                    break;
                }
            }
            (field_present, enable)
        };
        if json_field_missing {
            params.push(("alt", "json".to_string()));
        }

        let mut url = self.hub._base_url.clone() + "files/{fileId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> FileGetCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> FileGetCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> FileGetCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    ///
    /// Sets the *acknowledge abuse* query property to the given value.
    pub fn acknowledge_abuse(mut self, new_value: bool) -> FileGetCall<'a, C, A> {
        self._acknowledge_abuse = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> FileGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> FileGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> FileGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.get instead.
///
/// A builder for the *get* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().get("teamDriveId")
///              .use_domain_admin_access(false)
///              .doit();
/// # }
/// ```
pub struct TeamdriveGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _team_drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TeamdriveGetCall<'a, C, A> {}

impl<'a, C, A> TeamdriveGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TeamDrive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.teamdrives.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("teamDriveId", self._team_drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives/{teamDriveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamDriveId}", "teamDriveId")].iter() {
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
            for param_name in ["teamDriveId"].iter() {
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


    /// The ID of the Team Drive
    ///
    /// Sets the *team drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_drive_id(mut self, new_value: &str) -> TeamdriveGetCall<'a, C, A> {
        self._team_drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> TeamdriveGetCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TeamdriveGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TeamdriveGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.create instead.
///
/// A builder for the *create* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::TeamDrive;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TeamDrive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().create(req, "requestId")
///              .doit();
/// # }
/// ```
pub struct TeamdriveCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: TeamDrive,
    _request_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TeamdriveCreateCall<'a, C, A> {}

impl<'a, C, A> TeamdriveCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TeamDrive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.teamdrives.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("requestId", self._request_id.to_string()));
        for &field in ["alt", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: TeamDrive) -> TeamdriveCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
    ///
    /// Sets the *request id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request_id(mut self, new_value: &str) -> TeamdriveCreateCall<'a, C, A> {
        self._request_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TeamdriveCreateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TeamdriveCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.list instead.
///
/// A builder for the *list* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().list()
///              .use_domain_admin_access(true)
///              .q("sanctus")
///              .page_token("takimata")
///              .page_size(-27)
///              .doit();
/// # }
/// ```
pub struct TeamdriveListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _use_domain_admin_access: Option<bool>,
    _q: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TeamdriveListCall<'a, C, A> {}

impl<'a, C, A> TeamdriveListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TeamDriveList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.teamdrives.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "useDomainAdminAccess", "q", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
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


    /// Issue the request as a domain administrator; if set to true, then all Team Drives of the domain in which the requester is an administrator are returned.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> TeamdriveListCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Query string for searching Team Drives.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> TeamdriveListCall<'a, C, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Page token for Team Drives.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TeamdriveListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of Team Drives to return.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> TeamdriveListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TeamdriveListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TeamdriveListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.delete instead.
///
/// A builder for the *delete* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().delete("teamDriveId")
///              .doit();
/// # }
/// ```
pub struct TeamdriveDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _team_drive_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TeamdriveDeleteCall<'a, C, A> {}

impl<'a, C, A> TeamdriveDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.teamdrives.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        params.push(("teamDriveId", self._team_drive_id.to_string()));
        for &field in ["teamDriveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "teamdrives/{teamDriveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamDriveId}", "teamDriveId")].iter() {
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
            for param_name in ["teamDriveId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the Team Drive
    ///
    /// Sets the *team drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_drive_id(mut self, new_value: &str) -> TeamdriveDeleteCall<'a, C, A> {
        self._team_drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TeamdriveDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TeamdriveDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deprecated use drives.update instead
///
/// A builder for the *update* method supported by a *teamdrive* resource.
/// It is not used directly, but through a `TeamdriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::TeamDrive;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TeamDrive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.teamdrives().update(req, "teamDriveId")
///              .use_domain_admin_access(false)
///              .doit();
/// # }
/// ```
pub struct TeamdriveUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: TeamDrive,
    _team_drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for TeamdriveUpdateCall<'a, C, A> {}

impl<'a, C, A> TeamdriveUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TeamDrive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.teamdrives.update",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("teamDriveId", self._team_drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "teamdrives/{teamDriveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{teamDriveId}", "teamDriveId")].iter() {
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
            for param_name in ["teamDriveId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: TeamDrive) -> TeamdriveUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the Team Drive
    ///
    /// Sets the *team drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn team_drive_id(mut self, new_value: &str) -> TeamdriveUpdateCall<'a, C, A> {
        self._team_drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> TeamdriveUpdateCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TeamdriveUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TeamdriveUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> TeamdriveUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new shared drive.
///
/// A builder for the *create* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Drive;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Drive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().create(req, "requestId")
///              .doit();
/// # }
/// ```
pub struct DriveCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Drive,
    _request_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DriveCreateCall<'a, C, A> {}

impl<'a, C, A> DriveCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.drives.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("requestId", self._request_id.to_string()));
        for &field in ["alt", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Drive) -> DriveCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned.
    ///
    /// Sets the *request id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request_id(mut self, new_value: &str) -> DriveCreateCall<'a, C, A> {
        self._request_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DriveCreateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DriveCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DriveCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the metadate for a shared drive.
///
/// A builder for the *update* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Drive;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Drive::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().update(req, "driveId")
///              .use_domain_admin_access(true)
///              .doit();
/// # }
/// ```
pub struct DriveUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Drive,
    _drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DriveUpdateCall<'a, C, A> {}

impl<'a, C, A> DriveUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.drives.update",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "driveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
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
            for param_name in ["driveId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: Drive) -> DriveUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveUpdateCall<'a, C, A> {
        self._drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> DriveUpdateCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DriveUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DriveUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DriveUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Restores a shared drive to the default view.
///
/// A builder for the *unhide* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().unhide("driveId")
///              .doit();
/// # }
/// ```
pub struct DriveUnhideCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _drive_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DriveUnhideCall<'a, C, A> {}

impl<'a, C, A> DriveUnhideCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.drives.unhide",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        for &field in ["alt", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}/unhide";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
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
            for param_name in ["driveId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
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


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveUnhideCall<'a, C, A> {
        self._drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DriveUnhideCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DriveUnhideCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DriveUnhideCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.
///
/// A builder for the *delete* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().delete("driveId")
///              .doit();
/// # }
/// ```
pub struct DriveDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _drive_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DriveDeleteCall<'a, C, A> {}

impl<'a, C, A> DriveDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.drives.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        for &field in ["driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "drives/{driveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
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
            for param_name in ["driveId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveDeleteCall<'a, C, A> {
        self._drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DriveDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DriveDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DriveDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the user's shared drives.
///
/// A builder for the *list* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().list()
///              .use_domain_admin_access(false)
///              .q("sit")
///              .page_token("eirmod")
///              .page_size(-46)
///              .doit();
/// # }
/// ```
pub struct DriveListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _use_domain_admin_access: Option<bool>,
    _q: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DriveListCall<'a, C, A> {}

impl<'a, C, A> DriveListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DriveList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.drives.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "useDomainAdminAccess", "q", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
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


    /// Issue the request as a domain administrator; if set to true, then all shared drives of the domain in which the requester is an administrator are returned.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> DriveListCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Query string for searching shared drives.
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> DriveListCall<'a, C, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Page token for shared drives.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DriveListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of shared drives to return.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> DriveListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DriveListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DriveListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DriveListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a shared drive's metadata by ID.
///
/// A builder for the *get* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().get("driveId")
///              .use_domain_admin_access(true)
///              .doit();
/// # }
/// ```
pub struct DriveGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _drive_id: String,
    _use_domain_admin_access: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DriveGetCall<'a, C, A> {}

impl<'a, C, A> DriveGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.drives.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        for &field in ["alt", "driveId", "useDomainAdminAccess"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
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
            for param_name in ["driveId"].iter() {
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


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveGetCall<'a, C, A> {
        self._drive_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> DriveGetCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DriveGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DriveGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DriveGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Hides a shared drive from the default view.
///
/// A builder for the *hide* method supported by a *drive* resource.
/// It is not used directly, but through a `DriveMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.drives().hide("driveId")
///              .doit();
/// # }
/// ```
pub struct DriveHideCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _drive_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DriveHideCall<'a, C, A> {}

impl<'a, C, A> DriveHideCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Drive)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.drives.hide",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("driveId", self._drive_id.to_string()));
        for &field in ["alt", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "drives/{driveId}/hide";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{driveId}", "driveId")].iter() {
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
            for param_name in ["driveId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
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


    /// The ID of the shared drive.
    ///
    /// Sets the *drive id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn drive_id(mut self, new_value: &str) -> DriveHideCall<'a, C, A> {
        self._drive_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DriveHideCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> DriveHideCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DriveHideCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a comment.
///
/// A builder for the *delete* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().delete("fileId", "commentId")
///              .doit();
/// # }
/// ```
pub struct CommentDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _comment_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CommentDeleteCall<'a, C, A> {}

impl<'a, C, A> CommentDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.comments.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        for &field in ["fileId", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
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
            for param_name in ["commentId", "fileId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentDeleteCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentDeleteCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CommentDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CommentDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CommentDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a comment by ID.
///
/// A builder for the *get* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().get("fileId", "commentId")
///              .include_deleted(false)
///              .doit();
/// # }
/// ```
pub struct CommentGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _comment_id: String,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CommentGetCall<'a, C, A> {}

impl<'a, C, A> CommentGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Comment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.comments.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "commentId", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
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
            for param_name in ["commentId", "fileId"].iter() {
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


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentGetCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentGetCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// Whether to return deleted comments. Deleted comments will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> CommentGetCall<'a, C, A> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CommentGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CommentGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CommentGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a comment with patch semantics.
///
/// A builder for the *update* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Comment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Comment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().update(req, "fileId", "commentId")
///              .doit();
/// # }
/// ```
pub struct CommentUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Comment,
    _file_id: String,
    _comment_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CommentUpdateCall<'a, C, A> {}

impl<'a, C, A> CommentUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Comment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.comments.update",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        for &field in ["alt", "fileId", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
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
            for param_name in ["commentId", "fileId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: Comment) -> CommentUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentUpdateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentUpdateCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CommentUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CommentUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CommentUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new comment on a file.
///
/// A builder for the *create* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Comment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Comment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().create(req, "fileId")
///              .doit();
/// # }
/// ```
pub struct CommentCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Comment,
    _file_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CommentCreateCall<'a, C, A> {}

impl<'a, C, A> CommentCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Comment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.comments.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        for &field in ["alt", "fileId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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
    pub fn request(mut self, new_value: Comment) -> CommentCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentCreateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CommentCreateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CommentCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CommentCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a file's comments.
///
/// A builder for the *list* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().list("fileId")
///              .start_modified_time("sit")
///              .page_token("diam")
///              .page_size(-37)
///              .include_deleted(false)
///              .doit();
/// # }
/// ```
pub struct CommentListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _start_modified_time: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CommentListCall<'a, C, A> {}

impl<'a, C, A> CommentListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CommentList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.comments.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._start_modified_time {
            params.push(("startModifiedTime", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "startModifiedTime", "pageToken", "pageSize", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> CommentListCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time).
    ///
    /// Sets the *start modified time* query property to the given value.
    pub fn start_modified_time(mut self, new_value: &str) -> CommentListCall<'a, C, A> {
        self._start_modified_time = Some(new_value.to_string());
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CommentListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of comments to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CommentListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Whether to include deleted comments. Deleted comments will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> CommentListCall<'a, C, A> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CommentListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CommentListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CommentListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Stop watching resources through this channel
///
/// A builder for the *stop* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().stop(req)
///              .doit();
/// # }
/// ```
pub struct ChannelStopCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Channel,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ChannelStopCall<'a, C, A> {}

impl<'a, C, A> ChannelStopCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.channels.stop",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "channels/stop";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
                    let result_value = res;

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
    pub fn request(mut self, new_value: Channel) -> ChannelStopCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelStopCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelStopCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ChannelStopCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new reply to a comment.
///
/// A builder for the *create* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Reply;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Reply::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().create(req, "fileId", "commentId")
///              .doit();
/// # }
/// ```
pub struct ReplyCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Reply,
    _file_id: String,
    _comment_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ReplyCreateCall<'a, C, A> {}

impl<'a, C, A> ReplyCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Reply)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.replies.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        for &field in ["alt", "fileId", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
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
            for param_name in ["commentId", "fileId"].iter() {
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
    pub fn request(mut self, new_value: Reply) -> ReplyCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyCreateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyCreateCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ReplyCreateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ReplyCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a comment's replies.
///
/// A builder for the *list* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().list("fileId", "commentId")
///              .page_token("diam")
///              .page_size(-71)
///              .include_deleted(false)
///              .doit();
/// # }
/// ```
pub struct ReplyListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _comment_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ReplyListCall<'a, C, A> {}

impl<'a, C, A> ReplyListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ReplyList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.replies.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "commentId", "pageToken", "pageSize", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId")].iter() {
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
            for param_name in ["commentId", "fileId"].iter() {
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


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyListCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyListCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ReplyListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of replies to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ReplyListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Whether to include deleted replies. Deleted replies will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> ReplyListCall<'a, C, A> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ReplyListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ReplyListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a reply by ID.
///
/// A builder for the *get* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().get("fileId", "commentId", "replyId")
///              .include_deleted(true)
///              .doit();
/// # }
/// ```
pub struct ReplyGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _comment_id: String,
    _reply_id: String,
    _include_deleted: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ReplyGetCall<'a, C, A> {}

impl<'a, C, A> ReplyGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Reply)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.replies.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        params.push(("replyId", self._reply_id.to_string()));
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        for &field in ["alt", "fileId", "commentId", "replyId", "includeDeleted"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies/{replyId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId"), ("{replyId}", "replyId")].iter() {
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
            for param_name in ["replyId", "commentId", "fileId"].iter() {
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


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyGetCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyGetCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The ID of the reply.
    ///
    /// Sets the *reply id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn reply_id(mut self, new_value: &str) -> ReplyGetCall<'a, C, A> {
        self._reply_id = new_value.to_string();
        self
    }
    /// Whether to return deleted replies. Deleted replies will not include their original content.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> ReplyGetCall<'a, C, A> {
        self._include_deleted = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ReplyGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Readonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ReplyGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a reply.
///
/// A builder for the *delete* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().delete("fileId", "commentId", "replyId")
///              .doit();
/// # }
/// ```
pub struct ReplyDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _comment_id: String,
    _reply_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ReplyDeleteCall<'a, C, A> {}

impl<'a, C, A> ReplyDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.replies.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        params.push(("replyId", self._reply_id.to_string()));
        for &field in ["fileId", "commentId", "replyId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies/{replyId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId"), ("{replyId}", "replyId")].iter() {
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
            for param_name in ["replyId", "commentId", "fileId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyDeleteCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyDeleteCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The ID of the reply.
    ///
    /// Sets the *reply id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn reply_id(mut self, new_value: &str) -> ReplyDeleteCall<'a, C, A> {
        self._reply_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ReplyDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ReplyDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a reply with patch semantics.
///
/// A builder for the *update* method supported by a *reply* resource.
/// It is not used directly, but through a `ReplyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Reply;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Reply::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.replies().update(req, "fileId", "commentId", "replyId")
///              .doit();
/// # }
/// ```
pub struct ReplyUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Reply,
    _file_id: String,
    _comment_id: String,
    _reply_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ReplyUpdateCall<'a, C, A> {}

impl<'a, C, A> ReplyUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Reply)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.replies.update",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("commentId", self._comment_id.to_string()));
        params.push(("replyId", self._reply_id.to_string()));
        for &field in ["alt", "fileId", "commentId", "replyId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/comments/{commentId}/replies/{replyId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{commentId}", "commentId"), ("{replyId}", "replyId")].iter() {
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
            for param_name in ["replyId", "commentId", "fileId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: Reply) -> ReplyUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> ReplyUpdateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the comment.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> ReplyUpdateCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The ID of the reply.
    ///
    /// Sets the *reply id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn reply_id(mut self, new_value: &str) -> ReplyUpdateCall<'a, C, A> {
        self._reply_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ReplyUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ReplyUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ReplyUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets information about the user, the user's Drive, and system capabilities.
///
/// A builder for the *get* method supported by a *about* resource.
/// It is not used directly, but through a `AboutMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.about().get()
///              .doit();
/// # }
/// ```
pub struct AboutGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AboutGetCall<'a, C, A> {}

impl<'a, C, A> AboutGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, About)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.about.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "about";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
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


    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AboutGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AboutGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AboutGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a permission.
///
/// A builder for the *delete* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().delete("fileId", "permissionId")
///              .use_domain_admin_access(true)
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .doit();
/// # }
/// ```
pub struct PermissionDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _permission_id: String,
    _use_domain_admin_access: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PermissionDeleteCall<'a, C, A> {}

impl<'a, C, A> PermissionDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.permissions.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        for &field in ["fileId", "permissionId", "useDomainAdminAccess", "supportsTeamDrives", "supportsAllDrives"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions/{permissionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{permissionId}", "permissionId")].iter() {
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
            for param_name in ["permissionId", "fileId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionDeleteCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the permission.
    ///
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn permission_id(mut self, new_value: &str) -> PermissionDeleteCall<'a, C, A> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionDeleteCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionDeleteCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionDeleteCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PermissionDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PermissionDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a permission with patch semantics.
///
/// A builder for the *update* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Permission;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Permission::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().update(req, "fileId", "permissionId")
///              .use_domain_admin_access(true)
///              .transfer_ownership(false)
///              .supports_team_drives(true)
///              .supports_all_drives(false)
///              .remove_expiration(false)
///              .doit();
/// # }
/// ```
pub struct PermissionUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Permission,
    _file_id: String,
    _permission_id: String,
    _use_domain_admin_access: Option<bool>,
    _transfer_ownership: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _remove_expiration: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PermissionUpdateCall<'a, C, A> {}

impl<'a, C, A> PermissionUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Permission)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.permissions.update",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._transfer_ownership {
            params.push(("transferOwnership", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._remove_expiration {
            params.push(("removeExpiration", value.to_string()));
        }
        for &field in ["alt", "fileId", "permissionId", "useDomainAdminAccess", "transferOwnership", "supportsTeamDrives", "supportsAllDrives", "removeExpiration"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions/{permissionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{permissionId}", "permissionId")].iter() {
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
            for param_name in ["permissionId", "fileId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: Permission) -> PermissionUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionUpdateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the permission.
    ///
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn permission_id(mut self, new_value: &str) -> PermissionUpdateCall<'a, C, A> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionUpdateCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect.
    ///
    /// Sets the *transfer ownership* query property to the given value.
    pub fn transfer_ownership(mut self, new_value: bool) -> PermissionUpdateCall<'a, C, A> {
        self._transfer_ownership = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionUpdateCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionUpdateCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Whether to remove the expiration date.
    ///
    /// Sets the *remove expiration* query property to the given value.
    pub fn remove_expiration(mut self, new_value: bool) -> PermissionUpdateCall<'a, C, A> {
        self._remove_expiration = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PermissionUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PermissionUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a file's or shared drive's permissions.
///
/// A builder for the *list* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().list("fileId")
///              .use_domain_admin_access(true)
///              .supports_team_drives(false)
///              .supports_all_drives(true)
///              .page_token("takimata")
///              .page_size(-43)
///              .doit();
/// # }
/// ```
pub struct PermissionListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _use_domain_admin_access: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PermissionListCall<'a, C, A> {}

impl<'a, C, A> PermissionListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PermissionList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.permissions.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "fileId", "useDomainAdminAccess", "supportsTeamDrives", "supportsAllDrives", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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


    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionListCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionListCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionListCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionListCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PermissionListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> PermissionListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PermissionListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PermissionListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a permission by ID.
///
/// A builder for the *get* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().get("fileId", "permissionId")
///              .use_domain_admin_access(true)
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .doit();
/// # }
/// ```
pub struct PermissionGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _permission_id: String,
    _use_domain_admin_access: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PermissionGetCall<'a, C, A> {}

impl<'a, C, A> PermissionGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Permission)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.permissions.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        for &field in ["alt", "fileId", "permissionId", "useDomainAdminAccess", "supportsTeamDrives", "supportsAllDrives"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions/{permissionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{permissionId}", "permissionId")].iter() {
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
            for param_name in ["permissionId", "fileId"].iter() {
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


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionGetCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the permission.
    ///
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn permission_id(mut self, new_value: &str) -> PermissionGetCall<'a, C, A> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionGetCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionGetCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionGetCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PermissionGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PermissionGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a permission for a file or shared drive.
///
/// A builder for the *create* method supported by a *permission* resource.
/// It is not used directly, but through a `PermissionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Permission;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Permission::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.permissions().create(req, "fileId")
///              .use_domain_admin_access(true)
///              .transfer_ownership(false)
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .send_notification_email(true)
///              .email_message("sea")
///              .doit();
/// # }
/// ```
pub struct PermissionCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Permission,
    _file_id: String,
    _use_domain_admin_access: Option<bool>,
    _transfer_ownership: Option<bool>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _send_notification_email: Option<bool>,
    _email_message: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PermissionCreateCall<'a, C, A> {}

impl<'a, C, A> PermissionCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Permission)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.permissions.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._use_domain_admin_access {
            params.push(("useDomainAdminAccess", value.to_string()));
        }
        if let Some(value) = self._transfer_ownership {
            params.push(("transferOwnership", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._send_notification_email {
            params.push(("sendNotificationEmail", value.to_string()));
        }
        if let Some(value) = self._email_message {
            params.push(("emailMessage", value.to_string()));
        }
        for &field in ["alt", "fileId", "useDomainAdminAccess", "transferOwnership", "supportsTeamDrives", "supportsAllDrives", "sendNotificationEmail", "emailMessage"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/permissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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
    pub fn request(mut self, new_value: Permission) -> PermissionCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file or shared drive.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> PermissionCreateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
    ///
    /// Sets the *use domain admin access* query property to the given value.
    pub fn use_domain_admin_access(mut self, new_value: bool) -> PermissionCreateCall<'a, C, A> {
        self._use_domain_admin_access = Some(new_value);
        self
    }
    /// Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect.
    ///
    /// Sets the *transfer ownership* query property to the given value.
    pub fn transfer_ownership(mut self, new_value: bool) -> PermissionCreateCall<'a, C, A> {
        self._transfer_ownership = Some(new_value);
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> PermissionCreateCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> PermissionCreateCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers.
    ///
    /// Sets the *send notification email* query property to the given value.
    pub fn send_notification_email(mut self, new_value: bool) -> PermissionCreateCall<'a, C, A> {
        self._send_notification_email = Some(new_value);
        self
    }
    /// A plain text custom message to include in the notification email.
    ///
    /// Sets the *email message* query property to the given value.
    pub fn email_message(mut self, new_value: &str) -> PermissionCreateCall<'a, C, A> {
        self._email_message = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PermissionCreateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PermissionCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PermissionCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the changes for a user or shared drive.
///
/// A builder for the *list* method supported by a *change* resource.
/// It is not used directly, but through a `ChangeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.changes().list("pageToken")
///              .team_drive_id("eirmod")
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .spaces("dolor")
///              .restrict_to_my_drive(true)
///              .page_size(-85)
///              .include_team_drive_items(true)
///              .include_removed(true)
///              .include_items_from_all_drives(true)
///              .include_corpus_removals(true)
///              .drive_id("ipsum")
///              .doit();
/// # }
/// ```
pub struct ChangeListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _page_token: String,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _spaces: Option<String>,
    _restrict_to_my_drive: Option<bool>,
    _page_size: Option<i32>,
    _include_team_drive_items: Option<bool>,
    _include_removed: Option<bool>,
    _include_items_from_all_drives: Option<bool>,
    _include_corpus_removals: Option<bool>,
    _drive_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ChangeListCall<'a, C, A> {}

impl<'a, C, A> ChangeListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ChangeList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.changes.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(14 + self._additional_params.len());
        params.push(("pageToken", self._page_token.to_string()));
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._spaces {
            params.push(("spaces", value.to_string()));
        }
        if let Some(value) = self._restrict_to_my_drive {
            params.push(("restrictToMyDrive", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_team_drive_items {
            params.push(("includeTeamDriveItems", value.to_string()));
        }
        if let Some(value) = self._include_removed {
            params.push(("includeRemoved", value.to_string()));
        }
        if let Some(value) = self._include_items_from_all_drives {
            params.push(("includeItemsFromAllDrives", value.to_string()));
        }
        if let Some(value) = self._include_corpus_removals {
            params.push(("includeCorpusRemovals", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        for &field in ["alt", "pageToken", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "spaces", "restrictToMyDrive", "pageSize", "includeTeamDriveItems", "includeRemoved", "includeItemsFromAllDrives", "includeCorpusRemovals", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "changes";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
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


    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    ///
    /// Sets the *page token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn page_token(mut self, new_value: &str) -> ChangeListCall<'a, C, A> {
        self._page_token = new_value.to_string();
        self
    }
    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> ChangeListCall<'a, C, A> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> ChangeListCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> ChangeListCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    ///
    /// Sets the *spaces* query property to the given value.
    pub fn spaces(mut self, new_value: &str) -> ChangeListCall<'a, C, A> {
        self._spaces = Some(new_value.to_string());
        self
    }
    /// Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
    ///
    /// Sets the *restrict to my drive* query property to the given value.
    pub fn restrict_to_my_drive(mut self, new_value: bool) -> ChangeListCall<'a, C, A> {
        self._restrict_to_my_drive = Some(new_value);
        self
    }
    /// The maximum number of changes to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ChangeListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Deprecated use includeItemsFromAllDrives instead.
    ///
    /// Sets the *include team drive items* query property to the given value.
    pub fn include_team_drive_items(mut self, new_value: bool) -> ChangeListCall<'a, C, A> {
        self._include_team_drive_items = Some(new_value);
        self
    }
    /// Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
    ///
    /// Sets the *include removed* query property to the given value.
    pub fn include_removed(mut self, new_value: bool) -> ChangeListCall<'a, C, A> {
        self._include_removed = Some(new_value);
        self
    }
    /// Deprecated - Whether both My Drive and shared drive items should be included in results. This parameter will only be effective until June 1, 2020. Afterwards shared drive items will be included in the results.
    ///
    /// Sets the *include items from all drives* query property to the given value.
    pub fn include_items_from_all_drives(mut self, new_value: bool) -> ChangeListCall<'a, C, A> {
        self._include_items_from_all_drives = Some(new_value);
        self
    }
    /// Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
    ///
    /// Sets the *include corpus removals* query property to the given value.
    pub fn include_corpus_removals(mut self, new_value: bool) -> ChangeListCall<'a, C, A> {
        self._include_corpus_removals = Some(new_value);
        self
    }
    /// The shared drive from which changes will be returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> ChangeListCall<'a, C, A> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChangeListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChangeListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ChangeListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the starting pageToken for listing future changes.
///
/// A builder for the *getStartPageToken* method supported by a *change* resource.
/// It is not used directly, but through a `ChangeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.changes().get_start_page_token()
///              .team_drive_id("justo")
///              .supports_team_drives(true)
///              .supports_all_drives(true)
///              .drive_id("dolor")
///              .doit();
/// # }
/// ```
pub struct ChangeGetStartPageTokenCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _drive_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ChangeGetStartPageTokenCall<'a, C, A> {}

impl<'a, C, A> ChangeGetStartPageTokenCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, StartPageToken)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.changes.getStartPageToken",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        for &field in ["alt", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "changes/startPageToken";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
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


    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> ChangeGetStartPageTokenCall<'a, C, A> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> ChangeGetStartPageTokenCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> ChangeGetStartPageTokenCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive will be returned.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> ChangeGetStartPageTokenCall<'a, C, A> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChangeGetStartPageTokenCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChangeGetStartPageTokenCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ChangeGetStartPageTokenCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Subscribes to changes for a user.
///
/// A builder for the *watch* method supported by a *change* resource.
/// It is not used directly, but through a `ChangeMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.changes().watch(req, "pageToken")
///              .team_drive_id("et")
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .spaces("sed")
///              .restrict_to_my_drive(true)
///              .page_size(-38)
///              .include_team_drive_items(true)
///              .include_removed(false)
///              .include_items_from_all_drives(false)
///              .include_corpus_removals(false)
///              .drive_id("consetetur")
///              .doit();
/// # }
/// ```
pub struct ChangeWatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Channel,
    _page_token: String,
    _team_drive_id: Option<String>,
    _supports_team_drives: Option<bool>,
    _supports_all_drives: Option<bool>,
    _spaces: Option<String>,
    _restrict_to_my_drive: Option<bool>,
    _page_size: Option<i32>,
    _include_team_drive_items: Option<bool>,
    _include_removed: Option<bool>,
    _include_items_from_all_drives: Option<bool>,
    _include_corpus_removals: Option<bool>,
    _drive_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ChangeWatchCall<'a, C, A> {}

impl<'a, C, A> ChangeWatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.changes.watch",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        params.push(("pageToken", self._page_token.to_string()));
        if let Some(value) = self._team_drive_id {
            params.push(("teamDriveId", value.to_string()));
        }
        if let Some(value) = self._supports_team_drives {
            params.push(("supportsTeamDrives", value.to_string()));
        }
        if let Some(value) = self._supports_all_drives {
            params.push(("supportsAllDrives", value.to_string()));
        }
        if let Some(value) = self._spaces {
            params.push(("spaces", value.to_string()));
        }
        if let Some(value) = self._restrict_to_my_drive {
            params.push(("restrictToMyDrive", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._include_team_drive_items {
            params.push(("includeTeamDriveItems", value.to_string()));
        }
        if let Some(value) = self._include_removed {
            params.push(("includeRemoved", value.to_string()));
        }
        if let Some(value) = self._include_items_from_all_drives {
            params.push(("includeItemsFromAllDrives", value.to_string()));
        }
        if let Some(value) = self._include_corpus_removals {
            params.push(("includeCorpusRemovals", value.to_string()));
        }
        if let Some(value) = self._drive_id {
            params.push(("driveId", value.to_string()));
        }
        for &field in ["alt", "pageToken", "teamDriveId", "supportsTeamDrives", "supportsAllDrives", "spaces", "restrictToMyDrive", "pageSize", "includeTeamDriveItems", "includeRemoved", "includeItemsFromAllDrives", "includeCorpusRemovals", "driveId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "changes/watch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Channel) -> ChangeWatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
    ///
    /// Sets the *page token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn page_token(mut self, new_value: &str) -> ChangeWatchCall<'a, C, A> {
        self._page_token = new_value.to_string();
        self
    }
    /// Deprecated use driveId instead.
    ///
    /// Sets the *team drive id* query property to the given value.
    pub fn team_drive_id(mut self, new_value: &str) -> ChangeWatchCall<'a, C, A> {
        self._team_drive_id = Some(new_value.to_string());
        self
    }
    /// Deprecated use supportsAllDrives instead.
    ///
    /// Sets the *supports team drives* query property to the given value.
    pub fn supports_team_drives(mut self, new_value: bool) -> ChangeWatchCall<'a, C, A> {
        self._supports_team_drives = Some(new_value);
        self
    }
    /// Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives.
    ///
    /// Sets the *supports all drives* query property to the given value.
    pub fn supports_all_drives(mut self, new_value: bool) -> ChangeWatchCall<'a, C, A> {
        self._supports_all_drives = Some(new_value);
        self
    }
    /// A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
    ///
    /// Sets the *spaces* query property to the given value.
    pub fn spaces(mut self, new_value: &str) -> ChangeWatchCall<'a, C, A> {
        self._spaces = Some(new_value.to_string());
        self
    }
    /// Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
    ///
    /// Sets the *restrict to my drive* query property to the given value.
    pub fn restrict_to_my_drive(mut self, new_value: bool) -> ChangeWatchCall<'a, C, A> {
        self._restrict_to_my_drive = Some(new_value);
        self
    }
    /// The maximum number of changes to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ChangeWatchCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Deprecated use includeItemsFromAllDrives instead.
    ///
    /// Sets the *include team drive items* query property to the given value.
    pub fn include_team_drive_items(mut self, new_value: bool) -> ChangeWatchCall<'a, C, A> {
        self._include_team_drive_items = Some(new_value);
        self
    }
    /// Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
    ///
    /// Sets the *include removed* query property to the given value.
    pub fn include_removed(mut self, new_value: bool) -> ChangeWatchCall<'a, C, A> {
        self._include_removed = Some(new_value);
        self
    }
    /// Deprecated - Whether both My Drive and shared drive items should be included in results. This parameter will only be effective until June 1, 2020. Afterwards shared drive items will be included in the results.
    ///
    /// Sets the *include items from all drives* query property to the given value.
    pub fn include_items_from_all_drives(mut self, new_value: bool) -> ChangeWatchCall<'a, C, A> {
        self._include_items_from_all_drives = Some(new_value);
        self
    }
    /// Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
    ///
    /// Sets the *include corpus removals* query property to the given value.
    pub fn include_corpus_removals(mut self, new_value: bool) -> ChangeWatchCall<'a, C, A> {
        self._include_corpus_removals = Some(new_value);
        self
    }
    /// The shared drive from which changes will be returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
    ///
    /// Sets the *drive id* query property to the given value.
    pub fn drive_id(mut self, new_value: &str) -> ChangeWatchCall<'a, C, A> {
        self._drive_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChangeWatchCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChangeWatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ChangeWatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a revision with patch semantics.
///
/// A builder for the *update* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// use drive3::Revision;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Revision::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().update(req, "fileId", "revisionId")
///              .doit();
/// # }
/// ```
pub struct RevisionUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _request: Revision,
    _file_id: String,
    _revision_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RevisionUpdateCall<'a, C, A> {}

impl<'a, C, A> RevisionUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Revision)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.revisions.update",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("revisionId", self._revision_id.to_string()));
        for &field in ["alt", "fileId", "revisionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions/{revisionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{revisionId}", "revisionId")].iter() {
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
            for param_name in ["revisionId", "fileId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
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
    pub fn request(mut self, new_value: Revision) -> RevisionUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionUpdateCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the revision.
    ///
    /// Sets the *revision id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn revision_id(mut self, new_value: &str) -> RevisionUpdateCall<'a, C, A> {
        self._revision_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RevisionUpdateCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RevisionUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets a revision's metadata or content by ID.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Revision` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *get* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().get("fileId", "revisionId")
///              .acknowledge_abuse(false)
///              .doit();
/// # }
/// ```
pub struct RevisionGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _revision_id: String,
    _acknowledge_abuse: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RevisionGetCall<'a, C, A> {}

impl<'a, C, A> RevisionGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Revision)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.revisions.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("revisionId", self._revision_id.to_string()));
        if let Some(value) = self._acknowledge_abuse {
            params.push(("acknowledgeAbuse", value.to_string()));
        }
        for &field in ["fileId", "revisionId", "acknowledgeAbuse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let (json_field_missing, enable_resource_parsing) = {
            let mut enable = true;
            let mut field_present = true;
            for &(name, ref value) in params.iter() {
                if name == "alt" {
                    field_present = false;
                    if <String as AsRef<str>>::as_ref(&value) != "json" {
                        enable = false;
                    }
                    break;
                }
            }
            (field_present, enable)
        };
        if json_field_missing {
            params.push(("alt", "json".to_string()));
        }

        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions/{revisionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{revisionId}", "revisionId")].iter() {
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
            for param_name in ["revisionId", "fileId"].iter() {
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
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionGetCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the revision.
    ///
    /// Sets the *revision id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn revision_id(mut self, new_value: &str) -> RevisionGetCall<'a, C, A> {
        self._revision_id = new_value.to_string();
        self
    }
    /// Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
    ///
    /// Sets the *acknowledge abuse* query property to the given value.
    pub fn acknowledge_abuse(mut self, new_value: bool) -> RevisionGetCall<'a, C, A> {
        self._acknowledge_abuse = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RevisionGetCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RevisionGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
///
/// A builder for the *delete* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().delete("fileId", "revisionId")
///              .doit();
/// # }
/// ```
pub struct RevisionDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _revision_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RevisionDeleteCall<'a, C, A> {}

impl<'a, C, A> RevisionDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.revisions.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        params.push(("revisionId", self._revision_id.to_string()));
        for &field in ["fileId", "revisionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions/{revisionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId"), ("{revisionId}", "revisionId")].iter() {
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
            for param_name in ["revisionId", "fileId"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionDeleteCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The ID of the revision.
    ///
    /// Sets the *revision id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn revision_id(mut self, new_value: &str) -> RevisionDeleteCall<'a, C, A> {
        self._revision_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RevisionDeleteCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Full`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RevisionDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists a file's revisions.
///
/// A builder for the *list* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_drive3 as drive3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use drive3::DriveHub;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = DriveHub::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().list("fileId")
///              .page_token("consetetur")
///              .page_size(-54)
///              .doit();
/// # }
/// ```
pub struct RevisionListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a DriveHub<C, A>,
    _file_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for RevisionListCall<'a, C, A> {}

impl<'a, C, A> RevisionListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RevisionList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "drive.revisions.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("fileId", self._file_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "fileId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "files/{fileId}/revisions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::MetadataReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{fileId}", "fileId")].iter() {
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
            for param_name in ["fileId"].iter() {
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


    /// The ID of the file.
    ///
    /// Sets the *file id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn file_id(mut self, new_value: &str) -> RevisionListCall<'a, C, A> {
        self._file_id = new_value.to_string();
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> RevisionListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of revisions to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> RevisionListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RevisionListCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::MetadataReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> RevisionListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


