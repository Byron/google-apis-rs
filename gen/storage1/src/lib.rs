// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *storage* crate version *0.1.1+20150213*, where *20150213* is the exact revision of the *storage:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.1*.
//! 
//! Everything else about the *storage* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/storage/docs/json_api/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/storage1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Storage.html) ... 
//! 
//! * [bucket access controls](struct.BucketAccessControl.html)
//!  * [*delete*](struct.BucketAccessControlDeleteCall.html), [*get*](struct.BucketAccessControlGetCall.html), [*insert*](struct.BucketAccessControlInsertCall.html), [*list*](struct.BucketAccessControlListCall.html), [*patch*](struct.BucketAccessControlPatchCall.html) and [*update*](struct.BucketAccessControlUpdateCall.html)
//! * [buckets](struct.Bucket.html)
//!  * [*delete*](struct.BucketDeleteCall.html), [*get*](struct.BucketGetCall.html), [*insert*](struct.BucketInsertCall.html), [*list*](struct.BucketListCall.html), [*patch*](struct.BucketPatchCall.html) and [*update*](struct.BucketUpdateCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * default object access controls
//!  * [*delete*](struct.DefaultObjectAccessControlDeleteCall.html), [*get*](struct.DefaultObjectAccessControlGetCall.html), [*insert*](struct.DefaultObjectAccessControlInsertCall.html), [*list*](struct.DefaultObjectAccessControlListCall.html), [*patch*](struct.DefaultObjectAccessControlPatchCall.html) and [*update*](struct.DefaultObjectAccessControlUpdateCall.html)
//! * [object access controls](struct.ObjectAccessControl.html)
//!  * [*delete*](struct.ObjectAccessControlDeleteCall.html), [*get*](struct.ObjectAccessControlGetCall.html), [*insert*](struct.ObjectAccessControlInsertCall.html), [*list*](struct.ObjectAccessControlListCall.html), [*patch*](struct.ObjectAccessControlPatchCall.html) and [*update*](struct.ObjectAccessControlUpdateCall.html)
//! * [objects](struct.Object.html)
//!  * [*compose*](struct.ObjectComposeCall.html), [*copy*](struct.ObjectCopyCall.html), [*delete*](struct.ObjectDeleteCall.html), [*get*](struct.ObjectGetCall.html), [*insert*](struct.ObjectInsertCall.html), [*list*](struct.ObjectListCall.html), [*patch*](struct.ObjectPatchCall.html), [*update*](struct.ObjectUpdateCall.html) and [*watch all*](struct.ObjectWatchAllCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*insert objects*](struct.ObjectInsertCall.html)
//! 
//! Download supported by ...
//! 
//! * [*get objects*](struct.ObjectGetCall.html)
//! * [*update objects*](struct.ObjectUpdateCall.html)
//! * [*insert objects*](struct.ObjectInsertCall.html)
//! * [*compose objects*](struct.ObjectComposeCall.html)
//! * [*copy objects*](struct.ObjectCopyCall.html)
//! 
//! Subscription supported by ...
//! 
//! * [*watch all objects*](struct.ObjectWatchAllCall.html)
//! * [*list objects*](struct.ObjectListCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Storage.html)**
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
//! let r = hub.object_access_controls().get(...).doit()
//! let r = hub.default_object_access_controls().patch(...).doit()
//! let r = hub.object_access_controls().update(...).doit()
//! let r = hub.object_access_controls().list(...).doit()
//! let r = hub.object_access_controls().patch(...).doit()
//! let r = hub.default_object_access_controls().update(...).doit()
//! let r = hub.default_object_access_controls().insert(...).doit()
//! let r = hub.object_access_controls().insert(...).doit()
//! let r = hub.object_access_controls().delete(...).doit()
//! let r = hub.default_object_access_controls().get(...).doit()
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
//! google-storage1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-storage1" as storage1;
//! use storage1::ObjectAccessControl;
//! use storage1::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use storage1::Storage;
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
//! let mut hub = Storage::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req: ObjectAccessControl = Default::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.object_access_controls().update(&req, "bucket", "object", "entity")
//!              .generation("accusam")
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
    /// View your data in Google Cloud Storage
    DevstorageReadOnly,

    /// Manage your data in Google Cloud Storage
    DevstorageReadWrite,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// Manage your data and permissions in Google Cloud Storage
    DevstorageFullControl,
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
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

/// Central instance to access all Storage related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-storage1" as storage1;
/// use storage1::ObjectAccessControl;
/// use storage1::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
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
/// let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ObjectAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().update(&req, "bucket", "object", "entity")
///              .generation("erat")
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
pub struct Storage<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Storage<C, NC, A> {}

impl<'a, C, NC, A> Storage<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Storage<C, NC, A> {
        Storage {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.1".to_string(),
            _m: PhantomData
        }
    }

    pub fn bucket_access_controls(&'a self) -> BucketAccessControlMethods<'a, C, NC, A> {
        BucketAccessControlMethods { hub: &self }
    }
    pub fn buckets(&'a self) -> BucketMethods<'a, C, NC, A> {
        BucketMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, C, NC, A> {
        ChannelMethods { hub: &self }
    }
    pub fn default_object_access_controls(&'a self) -> DefaultObjectAccessControlMethods<'a, C, NC, A> {
        DefaultObjectAccessControlMethods { hub: &self }
    }
    pub fn object_access_controls(&'a self) -> ObjectAccessControlMethods<'a, C, NC, A> {
        ObjectAccessControlMethods { hub: &self }
    }
    pub fn objects(&'a self) -> ObjectMethods<'a, C, NC, A> {
        ObjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.1`.
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
/// An access-control list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list bucket access controls](struct.BucketAccessControlListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct BucketAccessControls {
    /// The list of items.    
    pub items: Vec<BucketAccessControl>,
    /// The kind of item this is. For lists of bucket access control entries, this is always storage#bucketAccessControls.    
    pub kind: String,
}

impl ResponseResult for BucketAccessControls {}


/// A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRule {
    /// The action to take.    
    pub action: BucketLifecycleRuleAction,
    /// The condition(s) under which the action will be taken.    
    pub condition: BucketLifecycleRuleCondition,
}

impl NestedType for BucketLifecycleRule {}
impl Part for BucketLifecycleRule {}


/// The project team associated with the entity, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAccessControlProjectTeam {
    /// The project number.    
    #[serde(alias="projectNumber")]
    pub project_number: String,
    /// The team. Can be owners, editors, or viewers.    
    pub team: String,
}

impl NestedType for ObjectAccessControlProjectTeam {}
impl Part for ObjectAccessControlProjectTeam {}


/// The condition(s) under which the action will be taken.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRuleCondition {
    /// Relevant only for versioned objects. If the value is true, this condition matches live objects; if the value is false, it matches archived objects.    
    #[serde(alias="isLive")]
    pub is_live: bool,
    /// Relevant only for versioned objects. If the value is N, this condition is satisfied when there are at least N versions (including the live version) newer than this version of the object.    
    #[serde(alias="numNewerVersions")]
    pub num_newer_versions: i32,
    /// Age of an object (in days). This condition is satisfied when an object reaches the specified age.    
    pub age: i32,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when an object is created before midnight of the specified date in UTC.    
    #[serde(alias="createdBefore")]
    pub created_before: String,
}

impl NestedType for BucketLifecycleRuleCondition {}
impl Part for BucketLifecycleRuleCondition {}


/// The bucket's website configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketWebsite {
    /// The custom object to return when a requested resource is not found.    
    #[serde(alias="notFoundPage")]
    pub not_found_page: String,
    /// Behaves as the bucket's directory index where missing objects are treated as potential directories.    
    #[serde(alias="mainPageSuffix")]
    pub main_page_suffix: String,
}

impl NestedType for BucketWebsite {}
impl Part for BucketWebsite {}


/// An notification channel used to watch for resource changes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [stop channels](struct.ChannelStopCall.html) (request)
/// * [watch all objects](struct.ObjectWatchAllCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// A version-specific identifier for the watched resource.    
    #[serde(alias="resourceUri")]
    pub resource_uri: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource. Value: the fixed string "api#channel".    
    pub kind: Option<String>,
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.    
    #[serde(alias="resourceId")]
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
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// A Boolean value to indicate whether payload is wanted. Optional.    
    pub payload: Option<bool>,
}

impl RequestValue for Channel {}
impl Resource for Channel {}
impl ResponseResult for Channel {}


/// The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLogging {
    /// A prefix for log object names.    
    #[serde(alias="logObjectPrefix")]
    pub log_object_prefix: String,
    /// The destination bucket where the current bucket's logs should be placed.    
    #[serde(alias="logBucket")]
    pub log_bucket: String,
}

impl NestedType for BucketLogging {}
impl Part for BucketLogging {}


/// The bucket's lifecycle configuration. See lifecycle management for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycle {
    /// A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken.    
    pub rule: Vec<BucketLifecycleRule>,
}

impl NestedType for BucketLifecycle {}
impl Part for BucketLifecycle {}


/// The bucket's versioning configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketVersioning {
    /// While set to true, versioning is fully enabled for this bucket.    
    pub enabled: bool,
}

impl NestedType for BucketVersioning {}
impl Part for BucketVersioning {}


/// An object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list objects](struct.ObjectListCall.html) (none)
/// * [update objects](struct.ObjectUpdateCall.html) (request|response)
/// * [copy objects](struct.ObjectCopyCall.html) (request|response)
/// * [watch all objects](struct.ObjectWatchAllCall.html) (none)
/// * [get objects](struct.ObjectGetCall.html) (response)
/// * [insert objects](struct.ObjectInsertCall.html) (request|response)
/// * [compose objects](struct.ObjectComposeCall.html) (response)
/// * [delete objects](struct.ObjectDeleteCall.html) (none)
/// * [patch objects](struct.ObjectPatchCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    /// The link to this object.    
    #[serde(alias="selfLink")]
    pub self_link: Option<String>,
    /// The creation or modification time of the object in RFC 3339 format. For buckets with versioning enabled, changing an object's metadata does not change this property.    
    pub updated: Option<String>,
    /// Content-Type of the object data.    
    #[serde(alias="contentType")]
    pub content_type: Option<String>,
    /// Content-Language of the object data.    
    #[serde(alias="contentLanguage")]
    pub content_language: Option<String>,
    /// The content generation of this object. Used for object versioning.    
    pub generation: Option<String>,
    /// Number of underlying components that make up this object. Components are accumulated by compose operations.    
    #[serde(alias="componentCount")]
    pub component_count: Option<i32>,
    /// The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object.    
    pub metageneration: Option<String>,
    /// Media download link.    
    #[serde(alias="mediaLink")]
    pub media_link: Option<String>,
    /// The owner of the object. This will always be the uploader of the object.    
    pub owner: Option<ObjectOwner>,
    /// Cache-Control directive for the object data.    
    #[serde(alias="cacheControl")]
    pub cache_control: Option<String>,
    /// Content-Encoding of the object data.    
    #[serde(alias="contentEncoding")]
    pub content_encoding: Option<String>,
    /// The ID of the object.    
    pub id: Option<String>,
    /// Content-Length of the data in bytes.    
    pub size: Option<String>,
    /// The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted.    
    #[serde(alias="timeDeleted")]
    pub time_deleted: Option<String>,
    /// The kind of item this is. For objects, this is always storage#object.    
    pub kind: Option<String>,
    /// The name of this object. Required if not specified by URL parameter.    
    pub name: Option<String>,
    /// MD5 hash of the data; encoded using base64.    
    #[serde(alias="md5Hash")]
    pub md5_hash: Option<String>,
    /// The name of the bucket containing this object.    
    pub bucket: Option<String>,
    /// Access controls on the object.    
    pub acl: Option<Vec<ObjectAccessControl>>,
    /// CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64.    
    pub crc32c: Option<String>,
    /// HTTP 1.1 Entity tag for the object.    
    pub etag: Option<String>,
    /// Storage class of the object.    
    #[serde(alias="storageClass")]
    pub storage_class: Option<String>,
    /// Content-Disposition of the object data.    
    #[serde(alias="contentDisposition")]
    pub content_disposition: Option<String>,
    /// User-provided metadata, in key/value pairs.    
    pub metadata: Option<HashMap<String, String>>,
}

impl RequestValue for Object {}
impl Resource for Object {}
impl ResponseResult for Object {}


/// A list of objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list objects](struct.ObjectListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Objects {
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The list of items.    
    pub items: Vec<Object>,
    /// The kind of item this is. For lists of objects, this is always storage#objects.    
    pub kind: String,
    /// The list of prefixes of objects matching-but-not-listed up to and including the requested delimiter.    
    pub prefixes: Vec<String>,
}

impl ResponseResult for Objects {}


/// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketCors {
    /// The list of Origins eligible to receive CORS response headers. Note: "*" is permitted in the list of origins, and means "any Origin".    
    pub origin: Vec<String>,
    /// The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains.    
    #[serde(alias="responseHeader")]
    pub response_header: Vec<String>,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: "*" is permitted in the list of methods, and means "any method".    
    pub method: Vec<String>,
    /// The value, in seconds, to return in the  Access-Control-Max-Age header used in preflight responses.    
    #[serde(alias="maxAgeSeconds")]
    pub max_age_seconds: i32,
}

impl NestedType for BucketCors {}
impl Part for BucketCors {}


/// An access-control list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list object access controls](struct.ObjectAccessControlListCall.html) (response)
/// * [list default object access controls](struct.DefaultObjectAccessControlListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ObjectAccessControls {
    /// The list of items.    
    pub items: Vec<String>,
    /// The kind of item this is. For lists of object access control entries, this is always storage#objectAccessControls.    
    pub kind: String,
}

impl ResponseResult for ObjectAccessControls {}


/// The project team associated with the entity, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAccessControlProjectTeam {
    /// The project number.    
    #[serde(alias="projectNumber")]
    pub project_number: String,
    /// The team. Can be owners, editors, or viewers.    
    pub team: String,
}

impl NestedType for BucketAccessControlProjectTeam {}
impl Part for BucketAccessControlProjectTeam {}


/// Conditions that must be met for this operation to execute.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct ComposeRequestSourceObjectsObjectPreconditions {
    /// Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail.    
    #[serde(alias="ifGenerationMatch")]
    pub if_generation_match: String,
}

impl NestedType for ComposeRequestSourceObjectsObjectPreconditions {}
impl Part for ComposeRequestSourceObjectsObjectPreconditions {}


/// The list of source objects that will be concatenated into a single object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct ComposeRequestSourceObjects {
    /// The generation of this object to use as the source.    
    pub generation: String,
    /// The source object's name. The source object's bucket is implicitly the destination bucket.    
    pub name: String,
    /// Conditions that must be met for this operation to execute.    
    #[serde(alias="objectPreconditions")]
    pub object_preconditions: ComposeRequestSourceObjectsObjectPreconditions,
}

impl NestedType for ComposeRequestSourceObjects {}
impl Part for ComposeRequestSourceObjects {}


/// A bucket.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update buckets](struct.BucketUpdateCall.html) (request|response)
/// * [insert buckets](struct.BucketInsertCall.html) (request|response)
/// * [delete buckets](struct.BucketDeleteCall.html) (none)
/// * [patch buckets](struct.BucketPatchCall.html) (request|response)
/// * [list buckets](struct.BucketListCall.html) (none)
/// * [get buckets](struct.BucketGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// The bucket's website configuration.    
    pub website: Option<BucketWebsite>,
    /// Creation time of the bucket in RFC 3339 format.    
    #[serde(alias="timeCreated")]
    pub time_created: Option<String>,
    /// The bucket's versioning configuration.    
    pub versioning: Option<BucketVersioning>,
    /// Default access controls to apply to new objects when no ACL is provided.    
    #[serde(alias="defaultObjectAcl")]
    pub default_object_acl: Option<Vec<ObjectAccessControl>>,
    /// The metadata generation of this bucket.    
    pub metageneration: Option<String>,
    /// The bucket's Cross-Origin Resource Sharing (CORS) configuration.    
    pub cors: Option<Vec<BucketCors>>,
    /// The owner of the bucket. This is always the project team's owner group.    
    pub owner: Option<BucketOwner>,
    /// Access controls on the bucket.    
    pub acl: Option<Vec<BucketAccessControl>>,
    /// The ID of the bucket.    
    pub id: Option<String>,
    /// The kind of item this is. For buckets, this is always storage#bucket.    
    pub kind: Option<String>,
    /// The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs.    
    pub logging: Option<BucketLogging>,
    /// The name of the bucket.    
    pub name: Option<String>,
    /// The project number of the project the bucket belongs to.    
    #[serde(alias="projectNumber")]
    pub project_number: Option<String>,
    /// HTTP 1.1 Entity tag for the bucket.    
    pub etag: Option<String>,
    /// The bucket's storage class. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Typical values are STANDARD and DURABLE_REDUCED_AVAILABILITY. Defaults to STANDARD. See the developer's guide for the authoritative list.    
    #[serde(alias="storageClass")]
    pub storage_class: Option<String>,
    /// The bucket's lifecycle configuration. See lifecycle management for more information.    
    pub lifecycle: Option<BucketLifecycle>,
    /// The URI of this bucket.    
    #[serde(alias="selfLink")]
    pub self_link: Option<String>,
    /// The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the developer's guide for the authoritative list.    
    pub location: Option<String>,
}

impl RequestValue for Bucket {}
impl Resource for Bucket {}
impl ResponseResult for Bucket {}


/// An access-control entry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list bucket access controls](struct.BucketAccessControlListCall.html) (none)
/// * [patch bucket access controls](struct.BucketAccessControlPatchCall.html) (request|response)
/// * [delete bucket access controls](struct.BucketAccessControlDeleteCall.html) (none)
/// * [update bucket access controls](struct.BucketAccessControlUpdateCall.html) (request|response)
/// * [get bucket access controls](struct.BucketAccessControlGetCall.html) (response)
/// * [insert bucket access controls](struct.BucketAccessControlInsertCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAccessControl {
    /// The domain associated with the entity, if any.    
    pub domain: Option<String>,
    /// The name of the bucket.    
    pub bucket: Option<String>,
    /// The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl.    
    pub kind: Option<String>,
    /// The entity holding the permission, in one of the following forms: 
    /// - user-userId 
    /// - user-email 
    /// - group-groupId 
    /// - group-email 
    /// - domain-domain 
    /// - project-team-projectId 
    /// - allUsers 
    /// - allAuthenticatedUsers Examples: 
    /// - The user liz@example.com would be user-liz@example.com. 
    /// - The group example@googlegroups.com would be group-example@googlegroups.com. 
    /// - To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
    pub entity: Option<String>,
    /// The email address associated with the entity, if any.    
    pub email: Option<String>,
    /// HTTP 1.1 Entity tag for the access-control entry.    
    pub etag: Option<String>,
    /// The access permission for the entity. Can be READER, WRITER, or OWNER.    
    pub role: Option<String>,
    /// The ID for the entity, if any.    
    #[serde(alias="entityId")]
    pub entity_id: Option<String>,
    /// The project team associated with the entity, if any.    
    #[serde(alias="projectTeam")]
    pub project_team: Option<BucketAccessControlProjectTeam>,
    /// The ID of the access-control entry.    
    pub id: Option<String>,
    /// The link to this access-control entry.    
    #[serde(alias="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for BucketAccessControl {}
impl Resource for BucketAccessControl {}
impl ResponseResult for BucketAccessControl {}


/// An access-control entry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get object access controls](struct.ObjectAccessControlGetCall.html) (response)
/// * [patch default object access controls](struct.DefaultObjectAccessControlPatchCall.html) (request|response)
/// * [update object access controls](struct.ObjectAccessControlUpdateCall.html) (request|response)
/// * [list object access controls](struct.ObjectAccessControlListCall.html) (none)
/// * [patch object access controls](struct.ObjectAccessControlPatchCall.html) (request|response)
/// * [update default object access controls](struct.DefaultObjectAccessControlUpdateCall.html) (request|response)
/// * [insert default object access controls](struct.DefaultObjectAccessControlInsertCall.html) (request|response)
/// * [insert object access controls](struct.ObjectAccessControlInsertCall.html) (request|response)
/// * [delete object access controls](struct.ObjectAccessControlDeleteCall.html) (none)
/// * [get default object access controls](struct.DefaultObjectAccessControlGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAccessControl {
    /// The domain associated with the entity, if any.    
    pub domain: Option<String>,
    /// The content generation of the object.    
    pub generation: Option<String>,
    /// The name of the object.    
    pub object: Option<String>,
    /// The name of the bucket.    
    pub bucket: Option<String>,
    /// The kind of item this is. For object access control entries, this is always storage#objectAccessControl.    
    pub kind: Option<String>,
    /// The entity holding the permission, in one of the following forms: 
    /// - user-userId 
    /// - user-email 
    /// - group-groupId 
    /// - group-email 
    /// - domain-domain 
    /// - project-team-projectId 
    /// - allUsers 
    /// - allAuthenticatedUsers Examples: 
    /// - The user liz@example.com would be user-liz@example.com. 
    /// - The group example@googlegroups.com would be group-example@googlegroups.com. 
    /// - To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
    pub entity: Option<String>,
    /// The email address associated with the entity, if any.    
    pub email: Option<String>,
    /// HTTP 1.1 Entity tag for the access-control entry.    
    pub etag: Option<String>,
    /// The access permission for the entity. Can be READER or OWNER.    
    pub role: Option<String>,
    /// The ID for the entity, if any.    
    #[serde(alias="entityId")]
    pub entity_id: Option<String>,
    /// The project team associated with the entity, if any.    
    #[serde(alias="projectTeam")]
    pub project_team: Option<ObjectAccessControlProjectTeam>,
    /// The ID of the access-control entry.    
    pub id: Option<String>,
    /// The link to this access-control entry.    
    #[serde(alias="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for ObjectAccessControl {}
impl Resource for ObjectAccessControl {}
impl ResponseResult for ObjectAccessControl {}


/// The owner of the object. This will always be the uploader of the object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectOwner {
    /// The ID for the entity.    
    #[serde(alias="entityId")]
    pub entity_id: String,
    /// The entity, in the form user-userId.    
    pub entity: String,
}

impl NestedType for ObjectOwner {}
impl Part for ObjectOwner {}


/// The action to take.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRuleAction {
    /// Type of the action. Currently, only Delete is supported.    
    #[serde(alias="type")]
    pub type_: String,
}

impl NestedType for BucketLifecycleRuleAction {}
impl Part for BucketLifecycleRuleAction {}


/// The owner of the bucket. This is always the project team's owner group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketOwner {
    /// The ID for the entity.    
    #[serde(alias="entityId")]
    pub entity_id: String,
    /// The entity, in the form project-owner-projectId.    
    pub entity: String,
}

impl NestedType for BucketOwner {}
impl Part for BucketOwner {}


/// A Compose request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [compose objects](struct.ObjectComposeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct ComposeRequest {
    /// The kind of item this is.    
    pub kind: Option<String>,
    /// Properties of the resulting object.    
    pub destination: Option<Object>,
    /// The list of source objects that will be concatenated into a single object.    
    #[serde(alias="sourceObjects")]
    pub source_objects: Option<Vec<ComposeRequestSourceObjects>>,
}

impl RequestValue for ComposeRequest {}


/// A list of buckets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list buckets](struct.BucketListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Buckets {
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The list of items.    
    pub items: Vec<Bucket>,
    /// The kind of item this is. For lists of buckets, this is always storage#buckets.    
    pub kind: String,
}

impl ResponseResult for Buckets {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *defaultObjectAccessControl* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-storage1" as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.default_object_access_controls();
/// # }
/// ```
pub struct DefaultObjectAccessControlMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for DefaultObjectAccessControlMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> DefaultObjectAccessControlMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new default object ACL entry on the specified bucket.    
    pub fn insert(&self, request: &ObjectAccessControl, bucket: &str) -> DefaultObjectAccessControlInsertCall<'a, C, NC, A> {
        DefaultObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a default object ACL entry on the specified bucket.    
    pub fn update(&self, request: &ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlUpdateCall<'a, C, NC, A> {
        DefaultObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves default object ACL entries on the specified bucket.    
    pub fn list(&self, bucket: &str) -> DefaultObjectAccessControlListCall<'a, C, NC, A> {
        DefaultObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a default object ACL entry on the specified bucket. This method supports patch semantics.    
    pub fn patch(&self, request: &ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlPatchCall<'a, C, NC, A> {
        DefaultObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.    
    pub fn delete(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlDeleteCall<'a, C, NC, A> {
        DefaultObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the default object ACL entry for the specified entity on the specified bucket.    
    pub fn get(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlGetCall<'a, C, NC, A> {
        DefaultObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *bucketAccessControl* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-storage1" as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.bucket_access_controls();
/// # }
/// ```
pub struct BucketAccessControlMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for BucketAccessControlMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketAccessControlMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified bucket. This method supports patch semantics.    
    pub fn patch(&self, request: &BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlPatchCall<'a, C, NC, A> {
        BucketAccessControlPatchCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.    
    pub fn delete(&self, bucket: &str, entity: &str) -> BucketAccessControlDeleteCall<'a, C, NC, A> {
        BucketAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified bucket.    
    pub fn insert(&self, request: &BucketAccessControl, bucket: &str) -> BucketAccessControlInsertCall<'a, C, NC, A> {
        BucketAccessControlInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified bucket.    
    pub fn get(&self, bucket: &str, entity: &str) -> BucketAccessControlGetCall<'a, C, NC, A> {
        BucketAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified bucket.    
    pub fn update(&self, request: &BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlUpdateCall<'a, C, NC, A> {
        BucketAccessControlUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified bucket.    
    pub fn list(&self, bucket: &str) -> BucketAccessControlListCall<'a, C, NC, A> {
        BucketAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-storage1" as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ChannelMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop watching resources through this channel    
    pub fn stop(&self, request: &Channel) -> ChannelStopCall<'a, C, NC, A> {
        ChannelStopCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *object* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-storage1" as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `compose(...)`, `copy(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `update(...)` and `watch_all(...)`
/// // to build up your call.
/// let rb = hub.objects();
/// # }
/// ```
pub struct ObjectMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ObjectMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an object or its metadata.    
    pub fn get(&self, bucket: &str, object: &str) -> ObjectGetCall<'a, C, NC, A> {
        ObjectGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _projection: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watch for changes on all objects in a bucket.    
    pub fn watch_all(&self, request: &Channel, bucket: &str) -> ObjectWatchAllCall<'a, C, NC, A> {
        ObjectWatchAllCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an object's metadata.    
    pub fn update(&self, request: &Object, bucket: &str, object: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        ObjectUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stores a new object and metadata.    
    pub fn insert(&self, request: &Object, bucket: &str) -> ObjectInsertCall<'a, C, NC, A> {
        ObjectInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _name: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _content_encoding: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Concatenates a list of existing objects into a new object in the same bucket.    
    pub fn compose(&self, request: &ComposeRequest, destination_bucket: &str, destination_object: &str) -> ObjectComposeCall<'a, C, NC, A> {
        ObjectComposeCall {
            hub: self.hub,
            _request: request.clone(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _if_metageneration_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.    
    pub fn delete(&self, bucket: &str, object: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        ObjectDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of objects matching the criteria.    
    pub fn list(&self, bucket: &str) -> ObjectListCall<'a, C, NC, A> {
        ObjectListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies an object to a specified location. Optionally overrides metadata.    
    pub fn copy(&self, request: &Object, source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str) -> ObjectCopyCall<'a, C, NC, A> {
        ObjectCopyCall {
            hub: self.hub,
            _request: request.clone(),
            _source_bucket: source_bucket.to_string(),
            _source_object: source_object.to_string(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _source_generation: Default::default(),
            _projection: Default::default(),
            _if_source_metageneration_not_match: Default::default(),
            _if_source_metageneration_match: Default::default(),
            _if_source_generation_not_match: Default::default(),
            _if_source_generation_match: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an object's metadata. This method supports patch semantics.    
    pub fn patch(&self, request: &Object, bucket: &str, object: &str) -> ObjectPatchCall<'a, C, NC, A> {
        ObjectPatchCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *objectAccessControl* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-storage1" as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.object_access_controls();
/// # }
/// ```
pub struct ObjectAccessControlMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ObjectAccessControlMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectAccessControlMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified object.    
    pub fn get(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlGetCall<'a, C, NC, A> {
        ObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified object.    
    pub fn insert(&self, request: &ObjectAccessControl, bucket: &str, object: &str) -> ObjectAccessControlInsertCall<'a, C, NC, A> {
        ObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified object. This method supports patch semantics.    
    pub fn patch(&self, request: &ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlPatchCall<'a, C, NC, A> {
        ObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified object.    
    pub fn list(&self, bucket: &str, object: &str) -> ObjectAccessControlListCall<'a, C, NC, A> {
        ObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified object.    
    pub fn delete(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlDeleteCall<'a, C, NC, A> {
        ObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified object.    
    pub fn update(&self, request: &ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlUpdateCall<'a, C, NC, A> {
        ObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *bucket* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-storage1" as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.buckets();
/// # }
/// ```
pub struct BucketMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for BucketMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a bucket.    
    pub fn update(&self, request: &Bucket, bucket: &str) -> BucketUpdateCall<'a, C, NC, A> {
        BucketUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for the specified bucket.    
    pub fn get(&self, bucket: &str) -> BucketGetCall<'a, C, NC, A> {
        BucketGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _projection: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes an empty bucket.    
    pub fn delete(&self, bucket: &str) -> BucketDeleteCall<'a, C, NC, A> {
        BucketDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new bucket.    
    pub fn insert(&self, request: &Bucket, project: &str) -> BucketInsertCall<'a, C, NC, A> {
        BucketInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _project: project.to_string(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a bucket. This method supports patch semantics.    
    pub fn patch(&self, request: &Bucket, bucket: &str) -> BucketPatchCall<'a, C, NC, A> {
        BucketPatchCall {
            hub: self.hub,
            _request: request.clone(),
            _bucket: bucket.to_string(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of buckets for a given project.    
    pub fn list(&self, project: &str) -> BucketListCall<'a, C, NC, A> {
        BucketListCall {
            hub: self.hub,
            _project: project.to_string(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Creates a new default object ACL entry on the specified bucket.
///
/// A builder for the *insert* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ObjectAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().insert(&req, "bucket")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DefaultObjectAccessControlInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DefaultObjectAccessControlInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        for &field in ["alt", "bucket"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/defaultObjectAcl".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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
    pub fn request(mut self, new_value: &ObjectAccessControl) -> DefaultObjectAccessControlInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlInsertCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlInsertCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlInsertCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> DefaultObjectAccessControlInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a default object ACL entry on the specified bucket.
///
/// A builder for the *update* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ObjectAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().update(&req, "bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DefaultObjectAccessControlUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DefaultObjectAccessControlUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["alt", "bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/defaultObjectAcl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
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
    pub fn request(mut self, new_value: &ObjectAccessControl) -> DefaultObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> DefaultObjectAccessControlUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves default object ACL entries on the specified bucket.
///
/// A builder for the *list* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().list("bucket")
///              .if_metageneration_not_match("gubergren")
///              .if_metageneration_match("sadipscing")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DefaultObjectAccessControlListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DefaultObjectAccessControlListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/defaultObjectAcl".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// If present, only return default ACL listing if the bucket's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// If present, only return default ACL listing if the bucket's current metageneration matches this value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> DefaultObjectAccessControlListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a default object ACL entry on the specified bucket. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ObjectAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().patch(&req, "bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlPatchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DefaultObjectAccessControlPatchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DefaultObjectAccessControlPatchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.patch", 
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["alt", "bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/defaultObjectAcl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.as_slice())
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
    pub fn request(mut self, new_value: &ObjectAccessControl) -> DefaultObjectAccessControlPatchCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlPatchCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlPatchCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> DefaultObjectAccessControlPatchCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *delete* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().delete("bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DefaultObjectAccessControlDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DefaultObjectAccessControlDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/defaultObjectAcl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> DefaultObjectAccessControlDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns the default object ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *get* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().get("bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for DefaultObjectAccessControlGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> DefaultObjectAccessControlGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["alt", "bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/defaultObjectAcl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> DefaultObjectAccessControlGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates an ACL entry on the specified bucket. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::BucketAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: BucketAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().patch(&req, "bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlPatchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: BucketAccessControl,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketAccessControlPatchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketAccessControlPatchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.patch", 
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["alt", "bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.as_slice())
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
    pub fn request(mut self, new_value: &BucketAccessControl) -> BucketAccessControlPatchCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlPatchCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlPatchCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketAccessControlPatchCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Permanently deletes the ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *delete* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().delete("bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketAccessControlDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketAccessControlDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketAccessControlDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a new ACL entry on the specified bucket.
///
/// A builder for the *insert* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::BucketAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: BucketAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().insert(&req, "bucket")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: BucketAccessControl,
    _bucket: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketAccessControlInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketAccessControlInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        for &field in ["alt", "bucket"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/acl".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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
    pub fn request(mut self, new_value: &BucketAccessControl) -> BucketAccessControlInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlInsertCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlInsertCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlInsertCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketAccessControlInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns the ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *get* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().get("bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketAccessControlGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketAccessControlGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["alt", "bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlGetCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlGetCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketAccessControlGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates an ACL entry on the specified bucket.
///
/// A builder for the *update* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::BucketAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: BucketAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().update(&req, "bucket", "entity")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: BucketAccessControl,
    _bucket: String,
    _entity: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketAccessControlUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketAccessControlUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        for &field in ["alt", "bucket", "entity"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "entity"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
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
    pub fn request(mut self, new_value: &BucketAccessControl) -> BucketAccessControlUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketAccessControlUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves ACL entries on the specified bucket.
///
/// A builder for the *list* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().list("bucket")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketAccessControlListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketAccessControlListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        for &field in ["alt", "bucket"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/acl".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlListCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketAccessControlListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Stop watching resources through this channel
///
/// A builder for the *stop* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Channel = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().stop(&req)
///              .doit();
/// # }
/// ```
pub struct ChannelStopCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Channel,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelStopCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelStopCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.channels.stop", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/storage/v1/channels/stop".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
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
                    let result_value = res;

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
    pub fn request(mut self, new_value: &Channel) -> ChannelStopCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelStopCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ChannelStopCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ChannelStopCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves an object or its metadata.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Object` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *get* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().get("bucket", "object")
///              .projection("dolor")
///              .if_metageneration_not_match("eirmod")
///              .if_metageneration_match("elitr")
///              .if_generation_not_match("amet")
///              .if_generation_match("no")
///              .generation("labore")
///              .doit();
/// # }
/// ```
pub struct ObjectGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _object: String,
    _projection: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "projection", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
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
                    if value.as_slice() != "json" {
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

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
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
            for param_name in ["bucket", "object"].iter() {
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
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which the object resides.    
    pub fn bucket(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl.    
    pub fn projection(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's generation does not match the given value.    
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's generation matches the given value.    
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectGetCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Watch for changes on all objects in a bucket.
///
/// A builder for the *watchAll* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Channel = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().watch_all(&req, "bucket")
///              .versions(true)
///              .projection("invidunt")
///              .prefix("aliquyam")
///              .page_token("accusam")
///              .max_results(45)
///              .delimiter("sea")
///              .doit();
/// # }
/// ```
pub struct ObjectWatchAllCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Channel,
    _bucket: String,
    _versions: Option<bool>,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delimiter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectWatchAllCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectWatchAllCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.watchAll", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._versions {
            params.push(("versions", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._prefix {
            params.push(("prefix", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._delimiter {
            params.push(("delimiter", value.to_string()));
        }
        for &field in ["alt", "bucket", "versions", "projection", "prefix", "pageToken", "maxResults", "delimiter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/watch".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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
    pub fn request(mut self, new_value: &Channel) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which to look for objects.    
    pub fn bucket(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *versions* query property to the given value.
    ///
    /// 
    /// If true, lists all versions of a file as distinct results.    
    pub fn versions(mut self, new_value: bool) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._versions = Some(new_value);
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl.    
    pub fn projection(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *prefix* query property to the given value.
    ///
    /// 
    /// Filter results to objects whose names begin with this prefix.    
    pub fn prefix(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// A previously-returned page token representing part of the larger set of results to view.    
    pub fn page_token(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of items plus prefixes to return. As duplicate prefixes are omitted, fewer total results may be returned than requested.    
    pub fn max_results(mut self, new_value: u32) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *delimiter* query property to the given value.
    ///
    /// 
    /// Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted.    
    pub fn delimiter(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, NC, A> {
        self._delimiter = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectWatchAllCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectWatchAllCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectWatchAllCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates an object's metadata.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Object` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *update* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Object;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Object = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().update(&req, "bucket", "object")
///              .projection("et")
///              .predefined_acl("eirmod")
///              .if_metageneration_not_match("sanctus")
///              .if_metageneration_match("et")
///              .if_generation_not_match("amet")
///              .if_generation_match("et")
///              .generation("consetetur")
///              .doit();
/// # }
/// ```
pub struct ObjectUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Object,
    _bucket: String,
    _object: String,
    _projection: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((11 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "projection", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
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
                    if value.as_slice() != "json" {
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

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
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
            for param_name in ["bucket", "object"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
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
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    } else { (res, Default::default()) };

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
    pub fn request(mut self, new_value: &Object) -> ObjectUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which the object resides.    
    pub fn bucket(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to full.    
    pub fn projection(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to this object.    
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation does not match the given value.    
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation matches the given value.    
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Stores a new object and metadata.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Object` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *insert* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Object;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Object = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().insert(&req, "bucket")
///              .projection("ea")
///              .predefined_acl("sed")
///              .name("dolor")
///              .if_metageneration_not_match("dolor")
///              .if_metageneration_match("dolor")
///              .if_generation_not_match("et")
///              .if_generation_match("consetetur")
///              .content_encoding("amet.")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct ObjectInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Object,
    _bucket: String,
    _projection: Option<String>,
    _predefined_acl: Option<String>,
    _name: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _content_encoding: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::client::Response, Object)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((11 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._content_encoding {
            params.push(("contentEncoding", value.to_string()));
        }
        for &field in ["bucket", "projection", "predefinedAcl", "name", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "contentEncoding"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
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
                    if value.as_slice() != "json" {
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

        let mut url = if protocol == "simple" {
                "https://www.googleapis.com/upload/storage/v1/b/{bucket}/o".to_string()
            } else if protocol == "resumable" {
                "https://www.googleapis.com/resumable/upload/storage/v1/b/{bucket}/o".to_string()
            } else { 
                unreachable!() 
        };
        params.push(("uploadType", protocol.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

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
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let mut response = hyper::client::Response::new(Box::new(cmn::DummyNetworkStream));
                    match response {
                        Ok(ref mut res) => {
                            res.status = hyper::status::StatusCode::Ok;
                            res.headers.set(Location(upload_url.as_ref().unwrap().clone()))
                        }
                        _ => unreachable!(),
                    }
                    response
                } else {
                    let mut mp_reader: MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, ContentType(mime_type))
                        },
                        _ => (&mut request_value_reader as &mut io::Read, ContentType(json_mime_type.clone())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url = &res.headers.get::<Location>().expect("Location header is part of protocol").0;
                            if upload_url_from_server {
                                dlg.store_upload_url(url);
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Result::Cancelled
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Result::HttpError(err)
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status.is_success() {
                                    dlg.finished(false);
                                    return Result::Failure(res)
                                }
                            }
                        }
                    }
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 0kb
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, Object)>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumeable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// TODO: Write more about how delegation works in this particular case.
    ///
    /// * *max size*: 0kb
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, Object)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Object) -> ObjectInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.    
    pub fn bucket(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.    
    pub fn projection(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to this object.    
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *name* query property to the given value.
    ///
    /// 
    /// Name of the object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any.    
    pub fn name(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation does not match the given value.    
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation matches the given value.    
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *content encoding* query property to the given value.
    ///
    /// 
    /// If set, sets the contentEncoding property of the final object to this value. Setting this parameter is equivalent to setting the contentEncoding metadata property. This can be useful when uploading an object with uploadType=media to indicate the encoding of the content being uploaded.    
    pub fn content_encoding(mut self, new_value: &str) -> ObjectInsertCall<'a, C, NC, A> {
        self._content_encoding = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectInsertCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectInsertCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Concatenates a list of existing objects into a new object in the same bucket.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Object` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *compose* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::ComposeRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ComposeRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().compose(&req, "destinationBucket", "destinationObject")
///              .if_metageneration_match("gubergren")
///              .if_generation_match("justo")
///              .destination_predefined_acl("sit")
///              .doit();
/// # }
/// ```
pub struct ObjectComposeCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: ComposeRequest,
    _destination_bucket: String,
    _destination_object: String,
    _if_metageneration_match: Option<String>,
    _if_generation_match: Option<String>,
    _destination_predefined_acl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectComposeCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectComposeCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.compose", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._destination_predefined_acl {
            params.push(("destinationPredefinedAcl", value.to_string()));
        }
        for &field in ["destinationBucket", "destinationObject", "ifMetagenerationMatch", "ifGenerationMatch", "destinationPredefinedAcl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
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
                    if value.as_slice() != "json" {
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

        let mut url = "https://www.googleapis.com/storage/v1/b/{destinationBucket}/o/{destinationObject}/compose".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{destinationBucket}", "destinationBucket"), ("{destinationObject}", "destinationObject")].iter() {
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
            for param_name in ["destinationBucket", "destinationObject"].iter() {
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
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    } else { (res, Default::default()) };

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
    pub fn request(mut self, new_value: &ComposeRequest) -> ObjectComposeCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which to store the new object.    
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectComposeCall<'a, C, NC, A> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the new object.    
    pub fn destination_object(mut self, new_value: &str) -> ObjectComposeCall<'a, C, NC, A> {
        self._destination_object = new_value.to_string();
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectComposeCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation matches the given value.    
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectComposeCall<'a, C, NC, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *destination predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to the destination object.    
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectComposeCall<'a, C, NC, A> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectComposeCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectComposeCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectComposeCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.
///
/// A builder for the *delete* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().delete("bucket", "object")
///              .if_metageneration_not_match("rebum.")
///              .if_metageneration_match("consetetur")
///              .if_generation_not_match("sadipscing")
///              .if_generation_match("vero")
///              .generation("sadipscing")
///              .doit();
/// # }
/// ```
pub struct ObjectDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _object: String,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
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
            for param_name in ["bucket", "object"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which the object resides.    
    pub fn bucket(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation does not match the given value.    
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation matches the given value.    
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, permanently deletes a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves a list of objects matching the criteria.
///
/// A builder for the *list* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().list("bucket")
///              .versions(false)
///              .projection("dolore")
///              .prefix("duo")
///              .page_token("aliquyam")
///              .max_results(96)
///              .delimiter("et")
///              .doit();
/// # }
/// ```
pub struct ObjectListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _versions: Option<bool>,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delimiter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Objects)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._versions {
            params.push(("versions", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._prefix {
            params.push(("prefix", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._delimiter {
            params.push(("delimiter", value.to_string()));
        }
        for &field in ["alt", "bucket", "versions", "projection", "prefix", "pageToken", "maxResults", "delimiter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which to look for objects.    
    pub fn bucket(mut self, new_value: &str) -> ObjectListCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *versions* query property to the given value.
    ///
    /// 
    /// If true, lists all versions of a file as distinct results.    
    pub fn versions(mut self, new_value: bool) -> ObjectListCall<'a, C, NC, A> {
        self._versions = Some(new_value);
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl.    
    pub fn projection(mut self, new_value: &str) -> ObjectListCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *prefix* query property to the given value.
    ///
    /// 
    /// Filter results to objects whose names begin with this prefix.    
    pub fn prefix(mut self, new_value: &str) -> ObjectListCall<'a, C, NC, A> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// A previously-returned page token representing part of the larger set of results to view.    
    pub fn page_token(mut self, new_value: &str) -> ObjectListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of items plus prefixes to return. As duplicate prefixes are omitted, fewer total results may be returned than requested.    
    pub fn max_results(mut self, new_value: u32) -> ObjectListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *delimiter* query property to the given value.
    ///
    /// 
    /// Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted.    
    pub fn delimiter(mut self, new_value: &str) -> ObjectListCall<'a, C, NC, A> {
        self._delimiter = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Copies an object to a specified location. Optionally overrides metadata.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Object` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *copy* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Object;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Object = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().copy(&req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .source_generation("kasd")
///              .projection("sanctus")
///              .if_source_metageneration_not_match("takimata")
///              .if_source_metageneration_match("At")
///              .if_source_generation_not_match("labore")
///              .if_source_generation_match("invidunt")
///              .if_metageneration_not_match("ea")
///              .if_metageneration_match("sadipscing")
///              .if_generation_not_match("rebum.")
///              .if_generation_match("dolore")
///              .destination_predefined_acl("nonumy")
///              .doit();
/// # }
/// ```
pub struct ObjectCopyCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Object,
    _source_bucket: String,
    _source_object: String,
    _destination_bucket: String,
    _destination_object: String,
    _source_generation: Option<String>,
    _projection: Option<String>,
    _if_source_metageneration_not_match: Option<String>,
    _if_source_metageneration_match: Option<String>,
    _if_source_generation_not_match: Option<String>,
    _if_source_generation_match: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _destination_predefined_acl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectCopyCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectCopyCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.copy", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((17 + self._additional_params.len()));
        params.push(("sourceBucket", self._source_bucket.to_string()));
        params.push(("sourceObject", self._source_object.to_string()));
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._source_generation {
            params.push(("sourceGeneration", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._if_source_metageneration_not_match {
            params.push(("ifSourceMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_metageneration_match {
            params.push(("ifSourceMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_generation_not_match {
            params.push(("ifSourceGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_generation_match {
            params.push(("ifSourceGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._destination_predefined_acl {
            params.push(("destinationPredefinedAcl", value.to_string()));
        }
        for &field in ["sourceBucket", "sourceObject", "destinationBucket", "destinationObject", "sourceGeneration", "projection", "ifSourceMetagenerationNotMatch", "ifSourceMetagenerationMatch", "ifSourceGenerationNotMatch", "ifSourceGenerationMatch", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "destinationPredefinedAcl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
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
                    if value.as_slice() != "json" {
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

        let mut url = "https://www.googleapis.com/storage/v1/b/{sourceBucket}/o/{sourceObject}/copyTo/b/{destinationBucket}/o/{destinationObject}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{sourceBucket}", "sourceBucket"), ("{sourceObject}", "sourceObject"), ("{destinationBucket}", "destinationBucket"), ("{destinationObject}", "destinationObject")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(4);
            for param_name in ["sourceBucket", "sourceObject", "destinationBucket", "destinationObject"].iter() {
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
                    let result_value = if enable_resource_parsing {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    } else { (res, Default::default()) };

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
    pub fn request(mut self, new_value: &Object) -> ObjectCopyCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *source bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which to find the source object.    
    pub fn source_bucket(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._source_bucket = new_value.to_string();
        self
    }
    /// Sets the *source object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the source object.    
    pub fn source_object(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._source_object = new_value.to_string();
        self
    }
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.    
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any.    
    pub fn destination_object(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._destination_object = new_value.to_string();
        self
    }
    /// Sets the *source generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of the source object (as opposed to the latest version, the default).    
    pub fn source_generation(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._source_generation = Some(new_value.to_string());
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.    
    pub fn projection(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *if source metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the source object's current metageneration does not match the given value.    
    pub fn if_source_metageneration_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_source_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if source metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the source object's current metageneration matches the given value.    
    pub fn if_source_metageneration_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_source_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if source generation not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the source object's generation does not match the given value.    
    pub fn if_source_generation_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_source_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if source generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the source object's generation matches the given value.    
    pub fn if_source_generation_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_source_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the destination object's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the destination object's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the destination object's current generation does not match the given value.    
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the destination object's current generation matches the given value.    
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *destination predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to the destination object.    
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectCopyCall<'a, C, NC, A> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectCopyCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectCopyCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectCopyCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates an object's metadata. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Object;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Object = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().patch(&req, "bucket", "object")
///              .projection("sit")
///              .predefined_acl("eirmod")
///              .if_metageneration_not_match("consetetur")
///              .if_metageneration_match("labore")
///              .if_generation_not_match("sed")
///              .if_generation_match("ea")
///              .generation("gubergren")
///              .doit();
/// # }
/// ```
pub struct ObjectPatchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Object,
    _bucket: String,
    _object: String,
    _projection: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectPatchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectPatchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.patch", 
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "projection", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
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
            for param_name in ["bucket", "object"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.as_slice())
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
    pub fn request(mut self, new_value: &Object) -> ObjectPatchCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the bucket in which the object resides.    
    pub fn bucket(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to full.    
    pub fn projection(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to this object.    
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation not match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation does not match the given value.    
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if generation match* query property to the given value.
    ///
    /// 
    /// Makes the operation conditional on whether the object's current generation matches the given value.    
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectPatchCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectPatchCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectPatchCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectPatchCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns the ACL entry for the specified entity on the specified object.
///
/// A builder for the *get* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().get("bucket", "object", "entity")
///              .generation("sea")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _object: String,
    _entity: String,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectAccessControlGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectAccessControlGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "object", "entity"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectAccessControlGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a new ACL entry on the specified object.
///
/// A builder for the *insert* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ObjectAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().insert(&req, "bucket", "object")
///              .generation("aliquyam")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectAccessControlInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectAccessControlInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}/acl".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
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
            for param_name in ["bucket", "object"].iter() {
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
    pub fn request(mut self, new_value: &ObjectAccessControl) -> ObjectAccessControlInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlInsertCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlInsertCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectAccessControlInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates an ACL entry on the specified object. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ObjectAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().patch(&req, "bucket", "object", "entity")
///              .generation("ut")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlPatchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _entity: String,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectAccessControlPatchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectAccessControlPatchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.patch", 
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "object", "entity"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.as_slice())
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
    pub fn request(mut self, new_value: &ObjectAccessControl) -> ObjectAccessControlPatchCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlPatchCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlPatchCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectAccessControlPatchCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves ACL entries on the specified object.
///
/// A builder for the *list* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().list("bucket", "object")
///              .generation("amet")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _object: String,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectAccessControlListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectAccessControlListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}/acl".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
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
            for param_name in ["bucket", "object"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlListCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlListCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlListCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectAccessControlListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Permanently deletes the ACL entry for the specified entity on the specified object.
///
/// A builder for the *delete* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().delete("bucket", "object", "entity")
///              .generation("justo")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _object: String,
    _entity: String,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectAccessControlDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectAccessControlDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "entity", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "object", "entity"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectAccessControlDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates an ACL entry on the specified object.
///
/// A builder for the *update* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ObjectAccessControl = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().update(&req, "bucket", "object", "entity")
///              .generation("ut")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _entity: String,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ObjectAccessControlUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ObjectAccessControlUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}/o/{object}/acl/{entity}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
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
            for param_name in ["bucket", "object", "entity"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
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
    pub fn request(mut self, new_value: &ObjectAccessControl) -> ObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of the object.    
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._object = new_value.to_string();
        self
    }
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.    
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._entity = new_value.to_string();
        self
    }
    /// Sets the *generation* query property to the given value.
    ///
    /// 
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).    
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, NC, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> ObjectAccessControlUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a bucket.
///
/// A builder for the *update* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Bucket;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Bucket = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().update(&req, "bucket")
///              .projection("eos")
///              .predefined_default_object_acl("voluptua.")
///              .predefined_acl("duo")
///              .if_metageneration_not_match("sed")
///              .if_metageneration_match("aliquyam")
///              .doit();
/// # }
/// ```
pub struct BucketUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Bucket,
    _bucket: String,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_default_object_acl {
            params.push(("predefinedDefaultObjectAcl", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "projection", "predefinedDefaultObjectAcl", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
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
    pub fn request(mut self, new_value: &Bucket) -> BucketUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketUpdateCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to full.    
    pub fn projection(mut self, new_value: &str) -> BucketUpdateCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined default object acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of default object access controls to this bucket.    
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketUpdateCall<'a, C, NC, A> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to this bucket.    
    pub fn predefined_acl(mut self, new_value: &str) -> BucketUpdateCall<'a, C, NC, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketUpdateCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketUpdateCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns metadata for the specified bucket.
///
/// A builder for the *get* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().get("bucket")
///              .projection("ea")
///              .if_metageneration_not_match("et")
///              .if_metageneration_match("dolor")
///              .doit();
/// # }
/// ```
pub struct BucketGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _projection: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "projection", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketGetCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl.    
    pub fn projection(mut self, new_value: &str) -> BucketGetCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketGetCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketGetCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Permanently deletes an empty bucket.
///
/// A builder for the *delete* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().delete("bucket")
///              .if_metageneration_not_match("kasd")
///              .if_metageneration_match("invidunt")
///              .doit();
/// # }
/// ```
pub struct BucketDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _bucket: String,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["bucket", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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


    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketDeleteCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// If set, only deletes the bucket if its metageneration does not match this value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketDeleteCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// If set, only deletes the bucket if its metageneration matches this value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketDeleteCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a new bucket.
///
/// A builder for the *insert* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Bucket;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Bucket = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().insert(&req, "project")
///              .projection("Lorem")
///              .predefined_default_object_acl("clita")
///              .predefined_acl("invidunt")
///              .doit();
/// # }
/// ```
pub struct BucketInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Bucket,
    _project: String,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("project", self._project.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_default_object_acl {
            params.push(("predefinedDefaultObjectAcl", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        for &field in ["alt", "project", "projection", "predefinedDefaultObjectAcl", "predefinedAcl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
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
    pub fn request(mut self, new_value: &Bucket) -> BucketInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *project* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A valid API project identifier.    
    pub fn project(mut self, new_value: &str) -> BucketInsertCall<'a, C, NC, A> {
        self._project = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl, unless the bucket resource specifies acl or defaultObjectAcl properties, when it defaults to full.    
    pub fn projection(mut self, new_value: &str) -> BucketInsertCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined default object acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of default object access controls to this bucket.    
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketInsertCall<'a, C, NC, A> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to this bucket.    
    pub fn predefined_acl(mut self, new_value: &str) -> BucketInsertCall<'a, C, NC, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketInsertCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketInsertCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a bucket. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// use storage1::Bucket;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Bucket = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().patch(&req, "bucket")
///              .projection("At")
///              .predefined_default_object_acl("consetetur")
///              .predefined_acl("et")
///              .if_metageneration_not_match("sed")
///              .if_metageneration_match("sit")
///              .doit();
/// # }
/// ```
pub struct BucketPatchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _request: Bucket,
    _bucket: String,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketPatchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketPatchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.patch", 
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_default_object_acl {
            params.push(("predefinedDefaultObjectAcl", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "projection", "predefinedDefaultObjectAcl", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b/{bucket}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
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
            for param_name in ["bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.as_slice())
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
    pub fn request(mut self, new_value: &Bucket) -> BucketPatchCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Name of a bucket.    
    pub fn bucket(mut self, new_value: &str) -> BucketPatchCall<'a, C, NC, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to full.    
    pub fn projection(mut self, new_value: &str) -> BucketPatchCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined default object acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of default object access controls to this bucket.    
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketPatchCall<'a, C, NC, A> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *predefined acl* query property to the given value.
    ///
    /// 
    /// Apply a predefined set of access controls to this bucket.    
    pub fn predefined_acl(mut self, new_value: &str) -> BucketPatchCall<'a, C, NC, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration not match* query property to the given value.
    ///
    /// 
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.    
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketPatchCall<'a, C, NC, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// 
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.    
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketPatchCall<'a, C, NC, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketPatchCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketPatchCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketPatchCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves a list of buckets for a given project.
///
/// A builder for the *list* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-storage1" as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().list("project")
///              .projection("elitr")
///              .prefix("nonumy")
///              .page_token("rebum.")
///              .max_results(95)
///              .doit();
/// # }
/// ```
pub struct BucketListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Storage<C, NC, A>,
    _project: String,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for BucketListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> BucketListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Buckets)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("project", self._project.to_string()));
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._prefix {
            params.push(("prefix", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "project", "projection", "prefix", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/storage/v1/b".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_slice().to_string(), ());
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


    /// Sets the *project* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A valid API project identifier.    
    pub fn project(mut self, new_value: &str) -> BucketListCall<'a, C, NC, A> {
        self._project = new_value.to_string();
        self
    }
    /// Sets the *projection* query property to the given value.
    ///
    /// 
    /// Set of properties to return. Defaults to noAcl.    
    pub fn projection(mut self, new_value: &str) -> BucketListCall<'a, C, NC, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Sets the *prefix* query property to the given value.
    ///
    /// 
    /// Filter results to buckets whose names begin with this prefix.    
    pub fn prefix(mut self, new_value: &str) -> BucketListCall<'a, C, NC, A> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// A previously-returned page token representing part of the larger set of results to view.    
    pub fn page_token(mut self, new_value: &str) -> BucketListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Maximum number of buckets to return.    
    pub fn max_results(mut self, new_value: u32) -> BucketListCall<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> BucketListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


