// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *appsactivity* crate version *0.1.5+20140828*, where *20140828* is the exact revision of the *appsactivity:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.5*.
//! 
//! Everything else about the *appsactivity* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/google-apps/activity/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/appsactivity1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Appsactivity.html) ... 
//! 
//! * [activities](struct.Activity.html)
//!  * [*list*](struct.ActivityListCall.html)
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
//! * **[Hub](struct.Appsactivity.html)**
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
//! let r = hub.activities().list(...).doit()
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
//! google-appsactivity1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_appsactivity1 as appsactivity1;
//! use appsactivity1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use appsactivity1::Appsactivity;
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
//! let mut hub = Appsactivity::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.activities().list()
//!              .user_id("accusam")
//!              .source("takimata")
//!              .page_token("justo")
//!              .page_size(-1)
//!              .grouping_strategy("erat")
//!              .drive_file_id("labore")
//!              .drive_ancestor_id("sea")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!         Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
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
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(std_misc)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin, slice_patterns)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage the files in your Google Drive
    Drive,

    /// View the activity history of your Google Apps
    Activity,

    /// View metadata for files in your Google Drive
    DriveMetadataReadonly,

    /// View the files in your Google Drive
    DriveReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::Activity => "https://www.googleapis.com/auth/activity",
            Scope::DriveMetadataReadonly => "https://www.googleapis.com/auth/drive.metadata.readonly",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DriveMetadataReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Appsactivity related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_appsactivity1 as appsactivity1;
/// use appsactivity1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use appsactivity1::Appsactivity;
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
/// let mut hub = Appsactivity::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list()
///              .user_id("nonumy")
///              .source("dolores")
///              .page_token("gubergren")
///              .page_size(-95)
///              .grouping_strategy("aliquyam")
///              .drive_file_id("ea")
///              .drive_ancestor_id("no")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///         Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Appsactivity<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
}

impl<'a, C, A> Hub for Appsactivity<C, A> {}

impl<'a, C, A> Appsactivity<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Appsactivity<C, A> {
        Appsactivity {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.5".to_string(),
        }
    }

    pub fn activities(&'a self) -> ActivityMethods<'a, C, A> {
        ActivityMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.5`.
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
/// Contains information about a renametype event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rename {
    /// The new title.
    #[serde(rename="newTitle")]
    pub new_title: String,
    /// The old title.
    #[serde(rename="oldTitle")]
    pub old_title: String,
}

impl Part for Rename {}


/// Contains information about a Drive object's permissions that changed as a result of a permissionChange type event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionChange {
    /// Lists all Permission objects removed.
    #[serde(rename="removedPermissions")]
    pub removed_permissions: Vec<Permission>,
    /// Lists all Permission objects added.
    #[serde(rename="addedPermissions")]
    pub added_permissions: Vec<Permission>,
}

impl Part for PermissionChange {}


/// Information about the object modified by the event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// The MIME type of the target.
    #[serde(rename="mimeType")]
    pub mime_type: String,
    /// The ID of the target. For example, in Google Drive, this is the file or folder ID.
    pub id: String,
    /// The name of the target. For example, in Google Drive, this is the title of the file.
    pub name: String,
}

impl Part for Target {}


/// Contains information about a parent object. For example, a folder in Drive is a parent for all files within it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parent {
    /// The parent's ID.
    pub id: String,
    /// Whether this is the root folder.
    #[serde(rename="isRoot")]
    pub is_root: bool,
    /// The parent's title.
    pub title: String,
}

impl Part for Parent {}


/// Contains information about the permissions and type of access allowed with regards to a Google Drive object. This is a subset of the fields contained in a corresponding Drive Permissions object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// Whether the permission requires a link to the file.
    #[serde(rename="withLink")]
    pub with_link: bool,
    /// The ID for this permission. Corresponds to the Drive API's permission ID returned as part of the Drive Permissions resource.
    #[serde(rename="permissionId")]
    pub permission_id: String,
    /// Indicates the Google Drive permissions role. The role determines a user's ability to read, write, or comment on the file.
    pub role: String,
    /// The name of the user or group the permission applies to.
    pub name: String,
    /// Indicates how widely permissions are granted.
    #[serde(rename="type")]
    pub type_: String,
    /// The user's information if the type is USER.
    pub user: User,
}

impl Part for Permission {}


/// Photo information for a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// The URL of the photo.
    pub url: String,
}

impl Part for Photo {}


/// Contains information about changes in an object's parents as a result of a move type event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    /// The removed parent(s).
    #[serde(rename="removedParents")]
    pub removed_parents: Vec<Parent>,
    /// The added parent(s).
    #[serde(rename="addedParents")]
    pub added_parents: Vec<Parent>,
}

impl Part for Move {}


/// The response from the list request. Contains a list of activities and a token to retrieve the next page of results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](struct.ActivityListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListActivitiesResponse {
    /// Token for the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: String,
    /// List of activities.
    pub activities: Vec<Activity>,
}

impl ResponseResult for ListActivitiesResponse {}


/// A representation of a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// The profile photo of the user.
    pub photo: Photo,
    /// The displayable name of the user.
    pub name: String,
}

impl Part for User {}


/// An Activity resource is a combined view of multiple events. An activity has a list of individual events and a combined view of the common fields among all events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// The fields common to all of the singleEvents that make up the Activity.
    #[serde(rename="combinedEvent")]
    pub combined_event: Event,
    /// A list of all the Events that make up the Activity.
    #[serde(rename="singleEvents")]
    pub single_events: Vec<Event>,
}

impl Part for Activity {}


/// Represents the changes associated with an action taken by a user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Extra information for rename type events, such as the old and new names.
    pub rename: Rename,
    /// Information specific to the Target object modified by the event.
    pub target: Target,
    /// Additional event types. Some events may have multiple types when multiple actions are part of a single event. For example, creating a document, renaming it, and sharing it may be part of a single file-creation event.
    #[serde(rename="additionalEventTypes")]
    pub additional_event_types: Vec<String>,
    /// Extra information for move type events, such as changes in an object's parents.
    #[serde(rename="move")]
    pub move_: Move,
    /// Extra information for permissionChange type events, such as the user or group the new permission applies to.
    #[serde(rename="permissionChanges")]
    pub permission_changes: Vec<PermissionChange>,
    /// Represents the user responsible for the event.
    pub user: User,
    /// The time at which the event occurred formatted as Unix time in milliseconds.
    #[serde(rename="eventTimeMillis")]
    pub event_time_millis: String,
    /// The main type of event that occurred.
    #[serde(rename="primaryEventType")]
    pub primary_event_type: String,
    /// Whether this event is caused by a user being deleted.
    #[serde(rename="fromUserDeletion")]
    pub from_user_deletion: bool,
}

impl Part for Event {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the `Appsactivity` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_appsactivity1 as appsactivity1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use appsactivity1::Appsactivity;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Appsactivity::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Appsactivity<C, A>,
}

impl<'a, C, A> MethodsBuilder for ActivityMethods<'a, C, A> {}

impl<'a, C, A> ActivityMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of activities visible to the current logged in user. Visible activities are determined by the visiblity settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter.
    pub fn list(&self) -> ActivityListCall<'a, C, A> {
        ActivityListCall {
            hub: self.hub,
            _user_id: Default::default(),
            _source: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _grouping_strategy: Default::default(),
            _drive_file_id: Default::default(),
            _drive_ancestor_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns a list of activities visible to the current logged in user. Visible activities are determined by the visiblity settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter.
///
/// A builder for the *list* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_appsactivity1 as appsactivity1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use appsactivity1::Appsactivity;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Appsactivity::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list()
///              .user_id("justo")
///              .source("justo")
///              .page_token("et")
///              .page_size(-17)
///              .grouping_strategy("diam")
///              .drive_file_id("ipsum")
///              .drive_ancestor_id("Lorem")
///              .doit();
/// # }
/// ```
pub struct ActivityListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Appsactivity<C, A>,
    _user_id: Option<String>,
    _source: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _grouping_strategy: Option<String>,
    _drive_file_id: Option<String>,
    _drive_ancestor_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActivityListCall<'a, C, A> {}

impl<'a, C, A> ActivityListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListActivitiesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "appsactivity.activities.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        if let Some(value) = self._user_id {
            params.push(("userId", value.to_string()));
        }
        if let Some(value) = self._source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._grouping_strategy {
            params.push(("groupingStrategy", value.to_string()));
        }
        if let Some(value) = self._drive_file_id {
            params.push(("drive.fileId", value.to_string()));
        }
        if let Some(value) = self._drive_ancestor_id {
            params.push(("drive.ancestorId", value.to_string()));
        }
        for &field in ["alt", "userId", "source", "pageToken", "pageSize", "groupingStrategy", "drive.fileId", "drive.ancestorId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/appsactivity/v1/activities".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveMetadataReadonly.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Indicates the user to return activity for. Use the special value me to indicate the currently authenticated user.
    ///
    /// Sets the *user id* query property to the given value.
    pub fn user_id(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._user_id = Some(new_value.to_string());
        self
    }
    /// The Google service from which to return activities. Possible values of source are: 
    /// - drive.google.com
    ///
    /// Sets the *source* query property to the given value.
    pub fn source(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._source = Some(new_value.to_string());
        self
    }
    /// A token to retrieve a specific page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of events to return on a page. The response includes a continuation token if there are more events.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ActivityListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Indicates the strategy to use when grouping singleEvents items in the associated combinedEvent object.
    ///
    /// Sets the *grouping strategy* query property to the given value.
    pub fn grouping_strategy(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._grouping_strategy = Some(new_value.to_string());
        self
    }
    /// Identifies the Drive item to return activities for.
    ///
    /// Sets the *drive.file id* query property to the given value.
    pub fn drive_file_id(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._drive_file_id = Some(new_value.to_string());
        self
    }
    /// Identifies the Drive folder containing the items for which to return activities.
    ///
    /// Sets the *drive.ancestor id* query property to the given value.
    pub fn drive_ancestor_id(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._drive_ancestor_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ActivityListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveMetadataReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ActivityListCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


