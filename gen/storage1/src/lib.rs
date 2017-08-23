// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *storage* crate version *1.0.5+20170504*, where *20170504* is the exact revision of the *storage:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.5*.
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
//!  * [*delete*](struct.BucketDeleteCall.html), [*get*](struct.BucketGetCall.html), [*get iam policy*](struct.BucketGetIamPolicyCall.html), [*insert*](struct.BucketInsertCall.html), [*list*](struct.BucketListCall.html), [*patch*](struct.BucketPatchCall.html), [*set iam policy*](struct.BucketSetIamPolicyCall.html), [*test iam permissions*](struct.BucketTestIamPermissionCall.html) and [*update*](struct.BucketUpdateCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * default object access controls
//!  * [*delete*](struct.DefaultObjectAccessControlDeleteCall.html), [*get*](struct.DefaultObjectAccessControlGetCall.html), [*insert*](struct.DefaultObjectAccessControlInsertCall.html), [*list*](struct.DefaultObjectAccessControlListCall.html), [*patch*](struct.DefaultObjectAccessControlPatchCall.html) and [*update*](struct.DefaultObjectAccessControlUpdateCall.html)
//! * [notifications](struct.Notification.html)
//!  * [*delete*](struct.NotificationDeleteCall.html), [*get*](struct.NotificationGetCall.html), [*insert*](struct.NotificationInsertCall.html) and [*list*](struct.NotificationListCall.html)
//! * [object access controls](struct.ObjectAccessControl.html)
//!  * [*delete*](struct.ObjectAccessControlDeleteCall.html), [*get*](struct.ObjectAccessControlGetCall.html), [*insert*](struct.ObjectAccessControlInsertCall.html), [*list*](struct.ObjectAccessControlListCall.html), [*patch*](struct.ObjectAccessControlPatchCall.html) and [*update*](struct.ObjectAccessControlUpdateCall.html)
//! * [objects](struct.Object.html)
//!  * [*compose*](struct.ObjectComposeCall.html), [*copy*](struct.ObjectCopyCall.html), [*delete*](struct.ObjectDeleteCall.html), [*get*](struct.ObjectGetCall.html), [*get iam policy*](struct.ObjectGetIamPolicyCall.html), [*insert*](struct.ObjectInsertCall.html), [*list*](struct.ObjectListCall.html), [*patch*](struct.ObjectPatchCall.html), [*rewrite*](struct.ObjectRewriteCall.html), [*set iam policy*](struct.ObjectSetIamPolicyCall.html), [*test iam permissions*](struct.ObjectTestIamPermissionCall.html), [*update*](struct.ObjectUpdateCall.html) and [*watch all*](struct.ObjectWatchAllCall.html)
//! * projects
//!  * [*service account get*](struct.ProjectServiceAccountGetCall.html)
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
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Storage.html)**
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
//! let r = hub.objects().get_iam_policy(...).doit()
//! let r = hub.objects().list(...).doit()
//! let r = hub.objects().rewrite(...).doit()
//! let r = hub.objects().test_iam_permissions(...).doit()
//! let r = hub.objects().copy(...).doit()
//! let r = hub.objects().watch_all(...).doit()
//! let r = hub.objects().get(...).doit()
//! let r = hub.objects().set_iam_policy(...).doit()
//! let r = hub.objects().insert(...).doit()
//! let r = hub.objects().compose(...).doit()
//! let r = hub.objects().update(...).doit()
//! let r = hub.objects().delete(...).doit()
//! let r = hub.objects().patch(...).doit()
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
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_storage1 as storage1;
//! use storage1::Object;
//! use storage1::{Result, Error};
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
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Object::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
//!              .user_project("justo")
//!              .source_generation("et")
//!              .rewrite_token("et")
//!              .projection("diam")
//!              .max_bytes_rewritten_per_call("ipsum")
//!              .if_source_metageneration_not_match("Lorem")
//!              .if_source_metageneration_match("et")
//!              .if_source_generation_not_match("duo")
//!              .if_source_generation_match("aliquyam")
//!              .if_metageneration_not_match("sea")
//!              .if_metageneration_match("Lorem")
//!              .if_generation_not_match("eos")
//!              .if_generation_match("erat")
//!              .destination_predefined_acl("sadipscing")
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
    /// View your data in Google Cloud Storage
    DevstorageReadOnly,

    /// Manage your data in Google Cloud Storage
    DevstorageReadWrite,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// Manage your data and permissions in Google Cloud Storage
    DevstorageFullControl,

    /// View your data across Google Cloud Platform services
    CloudPlatformReadOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
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

/// Central instance to access all Storage related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// use storage1::Object;
/// use storage1::{Result, Error};
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
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .user_project("no")
///              .source_generation("labore")
///              .rewrite_token("eirmod")
///              .projection("dolore")
///              .max_bytes_rewritten_per_call("invidunt")
///              .if_source_metageneration_not_match("aliquyam")
///              .if_source_metageneration_match("accusam")
///              .if_source_generation_not_match("Lorem")
///              .if_source_generation_match("sea")
///              .if_metageneration_not_match("et")
///              .if_metageneration_match("duo")
///              .if_generation_not_match("et")
///              .if_generation_match("eirmod")
///              .destination_predefined_acl("sanctus")
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
pub struct Storage<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Storage<C, A> {}

impl<'a, C, A> Storage<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Storage<C, A> {
        Storage {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.5".to_string(),
            _base_url: "https://www.googleapis.com/storage/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn bucket_access_controls(&'a self) -> BucketAccessControlMethods<'a, C, A> {
        BucketAccessControlMethods { hub: &self }
    }
    pub fn buckets(&'a self) -> BucketMethods<'a, C, A> {
        BucketMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, C, A> {
        ChannelMethods { hub: &self }
    }
    pub fn default_object_access_controls(&'a self) -> DefaultObjectAccessControlMethods<'a, C, A> {
        DefaultObjectAccessControlMethods { hub: &self }
    }
    pub fn notifications(&'a self) -> NotificationMethods<'a, C, A> {
        NotificationMethods { hub: &self }
    }
    pub fn object_access_controls(&'a self) -> ObjectAccessControlMethods<'a, C, A> {
        ObjectAccessControlMethods { hub: &self }
    }
    pub fn objects(&'a self) -> ObjectMethods<'a, C, A> {
        ObjectMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/storage/v1/`.
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
/// The action to take.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRuleAction {
    /// Type of the action. Currently, only Delete and SetStorageClass are supported.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Target storage class. Required iff the type of the action is SetStorageClass.
    #[serde(rename="storageClass")]
    pub storage_class: Option<String>,
}

impl NestedType for BucketLifecycleRuleAction {}
impl Part for BucketLifecycleRuleAction {}


/// Conditions that must be met for this operation to execute.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjectsObjectPreconditions {
    /// Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail.
    #[serde(rename="ifGenerationMatch")]
    pub if_generation_match: Option<String>,
}

impl NestedType for ComposeRequestSourceObjectsObjectPreconditions {}
impl Part for ComposeRequestSourceObjectsObjectPreconditions {}


/// The list of source objects that will be concatenated into a single object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjects {
    /// The generation of this object to use as the source.
    pub generation: Option<String>,
    /// The source object's name. The source object's bucket is implicitly the destination bucket.
    pub name: Option<String>,
    /// Conditions that must be met for this operation to execute.
    #[serde(rename="objectPreconditions")]
    pub object_preconditions: Option<ComposeRequestSourceObjectsObjectPreconditions>,
}

impl NestedType for ComposeRequestSourceObjects {}
impl Part for ComposeRequestSourceObjects {}


/// A subscription to receive Google PubSub notifications.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service account get projects](struct.ProjectServiceAccountGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// The kind of item this is. For notifications, this is always storage#notification.
    pub kind: Option<String>,
    /// The ID of the notification.
    pub email_address: Option<String>,
}

impl ResponseResult for ServiceAccount {}


/// An access-control list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list bucket access controls](struct.BucketAccessControlListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAccessControls {
    /// The list of items.
    pub items: Option<Vec<BucketAccessControl>>,
    /// The kind of item this is. For lists of bucket access control entries, this is always storage#bucketAccessControls.
    pub kind: Option<String>,
}

impl ResponseResult for BucketAccessControls {}


/// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketCors {
    /// The list of Origins eligible to receive CORS response headers. Note: "*" is permitted in the list of origins, and means "any Origin".
    pub origin: Option<Vec<String>>,
    /// The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains.
    #[serde(rename="responseHeader")]
    pub response_header: Option<Vec<String>>,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: "*" is permitted in the list of methods, and means "any method".
    pub method: Option<Vec<String>>,
    /// The value, in seconds, to return in the  Access-Control-Max-Age header used in preflight responses.
    #[serde(rename="maxAgeSeconds")]
    pub max_age_seconds: Option<i32>,
}

impl NestedType for BucketCors {}
impl Part for BucketCors {}


/// A bucket.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update buckets](struct.BucketUpdateCall.html) (request|response)
/// * [test iam permissions buckets](struct.BucketTestIamPermissionCall.html) (none)
/// * [insert buckets](struct.BucketInsertCall.html) (request|response)
/// * [delete buckets](struct.BucketDeleteCall.html) (none)
/// * [set iam policy buckets](struct.BucketSetIamPolicyCall.html) (none)
/// * [get iam policy buckets](struct.BucketGetIamPolicyCall.html) (none)
/// * [patch buckets](struct.BucketPatchCall.html) (request|response)
/// * [list buckets](struct.BucketListCall.html) (none)
/// * [get buckets](struct.BucketGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information.
    pub website: Option<BucketWebsite>,
    /// The URI of this bucket.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// The modification time of the bucket in RFC 3339 format.
    pub updated: Option<String>,
    /// The creation time of the bucket in RFC 3339 format.
    #[serde(rename="timeCreated")]
    pub time_created: Option<String>,
    /// User-provided labels, in key/value pairs.
    pub labels: Option<HashMap<String, String>>,
    /// Default access controls to apply to new objects when no ACL is provided.
    #[serde(rename="defaultObjectAcl")]
    pub default_object_acl: Option<Vec<ObjectAccessControl>>,
    /// The metadata generation of this bucket.
    pub metageneration: Option<String>,
    /// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
    pub cors: Option<Vec<BucketCors>>,
    /// The owner of the bucket. This is always the project team's owner group.
    pub owner: Option<BucketOwner>,
    /// Access controls on the bucket.
    pub acl: Option<Vec<BucketAccessControl>>,
    /// The ID of the bucket. For buckets, the id and name properities are the same.
    pub id: Option<String>,
    /// The kind of item this is. For buckets, this is always storage#bucket.
    pub kind: Option<String>,
    /// The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs.
    pub logging: Option<BucketLogging>,
    /// The name of the bucket.
    pub name: Option<String>,
    /// The bucket's billing configuration.
    pub billing: Option<BucketBilling>,
    /// The project number of the project the bucket belongs to.
    #[serde(rename="projectNumber")]
    pub project_number: Option<String>,
    /// HTTP 1.1 Entity tag for the bucket.
    pub etag: Option<String>,
    /// The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see storage classes.
    #[serde(rename="storageClass")]
    pub storage_class: Option<String>,
    /// The bucket's lifecycle configuration. See lifecycle management for more information.
    pub lifecycle: Option<BucketLifecycle>,
    /// The bucket's versioning configuration.
    pub versioning: Option<BucketVersioning>,
    /// The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the developer's guide for the authoritative list.
    pub location: Option<String>,
}

impl RequestValue for Bucket {}
impl Resource for Bucket {}
impl ResponseResult for Bucket {}


/// A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRule {
    /// The action to take.
    pub action: Option<BucketLifecycleRuleAction>,
    /// The condition(s) under which the action will be taken.
    pub condition: Option<BucketLifecycleRuleCondition>,
}

impl NestedType for BucketLifecycleRule {}
impl Part for BucketLifecycleRule {}


/// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectCustomerEncryption {
    /// The encryption algorithm.
    #[serde(rename="encryptionAlgorithm")]
    pub encryption_algorithm: Option<String>,
    /// SHA256 hash value of the encryption key.
    #[serde(rename="keySha256")]
    pub key_sha256: Option<String>,
}

impl NestedType for ObjectCustomerEncryption {}
impl Part for ObjectCustomerEncryption {}


/// The project team associated with the entity, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAccessControlProjectTeam {
    /// The project number.
    #[serde(rename="projectNumber")]
    pub project_number: Option<String>,
    /// The team.
    pub team: Option<String>,
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
    #[serde(rename="isLive")]
    pub is_live: Option<bool>,
    /// Objects having any of the storage classes specified by this condition will be matched. Values include MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, STANDARD, and DURABLE_REDUCED_AVAILABILITY.
    #[serde(rename="matchesStorageClass")]
    pub matches_storage_class: Option<Vec<String>>,
    /// Age of an object (in days). This condition is satisfied when an object reaches the specified age.
    pub age: Option<i32>,
    /// Relevant only for versioned objects. If the value is N, this condition is satisfied when there are at least N versions (including the live version) newer than this version of the object.
    #[serde(rename="numNewerVersions")]
    pub num_newer_versions: Option<i32>,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when an object is created before midnight of the specified date in UTC.
    #[serde(rename="createdBefore")]
    pub created_before: Option<String>,
}

impl NestedType for BucketLifecycleRuleCondition {}
impl Part for BucketLifecycleRuleCondition {}


/// A bucket/object IAM policy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy objects](struct.ObjectGetIamPolicyCall.html) (response)
/// * [set iam policy buckets](struct.BucketSetIamPolicyCall.html) (request|response)
/// * [set iam policy objects](struct.ObjectSetIamPolicyCall.html) (request|response)
/// * [get iam policy buckets](struct.BucketGetIamPolicyCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// The ID of the resource to which this policy belongs. Will be of the form buckets/bucket for buckets, and buckets/bucket/objects/object for objects. A specific generation may be specified by appending #generationNumber to the end of the object name, e.g. buckets/my-bucket/objects/data.txt#17. The current generation can be denoted with #0. This field is ignored on input.
    #[serde(rename="resourceId")]
    pub resource_id: Option<String>,
    /// An association between a role, which comes with a set of permissions, and members who may assume that role.
    pub bindings: Option<Vec<PolicyBindings>>,
    /// HTTP 1.1  Entity tag for the policy.
    pub etag: Option<String>,
    /// The kind of item this is. For policies, this is always storage#policy. This field is ignored on input.
    pub kind: Option<String>,
}

impl RequestValue for Policy {}
impl ResponseResult for Policy {}


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
    /// The content generation of the object, if applied to an object.
    pub generation: Option<String>,
    /// The name of the object, if applied to an object.
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
    /// The access permission for the entity.
    pub role: Option<String>,
    /// The ID for the entity, if any.
    #[serde(rename="entityId")]
    pub entity_id: Option<String>,
    /// The project team associated with the entity, if any.
    #[serde(rename="projectTeam")]
    pub project_team: Option<ObjectAccessControlProjectTeam>,
    /// The ID of the access-control entry.
    pub id: Option<String>,
    /// The link to this access-control entry.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for ObjectAccessControl {}
impl Resource for ObjectAccessControl {}
impl ResponseResult for ObjectAccessControl {}


/// A rewrite response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rewrite objects](struct.ObjectRewriteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RewriteResponse {
    /// The total size of the object being copied in bytes. This property is always present in the response.
    #[serde(rename="objectSize")]
    pub object_size: Option<String>,
    /// The kind of item this is.
    pub kind: Option<String>,
    /// A resource containing the metadata for the copied-to object. This property is present in the response only when copying completes.
    pub resource: Option<Object>,
    /// true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response.
    pub done: Option<bool>,
    /// The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response.
    #[serde(rename="totalBytesRewritten")]
    pub total_bytes_rewritten: Option<String>,
    /// A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy.
    #[serde(rename="rewriteToken")]
    pub rewrite_token: Option<String>,
}

impl ResponseResult for RewriteResponse {}


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
    /// The access permission for the entity.
    pub role: Option<String>,
    /// The ID for the entity, if any.
    #[serde(rename="entityId")]
    pub entity_id: Option<String>,
    /// The project team associated with the entity, if any.
    #[serde(rename="projectTeam")]
    pub project_team: Option<BucketAccessControlProjectTeam>,
    /// The ID of the access-control entry.
    pub id: Option<String>,
    /// The link to this access-control entry.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for BucketAccessControl {}
impl Resource for BucketAccessControl {}
impl ResponseResult for BucketAccessControl {}


/// An association between a role, which comes with a set of permissions, and members who may assume that role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyBindings {
    /// The role to which members belong. Two types of roles are supported: new IAM roles, which grant permissions that do not map directly to those provided by ACLs, and legacy IAM roles, which do map directly to ACL permissions. All roles are of the format roles/storage.specificRole.
    /// The new IAM roles are:  
    /// - roles/storage.admin — Full control of Google Cloud Storage resources.  
    /// - roles/storage.objectViewer — Read-Only access to Google Cloud Storage objects.  
    /// - roles/storage.objectCreator — Access to create objects in Google Cloud Storage.  
    /// - roles/storage.objectAdmin — Full control of Google Cloud Storage objects.   The legacy IAM roles are:  
    /// - roles/storage.legacyObjectReader — Read-only access to objects without listing. Equivalent to an ACL entry on an object with the READER role.  
    /// - roles/storage.legacyObjectOwner — Read/write access to existing objects without listing. Equivalent to an ACL entry on an object with the OWNER role.  
    /// - roles/storage.legacyBucketReader — Read access to buckets with object listing. Equivalent to an ACL entry on a bucket with the READER role.  
    /// - roles/storage.legacyBucketWriter — Read access to buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the WRITER role.  
    /// - roles/storage.legacyBucketOwner — Read and write access to existing buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the OWNER role.
    pub role: Option<String>,
    /// A collection of identifiers for members who may assume the provided role. Recognized identifiers are as follows:  
    /// - allUsers — A special identifier that represents anyone on the internet; with or without a Google account.  
    /// - allAuthenticatedUsers — A special identifier that represents anyone who is authenticated with a Google account or a service account.  
    /// - user:emailid — An email address that represents a specific account. For example, user:alice@gmail.com or user:joe@example.com.  
    /// - serviceAccount:emailid — An email address that represents a service account. For example,  serviceAccount:my-other-app@appspot.gserviceaccount.com .  
    /// - group:emailid — An email address that represents a Google group. For example, group:admins@example.com.  
    /// - domain:domain — A Google Apps domain name that represents all the users of that domain. For example, domain:google.com or domain:example.com.  
    /// - projectOwner:projectid — Owners of the given project. For example, projectOwner:my-example-project  
    /// - projectEditor:projectid — Editors of the given project. For example, projectEditor:my-example-project  
    /// - projectViewer:projectid — Viewers of the given project. For example, projectViewer:my-example-project
    pub members: Option<Vec<String>>,
}

impl NestedType for PolicyBindings {}
impl Part for PolicyBindings {}


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
    #[serde(rename="resourceUri")]
    pub resource_uri: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource. Value: the fixed string "api#channel".
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


/// The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLogging {
    /// A prefix for log object names.
    #[serde(rename="logObjectPrefix")]
    pub log_object_prefix: Option<String>,
    /// The destination bucket where the current bucket's logs should be placed.
    #[serde(rename="logBucket")]
    pub log_bucket: Option<String>,
}

impl NestedType for BucketLogging {}
impl Part for BucketLogging {}


/// A storage.(buckets|objects).testIamPermissions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions buckets](struct.BucketTestIamPermissionCall.html) (response)
/// * [test iam permissions objects](struct.ObjectTestIamPermissionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// The kind of item this is.
    pub kind: Option<String>,
    /// The permissions held by the caller. Permissions are always of the format storage.resource.capability, where resource is one of buckets or objects. The supported permissions are as follows:  
    /// - storage.buckets.delete — Delete bucket.  
    /// - storage.buckets.get — Read bucket metadata.  
    /// - storage.buckets.getIamPolicy — Read bucket IAM policy.  
    /// - storage.buckets.create — Create bucket.  
    /// - storage.buckets.list — List buckets.  
    /// - storage.buckets.setIamPolicy — Update bucket IAM policy.  
    /// - storage.buckets.update — Update bucket metadata.  
    /// - storage.objects.delete — Delete object.  
    /// - storage.objects.get — Read object data and metadata.  
    /// - storage.objects.getIamPolicy — Read object IAM policy.  
    /// - storage.objects.create — Create object.  
    /// - storage.objects.list — List objects.  
    /// - storage.objects.setIamPolicy — Update object IAM policy.  
    /// - storage.objects.update — Update object metadata.
    pub permissions: Option<Vec<String>>,
}

impl ResponseResult for TestIamPermissionsResponse {}


/// The bucket's lifecycle configuration. See lifecycle management for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycle {
    /// A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken.
    pub rule: Option<Vec<BucketLifecycleRule>>,
}

impl NestedType for BucketLifecycle {}
impl Part for BucketLifecycle {}


/// A Compose request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [compose objects](struct.ObjectComposeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequest {
    /// The kind of item this is.
    pub kind: Option<String>,
    /// Properties of the resulting object.
    pub destination: Option<Object>,
    /// The list of source objects that will be concatenated into a single object.
    #[serde(rename="sourceObjects")]
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Buckets {
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of items.
    pub items: Option<Vec<Bucket>>,
    /// The kind of item this is. For lists of buckets, this is always storage#buckets.
    pub kind: Option<String>,
}

impl ResponseResult for Buckets {}


/// An object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy objects](struct.ObjectGetIamPolicyCall.html) (none)
/// * [list objects](struct.ObjectListCall.html) (none)
/// * [rewrite objects](struct.ObjectRewriteCall.html) (request)
/// * [test iam permissions objects](struct.ObjectTestIamPermissionCall.html) (none)
/// * [copy objects](struct.ObjectCopyCall.html) (request|response)
/// * [watch all objects](struct.ObjectWatchAllCall.html) (none)
/// * [get objects](struct.ObjectGetCall.html) (response)
/// * [set iam policy objects](struct.ObjectSetIamPolicyCall.html) (none)
/// * [insert objects](struct.ObjectInsertCall.html) (request|response)
/// * [compose objects](struct.ObjectComposeCall.html) (response)
/// * [update objects](struct.ObjectUpdateCall.html) (request|response)
/// * [delete objects](struct.ObjectDeleteCall.html) (none)
/// * [patch objects](struct.ObjectPatchCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    /// The link to this object.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// The modification time of the object metadata in RFC 3339 format.
    pub updated: Option<String>,
    /// Content-Type of the object data. If contentType is not specified, object downloads will be served as application/octet-stream.
    #[serde(rename="contentType")]
    pub content_type: Option<String>,
    /// The creation time of the object in RFC 3339 format.
    #[serde(rename="timeCreated")]
    pub time_created: Option<String>,
    /// The content generation of this object. Used for object versioning.
    pub generation: Option<String>,
    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    #[serde(rename="customerEncryption")]
    pub customer_encryption: Option<ObjectCustomerEncryption>,
    /// Number of underlying components that make up this object. Components are accumulated by compose operations.
    #[serde(rename="componentCount")]
    pub component_count: Option<i32>,
    /// The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated.
    #[serde(rename="timeStorageClassUpdated")]
    pub time_storage_class_updated: Option<String>,
    /// The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object.
    pub metageneration: Option<String>,
    /// Media download link.
    #[serde(rename="mediaLink")]
    pub media_link: Option<String>,
    /// The owner of the object. This will always be the uploader of the object.
    pub owner: Option<ObjectOwner>,
    /// Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600.
    #[serde(rename="cacheControl")]
    pub cache_control: Option<String>,
    /// Access controls on the object.
    pub acl: Option<Vec<ObjectAccessControl>>,
    /// The ID of the object, including the bucket name, object name, and generation number.
    pub id: Option<String>,
    /// Content-Length of the data in bytes.
    pub size: Option<String>,
    /// The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted.
    #[serde(rename="timeDeleted")]
    pub time_deleted: Option<String>,
    /// The kind of item this is. For objects, this is always storage#object.
    pub kind: Option<String>,
    /// Content-Language of the object data.
    #[serde(rename="contentLanguage")]
    pub content_language: Option<String>,
    /// The name of the object. Required if not specified by URL parameter.
    pub name: Option<String>,
    /// MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see Hashes and ETags: Best Practices.
    #[serde(rename="md5Hash")]
    pub md5_hash: Option<String>,
    /// The name of the bucket containing this object.
    pub bucket: Option<String>,
    /// Content-Encoding of the object data.
    #[serde(rename="contentEncoding")]
    pub content_encoding: Option<String>,
    /// CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best Practices.
    pub crc32c: Option<String>,
    /// HTTP 1.1 Entity tag for the object.
    pub etag: Option<String>,
    /// Storage class of the object.
    #[serde(rename="storageClass")]
    pub storage_class: Option<String>,
    /// Content-Disposition of the object data.
    #[serde(rename="contentDisposition")]
    pub content_disposition: Option<String>,
    /// User-provided metadata, in key/value pairs.
    pub metadata: Option<HashMap<String, String>>,
}

impl RequestValue for Object {}
impl Resource for Object {}
impl ResponseResult for Object {}


/// The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketWebsite {
    /// If the requested object path is missing, and any mainPageSuffix object is missing, if applicable, the service will return the named object from this bucket as the content for a 404 Not Found result.
    #[serde(rename="notFoundPage")]
    pub not_found_page: Option<String>,
    /// If the requested object path is missing, the service will ensure the path has a trailing '/', append this suffix, and attempt to retrieve the resulting object. This allows the creation of index.html objects to represent directory pages.
    #[serde(rename="mainPageSuffix")]
    pub main_page_suffix: Option<String>,
}

impl NestedType for BucketWebsite {}
impl Part for BucketWebsite {}


/// The owner of the object. This will always be the uploader of the object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectOwner {
    /// The ID for the entity.
    #[serde(rename="entityId")]
    pub entity_id: Option<String>,
    /// The entity, in the form user-userId.
    pub entity: Option<String>,
}

impl NestedType for ObjectOwner {}
impl Part for ObjectOwner {}


/// The owner of the bucket. This is always the project team's owner group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketOwner {
    /// The ID for the entity.
    #[serde(rename="entityId")]
    pub entity_id: Option<String>,
    /// The entity, in the form project-owner-projectId.
    pub entity: Option<String>,
}

impl NestedType for BucketOwner {}
impl Part for BucketOwner {}


/// A list of notification subscriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list notifications](struct.NotificationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notifications {
    /// The list of items.
    pub items: Option<Vec<Notification>>,
    /// The kind of item this is. For lists of notifications, this is always storage#notifications.
    pub kind: Option<String>,
}

impl ResponseResult for Notifications {}


/// A list of objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list objects](struct.ObjectListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Objects {
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of items.
    pub items: Option<Vec<Object>>,
    /// The kind of item this is. For lists of objects, this is always storage#objects.
    pub kind: Option<String>,
    /// The list of prefixes of objects matching-but-not-listed up to and including the requested delimiter.
    pub prefixes: Option<Vec<String>>,
}

impl ResponseResult for Objects {}


/// The bucket's billing configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketBilling {
    /// When set to true, bucket is requester pays.
    #[serde(rename="requesterPays")]
    pub requester_pays: Option<bool>,
}

impl NestedType for BucketBilling {}
impl Part for BucketBilling {}


/// A subscription to receive Google PubSub notifications.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert notifications](struct.NotificationInsertCall.html) (request|response)
/// * [list notifications](struct.NotificationListCall.html) (none)
/// * [delete notifications](struct.NotificationDeleteCall.html) (none)
/// * [get notifications](struct.NotificationGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// The kind of item this is. For notifications, this is always storage#notification.
    pub kind: Option<String>,
    /// An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription.
    pub custom_attributes: Option<HashMap<String, String>>,
    /// The ID of the notification.
    pub id: Option<String>,
    /// The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'
    pub topic: Option<String>,
    /// HTTP 1.1 Entity tag for this subscription notification.
    pub etag: Option<String>,
    /// If present, only send notifications about listed event types. If empty, sent notifications for all event types.
    pub event_types: Option<Vec<String>>,
    /// If present, only apply this notification configuration to object names that begin with this prefix.
    pub object_name_prefix: Option<String>,
    /// The desired content of the Payload.
    pub payload_format: Option<String>,
    /// The canonical URL of this notification.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl RequestValue for Notification {}
impl Resource for Notification {}
impl ResponseResult for Notification {}


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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAccessControls {
    /// The list of items.
    pub items: Option<Vec<ObjectAccessControl>>,
    /// The kind of item this is. For lists of object access control entries, this is always storage#objectAccessControls.
    pub kind: Option<String>,
}

impl ResponseResult for ObjectAccessControls {}


/// The project team associated with the entity, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAccessControlProjectTeam {
    /// The project number.
    #[serde(rename="projectNumber")]
    pub project_number: Option<String>,
    /// The team.
    pub team: Option<String>,
}

impl NestedType for BucketAccessControlProjectTeam {}
impl Part for BucketAccessControlProjectTeam {}


/// The bucket's versioning configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketVersioning {
    /// While set to true, versioning is fully enabled for this bucket.
    pub enabled: Option<bool>,
}

impl NestedType for BucketVersioning {}
impl Part for BucketVersioning {}



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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.default_object_access_controls();
/// # }
/// ```
pub struct DefaultObjectAccessControlMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
}

impl<'a, C, A> MethodsBuilder for DefaultObjectAccessControlMethods<'a, C, A> {}

impl<'a, C, A> DefaultObjectAccessControlMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn insert(&self, request: ObjectAccessControl, bucket: &str) -> DefaultObjectAccessControlInsertCall<'a, C, A> {
        DefaultObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves default object ACL entries on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn list(&self, bucket: &str) -> DefaultObjectAccessControlListCall<'a, C, A> {
        DefaultObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlPatchCall<'a, C, A> {
        DefaultObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlDeleteCall<'a, C, A> {
        DefaultObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlUpdateCall<'a, C, A> {
        DefaultObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the default object ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlGetCall<'a, C, A> {
        DefaultObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.bucket_access_controls();
/// # }
/// ```
pub struct BucketAccessControlMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
}

impl<'a, C, A> MethodsBuilder for BucketAccessControlMethods<'a, C, A> {}

impl<'a, C, A> BucketAccessControlMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified bucket. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlPatchCall<'a, C, A> {
        BucketAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, entity: &str) -> BucketAccessControlDeleteCall<'a, C, A> {
        BucketAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn insert(&self, request: BucketAccessControl, bucket: &str) -> BucketAccessControlInsertCall<'a, C, A> {
        BucketAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, entity: &str) -> BucketAccessControlGetCall<'a, C, A> {
        BucketAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlUpdateCall<'a, C, A> {
        BucketAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn list(&self, bucket: &str) -> BucketAccessControlListCall<'a, C, A> {
        BucketAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
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



/// A builder providing access to all methods supported on *notification* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.notifications();
/// # }
/// ```
pub struct NotificationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
}

impl<'a, C, A> MethodsBuilder for NotificationMethods<'a, C, A> {}

impl<'a, C, A> NotificationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a notification subscription for a given bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - The parent bucket of the notification.
    pub fn insert(&self, request: Notification, bucket: &str) -> NotificationInsertCall<'a, C, A> {
        NotificationInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a notification subscription.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - The parent bucket of the notification.
    /// * `notification` - ID of the notification to delete.
    pub fn delete(&self, bucket: &str, notification: &str) -> NotificationDeleteCall<'a, C, A> {
        NotificationDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _notification: notification.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// View a notification configuration.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - The parent bucket of the notification.
    /// * `notification` - Notification ID
    pub fn get(&self, bucket: &str, notification: &str) -> NotificationGetCall<'a, C, A> {
        NotificationGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _notification: notification.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of notification subscriptions for a given bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a GCS bucket.
    pub fn list(&self, bucket: &str) -> NotificationListCall<'a, C, A> {
        NotificationListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `compose(...)`, `copy(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `rewrite(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)`, `update(...)` and `watch_all(...)`
/// // to build up your call.
/// let rb = hub.objects();
/// # }
/// ```
pub struct ObjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
}

impl<'a, C, A> MethodsBuilder for ObjectMethods<'a, C, A> {}

impl<'a, C, A> ObjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rewrites a source object to a destination object. Optionally overrides metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceBucket` - Name of the bucket in which to find the source object.
    /// * `sourceObject` - Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationBucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    /// * `destinationObject` - Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn rewrite(&self, request: Object, source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str) -> ObjectRewriteCall<'a, C, A> {
        ObjectRewriteCall {
            hub: self.hub,
            _request: request,
            _source_bucket: source_bucket.to_string(),
            _source_object: source_object.to_string(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
            _source_generation: Default::default(),
            _rewrite_token: Default::default(),
            _projection: Default::default(),
            _max_bytes_rewritten_per_call: Default::default(),
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
    /// Retrieves an object or its metadata.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn get(&self, bucket: &str, object: &str) -> ObjectGetCall<'a, C, A> {
        ObjectGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which to look for objects.
    pub fn watch_all(&self, request: Channel, bucket: &str) -> ObjectWatchAllCall<'a, C, A> {
        ObjectWatchAllCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _user_project: Default::default(),
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
    /// Updates an IAM policy for the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn set_iam_policy(&self, request: Policy, bucket: &str, object: &str) -> ObjectSetIamPolicyCall<'a, C, A> {
        ObjectSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an IAM policy for the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn get_iam_policy(&self, bucket: &str, object: &str) -> ObjectGetIamPolicyCall<'a, C, A> {
        ObjectGetIamPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an object's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn update(&self, request: Object, bucket: &str, object: &str) -> ObjectUpdateCall<'a, C, A> {
        ObjectUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    pub fn insert(&self, request: Object, bucket: &str) -> ObjectInsertCall<'a, C, A> {
        ObjectInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `destinationBucket` - Name of the bucket in which to store the new object.
    /// * `destinationObject` - Name of the new object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn compose(&self, request: ComposeRequest, destination_bucket: &str, destination_object: &str) -> ObjectComposeCall<'a, C, A> {
        ObjectComposeCall {
            hub: self.hub,
            _request: request,
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn delete(&self, bucket: &str, object: &str) -> ObjectDeleteCall<'a, C, A> {
        ObjectDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which to look for objects.
    pub fn list(&self, bucket: &str) -> ObjectListCall<'a, C, A> {
        ObjectListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _user_project: Default::default(),
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
    /// Tests a set of permissions on the given object to see which, if any, are held by the caller.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `permissions` - Permissions to test.
    pub fn test_iam_permissions(&self, bucket: &str, object: &str, permissions: &Vec<String>) -> ObjectTestIamPermissionCall<'a, C, A> {
        ObjectTestIamPermissionCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _permissions: permissions.clone(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies a source object to a destination object. Optionally overrides metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceBucket` - Name of the bucket in which to find the source object.
    /// * `sourceObject` - Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationBucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationObject` - Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any.
    pub fn copy(&self, request: Object, source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str) -> ObjectCopyCall<'a, C, A> {
        ObjectCopyCall {
            hub: self.hub,
            _request: request,
            _source_bucket: source_bucket.to_string(),
            _source_object: source_object.to_string(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn patch(&self, request: Object, bucket: &str, object: &str) -> ObjectPatchCall<'a, C, A> {
        ObjectPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.object_access_controls();
/// # }
/// ```
pub struct ObjectAccessControlMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
}

impl<'a, C, A> MethodsBuilder for ObjectAccessControlMethods<'a, C, A> {}

impl<'a, C, A> ObjectAccessControlMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlGetCall<'a, C, A> {
        ObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified object. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlPatchCall<'a, C, A> {
        ObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn list(&self, bucket: &str, object: &str) -> ObjectAccessControlListCall<'a, C, A> {
        ObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlDeleteCall<'a, C, A> {
        ObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlUpdateCall<'a, C, A> {
        ObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn insert(&self, request: ObjectAccessControl, bucket: &str, object: &str) -> ObjectAccessControlInsertCall<'a, C, A> {
        ObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.buckets();
/// # }
/// ```
pub struct BucketMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
}

impl<'a, C, A> MethodsBuilder for BucketMethods<'a, C, A> {}

impl<'a, C, A> BucketMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn patch(&self, request: Bucket, bucket: &str) -> BucketPatchCall<'a, C, A> {
        BucketPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
    /// Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn update(&self, request: Bucket, bucket: &str) -> BucketUpdateCall<'a, C, A> {
        BucketUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn get(&self, bucket: &str) -> BucketGetCall<'a, C, A> {
        BucketGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn delete(&self, bucket: &str) -> BucketDeleteCall<'a, C, A> {
        BucketDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
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
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - A valid API project identifier.
    pub fn insert(&self, request: Bucket, project: &str) -> BucketInsertCall<'a, C, A> {
        BucketInsertCall {
            hub: self.hub,
            _request: request,
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
    /// Tests a set of permissions on the given bucket to see which, if any, are held by the caller.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `permissions` - Permissions to test.
    pub fn test_iam_permissions(&self, bucket: &str, permissions: &Vec<String>) -> BucketTestIamPermissionCall<'a, C, A> {
        BucketTestIamPermissionCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _permissions: permissions.clone(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an IAM policy for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn set_iam_policy(&self, request: Policy, bucket: &str) -> BucketSetIamPolicyCall<'a, C, A> {
        BucketSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an IAM policy for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn get_iam_policy(&self, bucket: &str) -> BucketGetIamPolicyCall<'a, C, A> {
        BucketGetIamPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of buckets for a given project.
    /// 
    /// # Arguments
    ///
    /// * `project` - A valid API project identifier.
    pub fn list(&self, project: &str) -> BucketListCall<'a, C, A> {
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



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use storage1::Storage;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `service_account_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the email address of this project's GCS service account.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID
    pub fn service_account_get(&self, project_id: &str) -> ProjectServiceAccountGetCall<'a, C, A> {
        ProjectServiceAccountGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
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
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().insert(req, "bucket")
///              .user_project("amet")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DefaultObjectAccessControlInsertCall<'a, C, A> {}

impl<'a, C, A> DefaultObjectAccessControlInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.insert",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: ObjectAccessControl) -> DefaultObjectAccessControlInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlInsertCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlInsertCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlInsertCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves default object ACL entries on the specified bucket.
///
/// A builder for the *list* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().list("bucket")
///              .user_project("consetetur")
///              .if_metageneration_not_match("ut")
///              .if_metageneration_match("ea")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _user_project: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DefaultObjectAccessControlListCall<'a, C, A> {}

impl<'a, C, A> DefaultObjectAccessControlListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, only return default ACL listing if the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// If present, only return default ACL listing if the bucket's current metageneration matches this value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a default object ACL entry on the specified bucket. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().patch(req, "bucket", "entity")
///              .user_project("dolor")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DefaultObjectAccessControlPatchCall<'a, C, A> {}

impl<'a, C, A> DefaultObjectAccessControlPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
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
    pub fn request(mut self, new_value: ObjectAccessControl) -> DefaultObjectAccessControlPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *delete* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().delete("bucket", "entity")
///              .user_project("consetetur")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DefaultObjectAccessControlDeleteCall<'a, C, A> {}

impl<'a, C, A> DefaultObjectAccessControlDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a default object ACL entry on the specified bucket.
///
/// A builder for the *update* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().update(req, "bucket", "entity")
///              .user_project("Lorem")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DefaultObjectAccessControlUpdateCall<'a, C, A> {}

impl<'a, C, A> DefaultObjectAccessControlUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
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
    pub fn request(mut self, new_value: ObjectAccessControl) -> DefaultObjectAccessControlUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the default object ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *get* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().get("bucket", "entity")
///              .user_project("sit")
///              .doit();
/// # }
/// ```
pub struct DefaultObjectAccessControlGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for DefaultObjectAccessControlGetCall<'a, C, A> {}

impl<'a, C, A> DefaultObjectAccessControlGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.defaultObjectAccessControls.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> DefaultObjectAccessControlGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an ACL entry on the specified bucket. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::BucketAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BucketAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().patch(req, "bucket", "entity")
///              .user_project("rebum.")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: BucketAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketAccessControlPatchCall<'a, C, A> {}

impl<'a, C, A> BucketAccessControlPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
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
    pub fn request(mut self, new_value: BucketAccessControl) -> BucketAccessControlPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes the ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *delete* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().delete("bucket", "entity")
///              .user_project("vero")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketAccessControlDeleteCall<'a, C, A> {}

impl<'a, C, A> BucketAccessControlDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new ACL entry on the specified bucket.
///
/// A builder for the *insert* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::BucketAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BucketAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().insert(req, "bucket")
///              .user_project("invidunt")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: BucketAccessControl,
    _bucket: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketAccessControlInsertCall<'a, C, A> {}

impl<'a, C, A> BucketAccessControlInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.insert",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: BucketAccessControl) -> BucketAccessControlInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlInsertCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlInsertCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlInsertCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *get* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().get("bucket", "entity")
///              .user_project("duo")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketAccessControlGetCall<'a, C, A> {}

impl<'a, C, A> BucketAccessControlGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlGetCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlGetCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlGetCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an ACL entry on the specified bucket.
///
/// A builder for the *update* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::BucketAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BucketAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().update(req, "bucket", "entity")
///              .user_project("et")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: BucketAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketAccessControlUpdateCall<'a, C, A> {}

impl<'a, C, A> BucketAccessControlUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
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
    pub fn request(mut self, new_value: BucketAccessControl) -> BucketAccessControlUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves ACL entries on the specified bucket.
///
/// A builder for the *list* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().list("bucket")
///              .user_project("consetetur")
///              .doit();
/// # }
/// ```
pub struct BucketAccessControlListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketAccessControlListCall<'a, C, A> {}

impl<'a, C, A> BucketAccessControlListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BucketAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.bucketAccessControls.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlListCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlListCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketAccessControlListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlListCall<'a, C, A>
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
/// # extern crate google_storage1 as storage1;
/// use storage1::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
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

    hub: &'a Storage<C, A>,
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
        dlg.begin(MethodInfo { id: "storage.channels.stop",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
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
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn param<T>(mut self, name: T, value: T) -> ChannelStopCall<'a, C, A>
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


/// Creates a notification subscription for a given bucket.
///
/// A builder for the *insert* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Notification;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Notification::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().insert(req, "bucket")
///              .user_project("nonumy")
///              .doit();
/// # }
/// ```
pub struct NotificationInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Notification,
    _bucket: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for NotificationInsertCall<'a, C, A> {}

impl<'a, C, A> NotificationInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Notification)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.notifications.insert",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Notification) -> NotificationInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The parent bucket of the notification.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationInsertCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationInsertCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> NotificationInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NotificationInsertCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a notification subscription.
///
/// A builder for the *delete* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().delete("bucket", "notification")
///              .user_project("takimata")
///              .doit();
/// # }
/// ```
pub struct NotificationDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _notification: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for NotificationDeleteCall<'a, C, A> {}

impl<'a, C, A> NotificationDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.notifications.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("notification", self._notification.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["bucket", "notification", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs/{notification}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{notification}", "notification")].iter() {
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
            for param_name in ["notification", "bucket"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The parent bucket of the notification.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationDeleteCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// ID of the notification to delete.
    ///
    /// Sets the *notification* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn notification(mut self, new_value: &str) -> NotificationDeleteCall<'a, C, A> {
        self._notification = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationDeleteCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> NotificationDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NotificationDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// View a notification configuration.
///
/// A builder for the *get* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().get("bucket", "notification")
///              .user_project("invidunt")
///              .doit();
/// # }
/// ```
pub struct NotificationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _notification: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for NotificationGetCall<'a, C, A> {}

impl<'a, C, A> NotificationGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Notification)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.notifications.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("notification", self._notification.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "notification", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs/{notification}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{notification}", "notification")].iter() {
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
            for param_name in ["notification", "bucket"].iter() {
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


    /// The parent bucket of the notification.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationGetCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Notification ID
    ///
    /// Sets the *notification* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn notification(mut self, new_value: &str) -> NotificationGetCall<'a, C, A> {
        self._notification = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationGetCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> NotificationGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NotificationGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of notification subscriptions for a given bucket.
///
/// A builder for the *list* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().list("bucket")
///              .user_project("sadipscing")
///              .doit();
/// # }
/// ```
pub struct NotificationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for NotificationListCall<'a, C, A> {}

impl<'a, C, A> NotificationListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Notifications)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.notifications.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Name of a GCS bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationListCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationListCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> NotificationListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> NotificationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Rewrites a source object to a destination object. Optionally overrides metadata.
///
/// A builder for the *rewrite* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Object;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .user_project("aliquyam")
///              .source_generation("sit")
///              .rewrite_token("eirmod")
///              .projection("consetetur")
///              .max_bytes_rewritten_per_call("labore")
///              .if_source_metageneration_not_match("sed")
///              .if_source_metageneration_match("ea")
///              .if_source_generation_not_match("gubergren")
///              .if_source_generation_match("aliquyam")
///              .if_metageneration_not_match("eos")
///              .if_metageneration_match("tempor")
///              .if_generation_not_match("sea")
///              .if_generation_match("labore")
///              .destination_predefined_acl("ipsum")
///              .doit();
/// # }
/// ```
pub struct ObjectRewriteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Object,
    _source_bucket: String,
    _source_object: String,
    _destination_bucket: String,
    _destination_object: String,
    _user_project: Option<String>,
    _source_generation: Option<String>,
    _rewrite_token: Option<String>,
    _projection: Option<String>,
    _max_bytes_rewritten_per_call: Option<String>,
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

impl<'a, C, A> CallBuilder for ObjectRewriteCall<'a, C, A> {}

impl<'a, C, A> ObjectRewriteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RewriteResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.rewrite",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((21 + self._additional_params.len()));
        params.push(("sourceBucket", self._source_bucket.to_string()));
        params.push(("sourceObject", self._source_object.to_string()));
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._source_generation {
            params.push(("sourceGeneration", value.to_string()));
        }
        if let Some(value) = self._rewrite_token {
            params.push(("rewriteToken", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._max_bytes_rewritten_per_call {
            params.push(("maxBytesRewrittenPerCall", value.to_string()));
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
        for &field in ["alt", "sourceBucket", "sourceObject", "destinationBucket", "destinationObject", "userProject", "sourceGeneration", "rewriteToken", "projection", "maxBytesRewrittenPerCall", "ifSourceMetagenerationNotMatch", "ifSourceMetagenerationMatch", "ifSourceGenerationNotMatch", "ifSourceGenerationMatch", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "destinationPredefinedAcl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{sourceBucket}/o/{sourceObject}/rewriteTo/b/{destinationBucket}/o/{destinationObject}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["destinationObject", "destinationBucket", "sourceObject", "sourceBucket"].iter() {
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
    pub fn request(mut self, new_value: Object) -> ObjectRewriteCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to find the source object.
    ///
    /// Sets the *source bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_bucket(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._source_bucket = new_value.to_string();
        self
    }
    /// Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *source object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_object(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._source_object = new_value.to_string();
        self
    }
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    ///
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_object(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._destination_object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of the source object (as opposed to the latest version, the default).
    ///
    /// Sets the *source generation* query property to the given value.
    pub fn source_generation(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._source_generation = Some(new_value.to_string());
        self
    }
    /// Include this field (from the previous rewrite response) on each rewrite request after the first one, until the rewrite response 'done' flag is true. Calls that provide a rewriteToken can omit all other request fields, but if included those fields must match the values provided in the first rewrite request.
    ///
    /// Sets the *rewrite token* query property to the given value.
    pub fn rewrite_token(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._rewrite_token = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// The maximum number of bytes that will be rewritten per rewrite request. Most callers shouldn't need to specify this parameter - it is primarily in place to support testing. If specified the value must be an integral multiple of 1 MiB (1048576). Also, this only applies to requests where the source and destination span locations and/or storage classes. Finally, this value must not change across rewrite calls else you'll get an error that the rewriteToken is invalid.
    ///
    /// Sets the *max bytes rewritten per call* query property to the given value.
    pub fn max_bytes_rewritten_per_call(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._max_bytes_rewritten_per_call = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration does not match the given value.
    ///
    /// Sets the *if source metageneration not match* query property to the given value.
    pub fn if_source_metageneration_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_source_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration matches the given value.
    ///
    /// Sets the *if source metageneration match* query property to the given value.
    pub fn if_source_metageneration_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_source_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's generation does not match the given value.
    ///
    /// Sets the *if source generation not match* query property to the given value.
    pub fn if_source_generation_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_source_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's generation matches the given value.
    ///
    /// Sets the *if source generation match* query property to the given value.
    pub fn if_source_generation_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_source_generation_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current generation does not match the given value.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to the destination object.
    ///
    /// Sets the *destination predefined acl* query property to the given value.
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectRewriteCall<'a, C, A> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectRewriteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectRewriteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectRewriteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
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
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().get("bucket", "object")
///              .user_project("sit")
///              .projection("diam")
///              .if_metageneration_not_match("ut")
///              .if_metageneration_match("justo")
///              .if_generation_not_match("est")
///              .if_generation_match("amet")
///              .generation("accusam")
///              .doit();
/// # }
/// ```
pub struct ObjectGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
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

impl<'a, C, A> CallBuilder for ObjectGetCall<'a, C, A> {}

impl<'a, C, A> ObjectGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
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
        for &field in ["bucket", "object", "userProject", "projection", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
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

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's generation does not match the given value.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectGetCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Watch for changes on all objects in a bucket.
///
/// A builder for the *watchAll* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().watch_all(req, "bucket")
///              .versions(false)
///              .user_project("justo")
///              .projection("est")
///              .prefix("clita")
///              .page_token("invidunt")
///              .max_results(14)
///              .delimiter("dolores")
///              .doit();
/// # }
/// ```
pub struct ObjectWatchAllCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Channel,
    _bucket: String,
    _versions: Option<bool>,
    _user_project: Option<String>,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delimiter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectWatchAllCall<'a, C, A> {}

impl<'a, C, A> ObjectWatchAllCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.watchAll",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((11 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._versions {
            params.push(("versions", value.to_string()));
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
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
        for &field in ["alt", "bucket", "versions", "userProject", "projection", "prefix", "pageToken", "maxResults", "delimiter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/watch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Channel) -> ObjectWatchAllCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to look for objects.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning.
    ///
    /// Sets the *versions* query property to the given value.
    pub fn versions(mut self, new_value: bool) -> ObjectWatchAllCall<'a, C, A> {
        self._versions = Some(new_value);
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Filter results to objects whose names begin with this prefix.
    ///
    /// Sets the *prefix* query property to the given value.
    pub fn prefix(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, A> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// A previously-returned page token representing part of the larger set of results to view.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ObjectWatchAllCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted.
    ///
    /// Sets the *delimiter* query property to the given value.
    pub fn delimiter(mut self, new_value: &str) -> ObjectWatchAllCall<'a, C, A> {
        self._delimiter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectWatchAllCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectWatchAllCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectWatchAllCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an IAM policy for the specified object.
///
/// A builder for the *setIamPolicy* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Policy;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Policy::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().set_iam_policy(req, "bucket", "object")
///              .user_project("duo")
///              .generation("sed")
///              .doit();
/// # }
/// ```
pub struct ObjectSetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Policy,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectSetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> ObjectSetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.setIamPolicy",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
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
    pub fn request(mut self, new_value: Policy) -> ObjectSetIamPolicyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectSetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectSetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectSetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns an IAM policy for the specified object.
///
/// A builder for the *getIamPolicy* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().get_iam_policy("bucket", "object")
///              .user_project("ea")
///              .generation("et")
///              .doit();
/// # }
/// ```
pub struct ObjectGetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectGetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> ObjectGetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.getIamPolicy",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectGetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectGetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectGetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
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
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Object;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().update(req, "bucket", "object")
///              .user_project("kasd")
///              .projection("invidunt")
///              .predefined_acl("rebum.")
///              .if_metageneration_not_match("Lorem")
///              .if_metageneration_match("clita")
///              .if_generation_not_match("invidunt")
///              .if_generation_match("eirmod")
///              .generation("At")
///              .doit();
/// # }
/// ```
pub struct ObjectUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Object,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
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

impl<'a, C, A> CallBuilder for ObjectUpdateCall<'a, C, A> {}

impl<'a, C, A> ObjectUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
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
        for &field in ["bucket", "object", "userProject", "projection", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
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

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
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
    pub fn request(mut self, new_value: Object) -> ObjectUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this object.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectUpdateCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
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
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Object;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().insert(req, "bucket")
///              .user_project("et")
///              .projection("sed")
///              .predefined_acl("sit")
///              .name("takimata")
///              .if_metageneration_not_match("elitr")
///              .if_metageneration_match("nonumy")
///              .if_generation_not_match("rebum.")
///              .if_generation_match("Lorem")
///              .content_encoding("Lorem")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct ObjectInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Object,
    _bucket: String,
    _user_project: Option<String>,
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

impl<'a, C, A> CallBuilder for ObjectInsertCall<'a, C, A> {}

impl<'a, C, A> ObjectInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::client::Response, Object)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.insert",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
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
        for &field in ["bucket", "userProject", "projection", "predefinedAcl", "name", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "contentEncoding"].iter() {
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

        let (mut url, upload_type) =
            if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/storage/v1/b/{bucket}/o", "multipart")
            } else if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/storage/v1/b/{bucket}/o", "resumable")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
                        
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, ContentType(mime_type))
                        },
                        _ => (&mut request_value_reader as &mut io::Read, ContentType(json_mime_type.clone())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
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
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 0kb
    /// * *multipart*: yes
    /// * *valid mime types*: '*/*'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, Object)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Object) -> ObjectInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this object.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Name of the object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If set, sets the contentEncoding property of the final object to this value. Setting this parameter is equivalent to setting the contentEncoding metadata property. This can be useful when uploading an object with uploadType=media to indicate the encoding of the content being uploaded.
    ///
    /// Sets the *content encoding* query property to the given value.
    pub fn content_encoding(mut self, new_value: &str) -> ObjectInsertCall<'a, C, A> {
        self._content_encoding = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectInsertCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
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
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::ComposeRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ComposeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().compose(req, "destinationBucket", "destinationObject")
///              .user_project("ut")
///              .if_metageneration_match("amet.")
///              .if_generation_match("ipsum")
///              .destination_predefined_acl("ut")
///              .doit();
/// # }
/// ```
pub struct ObjectComposeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: ComposeRequest,
    _destination_bucket: String,
    _destination_object: String,
    _user_project: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_match: Option<String>,
    _destination_predefined_acl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectComposeCall<'a, C, A> {}

impl<'a, C, A> ObjectComposeCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.compose",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._destination_predefined_acl {
            params.push(("destinationPredefinedAcl", value.to_string()));
        }
        for &field in ["destinationBucket", "destinationObject", "userProject", "ifMetagenerationMatch", "ifGenerationMatch", "destinationPredefinedAcl"].iter() {
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

        let mut url = self.hub._base_url.clone() + "b/{destinationBucket}/o/{destinationObject}/compose";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["destinationObject", "destinationBucket"].iter() {
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
    pub fn request(mut self, new_value: ComposeRequest) -> ObjectComposeCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to store the new object.
    ///
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectComposeCall<'a, C, A> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Name of the new object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_object(mut self, new_value: &str) -> ObjectComposeCall<'a, C, A> {
        self._destination_object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectComposeCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectComposeCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectComposeCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to the destination object.
    ///
    /// Sets the *destination predefined acl* query property to the given value.
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectComposeCall<'a, C, A> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectComposeCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectComposeCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectComposeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.
///
/// A builder for the *delete* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().delete("bucket", "object")
///              .user_project("ut")
///              .if_metageneration_not_match("eirmod")
///              .if_metageneration_match("sanctus")
///              .if_generation_not_match("voluptua.")
///              .if_generation_match("dolor")
///              .generation("et")
///              .doit();
/// # }
/// ```
pub struct ObjectDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectDeleteCall<'a, C, A> {}

impl<'a, C, A> ObjectDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
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
        for &field in ["bucket", "object", "userProject", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, permanently deletes a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectDeleteCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of objects matching the criteria.
///
/// A builder for the *list* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().list("bucket")
///              .versions(true)
///              .user_project("ut")
///              .projection("sed")
///              .prefix("et")
///              .page_token("ipsum")
///              .max_results(81)
///              .delimiter("dolore")
///              .doit();
/// # }
/// ```
pub struct ObjectListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _versions: Option<bool>,
    _user_project: Option<String>,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delimiter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectListCall<'a, C, A> {}

impl<'a, C, A> ObjectListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Objects)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._versions {
            params.push(("versions", value.to_string()));
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
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
        for &field in ["alt", "bucket", "versions", "userProject", "projection", "prefix", "pageToken", "maxResults", "delimiter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Name of the bucket in which to look for objects.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectListCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning.
    ///
    /// Sets the *versions* query property to the given value.
    pub fn versions(mut self, new_value: bool) -> ObjectListCall<'a, C, A> {
        self._versions = Some(new_value);
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectListCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectListCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Filter results to objects whose names begin with this prefix.
    ///
    /// Sets the *prefix* query property to the given value.
    pub fn prefix(mut self, new_value: &str) -> ObjectListCall<'a, C, A> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// A previously-returned page token representing part of the larger set of results to view.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ObjectListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ObjectListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted.
    ///
    /// Sets the *delimiter* query property to the given value.
    pub fn delimiter(mut self, new_value: &str) -> ObjectListCall<'a, C, A> {
        self._delimiter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Tests a set of permissions on the given object to see which, if any, are held by the caller.
///
/// A builder for the *testIamPermissions* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().test_iam_permissions("bucket", "object", "permissions")
///              .user_project("et")
///              .generation("nonumy")
///              .doit();
/// # }
/// ```
pub struct ObjectTestIamPermissionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _object: String,
    _permissions: Vec<String>,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectTestIamPermissionCall<'a, C, A> {}

impl<'a, C, A> ObjectTestIamPermissionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TestIamPermissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.testIamPermissions",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if self._permissions.len() > 0 {
            for f in self._permissions.iter() {
                params.push(("permissions", f.to_string()));
            }
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "permissions", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/iam/testPermissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// Permissions to test.
    ///
    /// Append the given value to the *permissions* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_permissions(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a, C, A> {
        self._permissions.push(new_value.to_string());
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectTestIamPermissionCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectTestIamPermissionCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectTestIamPermissionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Copies a source object to a destination object. Optionally overrides metadata.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Object` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *copy* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Object;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().copy(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .user_project("rebum.")
///              .source_generation("labore")
///              .projection("aliquyam")
///              .if_source_metageneration_not_match("elitr")
///              .if_source_metageneration_match("consetetur")
///              .if_source_generation_not_match("sea")
///              .if_source_generation_match("elitr")
///              .if_metageneration_not_match("At")
///              .if_metageneration_match("sea")
///              .if_generation_not_match("consetetur")
///              .if_generation_match("diam")
///              .destination_predefined_acl("accusam")
///              .doit();
/// # }
/// ```
pub struct ObjectCopyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Object,
    _source_bucket: String,
    _source_object: String,
    _destination_bucket: String,
    _destination_object: String,
    _user_project: Option<String>,
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

impl<'a, C, A> CallBuilder for ObjectCopyCall<'a, C, A> {}

impl<'a, C, A> ObjectCopyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.copy",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((18 + self._additional_params.len()));
        params.push(("sourceBucket", self._source_bucket.to_string()));
        params.push(("sourceObject", self._source_object.to_string()));
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
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
        for &field in ["sourceBucket", "sourceObject", "destinationBucket", "destinationObject", "userProject", "sourceGeneration", "projection", "ifSourceMetagenerationNotMatch", "ifSourceMetagenerationMatch", "ifSourceGenerationNotMatch", "ifSourceGenerationMatch", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "destinationPredefinedAcl"].iter() {
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

        let mut url = self.hub._base_url.clone() + "b/{sourceBucket}/o/{sourceObject}/copyTo/b/{destinationBucket}/o/{destinationObject}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["destinationObject", "destinationBucket", "sourceObject", "sourceBucket"].iter() {
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
    pub fn request(mut self, new_value: Object) -> ObjectCopyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to find the source object.
    ///
    /// Sets the *source bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_bucket(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._source_bucket = new_value.to_string();
        self
    }
    /// Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *source object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_object(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._source_object = new_value.to_string();
        self
    }
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any.
    ///
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_object(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._destination_object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of the source object (as opposed to the latest version, the default).
    ///
    /// Sets the *source generation* query property to the given value.
    pub fn source_generation(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._source_generation = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration does not match the given value.
    ///
    /// Sets the *if source metageneration not match* query property to the given value.
    pub fn if_source_metageneration_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_source_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration matches the given value.
    ///
    /// Sets the *if source metageneration match* query property to the given value.
    pub fn if_source_metageneration_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_source_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's generation does not match the given value.
    ///
    /// Sets the *if source generation not match* query property to the given value.
    pub fn if_source_generation_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_source_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's generation matches the given value.
    ///
    /// Sets the *if source generation match* query property to the given value.
    pub fn if_source_generation_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_source_generation_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current generation does not match the given value.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to the destination object.
    ///
    /// Sets the *destination predefined acl* query property to the given value.
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectCopyCall<'a, C, A> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectCopyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectCopyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectCopyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an object's metadata. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Object;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().patch(req, "bucket", "object")
///              .user_project("dolor")
///              .projection("aliquyam")
///              .predefined_acl("elitr")
///              .if_metageneration_not_match("ea")
///              .if_metageneration_match("et")
///              .if_generation_not_match("Stet")
///              .if_generation_match("sed")
///              .generation("dolor")
///              .doit();
/// # }
/// ```
pub struct ObjectPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Object,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
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

impl<'a, C, A> CallBuilder for ObjectPatchCall<'a, C, A> {}

impl<'a, C, A> ObjectPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objects.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((13 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
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
        for &field in ["alt", "bucket", "object", "userProject", "projection", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
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
    pub fn request(mut self, new_value: Object) -> ObjectPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this object.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectPatchCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the ACL entry for the specified entity on the specified object.
///
/// A builder for the *get* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().get("bucket", "object", "entity")
///              .user_project("consetetur")
///              .generation("consetetur")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectAccessControlGetCall<'a, C, A> {}

impl<'a, C, A> ObjectAccessControlGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "object", "bucket"].iter() {
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an ACL entry on the specified object. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().patch(req, "bucket", "object", "entity")
///              .user_project("et")
///              .generation("sadipscing")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectAccessControlPatchCall<'a, C, A> {}

impl<'a, C, A> ObjectAccessControlPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "object", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
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
    pub fn request(mut self, new_value: ObjectAccessControl) -> ObjectAccessControlPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves ACL entries on the specified object.
///
/// A builder for the *list* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().list("bucket", "object")
///              .user_project("Lorem")
///              .generation("rebum.")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectAccessControlListCall<'a, C, A> {}

impl<'a, C, A> ObjectAccessControlListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlListCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlListCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlListCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlListCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes the ACL entry for the specified entity on the specified object.
///
/// A builder for the *delete* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().delete("bucket", "object", "entity")
///              .user_project("dolores")
///              .generation("vero")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectAccessControlDeleteCall<'a, C, A> {}

impl<'a, C, A> ObjectAccessControlDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "entity", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "object", "bucket"].iter() {
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an ACL entry on the specified object.
///
/// A builder for the *update* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().update(req, "bucket", "object", "entity")
///              .user_project("eos")
///              .generation("justo")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectAccessControlUpdateCall<'a, C, A> {}

impl<'a, C, A> ObjectAccessControlUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["entity", "object", "bucket"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
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
    pub fn request(mut self, new_value: ObjectAccessControl) -> ObjectAccessControlUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, A> {
        self._entity = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new ACL entry on the specified object.
///
/// A builder for the *insert* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::ObjectAccessControl;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().insert(req, "bucket", "object")
///              .user_project("dolore")
///              .generation("amet.")
///              .doit();
/// # }
/// ```
pub struct ObjectAccessControlInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ObjectAccessControlInsertCall<'a, C, A> {}

impl<'a, C, A> ObjectAccessControlInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.objectAccessControls.insert",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
            for param_name in ["object", "bucket"].iter() {
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
    pub fn request(mut self, new_value: ObjectAccessControl) -> ObjectAccessControlInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a, C, A> {
        self._object = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a, C, A> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ObjectAccessControlInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlInsertCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Bucket;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Bucket::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().patch(req, "bucket")
///              .user_project("magna")
///              .projection("elitr")
///              .predefined_default_object_acl("magna")
///              .predefined_acl("ipsum")
///              .if_metageneration_not_match("invidunt")
///              .if_metageneration_match("accusam")
///              .doit();
/// # }
/// ```
pub struct BucketPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Bucket,
    _bucket: String,
    _user_project: Option<String>,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketPatchCall<'a, C, A> {}

impl<'a, C, A> BucketPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
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
        for &field in ["alt", "bucket", "userProject", "projection", "predefinedDefaultObjectAcl", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, &url)
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
    pub fn request(mut self, new_value: Bucket) -> BucketPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketPatchCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketPatchCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketPatchCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of default object access controls to this bucket.
    ///
    /// Sets the *predefined default object acl* query property to the given value.
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketPatchCall<'a, C, A> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this bucket.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> BucketPatchCall<'a, C, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketPatchCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketPatchCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
///
/// A builder for the *update* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Bucket;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Bucket::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().update(req, "bucket")
///              .user_project("diam")
///              .projection("nonumy")
///              .predefined_default_object_acl("sed")
///              .predefined_acl("diam")
///              .if_metageneration_not_match("magna")
///              .if_metageneration_match("dolor")
///              .doit();
/// # }
/// ```
pub struct BucketUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Bucket,
    _bucket: String,
    _user_project: Option<String>,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketUpdateCall<'a, C, A> {}

impl<'a, C, A> BucketUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
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
        for &field in ["alt", "bucket", "userProject", "projection", "predefinedDefaultObjectAcl", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
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
    pub fn request(mut self, new_value: Bucket) -> BucketUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketUpdateCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketUpdateCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketUpdateCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of default object access controls to this bucket.
    ///
    /// Sets the *predefined default object acl* query property to the given value.
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketUpdateCall<'a, C, A> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this bucket.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> BucketUpdateCall<'a, C, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketUpdateCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketUpdateCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketUpdateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns metadata for the specified bucket.
///
/// A builder for the *get* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().get("bucket")
///              .user_project("dolor")
///              .projection("vero")
///              .if_metageneration_not_match("nonumy")
///              .if_metageneration_match("takimata")
///              .doit();
/// # }
/// ```
pub struct BucketGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _user_project: Option<String>,
    _projection: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketGetCall<'a, C, A> {}

impl<'a, C, A> BucketGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "projection", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketGetCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketGetCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketGetCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketGetCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketGetCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes an empty bucket.
///
/// A builder for the *delete* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().delete("bucket")
///              .user_project("consetetur")
///              .if_metageneration_not_match("erat")
///              .if_metageneration_match("amet.")
///              .doit();
/// # }
/// ```
pub struct BucketDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _user_project: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketDeleteCall<'a, C, A> {}

impl<'a, C, A> BucketDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["bucket", "userProject", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketDeleteCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketDeleteCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If set, only deletes the bucket if its metageneration does not match this value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketDeleteCall<'a, C, A> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// If set, only deletes the bucket if its metageneration matches this value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketDeleteCall<'a, C, A> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new bucket.
///
/// A builder for the *insert* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Bucket;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Bucket::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().insert(req, "project")
///              .projection("dolores")
///              .predefined_default_object_acl("et")
///              .predefined_acl("sed")
///              .doit();
/// # }
/// ```
pub struct BucketInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Bucket,
    _project: String,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketInsertCall<'a, C, A> {}

impl<'a, C, A> BucketInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
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
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: Bucket) -> BucketInsertCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// A valid API project identifier.
    ///
    /// Sets the *project* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> BucketInsertCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the bucket resource specifies acl or defaultObjectAcl properties, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketInsertCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of default object access controls to this bucket.
    ///
    /// Sets the *predefined default object acl* query property to the given value.
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketInsertCall<'a, C, A> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this bucket.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> BucketInsertCall<'a, C, A> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketInsertCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketInsertCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketInsertCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Tests a set of permissions on the given bucket to see which, if any, are held by the caller.
///
/// A builder for the *testIamPermissions* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().test_iam_permissions("bucket", "permissions")
///              .user_project("nonumy")
///              .doit();
/// # }
/// ```
pub struct BucketTestIamPermissionCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _permissions: Vec<String>,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketTestIamPermissionCall<'a, C, A> {}

impl<'a, C, A> BucketTestIamPermissionCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TestIamPermissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.testIamPermissions",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if self._permissions.len() > 0 {
            for f in self._permissions.iter() {
                params.push(("permissions", f.to_string()));
            }
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "permissions", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/iam/testPermissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketTestIamPermissionCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// Permissions to test.
    ///
    /// Append the given value to the *permissions* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_permissions(mut self, new_value: &str) -> BucketTestIamPermissionCall<'a, C, A> {
        self._permissions.push(new_value.to_string());
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketTestIamPermissionCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketTestIamPermissionCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketTestIamPermissionCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketTestIamPermissionCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an IAM policy for the specified bucket.
///
/// A builder for the *setIamPolicy* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::Policy;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Policy::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().set_iam_policy(req, "bucket")
///              .user_project("aliquyam")
///              .doit();
/// # }
/// ```
pub struct BucketSetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _request: Policy,
    _bucket: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketSetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> BucketSetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.setIamPolicy",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
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
    pub fn request(mut self, new_value: Policy) -> BucketSetIamPolicyCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketSetIamPolicyCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketSetIamPolicyCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketSetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketSetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketSetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns an IAM policy for the specified bucket.
///
/// A builder for the *getIamPolicy* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().get_iam_policy("bucket")
///              .user_project("magna")
///              .doit();
/// # }
/// ```
pub struct BucketGetIamPolicyCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _bucket: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketGetIamPolicyCall<'a, C, A> {}

impl<'a, C, A> BucketGetIamPolicyCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.buckets.getIamPolicy",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketGetIamPolicyCall<'a, C, A> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project number to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketGetIamPolicyCall<'a, C, A> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketGetIamPolicyCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketGetIamPolicyCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketGetIamPolicyCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of buckets for a given project.
///
/// A builder for the *list* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().list("project")
///              .projection("sit")
///              .prefix("gubergren")
///              .page_token("sit")
///              .max_results(4)
///              .doit();
/// # }
/// ```
pub struct BucketListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _project: String,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for BucketListCall<'a, C, A> {}

impl<'a, C, A> BucketListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Buckets)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
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
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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


    /// A valid API project identifier.
    ///
    /// Sets the *project* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> BucketListCall<'a, C, A> {
        self._project = new_value.to_string();
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketListCall<'a, C, A> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Filter results to buckets whose names begin with this prefix.
    ///
    /// Sets the *prefix* query property to the given value.
    pub fn prefix(mut self, new_value: &str) -> BucketListCall<'a, C, A> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// A previously-returned page token representing part of the larger set of results to view.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BucketListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of buckets to return in a single response. The service will use this parameter or 1,000 items, whichever is smaller.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> BucketListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> BucketListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BucketListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get the email address of this project's GCS service account.
///
/// A builder for the *serviceAccount.get* method supported by a *project* resource.
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
/// # extern crate google_storage1 as storage1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use storage1::Storage;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Storage::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_account_get("projectId")
///              .doit();
/// # }
/// ```
pub struct ProjectServiceAccountGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Storage<C, A>,
    _project_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectServiceAccountGetCall<'a, C, A> {}

impl<'a, C, A> ProjectServiceAccountGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ServiceAccount)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "storage.projects.serviceAccount.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
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

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/serviceAccount";
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


    /// Project ID
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectServiceAccountGetCall<'a, C, A> {
        self._project_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectServiceAccountGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectServiceAccountGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


