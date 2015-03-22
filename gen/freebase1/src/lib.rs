// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *freebase* crate version *0.1.0+20150313*, where *20150313* is the exact revision of the *freebase:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.0*.
//! 
//! Everything else about the *freebase* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/freebase/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/freebase1).
//! # Features
//! 
//! Use the following functionality with ease from the central [hub](struct.Freebase.html) ... 
//! 
//! 
//! * [reconcile](struct.MethodReconcileCall.html)
//! * [search](struct.MethodSearchCall.html)
//! 
//! Download supported by ...
//! 
//! * [*search*](struct.MethodSearchCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Freebase.html)**
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
//! let r = hub.methods().reconcile(...).doit()
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
//! google-freebase1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-freebase1" as freebase1;
//! use freebase1::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use freebase1::Freebase;
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
//! let mut hub = Freebase::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.methods().reconcile()
//!              .add_prop("kasd")
//!              .name("accusam")
//!              .limit(-8)
//!              .add_lang("justo")
//!              .add_kind("amet.")
//!              .confidence(0.204946002078)
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




// ########
// HUB ###
// ######

/// Central instance to access all Freebase related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-freebase1" as freebase1;
/// use freebase1::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use freebase1::Freebase;
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
/// let mut hub = Freebase::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().reconcile()
///              .add_prop("labore")
///              .name("sea")
///              .limit(-90)
///              .add_lang("dolores")
///              .add_kind("gubergren")
///              .confidence(0.0653431304201)
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
pub struct Freebase<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Freebase<C, NC, A> {}

impl<'a, C, NC, A> Freebase<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Freebase<C, NC, A> {
        Freebase {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.0".to_string(),
            _m: PhantomData
        }
    }

    pub fn methods(&'a self) -> MethodMethods<'a, C, NC, A> {
        MethodMethods { hub: &self }
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
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReconcileCandidate {
    /// Language code that candidate and notable names are displayed in.    
    pub lang: String,
    /// Percentage likelihood that this candidate is the unique matching entity. Value will be between 0.0 and 1.0    
    pub confidence: f32,
    /// Type or profession the candidate is notable for.    
    pub notable: ReconcileCandidateNotable,
    /// Freebase MID of candidate entity.    
    pub mid: String,
    /// Freebase name of matching entity in specified language.    
    pub name: String,
}

impl Part for ReconcileCandidate {}


/// If filled, then there were recoverable problems that affected the request. For example, some of the properties were ignored because they either are not valid Freebase predicates or are not indexed for reconciliation. The candidates returned should be considered valid results, with the caveat that sections of the request were ignored as specified by the warning text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReconcileGetWarning {
    /// Code for identifying classes of warnings.    
    pub reason: String,
    /// Warning message to display to the user.    
    pub message: String,
    /// Location of warning in the request e.g. invalid predicate.    
    pub location: String,
}

impl NestedType for ReconcileGetWarning {}
impl Part for ReconcileGetWarning {}


/// Server costs for reconciling.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReconcileGetCosts {
    /// Total number of hits found.    
    pub hits: i32,
    /// Total milliseconds spent.    
    pub ms: i32,
}

impl NestedType for ReconcileGetCosts {}
impl Part for ReconcileGetCosts {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reconcile](struct.MethodReconcileCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReconcileGet {
    /// Server costs for reconciling.    
    pub costs: ReconcileGetCosts,
    /// If filled, then there were recoverable problems that affected the request. For example, some of the properties were ignored because they either are not valid Freebase predicates or are not indexed for reconciliation. The candidates returned should be considered valid results, with the caveat that sections of the request were ignored as specified by the warning text.    
    pub warning: Vec<ReconcileGetWarning>,
    /// If filled, then the listed candidates are potential matches, and such should be evaluated by a more discerning algorithm or human. The matches are ordered by confidence.    
    pub candidate: Vec<ReconcileCandidate>,
    /// If filled, this entity is guaranteed to match at requested confidence probability (default 99%).    
    #[serde(alias="match")]
    pub match_: ReconcileCandidate,
}

impl ResponseResult for ReconcileGet {}


/// Type or profession the candidate is notable for.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ReconcileCandidateNotable {
    /// MID of notable category.    
    pub id: String,
    /// Name of notable category in specified language.    
    pub name: String,
}

impl NestedType for ReconcileCandidateNotable {}
impl Part for ReconcileCandidateNotable {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the `Freebase` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-freebase1" as freebase1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use freebase1::Freebase;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Freebase::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `reconcile(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Freebase<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for MethodMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> MethodMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reconcile entities to Freebase open data.    
    pub fn reconcile(&self) -> MethodReconcileCall<'a, C, NC, A> {
        MethodReconcileCall {
            hub: self.hub,
            _prop: Default::default(),
            _name: Default::default(),
            _limit: Default::default(),
            _lang: Default::default(),
            _kind: Default::default(),
            _confidence: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search Freebase open data.    
    pub fn search(&self) -> MethodSearchCall<'a, C, NC, A> {
        MethodSearchCall {
            hub: self.hub,
            _without: Default::default(),
            _with: Default::default(),
            _type_: Default::default(),
            _stemmed: Default::default(),
            _spell: Default::default(),
            _scoring: Default::default(),
            _query: Default::default(),
            _prefixed: Default::default(),
            _output: Default::default(),
            _mql_output: Default::default(),
            _mid: Default::default(),
            _limit: Default::default(),
            _lang: Default::default(),
            _indent: Default::default(),
            _help: Default::default(),
            _format: Default::default(),
            _filter: Default::default(),
            _exact: Default::default(),
            _encode: Default::default(),
            _domain: Default::default(),
            _cursor: Default::default(),
            _callback: Default::default(),
            _as_of_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Reconcile entities to Freebase open data.
///
/// A builder for the *reconcile* method.
/// It is not used directly, but through a `MethodMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-freebase1" as freebase1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use freebase1::Freebase;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Freebase::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().reconcile()
///              .add_prop("aliquyam")
///              .name("ea")
///              .limit(-61)
///              .add_lang("justo")
///              .add_kind("justo")
///              .confidence(0.663748882844)
///              .doit();
/// # }
/// ```
pub struct MethodReconcileCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Freebase<C, NC, A>,
    _prop: Vec<String>,
    _name: Option<String>,
    _limit: Option<i32>,
    _lang: Vec<String>,
    _kind: Vec<String>,
    _confidence: Option<f32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for MethodReconcileCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MethodReconcileCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ReconcileGet)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "freebase.reconcile", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        if self._prop.len() > 0 {
            let mut s = String::new();
            for f in self._prop.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("prop", s));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._limit {
            params.push(("limit", value.to_string()));
        }
        if self._lang.len() > 0 {
            let mut s = String::new();
            for f in self._lang.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("lang", s));
        }
        if self._kind.len() > 0 {
            let mut s = String::new();
            for f in self._kind.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("kind", s));
        }
        if let Some(value) = self._confidence {
            params.push(("confidence", value.to_string()));
        }
        for &field in ["alt", "prop", "name", "limit", "lang", "kind", "confidence"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/freebase/v1/reconcile".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()));

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


    /// Append the given value to the *prop* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// Property values for entity formatted as
    /// :
    pub fn add_prop(mut self, new_value: &str) -> MethodReconcileCall<'a, C, NC, A> {
        self._prop.push(new_value.to_string());
        self
    }
    /// Sets the *name* query property to the given value.
    ///
    /// 
    /// Name of entity.    
    pub fn name(mut self, new_value: &str) -> MethodReconcileCall<'a, C, NC, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Sets the *limit* query property to the given value.
    ///
    /// 
    /// Maximum number of candidates to return.    
    pub fn limit(mut self, new_value: i32) -> MethodReconcileCall<'a, C, NC, A> {
        self._limit = Some(new_value);
        self
    }
    /// Append the given value to the *lang* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// Languages for names and values. First language is used for display. Default is 'en'.    
    pub fn add_lang(mut self, new_value: &str) -> MethodReconcileCall<'a, C, NC, A> {
        self._lang.push(new_value.to_string());
        self
    }
    /// Append the given value to the *kind* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// Classifications of entity e.g. type, category, title.    
    pub fn add_kind(mut self, new_value: &str) -> MethodReconcileCall<'a, C, NC, A> {
        self._kind.push(new_value.to_string());
        self
    }
    /// Sets the *confidence* query property to the given value.
    ///
    /// 
    /// Required confidence for a candidate to match. Must be between .5 and 1.0    
    pub fn confidence(mut self, new_value: f32) -> MethodReconcileCall<'a, C, NC, A> {
        self._confidence = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodReconcileCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodReconcileCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Search Freebase open data.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
///
/// A builder for the *search* method.
/// It is not used directly, but through a `MethodMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-freebase1" as freebase1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use freebase1::Freebase;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Freebase::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().search()
///              .add_without("et")
///              .add_with("diam")
///              .add_type("ipsum")
///              .stemmed(true)
///              .spell("et")
///              .scoring("duo")
///              .query("aliquyam")
///              .prefixed(true)
///              .output("Lorem")
///              .mql_output("eos")
///              .add_mid("erat")
///              .limit(-95)
///              .add_lang("dolor")
///              .indent(true)
///              .help("elitr")
///              .format("amet")
///              .add_filter("no")
///              .exact(true)
///              .encode("eirmod")
///              .add_domain("dolore")
///              .cursor(-37)
///              .callback("aliquyam")
///              .as_of_time("accusam")
///              .doit();
/// # }
/// ```
pub struct MethodSearchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Freebase<C, NC, A>,
    _without: Vec<String>,
    _with: Vec<String>,
    _type_: Vec<String>,
    _stemmed: Option<bool>,
    _spell: Option<String>,
    _scoring: Option<String>,
    _query: Option<String>,
    _prefixed: Option<bool>,
    _output: Option<String>,
    _mql_output: Option<String>,
    _mid: Vec<String>,
    _limit: Option<i32>,
    _lang: Vec<String>,
    _indent: Option<bool>,
    _help: Option<String>,
    _format: Option<String>,
    _filter: Vec<String>,
    _exact: Option<bool>,
    _encode: Option<String>,
    _domain: Vec<String>,
    _cursor: Option<i32>,
    _callback: Option<String>,
    _as_of_time: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for MethodSearchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MethodSearchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "freebase.search", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((24 + self._additional_params.len()));
        if self._without.len() > 0 {
            let mut s = String::new();
            for f in self._without.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("without", s));
        }
        if self._with.len() > 0 {
            let mut s = String::new();
            for f in self._with.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("with", s));
        }
        if self._type_.len() > 0 {
            let mut s = String::new();
            for f in self._type_.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("type", s));
        }
        if let Some(value) = self._stemmed {
            params.push(("stemmed", value.to_string()));
        }
        if let Some(value) = self._spell {
            params.push(("spell", value.to_string()));
        }
        if let Some(value) = self._scoring {
            params.push(("scoring", value.to_string()));
        }
        if let Some(value) = self._query {
            params.push(("query", value.to_string()));
        }
        if let Some(value) = self._prefixed {
            params.push(("prefixed", value.to_string()));
        }
        if let Some(value) = self._output {
            params.push(("output", value.to_string()));
        }
        if let Some(value) = self._mql_output {
            params.push(("mql_output", value.to_string()));
        }
        if self._mid.len() > 0 {
            let mut s = String::new();
            for f in self._mid.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("mid", s));
        }
        if let Some(value) = self._limit {
            params.push(("limit", value.to_string()));
        }
        if self._lang.len() > 0 {
            let mut s = String::new();
            for f in self._lang.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("lang", s));
        }
        if let Some(value) = self._indent {
            params.push(("indent", value.to_string()));
        }
        if let Some(value) = self._help {
            params.push(("help", value.to_string()));
        }
        if let Some(value) = self._format {
            params.push(("format", value.to_string()));
        }
        if self._filter.len() > 0 {
            let mut s = String::new();
            for f in self._filter.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("filter", s));
        }
        if let Some(value) = self._exact {
            params.push(("exact", value.to_string()));
        }
        if let Some(value) = self._encode {
            params.push(("encode", value.to_string()));
        }
        if self._domain.len() > 0 {
            let mut s = String::new();
            for f in self._domain.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("domain", s));
        }
        if let Some(value) = self._cursor {
            params.push(("cursor", value.to_string()));
        }
        if let Some(value) = self._callback {
            params.push(("callback", value.to_string()));
        }
        if let Some(value) = self._as_of_time {
            params.push(("as_of_time", value.to_string()));
        }
        for &field in ["without", "with", "type", "stemmed", "spell", "scoring", "query", "prefixed", "output", "mql_output", "mid", "limit", "lang", "indent", "help", "format", "filter", "exact", "encode", "domain", "cursor", "callback", "as_of_time"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/freebase/v1/search".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()));

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


    /// Append the given value to the *without* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A rule to not match against.    
    pub fn add_without(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._without.push(new_value.to_string());
        self
    }
    /// Append the given value to the *with* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A rule to match against.    
    pub fn add_with(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._with.push(new_value.to_string());
        self
    }
    /// Append the given value to the *type* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// Restrict to topics with this Freebase type id.    
    pub fn add_type(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._type_.push(new_value.to_string());
        self
    }
    /// Sets the *stemmed* query property to the given value.
    ///
    /// 
    /// Query on stemmed names and aliases. May not be used with prefixed.    
    pub fn stemmed(mut self, new_value: bool) -> MethodSearchCall<'a, C, NC, A> {
        self._stemmed = Some(new_value);
        self
    }
    /// Sets the *spell* query property to the given value.
    ///
    /// 
    /// Request 'did you mean' suggestions    
    pub fn spell(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._spell = Some(new_value.to_string());
        self
    }
    /// Sets the *scoring* query property to the given value.
    ///
    /// 
    /// Relevance scoring algorithm to use.    
    pub fn scoring(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._scoring = Some(new_value.to_string());
        self
    }
    /// Sets the *query* query property to the given value.
    ///
    /// 
    /// Query term to search for.    
    pub fn query(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._query = Some(new_value.to_string());
        self
    }
    /// Sets the *prefixed* query property to the given value.
    ///
    /// 
    /// Prefix match against names and aliases.    
    pub fn prefixed(mut self, new_value: bool) -> MethodSearchCall<'a, C, NC, A> {
        self._prefixed = Some(new_value);
        self
    }
    /// Sets the *output* query property to the given value.
    ///
    /// 
    /// An output expression to request data from matches.    
    pub fn output(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._output = Some(new_value.to_string());
        self
    }
    /// Sets the *mql_output* query property to the given value.
    ///
    /// 
    /// The MQL query to run againist the results to extract more data.    
    pub fn mql_output(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._mql_output = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *mid* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A mid to use instead of a query.    
    pub fn add_mid(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._mid.push(new_value.to_string());
        self
    }
    /// Sets the *limit* query property to the given value.
    ///
    /// 
    /// Maximum number of results to return.    
    pub fn limit(mut self, new_value: i32) -> MethodSearchCall<'a, C, NC, A> {
        self._limit = Some(new_value);
        self
    }
    /// Append the given value to the *lang* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// The code of the language to run the query with. Default is 'en'.    
    pub fn add_lang(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._lang.push(new_value.to_string());
        self
    }
    /// Sets the *indent* query property to the given value.
    ///
    /// 
    /// Whether to indent the json results or not.    
    pub fn indent(mut self, new_value: bool) -> MethodSearchCall<'a, C, NC, A> {
        self._indent = Some(new_value);
        self
    }
    /// Sets the *help* query property to the given value.
    ///
    /// 
    /// The keyword to request help on.    
    pub fn help(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._help = Some(new_value.to_string());
        self
    }
    /// Sets the *format* query property to the given value.
    ///
    /// 
    /// Structural format of the json response.    
    pub fn format(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._format = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *filter* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A filter to apply to the query.    
    pub fn add_filter(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._filter.push(new_value.to_string());
        self
    }
    /// Sets the *exact* query property to the given value.
    ///
    /// 
    /// Query on exact name and keys only.    
    pub fn exact(mut self, new_value: bool) -> MethodSearchCall<'a, C, NC, A> {
        self._exact = Some(new_value);
        self
    }
    /// Sets the *encode* query property to the given value.
    ///
    /// 
    /// The encoding of the response. You can use this parameter to enable html encoding.    
    pub fn encode(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._encode = Some(new_value.to_string());
        self
    }
    /// Append the given value to the *domain* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// Restrict to topics with this Freebase domain id.    
    pub fn add_domain(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._domain.push(new_value.to_string());
        self
    }
    /// Sets the *cursor* query property to the given value.
    ///
    /// 
    /// The cursor value to use for the next page of results.    
    pub fn cursor(mut self, new_value: i32) -> MethodSearchCall<'a, C, NC, A> {
        self._cursor = Some(new_value);
        self
    }
    /// Sets the *callback* query property to the given value.
    ///
    /// 
    /// JS method name for JSONP callbacks.    
    pub fn callback(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._callback = Some(new_value.to_string());
        self
    }
    /// Sets the *as_of_time* query property to the given value.
    ///
    /// 
    /// A mql as_of_time value to use with mql_output queries.    
    pub fn as_of_time(mut self, new_value: &str) -> MethodSearchCall<'a, C, NC, A> {
        self._as_of_time = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MethodSearchCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodSearchCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


