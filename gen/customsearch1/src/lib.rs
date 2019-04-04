// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *customsearch* crate version *1.0.8+20181001*, where *20181001* is the exact revision of the *customsearch:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *customsearch* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/custom-search/v1/using_rest).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/customsearch1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Customsearch.html) ... 
//! 
//! * cse
//!  * [*list*](struct.CseListCall.html) and [*siterestrict list*](struct.CseSiterestrictListCall.html)
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
//! * **[Hub](struct.Customsearch.html)**
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
//! let r = hub.cse().siterestrict_list(...).doit()
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
//! extern crate google_customsearch1 as customsearch1;
//! use customsearch1::{Result, Error};
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
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Customsearch::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.cse().siterestrict_list("q")
//!              .start(46)
//!              .sort("eos")
//!              .site_search_filter("erat")
//!              .site_search("sadipscing")
//!              .search_type("dolor")
//!              .safe("eirmod")
//!              .rights("elitr")
//!              .related_site("amet")
//!              .or_terms("no")
//!              .num(65)
//!              .lr("eirmod")
//!              .low_range("dolore")
//!              .link_site("invidunt")
//!              .img_type("aliquyam")
//!              .img_size("accusam")
//!              .img_dominant_color("Lorem")
//!              .img_color_type("sea")
//!              .hq("et")
//!              .hl("duo")
//!              .high_range("et")
//!              .googlehost("eirmod")
//!              .gl("sanctus")
//!              .filter("et")
//!              .file_type("amet")
//!              .exclude_terms("et")
//!              .exact_terms("consetetur")
//!              .date_restrict("ut")
//!              .cx("ea")
//!              .cr("sed")
//!              .c2coff("dolor")
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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_customsearch1 as customsearch1;
/// use customsearch1::{Result, Error};
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
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Customsearch::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().siterestrict_list("q")
///              .start(53)
///              .sort("et")
///              .site_search_filter("consetetur")
///              .site_search("amet.")
///              .search_type("voluptua.")
///              .safe("Lorem")
///              .rights("gubergren")
///              .related_site("justo")
///              .or_terms("sit")
///              .num(75)
///              .lr("diam")
///              .low_range("rebum.")
///              .link_site("consetetur")
///              .img_type("sadipscing")
///              .img_size("vero")
///              .img_dominant_color("sadipscing")
///              .img_color_type("invidunt")
///              .hq("consetetur")
///              .hl("dolore")
///              .high_range("duo")
///              .googlehost("aliquyam")
///              .gl("Lorem")
///              .filter("et")
///              .file_type("clita")
///              .exclude_terms("consetetur")
///              .exact_terms("takimata")
///              .date_restrict("nonumy")
///              .cx("kasd")
///              .cr("sanctus")
///              .c2coff("takimata")
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
pub struct Customsearch<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Customsearch<C, A> {}

impl<'a, C, A> Customsearch<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Customsearch<C, A> {
        Customsearch {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://www.googleapis.com/customsearch/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn cse(&'a self) -> CseMethods<'a, C, A> {
        CseMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/customsearch/`.
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
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultImage {
    /// no description provided
    pub width: Option<i32>,
    /// no description provided
    #[serde(rename="contextLink")]
    pub context_link: Option<String>,
    /// no description provided
    #[serde(rename="thumbnailWidth")]
    pub thumbnail_width: Option<i32>,
    /// no description provided
    #[serde(rename="thumbnailLink")]
    pub thumbnail_link: Option<String>,
    /// no description provided
    #[serde(rename="byteSize")]
    pub byte_size: Option<i32>,
    /// no description provided
    #[serde(rename="thumbnailHeight")]
    pub thumbnail_height: Option<i32>,
    /// no description provided
    pub height: Option<i32>,
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
/// * [siterestrict list cse](struct.CseSiterestrictListCall.html) (response)
/// * [list cse](struct.CseListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Search {
    /// no description provided
    pub promotions: Option<Vec<Promotion>>,
    /// no description provided
    pub kind: Option<String>,
    /// no description provided
    pub url: Option<SearchUrl>,
    /// no description provided
    pub items: Option<Vec<ResultType>>,
    /// no description provided
    pub context: Option<Context>,
    /// no description provided
    pub queries: Option<HashMap<String, Vec<Query>>>,
    /// no description provided
    pub spelling: Option<SearchSpelling>,
    /// no description provided
    #[serde(rename="searchInformation")]
    pub search_information: Option<SearchSearchInformation>,
}

impl ResponseResult for Search {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotionImage {
    /// no description provided
    pub source: Option<String>,
    /// no description provided
    pub height: Option<i32>,
    /// no description provided
    pub width: Option<i32>,
}

impl NestedType for PromotionImage {}
impl Part for PromotionImage {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchUrl {
    /// no description provided
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// no description provided
    pub template: Option<String>,
}

impl NestedType for SearchUrl {}
impl Part for SearchUrl {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchSpelling {
    /// no description provided
    #[serde(rename="correctedQuery")]
    pub corrected_query: Option<String>,
    /// no description provided
    #[serde(rename="htmlCorrectedQuery")]
    pub html_corrected_query: Option<String>,
}

impl NestedType for SearchSpelling {}
impl Part for SearchSpelling {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotionBodyLines {
    /// no description provided
    pub url: Option<String>,
    /// no description provided
    #[serde(rename="htmlTitle")]
    pub html_title: Option<String>,
    /// no description provided
    pub link: Option<String>,
    /// no description provided
    pub title: Option<String>,
}

impl NestedType for PromotionBodyLines {}
impl Part for PromotionBodyLines {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultType {
    /// no description provided
    pub kind: Option<String>,
    /// no description provided
    pub labels: Option<Vec<ResultLabels>>,
    /// no description provided
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="displayLink")]
    pub display_link: Option<String>,
    /// no description provided
    #[serde(rename="cacheId")]
    pub cache_id: Option<String>,
    /// no description provided
    #[serde(rename="formattedUrl")]
    pub formatted_url: Option<String>,
    /// no description provided
    #[serde(rename="htmlFormattedUrl")]
    pub html_formatted_url: Option<String>,
    /// no description provided
    pub pagemap: Option<HashMap<String, Vec<HashMap<String, String>>>>,
    /// no description provided
    #[serde(rename="fileFormat")]
    pub file_format: Option<String>,
    /// no description provided
    pub snippet: Option<String>,
    /// no description provided
    #[serde(rename="htmlSnippet")]
    pub html_snippet: Option<String>,
    /// no description provided
    pub link: Option<String>,
    /// no description provided
    pub image: Option<ResultImage>,
    /// no description provided
    pub mime: Option<String>,
    /// no description provided
    #[serde(rename="htmlTitle")]
    pub html_title: Option<String>,
}

impl Part for ResultType {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    /// no description provided
    pub facets: Option<Vec<Vec<ContextFacets>>>,
    /// no description provided
    pub title: Option<String>,
}

impl Part for Context {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContextFacets {
    /// no description provided
    pub label_with_op: Option<String>,
    /// no description provided
    pub anchor: Option<String>,
    /// no description provided
    pub label: Option<String>,
}

impl NestedType for ContextFacets {}
impl Part for ContextFacets {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Query {
    /// no description provided
    #[serde(rename="dateRestrict")]
    pub date_restrict: Option<String>,
    /// no description provided
    #[serde(rename="inputEncoding")]
    pub input_encoding: Option<String>,
    /// no description provided
    #[serde(rename="orTerms")]
    pub or_terms: Option<String>,
    /// no description provided
    #[serde(rename="highRange")]
    pub high_range: Option<String>,
    /// no description provided
    pub cx: Option<String>,
    /// no description provided
    #[serde(rename="startPage")]
    pub start_page: Option<i32>,
    /// no description provided
    #[serde(rename="disableCnTwTranslation")]
    pub disable_cn_tw_translation: Option<String>,
    /// no description provided
    pub cr: Option<String>,
    /// no description provided
    #[serde(rename="imgType")]
    pub img_type: Option<String>,
    /// no description provided
    #[serde(rename="relatedSite")]
    pub related_site: Option<String>,
    /// no description provided
    pub gl: Option<String>,
    /// no description provided
    #[serde(rename="searchType")]
    pub search_type: Option<String>,
    /// no description provided
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="googleHost")]
    pub google_host: Option<String>,
    /// no description provided
    #[serde(rename="fileType")]
    pub file_type: Option<String>,
    /// no description provided
    #[serde(rename="imgDominantColor")]
    pub img_dominant_color: Option<String>,
    /// no description provided
    #[serde(rename="siteSearch")]
    pub site_search: Option<String>,
    /// no description provided
    pub sort: Option<String>,
    /// no description provided
    pub hq: Option<String>,
    /// no description provided
    #[serde(rename="outputEncoding")]
    pub output_encoding: Option<String>,
    /// no description provided
    pub safe: Option<String>,
    /// no description provided
    #[serde(rename="searchTerms")]
    pub search_terms: Option<String>,
    /// no description provided
    #[serde(rename="exactTerms")]
    pub exact_terms: Option<String>,
    /// no description provided
    #[serde(rename="imgColorType")]
    pub img_color_type: Option<String>,
    /// no description provided
    pub hl: Option<String>,
    /// no description provided
    #[serde(rename="totalResults")]
    pub total_results: Option<String>,
    /// no description provided
    #[serde(rename="lowRange")]
    pub low_range: Option<String>,
    /// no description provided
    pub count: Option<i32>,
    /// no description provided
    #[serde(rename="imgSize")]
    pub img_size: Option<String>,
    /// no description provided
    pub language: Option<String>,
    /// no description provided
    pub rights: Option<String>,
    /// no description provided
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// no description provided
    #[serde(rename="excludeTerms")]
    pub exclude_terms: Option<String>,
    /// no description provided
    pub filter: Option<String>,
    /// no description provided
    #[serde(rename="linkSite")]
    pub link_site: Option<String>,
    /// no description provided
    #[serde(rename="siteSearchFilter")]
    pub site_search_filter: Option<String>,
}

impl Part for Query {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Promotion {
    /// no description provided
    #[serde(rename="bodyLines")]
    pub body_lines: Option<Vec<PromotionBodyLines>>,
    /// no description provided
    pub title: Option<String>,
    /// no description provided
    pub link: Option<String>,
    /// no description provided
    #[serde(rename="displayLink")]
    pub display_link: Option<String>,
    /// no description provided
    #[serde(rename="htmlTitle")]
    pub html_title: Option<String>,
    /// no description provided
    pub image: Option<PromotionImage>,
}

impl Part for Promotion {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchSearchInformation {
    /// no description provided
    #[serde(rename="formattedSearchTime")]
    pub formatted_search_time: Option<String>,
    /// no description provided
    #[serde(rename="formattedTotalResults")]
    pub formatted_total_results: Option<String>,
    /// no description provided
    #[serde(rename="totalResults")]
    pub total_results: Option<String>,
    /// no description provided
    #[serde(rename="searchTime")]
    pub search_time: Option<f64>,
}

impl NestedType for SearchSearchInformation {}
impl Part for SearchSearchInformation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultLabels {
    /// no description provided
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// no description provided
    pub label_with_op: Option<String>,
    /// no description provided
    pub name: Option<String>,
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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_customsearch1 as customsearch1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use customsearch1::Customsearch;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Customsearch::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `siterestrict_list(...)`
/// // to build up your call.
/// let rb = hub.cse();
/// # }
/// ```
pub struct CseMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Customsearch<C, A>,
}

impl<'a, C, A> MethodsBuilder for CseMethods<'a, C, A> {}

impl<'a, C, A> CseMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results. Uses a small set of url patterns.
    /// 
    /// # Arguments
    ///
    /// * `q` - Query
    pub fn siterestrict_list(&self, q: &str) -> CseSiterestrictListCall<'a, C, A> {
        CseSiterestrictListCall {
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
            _cr: Default::default(),
            _c2coff: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results.
    /// 
    /// # Arguments
    ///
    /// * `q` - Query
    pub fn list(&self, q: &str) -> CseListCall<'a, C, A> {
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

/// Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results. Uses a small set of url patterns.
///
/// A builder for the *siterestrict.list* method supported by a *cse* resource.
/// It is not used directly, but through a `CseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_customsearch1 as customsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use customsearch1::Customsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Customsearch::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().siterestrict_list("q")
///              .start(15)
///              .sort("invidunt")
///              .site_search_filter("ea")
///              .site_search("sadipscing")
///              .search_type("rebum.")
///              .safe("dolore")
///              .rights("nonumy")
///              .related_site("sed")
///              .or_terms("aliquyam")
///              .num(48)
///              .lr("eirmod")
///              .low_range("consetetur")
///              .link_site("labore")
///              .img_type("sed")
///              .img_size("ea")
///              .img_dominant_color("gubergren")
///              .img_color_type("aliquyam")
///              .hq("eos")
///              .hl("tempor")
///              .high_range("sea")
///              .googlehost("labore")
///              .gl("ipsum")
///              .filter("aliquyam")
///              .file_type("dolores")
///              .exclude_terms("sit")
///              .exact_terms("diam")
///              .date_restrict("ut")
///              .cx("justo")
///              .cr("est")
///              .c2coff("amet")
///              .doit();
/// # }
/// ```
pub struct CseSiterestrictListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Customsearch<C, A>,
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
    _cr: Option<String>,
    _c2coff: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CseSiterestrictListCall<'a, C, A> {}

impl<'a, C, A> CseSiterestrictListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "search.cse.siterestrict.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(33 + self._additional_params.len());
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
        if let Some(value) = self._cr {
            params.push(("cr", value.to_string()));
        }
        if let Some(value) = self._c2coff {
            params.push(("c2coff", value.to_string()));
        }
        for &field in ["alt", "q", "start", "sort", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/siterestrict";
        
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


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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


    /// Query
    ///
    /// Sets the *q* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn q(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._q = new_value.to_string();
        self
    }
    /// The index of the first result to return
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseSiterestrictListCall<'a, C, A> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Controls whether to include or exclude results from the site named in the as_sitesearch parameter
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies all search results should be pages from a given site
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: image.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these.
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should be pages that are related to the specified URL
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: u32) -> CseSiterestrictListCall<'a, C, A> {
        self._num = Some(new_value);
        self
    }
    /// The language restriction for the search results
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Creates a range in form as_nlo value..as_nhi value and attempts to append it to query
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular URL
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type, which can be one of: clipart, face, lineart, news, and photo.
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge.
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color: red, orange, yellow, green, teal, blue, purple, pink, white, gray, black and brown.
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, or color images: mono, gray, and color.
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the extra query terms to the query.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Creates a range in form as_nlo value..as_nhi value and attempts to append it to query
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// The local Google domain to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ...
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the search results
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Specifies all search results are from a time period
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The custom search engine ID to scope this search query
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Country restrict(s).
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Turns off the translation between zh-CN and zh-TW.
    ///
    /// Sets the *c2coff* query property to the given value.
    pub fn c2coff(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._c2coff = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CseSiterestrictListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseSiterestrictListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results.
///
/// A builder for the *list* method supported by a *cse* resource.
/// It is not used directly, but through a `CseMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_customsearch1 as customsearch1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use customsearch1::Customsearch;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Customsearch::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().list("q")
///              .start(88)
///              .sort("diam")
///              .site_search_filter("justo")
///              .site_search("est")
///              .search_type("clita")
///              .safe("invidunt")
///              .rights("ut")
///              .related_site("dolores")
///              .or_terms("eos")
///              .num(23)
///              .lr("duo")
///              .low_range("sed")
///              .link_site("aliquyam")
///              .img_type("ea")
///              .img_size("ea")
///              .img_dominant_color("et")
///              .img_color_type("dolor")
///              .hq("diam")
///              .hl("kasd")
///              .high_range("invidunt")
///              .googlehost("rebum.")
///              .gl("Lorem")
///              .filter("clita")
///              .file_type("invidunt")
///              .exclude_terms("eirmod")
///              .exact_terms("At")
///              .date_restrict("consetetur")
///              .cx("et")
///              .cr("sed")
///              .c2coff("sit")
///              .doit();
/// # }
/// ```
pub struct CseListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Customsearch<C, A>,
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
    _cr: Option<String>,
    _c2coff: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CseListCall<'a, C, A> {}

impl<'a, C, A> CseListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "search.cse.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(33 + self._additional_params.len());
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
        if let Some(value) = self._cr {
            params.push(("cr", value.to_string()));
        }
        if let Some(value) = self._c2coff {
            params.push(("c2coff", value.to_string()));
        }
        for &field in ["alt", "q", "start", "sort", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1";
        
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


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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


    /// Query
    ///
    /// Sets the *q* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn q(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._q = new_value.to_string();
        self
    }
    /// The index of the first result to return
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseListCall<'a, C, A> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Controls whether to include or exclude results from the site named in the as_sitesearch parameter
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies all search results should be pages from a given site
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: image.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: cc_publicdomain, cc_attribute, cc_sharealike, cc_noncommercial, cc_nonderived and combinations of these.
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should be pages that are related to the specified URL
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: u32) -> CseListCall<'a, C, A> {
        self._num = Some(new_value);
        self
    }
    /// The language restriction for the search results
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Creates a range in form as_nlo value..as_nhi value and attempts to append it to query
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular URL
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type, which can be one of: clipart, face, lineart, news, and photo.
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size, where size can be one of: icon, small, medium, large, xlarge, xxlarge, and huge.
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color: red, orange, yellow, green, teal, blue, purple, pink, white, gray, black and brown.
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, or color images: mono, gray, and color.
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the extra query terms to the query.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Creates a range in form as_nlo value..as_nhi value and attempts to append it to query
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// The local Google domain to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified type. Some of the allowed values are: bmp, gif, png, jpg, svg, pdf, ...
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the search results
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Specifies all search results are from a time period
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The custom search engine ID to scope this search query
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Country restrict(s).
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Turns off the translation between zh-CN and zh-TW.
    ///
    /// Sets the *c2coff* query property to the given value.
    pub fn c2coff(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._c2coff = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CseListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


