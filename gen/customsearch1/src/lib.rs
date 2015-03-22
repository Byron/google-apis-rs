// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *customsearch* crate version *0.1.0+20131205*, where *20131205* is the exact revision of the *customsearch:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.0*.
//! 
//! Everything else about the *customsearch* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/custom-search/v1/using_rest).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/customsearch1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Customsearch.html) ... 
//! 
//! * cse
//!  * [*list*](struct.CseListCall.html)
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
//! * **[Hub](struct.Customsearch.html)**
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
//! let r = hub.cse().list(...).doit()
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
//! google-customsearch1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-customsearch1" as customsearch1;
//! use customsearch1::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use customsearch1::Customsearch;
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
//! let mut hub = Customsearch::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.cse().list("q")
//!              .start(26)
//!              .sort("erat")
//!              .site_search_filter("sadipscing")
//!              .site_search("dolor")
//!              .search_type("eirmod")
//!              .safe("elitr")
//!              .rights("amet")
//!              .related_site("no")
//!              .or_terms("labore")
//!              .num(62)
//!              .lr("dolore")
//!              .low_range("invidunt")
//!              .link_site("aliquyam")
//!              .img_type("accusam")
//!              .img_size("Lorem")
//!              .img_dominant_color("sea")
//!              .img_color_type("et")
//!              .hq("duo")
//!              .hl("et")
//!              .high_range("eirmod")
//!              .googlehost("sanctus")
//!              .gl("et")
//!              .filter("amet")
//!              .file_type("et")
//!              .exclude_terms("consetetur")
//!              .exact_terms("ut")
//!              .date_restrict("ea")
//!              .cx("sed")
//!              .cref("dolor")
//!              .cr("dolor")
//!              .c2coff("dolor")
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

/// Central instance to access all Customsearch related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-customsearch1" as customsearch1;
/// use customsearch1::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use customsearch1::Customsearch;
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
/// let mut hub = Customsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().list("q")
///              .start(5)
///              .sort("amet.")
///              .site_search_filter("voluptua.")
///              .site_search("Lorem")
///              .search_type("gubergren")
///              .safe("justo")
///              .rights("sit")
///              .related_site("vero")
///              .or_terms("diam")
///              .num(35)
///              .lr("consetetur")
///              .low_range("sadipscing")
///              .link_site("vero")
///              .img_type("sadipscing")
///              .img_size("invidunt")
///              .img_dominant_color("consetetur")
///              .img_color_type("dolore")
///              .hq("duo")
///              .hl("aliquyam")
///              .high_range("Lorem")
///              .googlehost("et")
///              .gl("clita")
///              .filter("consetetur")
///              .file_type("takimata")
///              .exclude_terms("nonumy")
///              .exact_terms("kasd")
///              .date_restrict("sanctus")
///              .cx("takimata")
///              .cref("At")
///              .cr("labore")
///              .c2coff("invidunt")
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
pub struct Customsearch<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Customsearch<C, NC, A> {}

impl<'a, C, NC, A> Customsearch<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Customsearch<C, NC, A> {
        Customsearch {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.0".to_string(),
            _m: PhantomData
        }
    }

    pub fn cse(&'a self) -> CseMethods<'a, C, NC, A> {
        CseMethods { hub: &self }
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
pub struct ResultImage {
    /// no description provided    
    pub width: i32,
    /// no description provided    
    #[serde(alias="contextLink")]
    pub context_link: String,
    /// no description provided    
    #[serde(alias="thumbnailWidth")]
    pub thumbnail_width: i32,
    /// no description provided    
    #[serde(alias="thumbnailLink")]
    pub thumbnail_link: String,
    /// no description provided    
    #[serde(alias="byteSize")]
    pub byte_size: i32,
    /// no description provided    
    #[serde(alias="thumbnailHeight")]
    pub thumbnail_height: i32,
    /// no description provided    
    pub height: i32,
}

impl NestedType for ResultImage {}
impl Part for ResultImage {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list cse](struct.CseListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Search {
    /// no description provided    
    pub promotions: Vec<Promotion>,
    /// no description provided    
    pub kind: String,
    /// no description provided    
    pub url: SearchUrl,
    /// no description provided    
    pub items: Vec<ResultType>,
    /// no description provided    
    pub context: Context,
    /// no description provided    
    pub queries: HashMap<String, Vec<Query>>,
    /// no description provided    
    pub spelling: SearchSpelling,
    /// no description provided    
    #[serde(alias="searchInformation")]
    pub search_information: SearchSearchInformation,
}

impl ResponseResult for Search {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PromotionImage {
    /// no description provided    
    pub source: String,
    /// no description provided    
    pub height: i32,
    /// no description provided    
    pub width: i32,
}

impl NestedType for PromotionImage {}
impl Part for PromotionImage {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchUrl {
    /// no description provided    
    #[serde(alias="type")]
    pub type_: String,
    /// no description provided    
    pub template: String,
}

impl NestedType for SearchUrl {}
impl Part for SearchUrl {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchSpelling {
    /// no description provided    
    #[serde(alias="correctedQuery")]
    pub corrected_query: String,
    /// no description provided    
    #[serde(alias="htmlCorrectedQuery")]
    pub html_corrected_query: String,
}

impl NestedType for SearchSpelling {}
impl Part for SearchSpelling {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PromotionBodyLines {
    /// no description provided    
    pub url: String,
    /// no description provided    
    #[serde(alias="htmlTitle")]
    pub html_title: String,
    /// no description provided    
    pub link: String,
    /// no description provided    
    pub title: String,
}

impl NestedType for PromotionBodyLines {}
impl Part for PromotionBodyLines {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultType {
    /// no description provided    
    pub kind: String,
    /// no description provided    
    pub labels: Vec<ResultLabels>,
    /// no description provided    
    pub title: String,
    /// no description provided    
    #[serde(alias="displayLink")]
    pub display_link: String,
    /// no description provided    
    #[serde(alias="cacheId")]
    pub cache_id: String,
    /// no description provided    
    #[serde(alias="formattedUrl")]
    pub formatted_url: String,
    /// no description provided    
    #[serde(alias="htmlFormattedUrl")]
    pub html_formatted_url: String,
    /// no description provided    
    pub pagemap: HashMap<String, Vec<HashMap<String, String>>>,
    /// no description provided    
    #[serde(alias="fileFormat")]
    pub file_format: String,
    /// no description provided    
    pub snippet: String,
    /// no description provided    
    #[serde(alias="htmlSnippet")]
    pub html_snippet: String,
    /// no description provided    
    pub link: String,
    /// no description provided    
    pub image: ResultImage,
    /// no description provided    
    pub mime: String,
    /// no description provided    
    #[serde(alias="htmlTitle")]
    pub html_title: String,
}

impl Part for ResultType {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Context {
    /// no description provided    
    pub facets: Vec<Vec<ContextFacets>>,
    /// no description provided    
    pub title: String,
}

impl Part for Context {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ContextFacets {
    /// no description provided    
    pub label_with_op: String,
    /// no description provided    
    pub anchor: String,
    /// no description provided    
    pub label: String,
}

impl NestedType for ContextFacets {}
impl Part for ContextFacets {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Query {
    /// no description provided    
    #[serde(alias="dateRestrict")]
    pub date_restrict: String,
    /// no description provided    
    #[serde(alias="inputEncoding")]
    pub input_encoding: String,
    /// no description provided    
    #[serde(alias="orTerms")]
    pub or_terms: String,
    /// no description provided    
    #[serde(alias="highRange")]
    pub high_range: String,
    /// no description provided    
    pub cx: String,
    /// no description provided    
    #[serde(alias="startPage")]
    pub start_page: i32,
    /// no description provided    
    #[serde(alias="disableCnTwTranslation")]
    pub disable_cn_tw_translation: String,
    /// no description provided    
    pub cr: String,
    /// no description provided    
    #[serde(alias="imgType")]
    pub img_type: String,
    /// no description provided    
    #[serde(alias="relatedSite")]
    pub related_site: String,
    /// no description provided    
    pub gl: String,
    /// no description provided    
    #[serde(alias="searchType")]
    pub search_type: String,
    /// no description provided    
    pub title: String,
    /// no description provided    
    #[serde(alias="googleHost")]
    pub google_host: String,
    /// no description provided    
    #[serde(alias="fileType")]
    pub file_type: String,
    /// no description provided    
    #[serde(alias="imgDominantColor")]
    pub img_dominant_color: String,
    /// no description provided    
    #[serde(alias="siteSearch")]
    pub site_search: String,
    /// no description provided    
    pub cref: String,
    /// no description provided    
    pub sort: String,
    /// no description provided    
    pub hq: String,
    /// no description provided    
    #[serde(alias="outputEncoding")]
    pub output_encoding: String,
    /// no description provided    
    pub safe: String,
    /// no description provided    
    #[serde(alias="searchTerms")]
    pub search_terms: String,
    /// no description provided    
    #[serde(alias="exactTerms")]
    pub exact_terms: String,
    /// no description provided    
    #[serde(alias="imgColorType")]
    pub img_color_type: String,
    /// no description provided    
    pub hl: String,
    /// no description provided    
    #[serde(alias="totalResults")]
    pub total_results: String,
    /// no description provided    
    #[serde(alias="lowRange")]
    pub low_range: String,
    /// no description provided    
    pub count: i32,
    /// no description provided    
    #[serde(alias="imgSize")]
    pub img_size: String,
    /// no description provided    
    pub language: String,
    /// no description provided    
    pub rights: String,
    /// no description provided    
    #[serde(alias="startIndex")]
    pub start_index: i32,
    /// no description provided    
    #[serde(alias="excludeTerms")]
    pub exclude_terms: String,
    /// no description provided    
    pub filter: String,
    /// no description provided    
    #[serde(alias="linkSite")]
    pub link_site: String,
    /// no description provided    
    #[serde(alias="siteSearchFilter")]
    pub site_search_filter: String,
}

impl Part for Query {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Promotion {
    /// no description provided    
    #[serde(alias="bodyLines")]
    pub body_lines: Vec<PromotionBodyLines>,
    /// no description provided    
    pub title: String,
    /// no description provided    
    pub link: String,
    /// no description provided    
    #[serde(alias="displayLink")]
    pub display_link: String,
    /// no description provided    
    #[serde(alias="htmlTitle")]
    pub html_title: String,
    /// no description provided    
    pub image: PromotionImage,
}

impl Part for Promotion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchSearchInformation {
    /// no description provided    
    #[serde(alias="formattedSearchTime")]
    pub formatted_search_time: String,
    /// no description provided    
    #[serde(alias="formattedTotalResults")]
    pub formatted_total_results: String,
    /// no description provided    
    #[serde(alias="totalResults")]
    pub total_results: String,
    /// no description provided    
    #[serde(alias="searchTime")]
    pub search_time: f64,
}

impl NestedType for SearchSearchInformation {}
impl Part for SearchSearchInformation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultLabels {
    /// no description provided    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// no description provided    
    pub label_with_op: String,
    /// no description provided    
    pub name: String,
}

impl NestedType for ResultLabels {}
impl Part for ResultLabels {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *cse* resources.
/// It is not used directly, but through the `Customsearch` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-customsearch1" as customsearch1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use customsearch1::Customsearch;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Customsearch::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.cse();
/// # }
/// ```
pub struct CseMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Customsearch<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for CseMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> CseMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results.    
    pub fn list(&self, q: &str) -> CseListCall<'a, C, NC, A> {
        CseListCall {
            hub: self.hub,
            _q: q.to_string(),
            _start: Default::default(),
            _sort: Default::default(),
            _site_search_filter: Default::default(),
            _site_search: Default::default(),
            _search_type: Default::default(),
            _safe: Default::default(),
            _rights: Default::default(),
            _related_site: Default::default(),
            _or_terms: Default::default(),
            _num: Default::default(),
            _lr: Default::default(),
            _low_range: Default::default(),
            _link_site: Default::default(),
            _img_type: Default::default(),
            _img_size: Default::default(),
            _img_dominant_color: Default::default(),
            _img_color_type: Default::default(),
            _hq: Default::default(),
            _hl: Default::default(),
            _high_range: Default::default(),
            _googlehost: Default::default(),
            _gl: Default::default(),
            _filter: Default::default(),
            _file_type: Default::default(),
            _exclude_terms: Default::default(),
            _exact_terms: Default::default(),
            _date_restrict: Default::default(),
            _cx: Default::default(),
            _cref: Default::default(),
            _cr: Default::default(),
            _c2coff: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results.
///
/// A builder for the *list* method supported by a *cse* resource.
/// It is not used directly, but through a `CseMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-customsearch1" as customsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use customsearch1::Customsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Customsearch::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().list("q")
///              .start(6)
///              .sort("rebum.")
///              .site_search_filter("dolore")
///              .site_search("nonumy")
///              .search_type("sed")
///              .safe("aliquyam")
///              .rights("sit")
///              .related_site("eirmod")
///              .or_terms("consetetur")
///              .num(16)
///              .lr("sed")
///              .low_range("ea")
///              .link_site("gubergren")
///              .img_type("aliquyam")
///              .img_size("eos")
///              .img_dominant_color("tempor")
///              .img_color_type("sea")
///              .hq("labore")
///              .hl("ipsum")
///              .high_range("aliquyam")
///              .googlehost("dolores")
///              .gl("sit")
///              .filter("diam")
///              .file_type("ut")
///              .exclude_terms("justo")
///              .exact_terms("est")
///              .date_restrict("amet")
///              .cx("accusam")
///              .cref("clita")
///              .cr("diam")
///              .c2coff("justo")
///              .doit();
/// # }
/// ```
pub struct CseListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Customsearch<C, NC, A>,
    _q: String,
    _start: Option<u32>,
    _sort: Option<String>,
    _site_search_filter: Option<String>,
    _site_search: Option<String>,
    _search_type: Option<String>,
    _safe: Option<String>,
    _rights: Option<String>,
    _related_site: Option<String>,
    _or_terms: Option<String>,
    _num: Option<u32>,
    _lr: Option<String>,
    _low_range: Option<String>,
    _link_site: Option<String>,
    _img_type: Option<String>,
    _img_size: Option<String>,
    _img_dominant_color: Option<String>,
    _img_color_type: Option<String>,
    _hq: Option<String>,
    _hl: Option<String>,
    _high_range: Option<String>,
    _googlehost: Option<String>,
    _gl: Option<String>,
    _filter: Option<String>,
    _file_type: Option<String>,
    _exclude_terms: Option<String>,
    _exact_terms: Option<String>,
    _date_restrict: Option<String>,
    _cx: Option<String>,
    _cref: Option<String>,
    _cr: Option<String>,
    _c2coff: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for CseListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> CseListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "search.cse.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((34 + self._additional_params.len()));
        params.push(("q", self._q.to_string()));
        if let Some(value) = self._start {
            params.push(("start", value.to_string()));
        }
        if let Some(value) = self._sort {
            params.push(("sort", value.to_string()));
        }
        if let Some(value) = self._site_search_filter {
            params.push(("siteSearchFilter", value.to_string()));
        }
        if let Some(value) = self._site_search {
            params.push(("siteSearch", value.to_string()));
        }
        if let Some(value) = self._search_type {
            params.push(("searchType", value.to_string()));
        }
        if let Some(value) = self._safe {
            params.push(("safe", value.to_string()));
        }
        if let Some(value) = self._rights {
            params.push(("rights", value.to_string()));
        }
        if let Some(value) = self._related_site {
            params.push(("relatedSite", value.to_string()));
        }
        if let Some(value) = self._or_terms {
            params.push(("orTerms", value.to_string()));
        }
        if let Some(value) = self._num {
            params.push(("num", value.to_string()));
        }
        if let Some(value) = self._lr {
            params.push(("lr", value.to_string()));
        }
        if let Some(value) = self._low_range {
            params.push(("lowRange", value.to_string()));
        }
        if let Some(value) = self._link_site {
            params.push(("linkSite", value.to_string()));
        }
        if let Some(value) = self._img_type {
            params.push(("imgType", value.to_string()));
        }
        if let Some(value) = self._img_size {
            params.push(("imgSize", value.to_string()));
        }
        if let Some(value) = self._img_dominant_color {
            params.push(("imgDominantColor", value.to_string()));
        }
        if let Some(value) = self._img_color_type {
            params.push(("imgColorType", value.to_string()));
        }
        if let Some(value) = self._hq {
            params.push(("hq", value.to_string()));
        }
        if let Some(value) = self._hl {
            params.push(("hl", value.to_string()));
        }
        if let Some(value) = self._high_range {
            params.push(("highRange", value.to_string()));
        }
        if let Some(value) = self._googlehost {
            params.push(("googlehost", value.to_string()));
        }
        if let Some(value) = self._gl {
            params.push(("gl", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        if let Some(value) = self._file_type {
            params.push(("fileType", value.to_string()));
        }
        if let Some(value) = self._exclude_terms {
            params.push(("excludeTerms", value.to_string()));
        }
        if let Some(value) = self._exact_terms {
            params.push(("exactTerms", value.to_string()));
        }
        if let Some(value) = self._date_restrict {
            params.push(("dateRestrict", value.to_string()));
        }
        if let Some(value) = self._cx {
            params.push(("cx", value.to_string()));
        }
        if let Some(value) = self._cref {
            params.push(("cref", value.to_string()));
        }
        if let Some(value) = self._cr {
            params.push(("cr", value.to_string()));
        }
        if let Some(value) = self._c2coff {
            params.push(("c2coff", value.to_string()));
        }
        for &field in ["alt", "q", "start", "sort", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cref", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/customsearch/v1".to_string();
        
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


    /// Sets the *q* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Query    
    pub fn q(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._q = new_value.to_string();
        self
    }
    /// Sets the *start* query property to the given value.
    ///
    /// 
    /// The index of the first result to return    
    pub fn start(mut self, new_value: u32) -> CseListCall<'a, C, NC, A> {
        self._start = Some(new_value);
        self
    }
    /// Sets the *sort* query property to the given value.
    ///
    /// 
    /// The sort expression to apply to the results    
    pub fn sort(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Sets the *site search filter* query property to the given value.
    ///
    /// 
    /// Controls whether to include or exclude results from the site named in the as_sitesearch parameter    
    pub fn site_search_filter(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Sets the *site search* query property to the given value.
    ///
    /// 
    /// Specifies all search results should be pages from a given site    
    pub fn site_search(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Sets the *search type* query property to the given value.
    ///
    /// 
    /// Specifies the search type: image.    
    pub fn search_type(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Sets the *safe* query property to the given value.
    ///
    /// 
    /// Search safety level    
    pub fn safe(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Sets the *rights* query property to the given value.
    ///
    /// 
    /// Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these.    
    pub fn rights(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Sets the *related site* query property to the given value.
    ///
    /// 
    /// Specifies that all search results should be pages that are related to the specified URL    
    pub fn related_site(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Sets the *or terms* query property to the given value.
    ///
    /// 
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms    
    pub fn or_terms(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Sets the *num* query property to the given value.
    ///
    /// 
    /// Number of search results to return    
    pub fn num(mut self, new_value: u32) -> CseListCall<'a, C, NC, A> {
        self._num = Some(new_value);
        self
    }
    /// Sets the *lr* query property to the given value.
    ///
    /// 
    /// The language restriction for the search results    
    pub fn lr(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Sets the *low range* query property to the given value.
    ///
    /// 
    /// Creates a range in form as_nlo value..as_nhi value and attempts to append it to query    
    pub fn low_range(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Sets the *link site* query property to the given value.
    ///
    /// 
    /// Specifies that all search results should contain a link to a particular URL    
    pub fn link_site(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Sets the *img type* query property to the given value.
    ///
    /// 
    /// Returns images of a type, which can be one of: clipart, face, lineart, news, and photo.    
    pub fn img_type(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Sets the *img size* query property to the given value.
    ///
    /// 
    /// Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge.    
    pub fn img_size(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Sets the *img dominant color* query property to the given value.
    ///
    /// 
    /// Returns images of a specific dominant color: yellow, green, teal, blue, purple, pink, white, gray, black and brown.    
    pub fn img_dominant_color(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Sets the *img color type* query property to the given value.
    ///
    /// 
    /// Returns black and white, grayscale, or color images: mono, gray, and color.    
    pub fn img_color_type(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Sets the *hq* query property to the given value.
    ///
    /// 
    /// Appends the extra query terms to the query.    
    pub fn hq(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// Sets the user interface language.    
    pub fn hl(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Sets the *high range* query property to the given value.
    ///
    /// 
    /// Creates a range in form as_nlo value..as_nhi value and attempts to append it to query    
    pub fn high_range(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// Sets the *googlehost* query property to the given value.
    ///
    /// 
    /// The local Google domain to use to perform the search.    
    pub fn googlehost(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Sets the *gl* query property to the given value.
    ///
    /// 
    /// Geolocation of end user.    
    pub fn gl(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Sets the *filter* query property to the given value.
    ///
    /// 
    /// Controls turning on or off the duplicate content filter.    
    pub fn filter(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Sets the *file type* query property to the given value.
    ///
    /// 
    /// Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ...    
    pub fn file_type(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Sets the *exclude terms* query property to the given value.
    ///
    /// 
    /// Identifies a word or phrase that should not appear in any documents in the search results    
    pub fn exclude_terms(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Sets the *exact terms* query property to the given value.
    ///
    /// 
    /// Identifies a phrase that all documents in the search results must contain    
    pub fn exact_terms(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Sets the *date restrict* query property to the given value.
    ///
    /// 
    /// Specifies all search results are from a time period    
    pub fn date_restrict(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// Sets the *cx* query property to the given value.
    ///
    /// 
    /// The custom search engine ID to scope this search query    
    pub fn cx(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Sets the *cref* query property to the given value.
    ///
    /// 
    /// The URL of a linked custom search engine    
    pub fn cref(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._cref = Some(new_value.to_string());
        self
    }
    /// Sets the *cr* query property to the given value.
    ///
    /// 
    /// Country restrict(s).    
    pub fn cr(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Sets the *c2coff* query property to the given value.
    ///
    /// 
    /// Turns off the translation between zh-CN and zh-TW.    
    pub fn c2coff(mut self, new_value: &str) -> CseListCall<'a, C, NC, A> {
        self._c2coff = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CseListCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


