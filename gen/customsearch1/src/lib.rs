// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *customsearch* crate version *1.0.14+20200708*, where *20200708* is the exact revision of the *customsearch:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.
//! 
//! Everything else about the *customsearch* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/custom-search/v1/introduction).
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
//! let result = hub.cse().siterestrict_list()
//!              .start(92)
//!              .sort("Lorem")
//!              .site_search_filter("eos")
//!              .site_search("erat")
//!              .search_type("sadipscing")
//!              .safe("dolor")
//!              .rights("eirmod")
//!              .related_site("elitr")
//!              .q("amet")
//!              .or_terms("no")
//!              .num(-36)
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
//! All structures provided by this library are made to be [encodable](trait.RequestValue.html) and 
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
/// let result = hub.cse().siterestrict_list()
///              .start(53)
///              .sort("dolor")
///              .site_search_filter("et")
///              .site_search("consetetur")
///              .search_type("amet.")
///              .safe("voluptua.")
///              .rights("Lorem")
///              .related_site("gubergren")
///              .q("justo")
///              .or_terms("sit")
///              .num(-26)
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
            _user_agent: "google-api-rust-client/1.0.14".to_string(),
            _base_url: "https://customsearch.googleapis.com/".to_string(),
            _root_url: "https://customsearch.googleapis.com/".to_string(),
        }
    }

    pub fn cse(&'a self) -> CseMethods<'a, C, A> {
        CseMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.14`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://customsearch.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://customsearch.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Image belonging to a custom search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultImage {
    /// The width of the image, in pixels.
    pub width: Option<i32>,
    /// A URL pointing to the webpage hosting the image.
    #[serde(rename="contextLink")]
    pub context_link: Option<String>,
    /// The width of the thumbnail image, in pixels.
    #[serde(rename="thumbnailWidth")]
    pub thumbnail_width: Option<i32>,
    /// A URL to the thumbnail image.
    #[serde(rename="thumbnailLink")]
    pub thumbnail_link: Option<String>,
    /// The size of the image, in pixels.
    #[serde(rename="byteSize")]
    pub byte_size: Option<i32>,
    /// The height of the thumbnail image, in pixels.
    #[serde(rename="thumbnailHeight")]
    pub thumbnail_height: Option<i32>,
    /// The height of the image, in pixels.
    pub height: Option<i32>,
}

impl NestedType for ResultImage {}
impl Part for ResultImage {}


/// Response to a custom search request.
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
    /// The set of
    /// [promotions](https://developers.google.com/custom-search/docs/promotions).
    /// Present only if the custom search engine's configuration files define any
    /// promotions for the given query.
    pub promotions: Option<Vec<Promotion>>,
    /// Unique identifier for the type of current object. For this API, it is
    /// customsearch#search.
    pub kind: Option<String>,
    /// OpenSearch template and URL.
    pub url: Option<SearchUrl>,
    /// The current set of custom search results.
    pub items: Option<Vec<ResultType>>,
    /// Metadata and refinements associated with the given search engine,
    /// including:
    /// 
    /// * The name of the search engine that was used for the query.
    /// 
    /// *   A set of [facet
    /// objects](https://developers.google.com/custom-search/docs/refinements#create)
    /// (refinements) you can use for refining a search.
    pub context: Option<HashMap<String, String>>,
    /// Query metadata for the previous, current, and next pages of results.
    pub queries: Option<SearchQueries>,
    /// Spell correction information for a query.
    pub spelling: Option<SearchSpelling>,
    /// Metadata about a search operation.
    #[serde(rename="searchInformation")]
    pub search_information: Option<SearchSearchInformation>,
}

impl ResponseResult for Search {}


/// Custom search request metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueriesRequest {
    /// Restricts results to URLs based on date. Supported values include:
    /// 
    /// * `d[number]`: requests results from the specified number of past days.
    /// 
    /// * `w[number]`: requests results from the specified number of past weeks.
    /// 
    /// * `m[number]`: requests results from the specified number of past months.
    /// 
    /// * `y[number]`: requests results from the specified number of past years.
    #[serde(rename="dateRestrict")]
    pub date_restrict: Option<String>,
    /// The character encoding supported for search requests.
    #[serde(rename="inputEncoding")]
    pub input_encoding: Option<String>,
    /// Provides additional search terms to check for in a document, where each
    /// document in the search results must contain at least one of the
    /// additional search terms. You can also use the [Boolean
    /// OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt)
    /// query term for this type of query.
    #[serde(rename="orTerms")]
    pub or_terms: Option<String>,
    /// Specifies the ending value for a search range. Use `cse:lowRange` and
    /// `cse:highrange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    #[serde(rename="highRange")]
    pub high_range: Option<String>,
    /// The identifier of a custom search engine created using the Custom Search
    /// [Control Panel](https://cse.google.com/). This is a custom property not
    /// defined in the OpenSearch spec. This parameter is **required**.
    pub cx: Option<String>,
    /// The page number of this set of results, where the page length is set by
    /// the `count` property.
    #[serde(rename="startPage")]
    pub start_page: Option<i32>,
    /// Enables or disables the [Simplified and Traditional Chinese
    /// Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch)
    /// feature.
    /// 
    /// Supported values are:
    /// 
    /// * `0`: enabled (default)
    /// 
    /// * `1`: disabled
    #[serde(rename="disableCnTwTranslation")]
    pub disable_cn_tw_translation: Option<String>,
    /// Restricts search results to documents originating in a particular
    /// country. You may use [Boolean
    /// operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators)
    /// in the `cr` parameter's value.
    /// 
    /// Google WebSearch determines the country of a document by analyzing the
    /// following:
    /// 
    /// * The top-level domain (TLD) of the document's URL.
    /// 
    /// * The geographic location of the web server's IP address.
    /// 
    /// See [Country (cr) Parameter
    /// Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections)
    /// for a list of valid values for this parameter.
    pub cr: Option<String>,
    /// Restricts results to images of a specified type. Supported values are:
    /// 
    /// * `clipart` (Clip art)
    /// 
    /// * `face` (Face)
    /// 
    /// * `lineart` (Line drawing)
    /// 
    /// * `photo` (Photo)
    /// 
    /// * `animated` (Animated)
    /// 
    /// * `stock` (Stock)
    #[serde(rename="imgType")]
    pub img_type: Option<String>,
    /// Specifies that all search results should be pages that are related to the
    /// specified URL. The parameter value should be a URL.
    #[serde(rename="relatedSite")]
    pub related_site: Option<String>,
    /// Boosts search results whose country of origin matches the parameter
    /// value. See [Country
    /// Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes)
    /// for a list of valid values.
    /// 
    /// Specifying a `gl` parameter value in WebSearch requests should improve
    /// the relevance of results. This is particularly true for international
    /// customers and, even more specifically, for customers in English-speaking
    /// countries other than the United States.
    pub gl: Option<String>,
    /// Allowed values are `web` or `image`. If unspecified, results are limited
    /// to webpages.
    #[serde(rename="searchType")]
    pub search_type: Option<String>,
    /// A description of the query.
    pub title: Option<String>,
    /// Specifies the Google domain (for example, google.com, google.de, or
    /// google.fr) to which the search should be limited.
    #[serde(rename="googleHost")]
    pub google_host: Option<String>,
    /// Restricts results to files of a specified extension. Filetypes supported
    /// by Google include:
    /// 
    /// * Adobe Portable Document Format (`pdf`)
    /// 
    /// * Adobe PostScript (`ps`)
    /// 
    /// * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`)
    /// 
    /// * Lotus WordPro (`lwp`)
    /// 
    /// * Macwrite (`mw`)
    /// 
    /// * Microsoft Excel (`xls`)
    /// 
    /// * Microsoft PowerPoint (`ppt`)
    /// 
    /// * Microsoft Word (`doc`)
    /// 
    /// * Microsoft Works (`wks`, `wps`, `wdb`)
    /// 
    /// * Microsoft Write (`wri`)
    /// 
    /// * Rich Text Format (`rtf`)
    /// 
    /// * Shockwave Flash (`swf`)
    /// 
    /// * Text (`ans`, `txt`).
    /// 
    /// Additional filetypes may be added in the future. An up-to-date list can
    /// always be found in Google's [file type
    /// FAQ](https://support.google.com/webmasters/answer/35287).
    #[serde(rename="fileType")]
    pub file_type: Option<String>,
    /// Restricts results to images with a specific dominant color. Supported
    /// values are:
    /// 
    /// * `red`
    /// 
    /// * `orange`
    /// 
    /// * `yellow`
    /// 
    /// * `green`
    /// 
    /// * `teal`
    /// 
    /// * `blue`
    /// 
    /// * `purple`
    /// 
    /// * `pink`
    /// 
    /// * `white`
    /// 
    /// * `gray`
    /// 
    /// * `black`
    /// 
    /// * `brown`
    #[serde(rename="imgDominantColor")]
    pub img_dominant_color: Option<String>,
    /// Restricts results to URLs from a specified site.
    #[serde(rename="siteSearch")]
    pub site_search: Option<String>,
    /// Specifies that results should be sorted according to the specified
    /// expression. For example, sort by date.
    pub sort: Option<String>,
    /// Appends the specified query terms to the query, as if they were combined
    /// with a logical `AND` operator.
    pub hq: Option<String>,
    /// The character encoding supported for search results.
    #[serde(rename="outputEncoding")]
    pub output_encoding: Option<String>,
    /// Specifies the [SafeSearch
    /// level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels)
    /// used for filtering out adult results. This is a custom property not
    /// defined in the OpenSearch spec. Valid parameter values are:
    /// 
    /// * `"off"`: Disable SafeSearch
    /// 
    /// * `"active"`: Enable SafeSearch
    pub safe: Option<String>,
    /// The search terms entered by the user.
    #[serde(rename="searchTerms")]
    pub search_terms: Option<String>,
    /// Identifies a phrase that all documents in the search results must
    /// contain.
    #[serde(rename="exactTerms")]
    pub exact_terms: Option<String>,
    /// Restricts results to images of a specified color type. Supported values
    ///   are:
    /// 
    /// * `mono` (black and white)
    /// 
    /// * `gray` (grayscale)
    /// 
    /// * `color` (color)
    #[serde(rename="imgColorType")]
    pub img_color_type: Option<String>,
    /// Specifies the interface language (host language) of your user interface.
    /// Explicitly setting this parameter improves the performance and the
    /// quality of your search results.
    /// 
    /// See the [Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)
    /// section of [Internationalizing Queries and Results
    /// Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)
    /// for more information, and [Supported Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages)
    /// for a list of supported languages.
    pub hl: Option<String>,
    /// Estimated number of total search results. May not be accurate.
    #[serde(rename="totalResults")]
    pub total_results: Option<String>,
    /// Specifies the starting value for a search range. Use `cse:lowRange` and
    /// `cse:highrange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    #[serde(rename="lowRange")]
    pub low_range: Option<String>,
    /// Number of search results returned in this set.
    pub count: Option<i32>,
    /// Restricts results to images of a specified size. Supported values are:
    /// 
    /// * `icon` (small)
    /// 
    /// * `small | medium | large | xlarge` (medium)
    /// 
    /// * `xxlarge` (large)
    /// 
    /// * `huge` (extra-large)
    #[serde(rename="imgSize")]
    pub img_size: Option<String>,
    /// The language of the search results.
    pub language: Option<String>,
    /// Filters based on licensing. Supported values include:
    /// 
    /// * `cc_publicdomain`
    /// 
    /// * `cc_attribute`
    /// 
    /// * `cc_sharealike`
    /// 
    /// * `cc_noncommercial`
    /// 
    /// * `cc_nonderived`
    pub rights: Option<String>,
    /// The index of the current set of search results into the total set of
    /// results, where the index of the first result is 1.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// Identifies a word or phrase that should not appear in any documents in
    /// the search results.
    #[serde(rename="excludeTerms")]
    pub exclude_terms: Option<String>,
    /// Activates or deactivates the automatic filtering of Google search
    /// results. See [Automatic
    /// Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)
    /// for more information about Google's search results filters. Valid values
    /// for this parameter are:
    /// 
    /// * `0`: Disabled
    /// 
    /// * `1`: Enabled (default)
    /// 
    /// **Note**: By default, Google applies filtering to all search results to
    /// improve the quality of those results.
    pub filter: Option<String>,
    /// Specifies that all results should contain a link to a specific URL.
    #[serde(rename="linkSite")]
    pub link_site: Option<String>,
    /// Specifies whether to include or exclude results from the site named in
    /// the `sitesearch` parameter. Supported values are:
    /// 
    /// * `i`: include content from site
    /// 
    /// * `e`: exclude content from site
    #[serde(rename="siteSearchFilter")]
    pub site_search_filter: Option<String>,
}

impl NestedType for SearchQueriesRequest {}
impl Part for SearchQueriesRequest {}


/// Image belonging to a promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotionImage {
    /// URL of the image for this promotion link.
    pub source: Option<String>,
    /// Image height in pixels.
    pub height: Option<i32>,
    /// Image width in pixels.
    pub width: Option<i32>,
}

impl NestedType for PromotionImage {}
impl Part for PromotionImage {}


/// Custom search request metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueriesNextPage {
    /// Restricts results to URLs based on date. Supported values include:
    /// 
    /// * `d[number]`: requests results from the specified number of past days.
    /// 
    /// * `w[number]`: requests results from the specified number of past weeks.
    /// 
    /// * `m[number]`: requests results from the specified number of past months.
    /// 
    /// * `y[number]`: requests results from the specified number of past years.
    #[serde(rename="dateRestrict")]
    pub date_restrict: Option<String>,
    /// The character encoding supported for search requests.
    #[serde(rename="inputEncoding")]
    pub input_encoding: Option<String>,
    /// Provides additional search terms to check for in a document, where each
    /// document in the search results must contain at least one of the
    /// additional search terms. You can also use the [Boolean
    /// OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt)
    /// query term for this type of query.
    #[serde(rename="orTerms")]
    pub or_terms: Option<String>,
    /// Specifies the ending value for a search range. Use `cse:lowRange` and
    /// `cse:highrange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    #[serde(rename="highRange")]
    pub high_range: Option<String>,
    /// The identifier of a custom search engine created using the Custom Search
    /// [Control Panel](https://cse.google.com/). This is a custom property not
    /// defined in the OpenSearch spec. This parameter is **required**.
    pub cx: Option<String>,
    /// The page number of this set of results, where the page length is set by
    /// the `count` property.
    #[serde(rename="startPage")]
    pub start_page: Option<i32>,
    /// Enables or disables the [Simplified and Traditional Chinese
    /// Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch)
    /// feature.
    /// 
    /// Supported values are:
    /// 
    /// * `0`: enabled (default)
    /// 
    /// * `1`: disabled
    #[serde(rename="disableCnTwTranslation")]
    pub disable_cn_tw_translation: Option<String>,
    /// Restricts search results to documents originating in a particular
    /// country. You may use [Boolean
    /// operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators)
    /// in the `cr` parameter's value.
    /// 
    /// Google WebSearch determines the country of a document by analyzing the
    /// following:
    /// 
    /// * The top-level domain (TLD) of the document's URL.
    /// 
    /// * The geographic location of the web server's IP address.
    /// 
    /// See [Country (cr) Parameter
    /// Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections)
    /// for a list of valid values for this parameter.
    pub cr: Option<String>,
    /// Restricts results to images of a specified type. Supported values are:
    /// 
    /// * `clipart` (Clip art)
    /// 
    /// * `face` (Face)
    /// 
    /// * `lineart` (Line drawing)
    /// 
    /// * `photo` (Photo)
    /// 
    /// * `animated` (Animated)
    /// 
    /// * `stock` (Stock)
    #[serde(rename="imgType")]
    pub img_type: Option<String>,
    /// Specifies that all search results should be pages that are related to the
    /// specified URL. The parameter value should be a URL.
    #[serde(rename="relatedSite")]
    pub related_site: Option<String>,
    /// Boosts search results whose country of origin matches the parameter
    /// value. See [Country
    /// Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes)
    /// for a list of valid values.
    /// 
    /// Specifying a `gl` parameter value in WebSearch requests should improve
    /// the relevance of results. This is particularly true for international
    /// customers and, even more specifically, for customers in English-speaking
    /// countries other than the United States.
    pub gl: Option<String>,
    /// Allowed values are `web` or `image`. If unspecified, results are limited
    /// to webpages.
    #[serde(rename="searchType")]
    pub search_type: Option<String>,
    /// A description of the query.
    pub title: Option<String>,
    /// Specifies the Google domain (for example, google.com, google.de, or
    /// google.fr) to which the search should be limited.
    #[serde(rename="googleHost")]
    pub google_host: Option<String>,
    /// Restricts results to files of a specified extension. Filetypes supported
    /// by Google include:
    /// 
    /// * Adobe Portable Document Format (`pdf`)
    /// 
    /// * Adobe PostScript (`ps`)
    /// 
    /// * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`)
    /// 
    /// * Lotus WordPro (`lwp`)
    /// 
    /// * Macwrite (`mw`)
    /// 
    /// * Microsoft Excel (`xls`)
    /// 
    /// * Microsoft PowerPoint (`ppt`)
    /// 
    /// * Microsoft Word (`doc`)
    /// 
    /// * Microsoft Works (`wks`, `wps`, `wdb`)
    /// 
    /// * Microsoft Write (`wri`)
    /// 
    /// * Rich Text Format (`rtf`)
    /// 
    /// * Shockwave Flash (`swf`)
    /// 
    /// * Text (`ans`, `txt`).
    /// 
    /// Additional filetypes may be added in the future. An up-to-date list can
    /// always be found in Google's [file type
    /// FAQ](https://support.google.com/webmasters/answer/35287).
    #[serde(rename="fileType")]
    pub file_type: Option<String>,
    /// Restricts results to images with a specific dominant color. Supported
    /// values are:
    /// 
    /// * `red`
    /// 
    /// * `orange`
    /// 
    /// * `yellow`
    /// 
    /// * `green`
    /// 
    /// * `teal`
    /// 
    /// * `blue`
    /// 
    /// * `purple`
    /// 
    /// * `pink`
    /// 
    /// * `white`
    /// 
    /// * `gray`
    /// 
    /// * `black`
    /// 
    /// * `brown`
    #[serde(rename="imgDominantColor")]
    pub img_dominant_color: Option<String>,
    /// Restricts results to URLs from a specified site.
    #[serde(rename="siteSearch")]
    pub site_search: Option<String>,
    /// Specifies that results should be sorted according to the specified
    /// expression. For example, sort by date.
    pub sort: Option<String>,
    /// Appends the specified query terms to the query, as if they were combined
    /// with a logical `AND` operator.
    pub hq: Option<String>,
    /// The character encoding supported for search results.
    #[serde(rename="outputEncoding")]
    pub output_encoding: Option<String>,
    /// Specifies the [SafeSearch
    /// level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels)
    /// used for filtering out adult results. This is a custom property not
    /// defined in the OpenSearch spec. Valid parameter values are:
    /// 
    /// * `"off"`: Disable SafeSearch
    /// 
    /// * `"active"`: Enable SafeSearch
    pub safe: Option<String>,
    /// The search terms entered by the user.
    #[serde(rename="searchTerms")]
    pub search_terms: Option<String>,
    /// Identifies a phrase that all documents in the search results must
    /// contain.
    #[serde(rename="exactTerms")]
    pub exact_terms: Option<String>,
    /// Restricts results to images of a specified color type. Supported values
    ///   are:
    /// 
    /// * `mono` (black and white)
    /// 
    /// * `gray` (grayscale)
    /// 
    /// * `color` (color)
    #[serde(rename="imgColorType")]
    pub img_color_type: Option<String>,
    /// Specifies the interface language (host language) of your user interface.
    /// Explicitly setting this parameter improves the performance and the
    /// quality of your search results.
    /// 
    /// See the [Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)
    /// section of [Internationalizing Queries and Results
    /// Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)
    /// for more information, and [Supported Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages)
    /// for a list of supported languages.
    pub hl: Option<String>,
    /// Estimated number of total search results. May not be accurate.
    #[serde(rename="totalResults")]
    pub total_results: Option<String>,
    /// Specifies the starting value for a search range. Use `cse:lowRange` and
    /// `cse:highrange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    #[serde(rename="lowRange")]
    pub low_range: Option<String>,
    /// Number of search results returned in this set.
    pub count: Option<i32>,
    /// Restricts results to images of a specified size. Supported values are:
    /// 
    /// * `icon` (small)
    /// 
    /// * `small | medium | large | xlarge` (medium)
    /// 
    /// * `xxlarge` (large)
    /// 
    /// * `huge` (extra-large)
    #[serde(rename="imgSize")]
    pub img_size: Option<String>,
    /// The language of the search results.
    pub language: Option<String>,
    /// Filters based on licensing. Supported values include:
    /// 
    /// * `cc_publicdomain`
    /// 
    /// * `cc_attribute`
    /// 
    /// * `cc_sharealike`
    /// 
    /// * `cc_noncommercial`
    /// 
    /// * `cc_nonderived`
    pub rights: Option<String>,
    /// The index of the current set of search results into the total set of
    /// results, where the index of the first result is 1.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// Identifies a word or phrase that should not appear in any documents in
    /// the search results.
    #[serde(rename="excludeTerms")]
    pub exclude_terms: Option<String>,
    /// Activates or deactivates the automatic filtering of Google search
    /// results. See [Automatic
    /// Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)
    /// for more information about Google's search results filters. Valid values
    /// for this parameter are:
    /// 
    /// * `0`: Disabled
    /// 
    /// * `1`: Enabled (default)
    /// 
    /// **Note**: By default, Google applies filtering to all search results to
    /// improve the quality of those results.
    pub filter: Option<String>,
    /// Specifies that all results should contain a link to a specific URL.
    #[serde(rename="linkSite")]
    pub link_site: Option<String>,
    /// Specifies whether to include or exclude results from the site named in
    /// the `sitesearch` parameter. Supported values are:
    /// 
    /// * `i`: include content from site
    /// 
    /// * `e`: exclude content from site
    #[serde(rename="siteSearchFilter")]
    pub site_search_filter: Option<String>,
}

impl NestedType for SearchQueriesNextPage {}
impl Part for SearchQueriesNextPage {}


/// OpenSearch template and URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchUrl {
    /// The MIME type of the OpenSearch URL template for the Custom Search API.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The actual [OpenSearch
    /// template](http://www.opensearch.org/specifications/opensearch/1.1#opensearch_url_template_syntax)
    /// for this API.
    pub template: Option<String>,
}

impl NestedType for SearchUrl {}
impl Part for SearchUrl {}


/// Spell correction information for a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchSpelling {
    /// The corrected query.
    #[serde(rename="correctedQuery")]
    pub corrected_query: Option<String>,
    /// The corrected query, formatted in HTML.
    #[serde(rename="htmlCorrectedQuery")]
    pub html_corrected_query: Option<String>,
}

impl NestedType for SearchSpelling {}
impl Part for SearchSpelling {}


/// Block object belonging to a promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotionBodyLines {
    /// The URL of the block object's link, if it has one.
    pub url: Option<String>,
    /// The block object's text in HTML, if it has text.
    #[serde(rename="htmlTitle")]
    pub html_title: Option<String>,
    /// The anchor text of the block object's link, if it has a link.
    pub link: Option<String>,
    /// The block object's text, if it has text.
    pub title: Option<String>,
}

impl NestedType for PromotionBodyLines {}
impl Part for PromotionBodyLines {}


/// A custom search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultType {
    /// A unique identifier for the type of current object. For this API, it is
    /// `customsearch#result.`
    pub kind: Option<String>,
    /// Encapsulates all information about [refinement
    /// labels](https://developers.google.com/custom-search/docs/xml_results).
    pub labels: Option<Vec<ResultLabels>>,
    /// The title of the search result, in plain text.
    pub title: Option<String>,
    /// An abridged version of this search result’s URL, e.g. www.example.com.
    #[serde(rename="displayLink")]
    pub display_link: Option<String>,
    /// Indicates the ID of Google's cached version of the search result.
    #[serde(rename="cacheId")]
    pub cache_id: Option<String>,
    /// The URL displayed after the snippet for each search result.
    #[serde(rename="formattedUrl")]
    pub formatted_url: Option<String>,
    /// The HTML-formatted URL displayed after the snippet for each search result.
    #[serde(rename="htmlFormattedUrl")]
    pub html_formatted_url: Option<String>,
    /// Contains
    /// [PageMap](https://developers.google.com/custom-search/docs/structured_data#pagemaps)
    /// information for this search result.
    pub pagemap: Option<HashMap<String, String>>,
    /// The file format of the search result.
    #[serde(rename="fileFormat")]
    pub file_format: Option<String>,
    /// The snippet of the search result, in plain text.
    pub snippet: Option<String>,
    /// The snippet of the search result, in HTML.
    #[serde(rename="htmlSnippet")]
    pub html_snippet: Option<String>,
    /// The full URL to which the search result is pointing, e.g.
    /// http://www.example.com/foo/bar.
    pub link: Option<String>,
    /// Image belonging to a custom search result.
    pub image: Option<ResultImage>,
    /// The MIME type of the search result.
    pub mime: Option<String>,
    /// The title of the search result, in HTML.
    #[serde(rename="htmlTitle")]
    pub html_title: Option<String>,
}

impl Part for ResultType {}


/// Query metadata for the previous, current, and next pages of results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueries {
    /// Metadata representing the current request.
    pub request: Option<Vec<SearchQueriesRequest>>,
    /// Metadata representing the next page of results, if applicable.
    #[serde(rename="nextPage")]
    pub next_page: Option<Vec<SearchQueriesNextPage>>,
    /// Metadata representing the previous page of results, if applicable.
    #[serde(rename="previousPage")]
    pub previous_page: Option<Vec<SearchQueriesPreviousPage>>,
}

impl NestedType for SearchQueries {}
impl Part for SearchQueries {}


/// Promotion result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Promotion {
    /// An array of block objects for this promotion. See [Google WebSearch
    /// Protocol
    /// reference](https://developers.google.com/custom-search/docs/xml_results)
    /// for more information.
    #[serde(rename="bodyLines")]
    pub body_lines: Option<Vec<PromotionBodyLines>>,
    /// The title of the promotion.
    pub title: Option<String>,
    /// The URL of the promotion.
    pub link: Option<String>,
    /// An abridged version of this search's result URL, e.g. www.example.com.
    #[serde(rename="displayLink")]
    pub display_link: Option<String>,
    /// The title of the promotion, in HTML.
    #[serde(rename="htmlTitle")]
    pub html_title: Option<String>,
    /// Image belonging to a promotion.
    pub image: Option<PromotionImage>,
}

impl Part for Promotion {}


/// Metadata about a search operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchSearchInformation {
    /// The time taken for the server to return search results, formatted
    /// according to locale style.
    #[serde(rename="formattedSearchTime")]
    pub formatted_search_time: Option<String>,
    /// The total number of search results, formatted according to locale style.
    #[serde(rename="formattedTotalResults")]
    pub formatted_total_results: Option<String>,
    /// The total number of search results returned by the query.
    #[serde(rename="totalResults")]
    pub total_results: Option<String>,
    /// The time taken for the server to return search results.
    #[serde(rename="searchTime")]
    pub search_time: Option<f64>,
}

impl NestedType for SearchSearchInformation {}
impl Part for SearchSearchInformation {}


/// Refinement label associated with a custom search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultLabels {
    /// The display name of a refinement label. This is the name you should
    /// display in your user interface.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Refinement label and the associated refinement operation.
    pub label_with_op: Option<String>,
    /// The name of a refinement label, which you can use to refine searches.
    /// Don't display this in your user interface; instead, use displayName.
    pub name: Option<String>,
}

impl NestedType for ResultLabels {}
impl Part for ResultLabels {}


/// Custom search request metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueriesPreviousPage {
    /// Restricts results to URLs based on date. Supported values include:
    /// 
    /// * `d[number]`: requests results from the specified number of past days.
    /// 
    /// * `w[number]`: requests results from the specified number of past weeks.
    /// 
    /// * `m[number]`: requests results from the specified number of past months.
    /// 
    /// * `y[number]`: requests results from the specified number of past years.
    #[serde(rename="dateRestrict")]
    pub date_restrict: Option<String>,
    /// The character encoding supported for search requests.
    #[serde(rename="inputEncoding")]
    pub input_encoding: Option<String>,
    /// Provides additional search terms to check for in a document, where each
    /// document in the search results must contain at least one of the
    /// additional search terms. You can also use the [Boolean
    /// OR](https://developers.google.com/custom-search/docs/xml_results#BooleanOrqt)
    /// query term for this type of query.
    #[serde(rename="orTerms")]
    pub or_terms: Option<String>,
    /// Specifies the ending value for a search range. Use `cse:lowRange` and
    /// `cse:highrange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    #[serde(rename="highRange")]
    pub high_range: Option<String>,
    /// The identifier of a custom search engine created using the Custom Search
    /// [Control Panel](https://cse.google.com/). This is a custom property not
    /// defined in the OpenSearch spec. This parameter is **required**.
    pub cx: Option<String>,
    /// The page number of this set of results, where the page length is set by
    /// the `count` property.
    #[serde(rename="startPage")]
    pub start_page: Option<i32>,
    /// Enables or disables the [Simplified and Traditional Chinese
    /// Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch)
    /// feature.
    /// 
    /// Supported values are:
    /// 
    /// * `0`: enabled (default)
    /// 
    /// * `1`: disabled
    #[serde(rename="disableCnTwTranslation")]
    pub disable_cn_tw_translation: Option<String>,
    /// Restricts search results to documents originating in a particular
    /// country. You may use [Boolean
    /// operators](https://developers.google.com/custom-search/docs/xml_results#booleanOperators)
    /// in the `cr` parameter's value.
    /// 
    /// Google WebSearch determines the country of a document by analyzing the
    /// following:
    /// 
    /// * The top-level domain (TLD) of the document's URL.
    /// 
    /// * The geographic location of the web server's IP address.
    /// 
    /// See [Country (cr) Parameter
    /// Values](https://developers.google.com/custom-search/docs/xml_results#countryCollections)
    /// for a list of valid values for this parameter.
    pub cr: Option<String>,
    /// Restricts results to images of a specified type. Supported values are:
    /// 
    /// * `clipart` (Clip art)
    /// 
    /// * `face` (Face)
    /// 
    /// * `lineart` (Line drawing)
    /// 
    /// * `photo` (Photo)
    /// 
    /// * `animated` (Animated)
    /// 
    /// * `stock` (Stock)
    #[serde(rename="imgType")]
    pub img_type: Option<String>,
    /// Specifies that all search results should be pages that are related to the
    /// specified URL. The parameter value should be a URL.
    #[serde(rename="relatedSite")]
    pub related_site: Option<String>,
    /// Boosts search results whose country of origin matches the parameter
    /// value. See [Country
    /// Codes](https://developers.google.com/custom-search/docs/xml_results#countryCodes)
    /// for a list of valid values.
    /// 
    /// Specifying a `gl` parameter value in WebSearch requests should improve
    /// the relevance of results. This is particularly true for international
    /// customers and, even more specifically, for customers in English-speaking
    /// countries other than the United States.
    pub gl: Option<String>,
    /// Allowed values are `web` or `image`. If unspecified, results are limited
    /// to webpages.
    #[serde(rename="searchType")]
    pub search_type: Option<String>,
    /// A description of the query.
    pub title: Option<String>,
    /// Specifies the Google domain (for example, google.com, google.de, or
    /// google.fr) to which the search should be limited.
    #[serde(rename="googleHost")]
    pub google_host: Option<String>,
    /// Restricts results to files of a specified extension. Filetypes supported
    /// by Google include:
    /// 
    /// * Adobe Portable Document Format (`pdf`)
    /// 
    /// * Adobe PostScript (`ps`)
    /// 
    /// * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`)
    /// 
    /// * Lotus WordPro (`lwp`)
    /// 
    /// * Macwrite (`mw`)
    /// 
    /// * Microsoft Excel (`xls`)
    /// 
    /// * Microsoft PowerPoint (`ppt`)
    /// 
    /// * Microsoft Word (`doc`)
    /// 
    /// * Microsoft Works (`wks`, `wps`, `wdb`)
    /// 
    /// * Microsoft Write (`wri`)
    /// 
    /// * Rich Text Format (`rtf`)
    /// 
    /// * Shockwave Flash (`swf`)
    /// 
    /// * Text (`ans`, `txt`).
    /// 
    /// Additional filetypes may be added in the future. An up-to-date list can
    /// always be found in Google's [file type
    /// FAQ](https://support.google.com/webmasters/answer/35287).
    #[serde(rename="fileType")]
    pub file_type: Option<String>,
    /// Restricts results to images with a specific dominant color. Supported
    /// values are:
    /// 
    /// * `red`
    /// 
    /// * `orange`
    /// 
    /// * `yellow`
    /// 
    /// * `green`
    /// 
    /// * `teal`
    /// 
    /// * `blue`
    /// 
    /// * `purple`
    /// 
    /// * `pink`
    /// 
    /// * `white`
    /// 
    /// * `gray`
    /// 
    /// * `black`
    /// 
    /// * `brown`
    #[serde(rename="imgDominantColor")]
    pub img_dominant_color: Option<String>,
    /// Restricts results to URLs from a specified site.
    #[serde(rename="siteSearch")]
    pub site_search: Option<String>,
    /// Specifies that results should be sorted according to the specified
    /// expression. For example, sort by date.
    pub sort: Option<String>,
    /// Appends the specified query terms to the query, as if they were combined
    /// with a logical `AND` operator.
    pub hq: Option<String>,
    /// The character encoding supported for search results.
    #[serde(rename="outputEncoding")]
    pub output_encoding: Option<String>,
    /// Specifies the [SafeSearch
    /// level](https://developers.google.com/custom-search/docs/xml_results#safeSearchLevels)
    /// used for filtering out adult results. This is a custom property not
    /// defined in the OpenSearch spec. Valid parameter values are:
    /// 
    /// * `"off"`: Disable SafeSearch
    /// 
    /// * `"active"`: Enable SafeSearch
    pub safe: Option<String>,
    /// The search terms entered by the user.
    #[serde(rename="searchTerms")]
    pub search_terms: Option<String>,
    /// Identifies a phrase that all documents in the search results must
    /// contain.
    #[serde(rename="exactTerms")]
    pub exact_terms: Option<String>,
    /// Restricts results to images of a specified color type. Supported values
    ///   are:
    /// 
    /// * `mono` (black and white)
    /// 
    /// * `gray` (grayscale)
    /// 
    /// * `color` (color)
    #[serde(rename="imgColorType")]
    pub img_color_type: Option<String>,
    /// Specifies the interface language (host language) of your user interface.
    /// Explicitly setting this parameter improves the performance and the
    /// quality of your search results.
    /// 
    /// See the [Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)
    /// section of [Internationalizing Queries and Results
    /// Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)
    /// for more information, and [Supported Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages)
    /// for a list of supported languages.
    pub hl: Option<String>,
    /// Estimated number of total search results. May not be accurate.
    #[serde(rename="totalResults")]
    pub total_results: Option<String>,
    /// Specifies the starting value for a search range. Use `cse:lowRange` and
    /// `cse:highrange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    #[serde(rename="lowRange")]
    pub low_range: Option<String>,
    /// Number of search results returned in this set.
    pub count: Option<i32>,
    /// Restricts results to images of a specified size. Supported values are:
    /// 
    /// * `icon` (small)
    /// 
    /// * `small | medium | large | xlarge` (medium)
    /// 
    /// * `xxlarge` (large)
    /// 
    /// * `huge` (extra-large)
    #[serde(rename="imgSize")]
    pub img_size: Option<String>,
    /// The language of the search results.
    pub language: Option<String>,
    /// Filters based on licensing. Supported values include:
    /// 
    /// * `cc_publicdomain`
    /// 
    /// * `cc_attribute`
    /// 
    /// * `cc_sharealike`
    /// 
    /// * `cc_noncommercial`
    /// 
    /// * `cc_nonderived`
    pub rights: Option<String>,
    /// The index of the current set of search results into the total set of
    /// results, where the index of the first result is 1.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// Identifies a word or phrase that should not appear in any documents in
    /// the search results.
    #[serde(rename="excludeTerms")]
    pub exclude_terms: Option<String>,
    /// Activates or deactivates the automatic filtering of Google search
    /// results. See [Automatic
    /// Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)
    /// for more information about Google's search results filters. Valid values
    /// for this parameter are:
    /// 
    /// * `0`: Disabled
    /// 
    /// * `1`: Enabled (default)
    /// 
    /// **Note**: By default, Google applies filtering to all search results to
    /// improve the quality of those results.
    pub filter: Option<String>,
    /// Specifies that all results should contain a link to a specific URL.
    #[serde(rename="linkSite")]
    pub link_site: Option<String>,
    /// Specifies whether to include or exclude results from the site named in
    /// the `sitesearch` parameter. Supported values are:
    /// 
    /// * `i`: include content from site
    /// 
    /// * `e`: exclude content from site
    #[serde(rename="siteSearchFilter")]
    pub site_search_filter: Option<String>,
}

impl NestedType for SearchQueriesPreviousPage {}
impl Part for SearchQueriesPreviousPage {}



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
    /// Returns metadata about the search performed, metadata about the custom
    /// search engine used for the search, and the search results. Uses a small set
    /// of url patterns.
    pub fn siterestrict_list(&self) -> CseSiterestrictListCall<'a, C, A> {
        CseSiterestrictListCall {
            hub: self.hub,
            _start: Default::default(),
            _sort: Default::default(),
            _site_search_filter: Default::default(),
            _site_search: Default::default(),
            _search_type: Default::default(),
            _safe: Default::default(),
            _rights: Default::default(),
            _related_site: Default::default(),
            _q: Default::default(),
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
    /// Returns metadata about the search performed, metadata about the custom
    /// search engine used for the search, and the search results.
    pub fn list(&self) -> CseListCall<'a, C, A> {
        CseListCall {
            hub: self.hub,
            _start: Default::default(),
            _sort: Default::default(),
            _site_search_filter: Default::default(),
            _site_search: Default::default(),
            _search_type: Default::default(),
            _safe: Default::default(),
            _rights: Default::default(),
            _related_site: Default::default(),
            _q: Default::default(),
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

/// Returns metadata about the search performed, metadata about the custom
/// search engine used for the search, and the search results. Uses a small set
/// of url patterns.
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
/// let result = hub.cse().siterestrict_list()
///              .start(74)
///              .sort("labore")
///              .site_search_filter("invidunt")
///              .site_search("ea")
///              .search_type("sadipscing")
///              .safe("rebum.")
///              .rights("dolore")
///              .related_site("nonumy")
///              .q("sed")
///              .or_terms("aliquyam")
///              .num(-53)
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
    _start: Option<u32>,
    _sort: Option<String>,
    _site_search_filter: Option<String>,
    _site_search: Option<String>,
    _search_type: Option<String>,
    _safe: Option<String>,
    _rights: Option<String>,
    _related_site: Option<String>,
    _q: Option<String>,
    _or_terms: Option<String>,
    _num: Option<i32>,
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
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CseSiterestrictListCall<'a, C, A> {}

impl<'a, C, A> CseSiterestrictListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "search.cse.siterestrict.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(33 + self._additional_params.len());
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
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
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
        for &field in ["alt", "start", "sort", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "q", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "customsearch/v1/siterestrict";
        
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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


    /// The index of the first result to return. The default number of results per
    /// page is 10, so `&start=11` would start at the top of the second page of
    /// results. **Note**: The JSON API will never return more than 100 results,
    /// even if more than 100 documents match the query, so setting the sum of
    /// `start + num` to a number greater than 100 will produce an error. Also note
    /// that the maximum value for `num` is 10.
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseSiterestrictListCall<'a, C, A> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results.
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Controls whether to include or exclude results from the site named in the
    /// `siteSearch` parameter.
    /// 
    /// Acceptable values are:
    /// 
    /// * `"e"`: exclude
    /// 
    /// * `"i"`: include
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies a given site which should always be included or excluded from
    /// results (see `siteSearchFilter` parameter, below).
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: `image`. If unspecified, results are limited to
    /// webpages.
    /// 
    /// Acceptable values are:
    /// 
    /// * `"image"`: custom image search.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level. Acceptable values are:
    /// 
    /// * `"active"`: Enables SafeSearch filtering.
    /// 
    /// * `"off"`: Disables SafeSearch filtering. (default)
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: `cc_publicdomain`,
    /// `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and
    /// combinations of these. See [typical
    /// combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration).
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should be pages that are related to the
    /// specified URL.
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Query
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each
    /// document in the search results must contain at least one of the additional
    /// search terms.
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return.
    /// 
    /// * Valid values are integers between 1 and 10, inclusive.
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: i32) -> CseSiterestrictListCall<'a, C, A> {
        self._num = Some(new_value);
        self
    }
    /// Restricts the search to documents written in a particular language (e.g.,
    /// `lr=lang_ja`).
    /// 
    /// Acceptable values are:
    /// 
    /// * `"lang_ar"`: Arabic
    /// 
    /// * `"lang_bg"`: Bulgarian
    /// 
    /// * `"lang_ca"`: Catalan
    /// 
    /// * `"lang_cs"`: Czech
    /// 
    /// * `"lang_da"`: Danish
    /// 
    /// * `"lang_de"`: German
    /// 
    /// * `"lang_el"`: Greek
    /// 
    /// * `"lang_en"`: English
    /// 
    /// * `"lang_es"`: Spanish
    /// 
    /// * `"lang_et"`: Estonian
    /// 
    /// * `"lang_fi"`: Finnish
    /// 
    /// * `"lang_fr"`: French
    /// 
    /// * `"lang_hr"`: Croatian
    /// 
    /// * `"lang_hu"`: Hungarian
    /// 
    /// * `"lang_id"`: Indonesian
    /// 
    /// * `"lang_is"`: Icelandic
    /// 
    /// * `"lang_it"`: Italian
    /// 
    /// * `"lang_iw"`: Hebrew
    /// 
    /// * `"lang_ja"`: Japanese
    /// 
    /// * `"lang_ko"`: Korean
    /// 
    /// * `"lang_lt"`: Lithuanian
    /// 
    /// * `"lang_lv"`: Latvian
    /// 
    /// * `"lang_nl"`: Dutch
    /// 
    /// * `"lang_no"`: Norwegian
    /// 
    /// * `"lang_pl"`: Polish
    /// 
    /// * `"lang_pt"`: Portuguese
    /// 
    /// * `"lang_ro"`: Romanian
    /// 
    /// * `"lang_ru"`: Russian
    /// 
    /// * `"lang_sk"`: Slovak
    /// 
    /// * `"lang_sl"`: Slovenian
    /// 
    /// * `"lang_sr"`: Serbian
    /// 
    /// * `"lang_sv"`: Swedish
    /// 
    /// * `"lang_tr"`: Turkish
    /// 
    /// * `"lang_zh-CN"`: Chinese (Simplified)
    /// 
    /// * `"lang_zh-TW"`: Chinese (Traditional)
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Specifies the starting value for a search range. Use `lowRange` and
    /// `highRange` to append an inclusive search range of `lowRange...highRange`
    /// to the query.
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular
    /// URL.
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type. Acceptable values are:
    /// 
    /// * `"clipart"`
    /// 
    /// * `"face"`
    /// 
    /// * `"lineart"`
    /// 
    /// * `"stock"`
    /// 
    /// * `"photo"`
    /// 
    /// * `"animated"`
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size. Acceptable values are:
    /// 
    /// * `"huge"`
    /// 
    /// * `"icon"`
    /// 
    /// * `"large"`
    /// 
    /// * `"medium"`
    /// 
    /// * `"small"`
    /// 
    /// * `"xlarge"`
    /// 
    /// * `"xxlarge"`
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color. Acceptable values are:
    /// 
    /// * `"black"`
    /// 
    /// * `"blue"`
    /// 
    /// * `"brown"`
    /// 
    /// * `"gray"`
    /// 
    /// * `"green"`
    /// 
    /// * `"orange"`
    /// 
    /// * `"pink"`
    /// 
    /// * `"purple"`
    /// 
    /// * `"red"`
    /// 
    /// * `"teal"`
    /// 
    /// * `"white"`
    /// 
    /// * `"yellow"`
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, transparent, or color images.
    /// Acceptable values are:
    /// 
    /// * `"color"`
    /// 
    /// * `"gray"`
    /// 
    /// * `"mono"`: black and white
    /// 
    /// * `"trans"`: transparent background
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the specified query terms to the query, as if they were combined
    /// with a logical AND operator.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language.
    /// 
    /// * Explicitly setting this parameter improves the performance and the
    /// quality of your search results.
    /// 
    /// * See the [Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)
    /// section of [Internationalizing Queries and Results
    /// Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)
    /// for more information, and (Supported Interface
    /// Languages)[https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages]
    /// for a list of supported languages.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Specifies the ending value for a search range.
    /// 
    /// * Use `lowRange` and `highRange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// **Deprecated**. Use the `gl` parameter for a similar effect.
    /// 
    /// The local Google domain (for example, google.com, google.de, or
    /// google.fr) to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user.
    /// 
    /// * The `gl` parameter value is a two-letter country code. The `gl` parameter
    /// boosts search results whose country of origin matches the parameter value.
    /// See the [Country
    /// Codes](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCodes)
    /// page for a list of valid values.
    /// 
    /// * Specifying a `gl` parameter value should lead to more relevant results.
    /// This is particularly true for international customers and, even more
    /// specifically, for customers in English- speaking countries other than the
    /// United States.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter.
    /// 
    /// * See [Automatic
    /// Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)
    /// for more information about Google's search results filters. Note that host
    /// crowding filtering applies only to multi-site searches.
    /// 
    /// * By default, Google applies filtering to all search results to improve the
    /// quality of those results.
    /// 
    /// Acceptable values are:
    /// 
    /// * `0`: Turns off duplicate content filter.
    /// 
    /// * `1`: Turns on duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restricts results to files of a specified extension. A list of file types
    /// indexable by Google can be found in Search Console [Help
    /// Center](https://support.google.com/webmasters/answer/35287).
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the
    /// search results.
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain.
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Restricts results to URLs based on date. Supported values include:
    /// 
    /// * `d[number]`: requests results from the specified number of past days.
    /// 
    /// * `w[number]`: requests results from the specified number of past weeks.
    /// 
    /// * `m[number]`: requests results from the specified number of past months.
    /// 
    /// * `y[number]`: requests results from the specified number of past years.
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The custom search engine ID to use for this request.
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Restricts search results to documents originating in a particular country.
    /// You may use [Boolean
    /// operators](https://developers.google.com/custom-search/docs/xml_results_appendices#booleanOperators)
    /// in the cr parameter's value.
    /// 
    /// Google Search determines the country of a document by analyzing:
    /// 
    /// * the top-level domain (TLD) of the document's URL
    /// 
    /// * the geographic location of the Web server's IP address
    /// 
    /// See the [Country Parameter
    /// Values](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCollections)
    /// page for a list of valid values for this parameter.
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, C, A> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Enables or disables [Simplified and Traditional Chinese
    /// Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch).
    /// 
    /// The default value for this parameter is 0 (zero), meaning that the feature
    /// is enabled. Supported values are:
    /// 
    /// * `1`: Disabled
    /// 
    /// * `0`: Enabled (default)
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
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> CseSiterestrictListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseSiterestrictListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns metadata about the search performed, metadata about the custom
/// search engine used for the search, and the search results.
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
/// let result = hub.cse().list()
///              .start(78)
///              .sort("clita")
///              .site_search_filter("diam")
///              .site_search("justo")
///              .search_type("est")
///              .safe("clita")
///              .rights("invidunt")
///              .related_site("ut")
///              .q("dolores")
///              .or_terms("eos")
///              .num(-78)
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
    _start: Option<u32>,
    _sort: Option<String>,
    _site_search_filter: Option<String>,
    _site_search: Option<String>,
    _search_type: Option<String>,
    _safe: Option<String>,
    _rights: Option<String>,
    _related_site: Option<String>,
    _q: Option<String>,
    _or_terms: Option<String>,
    _num: Option<i32>,
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
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for CseListCall<'a, C, A> {}

impl<'a, C, A> CseListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "search.cse.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(33 + self._additional_params.len());
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
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
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
        for &field in ["alt", "start", "sort", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "q", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "customsearch/v1";
        
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

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
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


    /// The index of the first result to return. The default number of results per
    /// page is 10, so `&start=11` would start at the top of the second page of
    /// results. **Note**: The JSON API will never return more than 100 results,
    /// even if more than 100 documents match the query, so setting the sum of
    /// `start + num` to a number greater than 100 will produce an error. Also note
    /// that the maximum value for `num` is 10.
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseListCall<'a, C, A> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results.
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Controls whether to include or exclude results from the site named in the
    /// `siteSearch` parameter.
    /// 
    /// Acceptable values are:
    /// 
    /// * `"e"`: exclude
    /// 
    /// * `"i"`: include
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies a given site which should always be included or excluded from
    /// results (see `siteSearchFilter` parameter, below).
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: `image`. If unspecified, results are limited to
    /// webpages.
    /// 
    /// Acceptable values are:
    /// 
    /// * `"image"`: custom image search.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level. Acceptable values are:
    /// 
    /// * `"active"`: Enables SafeSearch filtering.
    /// 
    /// * `"off"`: Disables SafeSearch filtering. (default)
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: `cc_publicdomain`,
    /// `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and
    /// combinations of these. See [typical
    /// combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration).
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should be pages that are related to the
    /// specified URL.
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Query
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each
    /// document in the search results must contain at least one of the additional
    /// search terms.
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return.
    /// 
    /// * Valid values are integers between 1 and 10, inclusive.
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: i32) -> CseListCall<'a, C, A> {
        self._num = Some(new_value);
        self
    }
    /// Restricts the search to documents written in a particular language (e.g.,
    /// `lr=lang_ja`).
    /// 
    /// Acceptable values are:
    /// 
    /// * `"lang_ar"`: Arabic
    /// 
    /// * `"lang_bg"`: Bulgarian
    /// 
    /// * `"lang_ca"`: Catalan
    /// 
    /// * `"lang_cs"`: Czech
    /// 
    /// * `"lang_da"`: Danish
    /// 
    /// * `"lang_de"`: German
    /// 
    /// * `"lang_el"`: Greek
    /// 
    /// * `"lang_en"`: English
    /// 
    /// * `"lang_es"`: Spanish
    /// 
    /// * `"lang_et"`: Estonian
    /// 
    /// * `"lang_fi"`: Finnish
    /// 
    /// * `"lang_fr"`: French
    /// 
    /// * `"lang_hr"`: Croatian
    /// 
    /// * `"lang_hu"`: Hungarian
    /// 
    /// * `"lang_id"`: Indonesian
    /// 
    /// * `"lang_is"`: Icelandic
    /// 
    /// * `"lang_it"`: Italian
    /// 
    /// * `"lang_iw"`: Hebrew
    /// 
    /// * `"lang_ja"`: Japanese
    /// 
    /// * `"lang_ko"`: Korean
    /// 
    /// * `"lang_lt"`: Lithuanian
    /// 
    /// * `"lang_lv"`: Latvian
    /// 
    /// * `"lang_nl"`: Dutch
    /// 
    /// * `"lang_no"`: Norwegian
    /// 
    /// * `"lang_pl"`: Polish
    /// 
    /// * `"lang_pt"`: Portuguese
    /// 
    /// * `"lang_ro"`: Romanian
    /// 
    /// * `"lang_ru"`: Russian
    /// 
    /// * `"lang_sk"`: Slovak
    /// 
    /// * `"lang_sl"`: Slovenian
    /// 
    /// * `"lang_sr"`: Serbian
    /// 
    /// * `"lang_sv"`: Swedish
    /// 
    /// * `"lang_tr"`: Turkish
    /// 
    /// * `"lang_zh-CN"`: Chinese (Simplified)
    /// 
    /// * `"lang_zh-TW"`: Chinese (Traditional)
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Specifies the starting value for a search range. Use `lowRange` and
    /// `highRange` to append an inclusive search range of `lowRange...highRange`
    /// to the query.
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular
    /// URL.
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type. Acceptable values are:
    /// 
    /// * `"clipart"`
    /// 
    /// * `"face"`
    /// 
    /// * `"lineart"`
    /// 
    /// * `"stock"`
    /// 
    /// * `"photo"`
    /// 
    /// * `"animated"`
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size. Acceptable values are:
    /// 
    /// * `"huge"`
    /// 
    /// * `"icon"`
    /// 
    /// * `"large"`
    /// 
    /// * `"medium"`
    /// 
    /// * `"small"`
    /// 
    /// * `"xlarge"`
    /// 
    /// * `"xxlarge"`
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color. Acceptable values are:
    /// 
    /// * `"black"`
    /// 
    /// * `"blue"`
    /// 
    /// * `"brown"`
    /// 
    /// * `"gray"`
    /// 
    /// * `"green"`
    /// 
    /// * `"orange"`
    /// 
    /// * `"pink"`
    /// 
    /// * `"purple"`
    /// 
    /// * `"red"`
    /// 
    /// * `"teal"`
    /// 
    /// * `"white"`
    /// 
    /// * `"yellow"`
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, transparent, or color images.
    /// Acceptable values are:
    /// 
    /// * `"color"`
    /// 
    /// * `"gray"`
    /// 
    /// * `"mono"`: black and white
    /// 
    /// * `"trans"`: transparent background
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the specified query terms to the query, as if they were combined
    /// with a logical AND operator.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language.
    /// 
    /// * Explicitly setting this parameter improves the performance and the
    /// quality of your search results.
    /// 
    /// * See the [Interface
    /// Languages](https://developers.google.com/custom-search/docs/xml_results#wsInterfaceLanguages)
    /// section of [Internationalizing Queries and Results
    /// Presentation](https://developers.google.com/custom-search/docs/xml_results#wsInternationalizing)
    /// for more information, and (Supported Interface
    /// Languages)[https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages]
    /// for a list of supported languages.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Specifies the ending value for a search range.
    /// 
    /// * Use `lowRange` and `highRange` to append an inclusive search range of
    /// `lowRange...highRange` to the query.
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// **Deprecated**. Use the `gl` parameter for a similar effect.
    /// 
    /// The local Google domain (for example, google.com, google.de, or
    /// google.fr) to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user.
    /// 
    /// * The `gl` parameter value is a two-letter country code. The `gl` parameter
    /// boosts search results whose country of origin matches the parameter value.
    /// See the [Country
    /// Codes](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCodes)
    /// page for a list of valid values.
    /// 
    /// * Specifying a `gl` parameter value should lead to more relevant results.
    /// This is particularly true for international customers and, even more
    /// specifically, for customers in English- speaking countries other than the
    /// United States.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter.
    /// 
    /// * See [Automatic
    /// Filtering](https://developers.google.com/custom-search/docs/xml_results#automaticFiltering)
    /// for more information about Google's search results filters. Note that host
    /// crowding filtering applies only to multi-site searches.
    /// 
    /// * By default, Google applies filtering to all search results to improve the
    /// quality of those results.
    /// 
    /// Acceptable values are:
    /// 
    /// * `0`: Turns off duplicate content filter.
    /// 
    /// * `1`: Turns on duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restricts results to files of a specified extension. A list of file types
    /// indexable by Google can be found in Search Console [Help
    /// Center](https://support.google.com/webmasters/answer/35287).
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the
    /// search results.
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain.
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Restricts results to URLs based on date. Supported values include:
    /// 
    /// * `d[number]`: requests results from the specified number of past days.
    /// 
    /// * `w[number]`: requests results from the specified number of past weeks.
    /// 
    /// * `m[number]`: requests results from the specified number of past months.
    /// 
    /// * `y[number]`: requests results from the specified number of past years.
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The custom search engine ID to use for this request.
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Restricts search results to documents originating in a particular country.
    /// You may use [Boolean
    /// operators](https://developers.google.com/custom-search/docs/xml_results_appendices#booleanOperators)
    /// in the cr parameter's value.
    /// 
    /// Google Search determines the country of a document by analyzing:
    /// 
    /// * the top-level domain (TLD) of the document's URL
    /// 
    /// * the geographic location of the Web server's IP address
    /// 
    /// See the [Country Parameter
    /// Values](https://developers.google.com/custom-search/docs/xml_results_appendices#countryCollections)
    /// page for a list of valid values for this parameter.
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseListCall<'a, C, A> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Enables or disables [Simplified and Traditional Chinese
    /// Search](https://developers.google.com/custom-search/docs/xml_results#chineseSearch).
    /// 
    /// The default value for this parameter is 0 (zero), meaning that the feature
    /// is enabled. Supported values are:
    /// 
    /// * `1`: Disabled
    /// 
    /// * `0`: Enabled (default)
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
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> CseListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


