// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Tag Manager* crate version *0.1.0+20150121*, where *20150121* is the exact revision of the *tagmanager:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.0*.
//! 
//! Everything else about the *Tag Manager* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/tag-manager/api/v1/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/tagmanager1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.TagManager.html) ... 
//! 
//! * [accounts](struct.Account.html)
//!  * [*containers create*](struct.AccountContainerCreateCall.html), [*containers delete*](struct.AccountContainerDeleteCall.html), [*containers get*](struct.AccountContainerGetCall.html), [*containers list*](struct.AccountContainerListCall.html), [*containers macros create*](struct.AccountContainerMacroCreateCall.html), [*containers macros delete*](struct.AccountContainerMacroDeleteCall.html), [*containers macros get*](struct.AccountContainerMacroGetCall.html), [*containers macros list*](struct.AccountContainerMacroListCall.html), [*containers macros update*](struct.AccountContainerMacroUpdateCall.html), [*containers rules create*](struct.AccountContainerRuleCreateCall.html), [*containers rules delete*](struct.AccountContainerRuleDeleteCall.html), [*containers rules get*](struct.AccountContainerRuleGetCall.html), [*containers rules list*](struct.AccountContainerRuleListCall.html), [*containers rules update*](struct.AccountContainerRuleUpdateCall.html), [*containers tags create*](struct.AccountContainerTagCreateCall.html), [*containers tags delete*](struct.AccountContainerTagDeleteCall.html), [*containers tags get*](struct.AccountContainerTagGetCall.html), [*containers tags list*](struct.AccountContainerTagListCall.html), [*containers tags update*](struct.AccountContainerTagUpdateCall.html), [*containers triggers create*](struct.AccountContainerTriggerCreateCall.html), [*containers triggers delete*](struct.AccountContainerTriggerDeleteCall.html), [*containers triggers get*](struct.AccountContainerTriggerGetCall.html), [*containers triggers list*](struct.AccountContainerTriggerListCall.html), [*containers triggers update*](struct.AccountContainerTriggerUpdateCall.html), [*containers update*](struct.AccountContainerUpdateCall.html), [*containers variables create*](struct.AccountContainerVariableCreateCall.html), [*containers variables delete*](struct.AccountContainerVariableDeleteCall.html), [*containers variables get*](struct.AccountContainerVariableGetCall.html), [*containers variables list*](struct.AccountContainerVariableListCall.html), [*containers variables update*](struct.AccountContainerVariableUpdateCall.html), [*containers versions create*](struct.AccountContainerVersionCreateCall.html), [*containers versions delete*](struct.AccountContainerVersionDeleteCall.html), [*containers versions get*](struct.AccountContainerVersionGetCall.html), [*containers versions list*](struct.AccountContainerVersionListCall.html), [*containers versions publish*](struct.AccountContainerVersionPublishCall.html), [*containers versions restore*](struct.AccountContainerVersionRestoreCall.html), [*containers versions undelete*](struct.AccountContainerVersionUndeleteCall.html), [*containers versions update*](struct.AccountContainerVersionUpdateCall.html), [*get*](struct.AccountGetCall.html), [*list*](struct.AccountListCall.html), [*permissions create*](struct.AccountPermissionCreateCall.html), [*permissions delete*](struct.AccountPermissionDeleteCall.html), [*permissions get*](struct.AccountPermissionGetCall.html), [*permissions list*](struct.AccountPermissionListCall.html), [*permissions update*](struct.AccountPermissionUpdateCall.html) and [*update*](struct.AccountUpdateCall.html)
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
//! * **[Hub](struct.TagManager.html)**
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
//! let r = hub.accounts().containers_list(...).doit()
//! let r = hub.accounts().permissions_list(...).doit()
//! let r = hub.accounts().containers_versions_undelete(...).doit()
//! let r = hub.accounts().permissions_create(...).doit()
//! let r = hub.accounts().permissions_delete(...).doit()
//! let r = hub.accounts().containers_get(...).doit()
//! let r = hub.accounts().containers_versions_list(...).doit()
//! let r = hub.accounts().containers_triggers_update(...).doit()
//! let r = hub.accounts().containers_triggers_get(...).doit()
//! let r = hub.accounts().containers_delete(...).doit()
//! let r = hub.accounts().containers_create(...).doit()
//! let r = hub.accounts().containers_tags_delete(...).doit()
//! let r = hub.accounts().containers_rules_update(...).doit()
//! let r = hub.accounts().containers_rules_delete(...).doit()
//! let r = hub.accounts().containers_tags_list(...).doit()
//! let r = hub.accounts().containers_versions_publish(...).doit()
//! let r = hub.accounts().containers_tags_create(...).doit()
//! let r = hub.accounts().containers_triggers_list(...).doit()
//! let r = hub.accounts().containers_versions_delete(...).doit()
//! let r = hub.accounts().update(...).doit()
//! let r = hub.accounts().containers_macros_delete(...).doit()
//! let r = hub.accounts().containers_versions_create(...).doit()
//! let r = hub.accounts().permissions_get(...).doit()
//! let r = hub.accounts().containers_rules_create(...).doit()
//! let r = hub.accounts().containers_versions_restore(...).doit()
//! let r = hub.accounts().containers_rules_get(...).doit()
//! let r = hub.accounts().containers_variables_create(...).doit()
//! let r = hub.accounts().containers_variables_list(...).doit()
//! let r = hub.accounts().containers_macros_create(...).doit()
//! let r = hub.accounts().containers_tags_get(...).doit()
//! let r = hub.accounts().containers_variables_get(...).doit()
//! let r = hub.accounts().containers_triggers_delete(...).doit()
//! let r = hub.accounts().containers_macros_list(...).doit()
//! let r = hub.accounts().containers_triggers_create(...).doit()
//! let r = hub.accounts().containers_macros_update(...).doit()
//! let r = hub.accounts().list(...).doit()
//! let r = hub.accounts().permissions_update(...).doit()
//! let r = hub.accounts().containers_variables_delete(...).doit()
//! let r = hub.accounts().get(...).doit()
//! let r = hub.accounts().containers_update(...).doit()
//! let r = hub.accounts().containers_rules_list(...).doit()
//! let r = hub.accounts().containers_tags_update(...).doit()
//! let r = hub.accounts().containers_macros_get(...).doit()
//! let r = hub.accounts().containers_versions_update(...).doit()
//! let r = hub.accounts().containers_variables_update(...).doit()
//! let r = hub.accounts().containers_versions_get(...).doit()
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
//! google-tagmanager1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-tagmanager1" as tagmanager1;
//! use tagmanager1::Trigger;
//! use tagmanager1::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use tagmanager1::TagManager;
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
//! let mut hub = TagManager::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req: Trigger = Default::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().containers_triggers_update(&req, "accountId", "containerId", "triggerId")
//!              .fingerprint("accusam")
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
    /// Manage user permissions of your Google Tag Manager data
    ManageUser,

    /// Manage your Google Tag Manager accounts
    ManageAccount,

    /// View your Google Tag Manager containers
    Readonly,

    /// Manage your Google Tag Manager container versions
    EditContainerversion,

    /// Manage your Google Tag Manager containers
    EditContainer,

    /// Delete your Google Tag Manager containers
    DeleteContainer,

    /// Publish your Google Tag Manager containers
    Publish,
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            Scope::ManageUser => "https://www.googleapis.com/auth/tagmanager.manage.users",
            Scope::ManageAccount => "https://www.googleapis.com/auth/tagmanager.manage.accounts",
            Scope::Readonly => "https://www.googleapis.com/auth/tagmanager.readonly",
            Scope::EditContainerversion => "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
            Scope::EditContainer => "https://www.googleapis.com/auth/tagmanager.edit.containers",
            Scope::DeleteContainer => "https://www.googleapis.com/auth/tagmanager.delete.containers",
            Scope::Publish => "https://www.googleapis.com/auth/tagmanager.publish",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all TagManager related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Trigger;
/// use tagmanager1::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use tagmanager1::TagManager;
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
/// let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Trigger = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_triggers_update(&req, "accountId", "containerId", "triggerId")
///              .fingerprint("erat")
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
pub struct TagManager<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for TagManager<C, NC, A> {}

impl<'a, C, NC, A> TagManager<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> TagManager<C, NC, A> {
        TagManager {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.0".to_string(),
            _m: PhantomData
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C, NC, A> {
        AccountMethods { hub: &self }
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
/// List Macros Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers macros list accounts](struct.AccountContainerMacroListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListMacrosResponse {
    /// All GTM Macros of a GTM Container.    
    pub macros: Vec<Macro>,
}

impl ResponseResult for ListMacrosResponse {}


/// Defines the Google Tag Manager Account access permissions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountAccess {
    /// List of Account permissions. Valid account permissions are read and manage.    
    pub permission: Vec<String>,
}

impl Part for AccountAccess {}


/// Represents a Google Tag Manager Tag.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers tags update accounts](struct.AccountContainerTagUpdateCall.html) (request|response)
/// * [containers tags get accounts](struct.AccountContainerTagGetCall.html) (response)
/// * [containers tags create accounts](struct.AccountContainerTagCreateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    /// The start timestamp in milliseconds to schedule a tag.    
    #[serde(alias="scheduleStartMs")]
    pub schedule_start_ms: Option<String>,
    /// The end timestamp in milliseconds to schedule a tag.    
    #[serde(alias="scheduleEndMs")]
    pub schedule_end_ms: Option<String>,
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: Option<String>,
    /// The Tag ID uniquely identifies the GTM Tag.    
    #[serde(alias="tagId")]
    pub tag_id: Option<String>,
    /// Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire.    
    #[serde(alias="blockingRuleId")]
    pub blocking_rule_id: Option<Vec<String>>,
    /// The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified.    
    pub fingerprint: Option<String>,
    /// Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false.    
    #[serde(alias="firingRuleId")]
    pub firing_rule_id: Option<Vec<String>>,
    /// Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false.    
    #[serde(alias="firingTriggerId")]
    pub firing_trigger_id: Option<Vec<String>>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
    /// Tag display name.    
    pub name: Option<String>,
    /// The tag's parameters.    
    pub parameter: Option<Vec<Parameter>>,
    /// User notes on how to apply this tag in the container.    
    pub notes: Option<String>,
    /// User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0.    
    pub priority: Option<Parameter>,
    /// If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode).    
    #[serde(alias="liveOnly")]
    pub live_only: Option<bool>,
    /// GTM Tag Type.    
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire.    
    #[serde(alias="blockingTriggerId")]
    pub blocking_trigger_id: Option<Vec<String>>,
}

impl RequestValue for Tag {}
impl ResponseResult for Tag {}


/// Represents a Google Tag Manager Container.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers update accounts](struct.AccountContainerUpdateCall.html) (request|response)
/// * [containers create accounts](struct.AccountContainerCreateCall.html) (request|response)
/// * [containers get accounts](struct.AccountContainerGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    /// Container Time Zone ID.    
    #[serde(alias="timeZoneId")]
    pub time_zone_id: Option<String>,
    /// List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId.    
    #[serde(alias="enabledBuiltInVariable")]
    pub enabled_built_in_variable: Option<Vec<String>>,
    /// Container Country ID.    
    #[serde(alias="timeZoneCountryId")]
    pub time_zone_country_id: Option<i64>,
    /// Container Public ID.    
    #[serde(alias="publicId")]
    pub public_id: Option<String>,
    /// The Container ID uniquely identifies the GTM Container.    
    #[serde(alias="containerId")]
    pub container_id: Option<String>,
    /// Optional list of domain names associated with the Container.    
    #[serde(alias="domainName")]
    pub domain_name: Option<Vec<String>>,
    /// Container Notes.    
    pub notes: Option<String>,
    /// Container display name.    
    pub name: Option<String>,
    /// List of Usage Contexts for the Container. Valid values include: web, android, ios.    
    #[serde(alias="usageContext")]
    pub usage_context: Option<Vec<String>>,
    /// The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified.    
    pub fingerprint: Option<String>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
}

impl RequestValue for Container {}
impl ResponseResult for Container {}


/// List Containers Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers list accounts](struct.AccountContainerListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListContainersResponse {
    /// All Containers of a GTM Account.    
    pub containers: Vec<Container>,
}

impl ResponseResult for ListContainersResponse {}


/// Represents a Google Tag Manager Macro.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers macros create accounts](struct.AccountContainerMacroCreateCall.html) (request|response)
/// * [containers macros get accounts](struct.AccountContainerMacroGetCall.html) (response)
/// * [containers macros update accounts](struct.AccountContainerMacroUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Macro {
    /// The start timestamp in milliseconds to schedule a macro.    
    #[serde(alias="scheduleStartMs")]
    pub schedule_start_ms: Option<String>,
    /// The end timestamp in milliseconds to schedule a macro.    
    #[serde(alias="scheduleEndMs")]
    pub schedule_end_ms: Option<String>,
    /// The Macro ID uniquely identifies the GTM Macro.    
    #[serde(alias="macroId")]
    pub macro_id: Option<String>,
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: Option<String>,
    /// The macro's parameters.    
    pub parameter: Option<Vec<Parameter>>,
    /// User notes on how to apply this macro in the container.    
    pub notes: Option<String>,
    /// The fingerprint of the GTM Macro as computed at storage time. This value is recomputed whenever the macro is modified.    
    pub fingerprint: Option<String>,
    /// For mobile containers only: A list of rule IDs for disabling conditional macros; the macro is enabled if one of the enabling rules is true while all the disabling rules are false. Treated as an unordered set.    
    #[serde(alias="disablingRuleId")]
    pub disabling_rule_id: Option<Vec<String>>,
    /// For mobile containers only: A list of rule IDs for enabling conditional macros; the macro is enabled if one of the enabling rules is true while all the disabling rules are false. Treated as an unordered set.    
    #[serde(alias="enablingRuleId")]
    pub enabling_rule_id: Option<Vec<String>>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
    /// GTM Macro Type.    
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// Macro display name.    
    pub name: Option<String>,
}

impl RequestValue for Macro {}
impl ResponseResult for Macro {}


/// List Variables Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers variables list accounts](struct.AccountContainerVariableListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListVariablesResponse {
    /// All GTM Variables of a GTM Container.    
    pub variables: Vec<Variable>,
}

impl ResponseResult for ListVariablesResponse {}


/// Represents a Google Tag Manager Account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers list accounts](struct.AccountContainerListCall.html) (none)
/// * [permissions list accounts](struct.AccountPermissionListCall.html) (none)
/// * [containers versions undelete accounts](struct.AccountContainerVersionUndeleteCall.html) (none)
/// * [permissions create accounts](struct.AccountPermissionCreateCall.html) (none)
/// * [permissions delete accounts](struct.AccountPermissionDeleteCall.html) (none)
/// * [containers get accounts](struct.AccountContainerGetCall.html) (none)
/// * [containers versions list accounts](struct.AccountContainerVersionListCall.html) (none)
/// * [containers triggers update accounts](struct.AccountContainerTriggerUpdateCall.html) (none)
/// * [containers triggers get accounts](struct.AccountContainerTriggerGetCall.html) (none)
/// * [containers delete accounts](struct.AccountContainerDeleteCall.html) (none)
/// * [containers create accounts](struct.AccountContainerCreateCall.html) (none)
/// * [containers tags delete accounts](struct.AccountContainerTagDeleteCall.html) (none)
/// * [containers rules update accounts](struct.AccountContainerRuleUpdateCall.html) (none)
/// * [containers rules delete accounts](struct.AccountContainerRuleDeleteCall.html) (none)
/// * [containers tags list accounts](struct.AccountContainerTagListCall.html) (none)
/// * [containers versions publish accounts](struct.AccountContainerVersionPublishCall.html) (none)
/// * [containers tags create accounts](struct.AccountContainerTagCreateCall.html) (none)
/// * [containers triggers list accounts](struct.AccountContainerTriggerListCall.html) (none)
/// * [containers versions delete accounts](struct.AccountContainerVersionDeleteCall.html) (none)
/// * [update accounts](struct.AccountUpdateCall.html) (request|response)
/// * [containers macros delete accounts](struct.AccountContainerMacroDeleteCall.html) (none)
/// * [containers versions create accounts](struct.AccountContainerVersionCreateCall.html) (none)
/// * [permissions get accounts](struct.AccountPermissionGetCall.html) (none)
/// * [containers rules create accounts](struct.AccountContainerRuleCreateCall.html) (none)
/// * [containers versions restore accounts](struct.AccountContainerVersionRestoreCall.html) (none)
/// * [containers rules get accounts](struct.AccountContainerRuleGetCall.html) (none)
/// * [containers variables create accounts](struct.AccountContainerVariableCreateCall.html) (none)
/// * [containers variables list accounts](struct.AccountContainerVariableListCall.html) (none)
/// * [containers macros create accounts](struct.AccountContainerMacroCreateCall.html) (none)
/// * [containers tags get accounts](struct.AccountContainerTagGetCall.html) (none)
/// * [containers variables get accounts](struct.AccountContainerVariableGetCall.html) (none)
/// * [containers triggers delete accounts](struct.AccountContainerTriggerDeleteCall.html) (none)
/// * [containers macros list accounts](struct.AccountContainerMacroListCall.html) (none)
/// * [containers triggers create accounts](struct.AccountContainerTriggerCreateCall.html) (none)
/// * [containers macros update accounts](struct.AccountContainerMacroUpdateCall.html) (none)
/// * [list accounts](struct.AccountListCall.html) (none)
/// * [permissions update accounts](struct.AccountPermissionUpdateCall.html) (none)
/// * [containers variables delete accounts](struct.AccountContainerVariableDeleteCall.html) (none)
/// * [get accounts](struct.AccountGetCall.html) (response)
/// * [containers update accounts](struct.AccountContainerUpdateCall.html) (none)
/// * [containers rules list accounts](struct.AccountContainerRuleListCall.html) (none)
/// * [containers tags update accounts](struct.AccountContainerTagUpdateCall.html) (none)
/// * [containers macros get accounts](struct.AccountContainerMacroGetCall.html) (none)
/// * [containers versions update accounts](struct.AccountContainerVersionUpdateCall.html) (none)
/// * [containers variables update accounts](struct.AccountContainerVariableUpdateCall.html) (none)
/// * [containers versions get accounts](struct.AccountContainerVersionGetCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Whether the account shares data anonymously with Google and others.    
    #[serde(alias="shareData")]
    pub share_data: Option<bool>,
    /// The Account ID uniquely identifies the GTM Account.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
    /// Account display name.    
    pub name: Option<String>,
    /// The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified.    
    pub fingerprint: Option<String>,
}

impl RequestValue for Account {}
impl Resource for Account {}
impl ResponseResult for Account {}


/// Represents a Google Tag Manager Rule.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers rules get accounts](struct.AccountContainerRuleGetCall.html) (response)
/// * [containers rules create accounts](struct.AccountContainerRuleCreateCall.html) (request|response)
/// * [containers rules update accounts](struct.AccountContainerRuleUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: Option<String>,
    /// Rule display name.    
    pub name: Option<String>,
    /// The Rule ID uniquely identifies the GTM Rule.    
    #[serde(alias="ruleId")]
    pub rule_id: Option<String>,
    /// User notes on how to apply this rule in the container.    
    pub notes: Option<String>,
    /// The fingerprint of the GTM Rule as computed at storage time. This value is recomputed whenever the rule is modified.    
    pub fingerprint: Option<String>,
    /// The list of conditions that make up this rule (implicit AND between them).    
    pub condition: Option<Vec<Condition>>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
}

impl RequestValue for Rule {}
impl ResponseResult for Rule {}


/// List Accounts Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](struct.AccountListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListAccountsResponse {
    /// List of GTM Accounts that a user has access to.    
    pub accounts: Vec<Account>,
}

impl ResponseResult for ListAccountsResponse {}


/// Publish container version response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions publish accounts](struct.AccountContainerVersionPublishCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PublishContainerVersionResponse {
    /// The container version created.    
    #[serde(alias="containerVersion")]
    pub container_version: ContainerVersion,
    /// Compiler errors or not.    
    #[serde(alias="compilerError")]
    pub compiler_error: bool,
}

impl ResponseResult for PublishContainerVersionResponse {}


/// Represents a Google Tag Manager Trigger
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers triggers update accounts](struct.AccountContainerTriggerUpdateCall.html) (request|response)
/// * [containers triggers get accounts](struct.AccountContainerTriggerGetCall.html) (response)
/// * [containers triggers create accounts](struct.AccountContainerTriggerCreateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Trigger {
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: Option<String>,
    /// Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers.    
    #[serde(alias="uniqueTriggerId")]
    pub unique_trigger_id: Option<Parameter>,
    /// The Trigger ID uniquely identifies the GTM Trigger.    
    #[serde(alias="triggerId")]
    pub trigger_id: Option<String>,
    /// The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified.    
    pub fingerprint: Option<String>,
    /// Reloads the videos in the page that don't already have the YT API enabled. If false, only capture events from videos that already have the API enabled. Only valid for YouTube triggers.    
    #[serde(alias="enableAllVideos")]
    pub enable_all_videos: Option<Parameter>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
    /// Name of the GTM event that is fired. Only valid for Timer triggers.    
    #[serde(alias="eventName")]
    pub event_name: Option<Parameter>,
    /// List of integer percentage values. The trigger will fire as each percentage is reached in any instrumented videos. Only valid for YouTube triggers.    
    #[serde(alias="videoPercentageList")]
    pub video_percentage_list: Option<Parameter>,
    /// Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers.    
    #[serde(alias="waitForTags")]
    pub wait_for_tags: Option<Parameter>,
    /// Trigger display name.    
    pub name: Option<String>,
    /// Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers.    
    pub interval: Option<Parameter>,
    /// Used in the case of auto event tracking.    
    #[serde(alias="autoEventFilter")]
    pub auto_event_filter: Option<Vec<Condition>>,
    /// The trigger will only fire iff all Conditions are true.    
    pub filter: Option<Vec<Condition>>,
    /// How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers.    
    #[serde(alias="waitForTagsTimeout")]
    pub wait_for_tags_timeout: Option<Parameter>,
    /// Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers.    
    pub limit: Option<Parameter>,
    /// Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers.    
    #[serde(alias="checkValidation")]
    pub check_validation: Option<Parameter>,
    /// Used in the case of custom event, which is fired iff all Conditions are true.    
    #[serde(alias="customEventFilter")]
    pub custom_event_filter: Option<Vec<Condition>>,
    /// Defines the data layer event that causes this trigger.    
    #[serde(alias="type")]
    pub type_: Option<String>,
}

impl RequestValue for Trigger {}
impl ResponseResult for Trigger {}


/// Create container versions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions create accounts](struct.AccountContainerVersionCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct CreateContainerVersionResponse {
    /// The container version created.    
    #[serde(alias="containerVersion")]
    pub container_version: ContainerVersion,
    /// Compiler errors or not.    
    #[serde(alias="compilerError")]
    pub compiler_error: bool,
}

impl ResponseResult for CreateContainerVersionResponse {}


/// List Rules Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers rules list accounts](struct.AccountContainerRuleListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListRulesResponse {
    /// All GTM Rules of a GTM Container.    
    pub rules: Vec<Rule>,
}

impl ResponseResult for ListRulesResponse {}


/// Defines the Google Tag Manager Container access permissions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerAccess {
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: String,
    /// List of Container permissions. Valid container permissions are: read, edit, delete, publish.    
    pub permission: Vec<String>,
}

impl Part for ContainerAccess {}


/// Represents a user's permissions to an account and its container.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions get accounts](struct.AccountPermissionGetCall.html) (response)
/// * [permissions create accounts](struct.AccountPermissionCreateCall.html) (request|response)
/// * [permissions update accounts](struct.AccountPermissionUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserAccess {
    /// GTM Container access permissions.    
    #[serde(alias="containerAccess")]
    pub container_access: Option<Vec<ContainerAccess>>,
    /// GTM Account access permissions.    
    #[serde(alias="accountAccess")]
    pub account_access: Option<AccountAccess>,
    /// User's email address.    
    #[serde(alias="emailAddress")]
    pub email_address: Option<String>,
    /// Account Permission ID.    
    #[serde(alias="permissionId")]
    pub permission_id: Option<String>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
}

impl RequestValue for UserAccess {}
impl ResponseResult for UserAccess {}


/// Represents a Google Tag Manager Parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    /// This map parameter's parameters (must have keys; keys must be unique).    
    pub map: Vec<Parameter>,
    /// This list parameter's parameters (keys will be ignored).    
    pub list: Vec<Parameter>,
    /// The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.    
    pub key: String,
    /// A parameter's value (may contain macro references such as "{{myMacro}}") as appropriate to the specified type.    
    pub value: String,
    /// The parameter type. Valid values are: 
    /// - boolean: The value represents a boolean, represented as 'true' or 'false' 
    /// - integer: The value represents a 64-bit signed integer value, in base 10 
    /// - list: A list of parameters should be specified 
    /// - map: A map of parameters should be specified 
    /// - template: The value represents any text; this can include macro references (even macro references that might return non-string types)
    #[serde(alias="type")]
    pub type_: String,
}

impl Part for Parameter {}


/// List Tags Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers tags list accounts](struct.AccountContainerTagListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListTagsResponse {
    /// All GTM Tags of a GTM Container.    
    pub tags: Vec<Tag>,
}

impl ResponseResult for ListTagsResponse {}


/// List AccountUsers Response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [permissions list accounts](struct.AccountPermissionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListAccountUsersResponse {
    /// All GTM AccountUsers of a GTM Account.    
    #[serde(alias="userAccess")]
    pub user_access: Vec<UserAccess>,
}

impl ResponseResult for ListAccountUsersResponse {}


/// Options for new container versions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions create accounts](struct.AccountContainerVersionCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct CreateContainerVersionRequestVersionOptions {
    /// The notes of the container version to be created.    
    pub notes: Option<String>,
    /// The name of the container version to be created.    
    pub name: Option<String>,
    /// The creation of this version may be for quick preview and shouldn't be saved.    
    #[serde(alias="quickPreview")]
    pub quick_preview: Option<bool>,
}

impl RequestValue for CreateContainerVersionRequestVersionOptions {}


/// List triggers response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers triggers list accounts](struct.AccountContainerTriggerListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListTriggersResponse {
    /// All GTM Triggers of a GTM Container.    
    pub triggers: Vec<Trigger>,
}

impl ResponseResult for ListTriggersResponse {}


/// Represents a Google Tag Manager Container Version Header.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ContainerVersionHeader {
    /// Container version display name.    
    pub name: String,
    /// A value of true indicates this container version has been deleted.    
    pub deleted: bool,
    /// Number of macros in the container version.    
    #[serde(alias="numMacros")]
    pub num_macros: String,
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: String,
    /// Number of tags in the container version.    
    #[serde(alias="numTags")]
    pub num_tags: String,
    /// Number of rules in the container version.    
    #[serde(alias="numRules")]
    pub num_rules: String,
    /// The Container Version ID uniquely identifies the GTM Container Version.    
    #[serde(alias="containerVersionId")]
    pub container_version_id: String,
    /// Number of triggers in the container version.    
    #[serde(alias="numTriggers")]
    pub num_triggers: String,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: String,
    /// Number of variables in the container version.    
    #[serde(alias="numVariables")]
    pub num_variables: String,
}

impl Part for ContainerVersionHeader {}


/// Represents a Google Tag Manager Container Version.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions update accounts](struct.AccountContainerVersionUpdateCall.html) (request|response)
/// * [containers versions get accounts](struct.AccountContainerVersionGetCall.html) (response)
/// * [containers versions undelete accounts](struct.AccountContainerVersionUndeleteCall.html) (response)
/// * [containers versions restore accounts](struct.AccountContainerVersionRestoreCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContainerVersion {
    /// The container that this version was taken from.    
    pub container: Option<Container>,
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: Option<String>,
    /// The triggers in the container that this version was taken from.    
    pub trigger: Option<Vec<Trigger>>,
    /// A value of true indicates this container version has been deleted.    
    pub deleted: Option<bool>,
    /// The macros in the container that this version was taken from.    
    #[serde(alias="macro")]
    pub macro_: Option<Vec<Macro>>,
    /// User notes on how to apply this container version in the container.    
    pub notes: Option<String>,
    /// The rules in the container that this version was taken from.    
    pub rule: Option<Vec<Rule>>,
    /// The tags in the container that this version was taken from.    
    pub tag: Option<Vec<Tag>>,
    /// The Container Version ID uniquely identifies the GTM Container Version.    
    #[serde(alias="containerVersionId")]
    pub container_version_id: Option<String>,
    /// The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified.    
    pub fingerprint: Option<String>,
    /// The variables in the container that this version was taken from.    
    pub variable: Option<Vec<Variable>>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
    /// Container version display name.    
    pub name: Option<String>,
}

impl RequestValue for ContainerVersion {}
impl ResponseResult for ContainerVersion {}


/// List container versions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers versions list accounts](struct.AccountContainerVersionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ListContainerVersionsResponse {
    /// All container version headers of a GTM Container.    
    #[serde(alias="containerVersionHeader")]
    pub container_version_header: Vec<ContainerVersionHeader>,
    /// All versions of a GTM Container.    
    #[serde(alias="containerVersion")]
    pub container_version: Vec<ContainerVersion>,
}

impl ResponseResult for ListContainerVersionsResponse {}


/// Represents a Google Tag Manager Variable.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [containers variables create accounts](struct.AccountContainerVariableCreateCall.html) (request|response)
/// * [containers variables update accounts](struct.AccountContainerVariableUpdateCall.html) (request|response)
/// * [containers variables get accounts](struct.AccountContainerVariableGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    /// The start timestamp in milliseconds to schedule a variable.    
    #[serde(alias="scheduleStartMs")]
    pub schedule_start_ms: Option<String>,
    /// The end timestamp in milliseconds to schedule a variable.    
    #[serde(alias="scheduleEndMs")]
    pub schedule_end_ms: Option<String>,
    /// Variable display name.    
    pub name: Option<String>,
    /// The variable's parameters.    
    pub parameter: Option<Vec<Parameter>>,
    /// User notes on how to apply this variable in the container.    
    pub notes: Option<String>,
    /// GTM Container ID.    
    #[serde(alias="containerId")]
    pub container_id: Option<String>,
    /// For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set.    
    #[serde(alias="enablingTriggerId")]
    pub enabling_trigger_id: Option<Vec<String>>,
    /// The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified.    
    pub fingerprint: Option<String>,
    /// GTM Account ID.    
    #[serde(alias="accountId")]
    pub account_id: Option<String>,
    /// GTM Variable Type.    
    #[serde(alias="type")]
    pub type_: Option<String>,
    /// For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set.    
    #[serde(alias="disablingTriggerId")]
    pub disabling_trigger_id: Option<Vec<String>>,
    /// The Variable ID uniquely identifies the GTM Variable.    
    #[serde(alias="variableId")]
    pub variable_id: Option<String>,
}

impl RequestValue for Variable {}
impl ResponseResult for Variable {}


/// Represents a predicate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    /// A list of named parameters (key/value), depending on the condition's type. Notes: 
    /// - For binary operators, include parameters named arg0 and arg1 for specifying the left and right operands, respectively. 
    /// - At this time, the left operand (arg0) must be a reference to a macro. 
    /// - For case-insensitive Regex matching, include a boolean parameter named ignore_case that is set to true. If not specified or set to any other value, the matching will be case sensitive. 
    /// - To negate an operator, include a boolean parameter named negate boolean parameter that is set to true.
    pub parameter: Vec<Parameter>,
    /// The type of operator for this condition.    
    #[serde(alias="type")]
    pub type_: String,
}

impl Part for Condition {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the `TagManager` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-tagmanager1" as tagmanager1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use tagmanager1::TagManager;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `containers_create(...)`, `containers_delete(...)`, `containers_get(...)`, `containers_list(...)`, `containers_macros_create(...)`, `containers_macros_delete(...)`, `containers_macros_get(...)`, `containers_macros_list(...)`, `containers_macros_update(...)`, `containers_rules_create(...)`, `containers_rules_delete(...)`, `containers_rules_get(...)`, `containers_rules_list(...)`, `containers_rules_update(...)`, `containers_tags_create(...)`, `containers_tags_delete(...)`, `containers_tags_get(...)`, `containers_tags_list(...)`, `containers_tags_update(...)`, `containers_triggers_create(...)`, `containers_triggers_delete(...)`, `containers_triggers_get(...)`, `containers_triggers_list(...)`, `containers_triggers_update(...)`, `containers_update(...)`, `containers_variables_create(...)`, `containers_variables_delete(...)`, `containers_variables_get(...)`, `containers_variables_list(...)`, `containers_variables_update(...)`, `containers_versions_create(...)`, `containers_versions_delete(...)`, `containers_versions_get(...)`, `containers_versions_list(...)`, `containers_versions_publish(...)`, `containers_versions_restore(...)`, `containers_versions_undelete(...)`, `containers_versions_update(...)`, `get(...)`, `list(...)`, `permissions_create(...)`, `permissions_delete(...)`, `permissions_get(...)`, `permissions_list(...)`, `permissions_update(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for AccountMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Containers that belongs to a GTM Account.    
    pub fn containers_list(&self, account_id: &str) -> AccountContainerListCall<'a, C, NC, A> {
        AccountContainerListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all users that have access to the account along with Account and Container Permissions granted to each of them.    
    pub fn permissions_list(&self, account_id: &str) -> AccountPermissionListCall<'a, C, NC, A> {
        AccountPermissionListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a Container Version.    
    pub fn containers_versions_undelete(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionUndeleteCall<'a, C, NC, A> {
        AccountContainerVersionUndeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a user's Account & Container Permissions.    
    pub fn permissions_create(&self, request: &UserAccess, account_id: &str) -> AccountPermissionCreateCall<'a, C, NC, A> {
        AccountPermissionCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a user from the account, revoking access to it and all of its containers.    
    pub fn permissions_delete(&self, account_id: &str, permission_id: &str) -> AccountPermissionDeleteCall<'a, C, NC, A> {
        AccountPermissionDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _permission_id: permission_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Container.    
    pub fn containers_get(&self, account_id: &str, container_id: &str) -> AccountContainerGetCall<'a, C, NC, A> {
        AccountContainerGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Container Versions of a GTM Container.    
    pub fn containers_versions_list(&self, account_id: &str, container_id: &str) -> AccountContainerVersionListCall<'a, C, NC, A> {
        AccountContainerVersionListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _headers: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Trigger.    
    pub fn containers_triggers_update(&self, request: &Trigger, account_id: &str, container_id: &str, trigger_id: &str) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> {
        AccountContainerTriggerUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _trigger_id: trigger_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Trigger.    
    pub fn containers_triggers_get(&self, account_id: &str, container_id: &str, trigger_id: &str) -> AccountContainerTriggerGetCall<'a, C, NC, A> {
        AccountContainerTriggerGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _trigger_id: trigger_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Container.    
    pub fn containers_delete(&self, account_id: &str, container_id: &str) -> AccountContainerDeleteCall<'a, C, NC, A> {
        AccountContainerDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Container.    
    pub fn containers_create(&self, request: &Container, account_id: &str) -> AccountContainerCreateCall<'a, C, NC, A> {
        AccountContainerCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a GTM Tag.    
    pub fn containers_tags_delete(&self, account_id: &str, container_id: &str, tag_id: &str) -> AccountContainerTagDeleteCall<'a, C, NC, A> {
        AccountContainerTagDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _tag_id: tag_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Rule.    
    pub fn containers_rules_update(&self, request: &Rule, account_id: &str, container_id: &str, rule_id: &str) -> AccountContainerRuleUpdateCall<'a, C, NC, A> {
        AccountContainerRuleUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _rule_id: rule_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a GTM Rule.    
    pub fn containers_rules_delete(&self, account_id: &str, container_id: &str, rule_id: &str) -> AccountContainerRuleDeleteCall<'a, C, NC, A> {
        AccountContainerRuleDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _rule_id: rule_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Tags of a Container.    
    pub fn containers_tags_list(&self, account_id: &str, container_id: &str) -> AccountContainerTagListCall<'a, C, NC, A> {
        AccountContainerTagListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Publishes a Container Version.    
    pub fn containers_versions_publish(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionPublishCall<'a, C, NC, A> {
        AccountContainerVersionPublishCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Tag.    
    pub fn containers_tags_create(&self, request: &Tag, account_id: &str, container_id: &str) -> AccountContainerTagCreateCall<'a, C, NC, A> {
        AccountContainerTagCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Triggers of a Container.    
    pub fn containers_triggers_list(&self, account_id: &str, container_id: &str) -> AccountContainerTriggerListCall<'a, C, NC, A> {
        AccountContainerTriggerListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Container Version.    
    pub fn containers_versions_delete(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionDeleteCall<'a, C, NC, A> {
        AccountContainerVersionDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Account.    
    pub fn update(&self, request: &Account, account_id: &str) -> AccountUpdateCall<'a, C, NC, A> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a GTM Macro.    
    pub fn containers_macros_delete(&self, account_id: &str, container_id: &str, macro_id: &str) -> AccountContainerMacroDeleteCall<'a, C, NC, A> {
        AccountContainerMacroDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _macro_id: macro_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Container Version.    
    pub fn containers_versions_create(&self, request: &CreateContainerVersionRequestVersionOptions, account_id: &str, container_id: &str) -> AccountContainerVersionCreateCall<'a, C, NC, A> {
        AccountContainerVersionCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a user's Account & Container Permissions.    
    pub fn permissions_get(&self, account_id: &str, permission_id: &str) -> AccountPermissionGetCall<'a, C, NC, A> {
        AccountPermissionGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _permission_id: permission_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Rule.    
    pub fn containers_rules_create(&self, request: &Rule, account_id: &str, container_id: &str) -> AccountContainerRuleCreateCall<'a, C, NC, A> {
        AccountContainerRuleCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores a Container Version. This will overwrite the container's current configuration (including its macros, rules and tags). The operation will not have any effect on the version that is being served (i.e. the published version).    
    pub fn containers_versions_restore(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionRestoreCall<'a, C, NC, A> {
        AccountContainerVersionRestoreCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Rule.    
    pub fn containers_rules_get(&self, account_id: &str, container_id: &str, rule_id: &str) -> AccountContainerRuleGetCall<'a, C, NC, A> {
        AccountContainerRuleGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _rule_id: rule_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Variable.    
    pub fn containers_variables_create(&self, request: &Variable, account_id: &str, container_id: &str) -> AccountContainerVariableCreateCall<'a, C, NC, A> {
        AccountContainerVariableCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Variables of a Container.    
    pub fn containers_variables_list(&self, account_id: &str, container_id: &str) -> AccountContainerVariableListCall<'a, C, NC, A> {
        AccountContainerVariableListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Macro.    
    pub fn containers_macros_create(&self, request: &Macro, account_id: &str, container_id: &str) -> AccountContainerMacroCreateCall<'a, C, NC, A> {
        AccountContainerMacroCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Tag.    
    pub fn containers_tags_get(&self, account_id: &str, container_id: &str, tag_id: &str) -> AccountContainerTagGetCall<'a, C, NC, A> {
        AccountContainerTagGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _tag_id: tag_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Variable.    
    pub fn containers_variables_get(&self, account_id: &str, container_id: &str, variable_id: &str) -> AccountContainerVariableGetCall<'a, C, NC, A> {
        AccountContainerVariableGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _variable_id: variable_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a GTM Trigger.    
    pub fn containers_triggers_delete(&self, account_id: &str, container_id: &str, trigger_id: &str) -> AccountContainerTriggerDeleteCall<'a, C, NC, A> {
        AccountContainerTriggerDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _trigger_id: trigger_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Macros of a Container.    
    pub fn containers_macros_list(&self, account_id: &str, container_id: &str) -> AccountContainerMacroListCall<'a, C, NC, A> {
        AccountContainerMacroListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GTM Trigger.    
    pub fn containers_triggers_create(&self, request: &Trigger, account_id: &str, container_id: &str) -> AccountContainerTriggerCreateCall<'a, C, NC, A> {
        AccountContainerTriggerCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Macro.    
    pub fn containers_macros_update(&self, request: &Macro, account_id: &str, container_id: &str, macro_id: &str) -> AccountContainerMacroUpdateCall<'a, C, NC, A> {
        AccountContainerMacroUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _macro_id: macro_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Accounts that a user has access to.    
    pub fn list(&self) -> AccountListCall<'a, C, NC, A> {
        AccountListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a user's Account & Container Permissions.    
    pub fn permissions_update(&self, request: &UserAccess, account_id: &str, permission_id: &str) -> AccountPermissionUpdateCall<'a, C, NC, A> {
        AccountPermissionUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _permission_id: permission_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a GTM Variable.    
    pub fn containers_variables_delete(&self, account_id: &str, container_id: &str, variable_id: &str) -> AccountContainerVariableDeleteCall<'a, C, NC, A> {
        AccountContainerVariableDeleteCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _variable_id: variable_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Account.    
    pub fn get(&self, account_id: &str) -> AccountGetCall<'a, C, NC, A> {
        AccountGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Container.    
    pub fn containers_update(&self, request: &Container, account_id: &str, container_id: &str) -> AccountContainerUpdateCall<'a, C, NC, A> {
        AccountContainerUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all GTM Rules of a Container.    
    pub fn containers_rules_list(&self, account_id: &str, container_id: &str) -> AccountContainerRuleListCall<'a, C, NC, A> {
        AccountContainerRuleListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Tag.    
    pub fn containers_tags_update(&self, request: &Tag, account_id: &str, container_id: &str, tag_id: &str) -> AccountContainerTagUpdateCall<'a, C, NC, A> {
        AccountContainerTagUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _tag_id: tag_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a GTM Macro.    
    pub fn containers_macros_get(&self, account_id: &str, container_id: &str, macro_id: &str) -> AccountContainerMacroGetCall<'a, C, NC, A> {
        AccountContainerMacroGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _macro_id: macro_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Container Version.    
    pub fn containers_versions_update(&self, request: &ContainerVersion, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionUpdateCall<'a, C, NC, A> {
        AccountContainerVersionUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a GTM Variable.    
    pub fn containers_variables_update(&self, request: &Variable, account_id: &str, container_id: &str, variable_id: &str) -> AccountContainerVariableUpdateCall<'a, C, NC, A> {
        AccountContainerVariableUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _variable_id: variable_id.to_string(),
            _fingerprint: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Container Version.    
    pub fn containers_versions_get(&self, account_id: &str, container_id: &str, container_version_id: &str) -> AccountContainerVersionGetCall<'a, C, NC, A> {
        AccountContainerVersionGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _container_id: container_id.to_string(),
            _container_version_id: container_version_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Lists all Containers that belongs to a GTM Account.
///
/// A builder for the *containers.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_list("accountId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListContainersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
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
            for param_name in ["accountId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// List all users that have access to the account along with Account and Container Permissions granted to each of them.
///
/// A builder for the *permissions.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().permissions_list("accountId")
///              .doit();
/// # }
/// ```
pub struct AccountPermissionListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountPermissionListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountPermissionListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListAccountUsersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.permissions.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/permissions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
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
            for param_name in ["accountId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID. @required tagmanager.accounts.permissions.list    
    pub fn account_id(mut self, new_value: &str) -> AccountPermissionListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountPermissionListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountPermissionListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountPermissionListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Undeletes a Container Version.
///
/// A builder for the *containers.versions.undelete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_undelete("accountId", "containerId", "containerVersionId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionUndeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _container_version_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionUndeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionUndeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ContainerVersion)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.undelete", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("containerVersionId", self._container_version_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "containerVersionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions/{containerVersionId}/undelete".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{containerVersionId}", "containerVersionId")].iter() {
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
            for param_name in ["accountId", "containerId", "containerVersionId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionUndeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionUndeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *container version id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container Version ID.    
    pub fn container_version_id(mut self, new_value: &str) -> AccountContainerVersionUndeleteCall<'a, C, NC, A> {
        self._container_version_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionUndeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionUndeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionUndeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a user's Account & Container Permissions.
///
/// A builder for the *permissions.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::UserAccess;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: UserAccess = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().permissions_create(&req, "accountId")
///              .doit();
/// # }
/// ```
pub struct AccountPermissionCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: UserAccess,
    _account_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountPermissionCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountPermissionCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UserAccess)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.permissions.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/permissions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
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
            for param_name in ["accountId"].iter() {
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
    pub fn request(mut self, new_value: &UserAccess) -> AccountPermissionCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountPermissionCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountPermissionCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountPermissionCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountPermissionCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Removes a user from the account, revoking access to it and all of its containers.
///
/// A builder for the *permissions.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().permissions_delete("accountId", "permissionId")
///              .doit();
/// # }
/// ```
pub struct AccountPermissionDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _permission_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountPermissionDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountPermissionDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.permissions.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        for &field in ["accountId", "permissionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/permissions/{permissionId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{permissionId}", "permissionId")].iter() {
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
            for param_name in ["accountId", "permissionId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountPermissionDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM User ID.    
    pub fn permission_id(mut self, new_value: &str) -> AccountPermissionDeleteCall<'a, C, NC, A> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountPermissionDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountPermissionDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountPermissionDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a Container.
///
/// A builder for the *containers.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_get("accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Container)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerGetCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all Container Versions of a GTM Container.
///
/// A builder for the *containers.versions.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_list("accountId", "containerId")
///              .headers(true)
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _headers: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListContainerVersionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        if let Some(value) = self._headers {
            params.push(("headers", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "headers"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionListCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *headers* query property to the given value.
    ///
    /// 
    /// Retrieve headers only when true.    
    pub fn headers(mut self, new_value: bool) -> AccountContainerVersionListCall<'a, C, NC, A> {
        self._headers = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a GTM Trigger.
///
/// A builder for the *containers.triggers.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Trigger;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Trigger = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_triggers_update(&req, "accountId", "containerId", "triggerId")
///              .fingerprint("et")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTriggerUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Trigger,
    _account_id: String,
    _container_id: String,
    _trigger_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTriggerUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTriggerUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Trigger)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.triggers.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("triggerId", self._trigger_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "triggerId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/triggers/{triggerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{triggerId}", "triggerId")].iter() {
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
            for param_name in ["accountId", "containerId", "triggerId"].iter() {
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
    pub fn request(mut self, new_value: &Trigger) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *trigger id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Trigger ID.    
    pub fn trigger_id(mut self, new_value: &str) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> {
        self._trigger_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the trigger in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTriggerUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTriggerUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a GTM Trigger.
///
/// A builder for the *containers.triggers.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_triggers_get("accountId", "containerId", "triggerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTriggerGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _trigger_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTriggerGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTriggerGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Trigger)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.triggers.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("triggerId", self._trigger_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "triggerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/triggers/{triggerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{triggerId}", "triggerId")].iter() {
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
            for param_name in ["accountId", "containerId", "triggerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTriggerGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTriggerGetCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *trigger id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Trigger ID.    
    pub fn trigger_id(mut self, new_value: &str) -> AccountContainerTriggerGetCall<'a, C, NC, A> {
        self._trigger_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTriggerGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTriggerGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTriggerGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a Container.
///
/// A builder for the *containers.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_delete("accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerDeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a Container.
///
/// A builder for the *containers.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Container;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Container = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_create(&req, "accountId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Container,
    _account_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Container)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
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
            for param_name in ["accountId"].iter() {
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
    pub fn request(mut self, new_value: &Container) -> AccountContainerCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a GTM Tag.
///
/// A builder for the *containers.tags.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_tags_delete("accountId", "containerId", "tagId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTagDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _tag_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTagDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTagDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.tags.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("tagId", self._tag_id.to_string()));
        for &field in ["accountId", "containerId", "tagId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/tags/{tagId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{tagId}", "tagId")].iter() {
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
            for param_name in ["accountId", "containerId", "tagId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTagDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTagDeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *tag id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Tag ID.    
    pub fn tag_id(mut self, new_value: &str) -> AccountContainerTagDeleteCall<'a, C, NC, A> {
        self._tag_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTagDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTagDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTagDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a GTM Rule.
///
/// A builder for the *containers.rules.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Rule;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Rule = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_rules_update(&req, "accountId", "containerId", "ruleId")
///              .fingerprint("labore")
///              .doit();
/// # }
/// ```
pub struct AccountContainerRuleUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Rule,
    _account_id: String,
    _container_id: String,
    _rule_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerRuleUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerRuleUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Rule)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.rules.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("ruleId", self._rule_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "ruleId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/rules/{ruleId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{ruleId}", "ruleId")].iter() {
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
            for param_name in ["accountId", "containerId", "ruleId"].iter() {
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
    pub fn request(mut self, new_value: &Rule) -> AccountContainerRuleUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerRuleUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerRuleUpdateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *rule id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Rule ID.    
    pub fn rule_id(mut self, new_value: &str) -> AccountContainerRuleUpdateCall<'a, C, NC, A> {
        self._rule_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the rule in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerRuleUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerRuleUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerRuleUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerRuleUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a GTM Rule.
///
/// A builder for the *containers.rules.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_rules_delete("accountId", "containerId", "ruleId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerRuleDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _rule_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerRuleDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerRuleDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.rules.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("ruleId", self._rule_id.to_string()));
        for &field in ["accountId", "containerId", "ruleId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/rules/{ruleId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{ruleId}", "ruleId")].iter() {
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
            for param_name in ["accountId", "containerId", "ruleId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerRuleDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerRuleDeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *rule id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Rule ID.    
    pub fn rule_id(mut self, new_value: &str) -> AccountContainerRuleDeleteCall<'a, C, NC, A> {
        self._rule_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerRuleDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerRuleDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerRuleDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all GTM Tags of a Container.
///
/// A builder for the *containers.tags.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_tags_list("accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTagListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTagListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTagListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListTagsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.tags.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/tags".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTagListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTagListCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTagListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTagListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTagListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Publishes a Container Version.
///
/// A builder for the *containers.versions.publish* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_publish("accountId", "containerId", "containerVersionId")
///              .fingerprint("duo")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionPublishCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _container_version_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionPublishCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionPublishCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PublishContainerVersionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.publish", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("containerVersionId", self._container_version_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "containerVersionId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions/{containerVersionId}/publish".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{containerVersionId}", "containerVersionId")].iter() {
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
            for param_name in ["accountId", "containerId", "containerVersionId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionPublishCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionPublishCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *container version id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container Version ID.    
    pub fn container_version_id(mut self, new_value: &str) -> AccountContainerVersionPublishCall<'a, C, NC, A> {
        self._container_version_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the container version in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerVersionPublishCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionPublishCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionPublishCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionPublishCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a GTM Tag.
///
/// A builder for the *containers.tags.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Tag;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Tag = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_tags_create(&req, "accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTagCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Tag,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTagCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTagCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Tag)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.tags.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/tags".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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
    pub fn request(mut self, new_value: &Tag) -> AccountContainerTagCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTagCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTagCreateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTagCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTagCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTagCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all GTM Triggers of a Container.
///
/// A builder for the *containers.triggers.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_triggers_list("accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTriggerListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTriggerListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTriggerListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListTriggersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.triggers.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/triggers".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTriggerListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTriggerListCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTriggerListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTriggerListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTriggerListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a Container Version.
///
/// A builder for the *containers.versions.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_delete("accountId", "containerId", "containerVersionId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _container_version_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("containerVersionId", self._container_version_id.to_string()));
        for &field in ["accountId", "containerId", "containerVersionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions/{containerVersionId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{containerVersionId}", "containerVersionId")].iter() {
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
            for param_name in ["accountId", "containerId", "containerVersionId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionDeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *container version id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container Version ID.    
    pub fn container_version_id(mut self, new_value: &str) -> AccountContainerVersionDeleteCall<'a, C, NC, A> {
        self._container_version_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a GTM Account.
///
/// A builder for the *update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Account;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Account = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().update(&req, "accountId")
///              .fingerprint("ea")
///              .doit();
/// # }
/// ```
pub struct AccountUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Account,
    _account_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Account)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
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
            for param_name in ["accountId"].iter() {
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
    pub fn request(mut self, new_value: &Account) -> AccountUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the account in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a GTM Macro.
///
/// A builder for the *containers.macros.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_macros_delete("accountId", "containerId", "macroId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerMacroDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _macro_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerMacroDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerMacroDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.macros.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("macroId", self._macro_id.to_string()));
        for &field in ["accountId", "containerId", "macroId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/macros/{macroId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{macroId}", "macroId")].iter() {
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
            for param_name in ["accountId", "containerId", "macroId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerMacroDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerMacroDeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *macro id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Macro ID.    
    pub fn macro_id(mut self, new_value: &str) -> AccountContainerMacroDeleteCall<'a, C, NC, A> {
        self._macro_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerMacroDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerMacroDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerMacroDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a Container Version.
///
/// A builder for the *containers.versions.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::CreateContainerVersionRequestVersionOptions;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: CreateContainerVersionRequestVersionOptions = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_create(&req, "accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: CreateContainerVersionRequestVersionOptions,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CreateContainerVersionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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
    pub fn request(mut self, new_value: &CreateContainerVersionRequestVersionOptions) -> AccountContainerVersionCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionCreateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a user's Account & Container Permissions.
///
/// A builder for the *permissions.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().permissions_get("accountId", "permissionId")
///              .doit();
/// # }
/// ```
pub struct AccountPermissionGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _permission_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountPermissionGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountPermissionGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UserAccess)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.permissions.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        for &field in ["alt", "accountId", "permissionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/permissions/{permissionId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{permissionId}", "permissionId")].iter() {
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
            for param_name in ["accountId", "permissionId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountPermissionGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM User ID.    
    pub fn permission_id(mut self, new_value: &str) -> AccountPermissionGetCall<'a, C, NC, A> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountPermissionGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountPermissionGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountPermissionGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a GTM Rule.
///
/// A builder for the *containers.rules.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Rule;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Rule = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_rules_create(&req, "accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerRuleCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Rule,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerRuleCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerRuleCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Rule)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.rules.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/rules".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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
    pub fn request(mut self, new_value: &Rule) -> AccountContainerRuleCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerRuleCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerRuleCreateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerRuleCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerRuleCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerRuleCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Restores a Container Version. This will overwrite the container's current configuration (including its macros, rules and tags). The operation will not have any effect on the version that is being served (i.e. the published version).
///
/// A builder for the *containers.versions.restore* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_restore("accountId", "containerId", "containerVersionId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionRestoreCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _container_version_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionRestoreCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionRestoreCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ContainerVersion)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.restore", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("containerVersionId", self._container_version_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "containerVersionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions/{containerVersionId}/restore".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{containerVersionId}", "containerVersionId")].iter() {
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
            for param_name in ["accountId", "containerId", "containerVersionId"].iter() {
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
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionRestoreCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionRestoreCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *container version id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container Version ID.    
    pub fn container_version_id(mut self, new_value: &str) -> AccountContainerVersionRestoreCall<'a, C, NC, A> {
        self._container_version_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionRestoreCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionRestoreCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionRestoreCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a GTM Rule.
///
/// A builder for the *containers.rules.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_rules_get("accountId", "containerId", "ruleId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerRuleGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _rule_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerRuleGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerRuleGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Rule)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.rules.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("ruleId", self._rule_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "ruleId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/rules/{ruleId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{ruleId}", "ruleId")].iter() {
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
            for param_name in ["accountId", "containerId", "ruleId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerRuleGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerRuleGetCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *rule id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Rule ID.    
    pub fn rule_id(mut self, new_value: &str) -> AccountContainerRuleGetCall<'a, C, NC, A> {
        self._rule_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerRuleGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerRuleGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerRuleGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a GTM Variable.
///
/// A builder for the *containers.variables.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Variable;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Variable = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_variables_create(&req, "accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVariableCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Variable,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVariableCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVariableCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Variable)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.variables.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/variables".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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
    pub fn request(mut self, new_value: &Variable) -> AccountContainerVariableCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVariableCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVariableCreateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVariableCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVariableCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVariableCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all GTM Variables of a Container.
///
/// A builder for the *containers.variables.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_variables_list("accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVariableListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVariableListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVariableListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListVariablesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.variables.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/variables".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVariableListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVariableListCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVariableListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVariableListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVariableListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a GTM Macro.
///
/// A builder for the *containers.macros.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Macro;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Macro = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_macros_create(&req, "accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerMacroCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Macro,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerMacroCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerMacroCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Macro)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.macros.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/macros".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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
    pub fn request(mut self, new_value: &Macro) -> AccountContainerMacroCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerMacroCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerMacroCreateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerMacroCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerMacroCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerMacroCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a GTM Tag.
///
/// A builder for the *containers.tags.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_tags_get("accountId", "containerId", "tagId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTagGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _tag_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTagGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTagGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Tag)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.tags.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("tagId", self._tag_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "tagId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/tags/{tagId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{tagId}", "tagId")].iter() {
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
            for param_name in ["accountId", "containerId", "tagId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTagGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTagGetCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *tag id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Tag ID.    
    pub fn tag_id(mut self, new_value: &str) -> AccountContainerTagGetCall<'a, C, NC, A> {
        self._tag_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTagGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTagGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTagGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a GTM Variable.
///
/// A builder for the *containers.variables.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_variables_get("accountId", "containerId", "variableId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVariableGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _variable_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVariableGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVariableGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Variable)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.variables.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("variableId", self._variable_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "variableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/variables/{variableId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{variableId}", "variableId")].iter() {
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
            for param_name in ["accountId", "containerId", "variableId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVariableGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVariableGetCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *variable id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Variable ID.    
    pub fn variable_id(mut self, new_value: &str) -> AccountContainerVariableGetCall<'a, C, NC, A> {
        self._variable_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVariableGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVariableGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVariableGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a GTM Trigger.
///
/// A builder for the *containers.triggers.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_triggers_delete("accountId", "containerId", "triggerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTriggerDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _trigger_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTriggerDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTriggerDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.triggers.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("triggerId", self._trigger_id.to_string()));
        for &field in ["accountId", "containerId", "triggerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/triggers/{triggerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{triggerId}", "triggerId")].iter() {
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
            for param_name in ["accountId", "containerId", "triggerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTriggerDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTriggerDeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *trigger id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Trigger ID.    
    pub fn trigger_id(mut self, new_value: &str) -> AccountContainerTriggerDeleteCall<'a, C, NC, A> {
        self._trigger_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTriggerDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTriggerDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTriggerDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all GTM Macros of a Container.
///
/// A builder for the *containers.macros.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_macros_list("accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerMacroListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerMacroListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerMacroListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListMacrosResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.macros.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/macros".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerMacroListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerMacroListCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerMacroListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerMacroListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerMacroListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a GTM Trigger.
///
/// A builder for the *containers.triggers.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Trigger;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Trigger = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_triggers_create(&req, "accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTriggerCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Trigger,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTriggerCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTriggerCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Trigger)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.triggers.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/triggers".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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
    pub fn request(mut self, new_value: &Trigger) -> AccountContainerTriggerCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTriggerCreateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTriggerCreateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTriggerCreateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTriggerCreateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTriggerCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a GTM Macro.
///
/// A builder for the *containers.macros.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Macro;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Macro = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_macros_update(&req, "accountId", "containerId", "macroId")
///              .fingerprint("sadipscing")
///              .doit();
/// # }
/// ```
pub struct AccountContainerMacroUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Macro,
    _account_id: String,
    _container_id: String,
    _macro_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerMacroUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerMacroUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Macro)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.macros.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("macroId", self._macro_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "macroId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/macros/{macroId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{macroId}", "macroId")].iter() {
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
            for param_name in ["accountId", "containerId", "macroId"].iter() {
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
    pub fn request(mut self, new_value: &Macro) -> AccountContainerMacroUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerMacroUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerMacroUpdateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *macro id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Macro ID.    
    pub fn macro_id(mut self, new_value: &str) -> AccountContainerMacroUpdateCall<'a, C, NC, A> {
        self._macro_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the macro in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerMacroUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerMacroUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerMacroUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerMacroUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all GTM Accounts that a user has access to.
///
/// A builder for the *list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().list()
///              .doit();
/// # }
/// ```
pub struct AccountListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListAccountsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
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


    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a user's Account & Container Permissions.
///
/// A builder for the *permissions.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::UserAccess;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: UserAccess = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().permissions_update(&req, "accountId", "permissionId")
///              .doit();
/// # }
/// ```
pub struct AccountPermissionUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: UserAccess,
    _account_id: String,
    _permission_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountPermissionUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountPermissionUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UserAccess)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.permissions.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("permissionId", self._permission_id.to_string()));
        for &field in ["alt", "accountId", "permissionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/permissions/{permissionId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{permissionId}", "permissionId")].iter() {
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
            for param_name in ["accountId", "permissionId"].iter() {
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
    pub fn request(mut self, new_value: &UserAccess) -> AccountPermissionUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountPermissionUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *permission id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM User ID.    
    pub fn permission_id(mut self, new_value: &str) -> AccountPermissionUpdateCall<'a, C, NC, A> {
        self._permission_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountPermissionUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountPermissionUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountPermissionUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a GTM Variable.
///
/// A builder for the *containers.variables.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_variables_delete("accountId", "containerId", "variableId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVariableDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _variable_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVariableDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVariableDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.variables.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("variableId", self._variable_id.to_string()));
        for &field in ["accountId", "containerId", "variableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/variables/{variableId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{variableId}", "variableId")].iter() {
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
            for param_name in ["accountId", "containerId", "variableId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVariableDeleteCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVariableDeleteCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *variable id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Variable ID.    
    pub fn variable_id(mut self, new_value: &str) -> AccountContainerVariableDeleteCall<'a, C, NC, A> {
        self._variable_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVariableDeleteCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVariableDeleteCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVariableDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a GTM Account.
///
/// A builder for the *get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().get("accountId")
///              .doit();
/// # }
/// ```
pub struct AccountGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Account)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
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
            for param_name in ["accountId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a Container.
///
/// A builder for the *containers.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Container;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Container = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_update(&req, "accountId", "containerId")
///              .fingerprint("labore")
///              .doit();
/// # }
/// ```
pub struct AccountContainerUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Container,
    _account_id: String,
    _container_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Container)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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
    pub fn request(mut self, new_value: &Container) -> AccountContainerUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerUpdateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the container in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all GTM Rules of a Container.
///
/// A builder for the *containers.rules.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_rules_list("accountId", "containerId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerRuleListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerRuleListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerRuleListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListRulesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.rules.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        for &field in ["alt", "accountId", "containerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/rules".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId")].iter() {
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
            for param_name in ["accountId", "containerId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerRuleListCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerRuleListCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerRuleListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerRuleListCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerRuleListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a GTM Tag.
///
/// A builder for the *containers.tags.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Tag;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Tag = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_tags_update(&req, "accountId", "containerId", "tagId")
///              .fingerprint("tempor")
///              .doit();
/// # }
/// ```
pub struct AccountContainerTagUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Tag,
    _account_id: String,
    _container_id: String,
    _tag_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerTagUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerTagUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Tag)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.tags.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("tagId", self._tag_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "tagId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/tags/{tagId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{tagId}", "tagId")].iter() {
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
            for param_name in ["accountId", "containerId", "tagId"].iter() {
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
    pub fn request(mut self, new_value: &Tag) -> AccountContainerTagUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerTagUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerTagUpdateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *tag id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Tag ID.    
    pub fn tag_id(mut self, new_value: &str) -> AccountContainerTagUpdateCall<'a, C, NC, A> {
        self._tag_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the tag in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerTagUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerTagUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerTagUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerTagUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a GTM Macro.
///
/// A builder for the *containers.macros.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_macros_get("accountId", "containerId", "macroId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerMacroGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _macro_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerMacroGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerMacroGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Macro)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.macros.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("macroId", self._macro_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "macroId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/macros/{macroId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{macroId}", "macroId")].iter() {
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
            for param_name in ["accountId", "containerId", "macroId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerMacroGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerMacroGetCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *macro id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Macro ID.    
    pub fn macro_id(mut self, new_value: &str) -> AccountContainerMacroGetCall<'a, C, NC, A> {
        self._macro_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerMacroGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerMacroGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerMacroGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a Container Version.
///
/// A builder for the *containers.versions.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::ContainerVersion;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ContainerVersion = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_update(&req, "accountId", "containerId", "containerVersionId")
///              .fingerprint("diam")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: ContainerVersion,
    _account_id: String,
    _container_id: String,
    _container_version_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ContainerVersion)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("containerVersionId", self._container_version_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "containerVersionId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions/{containerVersionId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{containerVersionId}", "containerVersionId")].iter() {
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
            for param_name in ["accountId", "containerId", "containerVersionId"].iter() {
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
    pub fn request(mut self, new_value: &ContainerVersion) -> AccountContainerVersionUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionUpdateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *container version id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container Version ID.    
    pub fn container_version_id(mut self, new_value: &str) -> AccountContainerVersionUpdateCall<'a, C, NC, A> {
        self._container_version_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the container version in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerVersionUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a GTM Variable.
///
/// A builder for the *containers.variables.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// use tagmanager1::Variable;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Variable = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_variables_update(&req, "accountId", "containerId", "variableId")
///              .fingerprint("amet")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVariableUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _request: Variable,
    _account_id: String,
    _container_id: String,
    _variable_id: String,
    _fingerprint: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVariableUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVariableUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Variable)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.variables.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("variableId", self._variable_id.to_string()));
        if let Some(value) = self._fingerprint {
            params.push(("fingerprint", value.to_string()));
        }
        for &field in ["alt", "accountId", "containerId", "variableId", "fingerprint"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/variables/{variableId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DeleteContainer.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{variableId}", "variableId")].iter() {
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
            for param_name in ["accountId", "containerId", "variableId"].iter() {
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
    pub fn request(mut self, new_value: &Variable) -> AccountContainerVariableUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVariableUpdateCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVariableUpdateCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *variable id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Variable ID.    
    pub fn variable_id(mut self, new_value: &str) -> AccountContainerVariableUpdateCall<'a, C, NC, A> {
        self._variable_id = new_value.to_string();
        self
    }
    /// Sets the *fingerprint* query property to the given value.
    ///
    /// 
    /// When provided, this fingerprint must match the fingerprint of the variable in storage.    
    pub fn fingerprint(mut self, new_value: &str) -> AccountContainerVariableUpdateCall<'a, C, NC, A> {
        self._fingerprint = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVariableUpdateCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVariableUpdateCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVariableUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Gets a Container Version.
///
/// A builder for the *containers.versions.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-tagmanager1" as tagmanager1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use tagmanager1::TagManager;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = TagManager::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().containers_versions_get("accountId", "containerId", "containerVersionId")
///              .doit();
/// # }
/// ```
pub struct AccountContainerVersionGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a TagManager<C, NC, A>,
    _account_id: String,
    _container_id: String,
    _container_version_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AccountContainerVersionGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AccountContainerVersionGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ContainerVersion)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "tagmanager.accounts.containers.versions.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("accountId", self._account_id.to_string()));
        params.push(("containerId", self._container_id.to_string()));
        params.push(("containerVersionId", self._container_version_id.to_string()));
        for &field in ["alt", "accountId", "containerId", "containerVersionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/tagmanager/v1/accounts/{accountId}/containers/{containerId}/versions/{containerVersionId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{containerId}", "containerId"), ("{containerVersionId}", "containerVersionId")].iter() {
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
            for param_name in ["accountId", "containerId", "containerVersionId"].iter() {
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


    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Account ID.    
    pub fn account_id(mut self, new_value: &str) -> AccountContainerVersionGetCall<'a, C, NC, A> {
        self._account_id = new_value.to_string();
        self
    }
    /// Sets the *container id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container ID.    
    pub fn container_id(mut self, new_value: &str) -> AccountContainerVersionGetCall<'a, C, NC, A> {
        self._container_id = new_value.to_string();
        self
    }
    /// Sets the *container version id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The GTM Container Version ID. Specify published to retrieve the currently published version.    
    pub fn container_version_id(mut self, new_value: &str) -> AccountContainerVersionGetCall<'a, C, NC, A> {
        self._container_version_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountContainerVersionGetCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountContainerVersionGetCall<'a, C, NC, A>
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
    pub fn add_scope<T>(mut self, scope: T) -> AccountContainerVersionGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


