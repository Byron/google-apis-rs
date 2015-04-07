// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *audit* crate version *0.1.3+20130108*, where *20130108* is the exact revision of the *audit:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.3*.
//! 
//! Everything else about the *audit* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/google-apps/admin-audit/get_started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/audit1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Audit.html) ... 
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
//! * **[Hub](struct.Audit.html)**
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
//! google-audit1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_audit1 as audit1;
//! use audit1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use audit1::Audit;
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
//! let mut hub = Audit::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.activities().list("customerId", "applicationId")
//!              .start_time("sea")
//!              .max_results(-90)
//!              .event_name("dolores")
//!              .end_time("gubergren")
//!              .continuation_token("sadipscing")
//!              .caller("aliquyam")
//!              .actor_ip_address("ea")
//!              .actor_email("no")
//!              .actor_application_id("justo")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!         Error::MissingToken => println!("OAuth2: Missing Token"),
//!         Error::Cancelled => println!("Operation canceled by user"),
//!         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     },
//!     Ok(_) => println!("Success (value doesn't print)"),
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
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all Audit related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_audit1 as audit1;
/// use audit1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use audit1::Audit;
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
/// let mut hub = Audit::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list("customerId", "applicationId")
///              .start_time("et")
///              .max_results(-41)
///              .event_name("ipsum")
///              .end_time("Lorem")
///              .continuation_token("et")
///              .caller("duo")
///              .actor_ip_address("aliquyam")
///              .actor_email("sea")
///              .actor_application_id("Lorem")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
///         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///         Error::MissingToken => println!("OAuth2: Missing Token"),
///         Error::Cancelled => println!("Operation canceled by user"),
///         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     },
///     Ok(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Audit<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Audit<C, NC, A> {}

impl<'a, C, NC, A> Audit<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Audit<C, NC, A> {
        Audit {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.3".to_string(),
            _m: PhantomData
        }
    }

    pub fn activities(&'a self) -> ActivityMethods<'a, C, NC, A> {
        ActivityMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.3`.
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
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](struct.ActivityListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Activities {
    /// Each record in read response.
    pub items: Vec<Activity>,
    /// Kind of list response this is.
    pub kind: String,
    /// Next page URL.
    pub next: String,
}

impl ResponseResult for Activities {}


/// User doing the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ActivityActor {
    /// ID of application which interacted on behalf of the user.
    #[serde(alias="applicationId")]
    pub application_id: String,
    /// Email address of the user.
    pub email: String,
    /// For OAuth 2LO API requests, consumer_key of the requestor.
    pub key: String,
    /// User or OAuth 2LO request.
    #[serde(alias="callerType")]
    pub caller_type: String,
}

impl NestedType for ActivityActor {}
impl Part for ActivityActor {}


/// Unique identifier for each activity record.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ActivityId {
    /// Unique qualifier if multiple events have the same time.
    #[serde(alias="uniqQualifier")]
    pub uniq_qualifier: String,
    /// Application ID of the source application.
    #[serde(alias="applicationId")]
    pub application_id: String,
    /// Obfuscated customer ID of the source customer.
    #[serde(alias="customerId")]
    pub customer_id: String,
    /// Time of occurrence of the activity.
    pub time: String,
}

impl NestedType for ActivityId {}
impl Part for ActivityId {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Activity {
    /// Kind of resource this is.
    pub kind: String,
    /// Activity events.
    pub events: Vec<ActivityEvents>,
    /// IP Address of the user doing the action.
    #[serde(alias="ipAddress")]
    pub ip_address: String,
    /// Domain of source customer.
    #[serde(alias="ownerDomain")]
    pub owner_domain: String,
    /// User doing the action.
    pub actor: ActivityActor,
    /// Unique identifier for each activity record.
    pub id: ActivityId,
}

impl Part for Activity {}


/// Event parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ActivityEventsParameters {
    /// Name of the parameter.
    pub name: String,
    /// Value of the parameter.
    pub value: String,
}

impl NestedType for ActivityEventsParameters {}
impl Part for ActivityEventsParameters {}


/// Activity events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ActivityEvents {
    /// Type of event.
    #[serde(alias="eventType")]
    pub event_type: String,
    /// Name of event.
    pub name: String,
    /// Event parameters.
    pub parameters: Vec<ActivityEventsParameters>,
}

impl NestedType for ActivityEvents {}
impl Part for ActivityEvents {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the `Audit` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_audit1 as audit1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use audit1::Audit;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Audit::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Audit<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for ActivityMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of activities for a specific customer and application.
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Represents the customer who is the owner of target object on which action was performed.
    /// * `applicationId` - Application ID of the application on which the event was performed.
    pub fn list(&self, customer_id: &str, application_id: &str) -> ActivityListCall<'a, C, NC, A> {
        ActivityListCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _application_id: application_id.to_string(),
            _start_time: Default::default(),
            _max_results: Default::default(),
            _event_name: Default::default(),
            _end_time: Default::default(),
            _continuation_token: Default::default(),
            _caller: Default::default(),
            _actor_ip_address: Default::default(),
            _actor_email: Default::default(),
            _actor_application_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Retrieves a list of activities for a specific customer and application.
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
/// # extern crate google_audit1 as audit1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use audit1::Audit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Audit::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list("customerId", "applicationId")
///              .start_time("sadipscing")
///              .max_results(-48)
///              .event_name("eirmod")
///              .end_time("elitr")
///              .continuation_token("amet")
///              .caller("no")
///              .actor_ip_address("labore")
///              .actor_email("eirmod")
///              .actor_application_id("dolore")
///              .doit();
/// # }
/// ```
pub struct ActivityListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Audit<C, NC, A>,
    _customer_id: String,
    _application_id: String,
    _start_time: Option<String>,
    _max_results: Option<i32>,
    _event_name: Option<String>,
    _end_time: Option<String>,
    _continuation_token: Option<String>,
    _caller: Option<String>,
    _actor_ip_address: Option<String>,
    _actor_email: Option<String>,
    _actor_application_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for ActivityListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Activities)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "audit.activities.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((13 + self._additional_params.len()));
        params.push(("customerId", self._customer_id.to_string()));
        params.push(("applicationId", self._application_id.to_string()));
        if let Some(value) = self._start_time {
            params.push(("startTime", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._event_name {
            params.push(("eventName", value.to_string()));
        }
        if let Some(value) = self._end_time {
            params.push(("endTime", value.to_string()));
        }
        if let Some(value) = self._continuation_token {
            params.push(("continuationToken", value.to_string()));
        }
        if let Some(value) = self._caller {
            params.push(("caller", value.to_string()));
        }
        if let Some(value) = self._actor_ip_address {
            params.push(("actorIpAddress", value.to_string()));
        }
        if let Some(value) = self._actor_email {
            params.push(("actorEmail", value.to_string()));
        }
        if let Some(value) = self._actor_application_id {
            params.push(("actorApplicationId", value.to_string()));
        }
        for &field in ["alt", "customerId", "applicationId", "startTime", "maxResults", "eventName", "endTime", "continuationToken", "caller", "actorIpAddress", "actorEmail", "actorApplicationId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/apps/reporting/audit/v1/{customerId}/{applicationId}".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{customerId}", "customerId"), ("{applicationId}", "applicationId")].iter() {
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
            for param_name in ["customerId", "applicationId"].iter() {
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
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()));

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


    /// Sets the *customer id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Represents the customer who is the owner of target object on which action was performed.
    pub fn customer_id(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._customer_id = new_value.to_string();
        self
    }
    /// Sets the *application id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Application ID of the application on which the event was performed.
    pub fn application_id(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._application_id = new_value.to_string();
        self
    }
    /// Sets the *start time* query property to the given value.
    ///
    /// 
    /// Return events which occured at or after this time.
    pub fn start_time(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._start_time = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// Number of activity records to be shown in each page.
    pub fn max_results(mut self, new_value: i32) -> ActivityListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *event name* query property to the given value.
    ///
    /// 
    /// Name of the event being queried.
    pub fn event_name(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._event_name = Some(new_value.to_string());
        self
    }
    /// Sets the *end time* query property to the given value.
    ///
    /// 
    /// Return events which occured at or before this time.
    pub fn end_time(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._end_time = Some(new_value.to_string());
        self
    }
    /// Sets the *continuation token* query property to the given value.
    ///
    /// 
    /// Next page URL.
    pub fn continuation_token(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._continuation_token = Some(new_value.to_string());
        self
    }
    /// Sets the *caller* query property to the given value.
    ///
    /// 
    /// Type of the caller.
    pub fn caller(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._caller = Some(new_value.to_string());
        self
    }
    /// Sets the *actor ip address* query property to the given value.
    ///
    /// 
    /// IP Address of host where the event was performed. Supports both IPv4 and IPv6 addresses.
    pub fn actor_ip_address(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._actor_ip_address = Some(new_value.to_string());
        self
    }
    /// Sets the *actor email* query property to the given value.
    ///
    /// 
    /// Email address of the user who performed the action.
    pub fn actor_email(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._actor_email = Some(new_value.to_string());
        self
    }
    /// Sets the *actor application id* query property to the given value.
    ///
    /// 
    /// Application ID of the application which interacted on behalf of the user while performing the event.
    pub fn actor_application_id(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._actor_application_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ActivityListCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


