// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *pagespeedonline* crate version *0.1.3+20150317*, where *20150317* is the exact revision of the *pagespeedonline:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.3*.
//! 
//! Everything else about the *pagespeedonline* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/speed/docs/insights/v2/getting-started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/pagespeedonline2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Pagespeedonline.html) ... 
//! 
//! * pagespeedapi
//!  * [*runpagespeed*](struct.PagespeedapiRunpagespeedCall.html)
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
//! * **[Hub](struct.Pagespeedonline.html)**
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
//! let r = hub.pagespeedapi().runpagespeed(...).doit()
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
//! google-pagespeedonline2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_pagespeedonline2 as pagespeedonline2;
//! use pagespeedonline2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use pagespeedonline2::Pagespeedonline;
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
//! let mut hub = Pagespeedonline::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.pagespeedapi().runpagespeed("url")
//!              .strategy("accusam")
//!              .screenshot(true)
//!              .add_rule("justo")
//!              .locale("amet.")
//!              .filter_third_party_resources(false)
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

/// Central instance to access all Pagespeedonline related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_pagespeedonline2 as pagespeedonline2;
/// use pagespeedonline2::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use pagespeedonline2::Pagespeedonline;
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
/// let mut hub = Pagespeedonline::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .strategy("sea")
///              .screenshot(false)
///              .add_rule("dolores")
///              .locale("gubergren")
///              .filter_third_party_resources(false)
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
pub struct Pagespeedonline<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Pagespeedonline<C, NC, A> {}

impl<'a, C, NC, A> Pagespeedonline<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Pagespeedonline<C, NC, A> {
        Pagespeedonline {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.3".to_string(),
            _m: PhantomData
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a, C, NC, A> {
        PagespeedapiMethods { hub: &self }
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
/// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PagespeedApiFormatStringV2ArgsSecondaryRects {
    /// The width of the rect.
    pub width: i32,
    /// The top coordinate of the rect, in page coordinates.
    pub top: i32,
    /// The height of the rect.
    pub height: i32,
    /// The left coordinate of the rect, in page coordinates.
    pub left: i32,
}

impl NestedType for PagespeedApiFormatStringV2ArgsSecondaryRects {}
impl Part for PagespeedApiFormatStringV2ArgsSecondaryRects {}


/// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PagespeedApiFormatStringV2ArgsRects {
    /// The width of the rect.
    pub width: i32,
    /// The top coordinate of the rect, in page coordinates.
    pub top: i32,
    /// The height of the rect.
    pub height: i32,
    /// The left coordinate of the rect, in page coordinates.
    pub left: i32,
}

impl NestedType for PagespeedApiFormatStringV2ArgsRects {}
impl Part for PagespeedApiFormatStringV2ArgsRects {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PagespeedApiFormatStringV2 {
    /// List of arguments for the format string.
    pub args: Vec<PagespeedApiFormatStringV2Args>,
    /// A localized format string with {{FOO}} placeholders, where 'FOO' is the key of the argument whose value should be substituted. For HYPERLINK arguments, the format string will instead contain {{BEGIN_FOO}} and {{END_FOO}} for the argument with key 'FOO'.
    pub format: String,
}

impl Part for PagespeedApiFormatStringV2 {}


/// List of entries that provide information about URLs in the url block. Optional.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultFormattedResultsRuleResultsUrlBlocksUrls {
    /// List of entries that provide additional details about a single URL. Optional.
    pub details: Vec<PagespeedApiFormatStringV2>,
    /// A format string that gives information about the URL, and a list of arguments for that format string.
    pub result: PagespeedApiFormatStringV2,
}

impl NestedType for ResultFormattedResultsRuleResultsUrlBlocksUrls {}
impl Part for ResultFormattedResultsRuleResultsUrlBlocksUrls {}


/// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultPageStats {
    /// Number of response bytes for flash resources on the page.
    #[serde(alias="flashResponseBytes")]
    pub flash_response_bytes: String,
    /// Total size of all request bytes sent by the page.
    #[serde(alias="totalRequestBytes")]
    pub total_request_bytes: String,
    /// Number of CSS resources referenced by the page.
    #[serde(alias="numberCssResources")]
    pub number_css_resources: i32,
    /// Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page.
    #[serde(alias="textResponseBytes")]
    pub text_response_bytes: String,
    /// Number of HTTP resources loaded by the page.
    #[serde(alias="numberResources")]
    pub number_resources: i32,
    /// Number of response bytes for other resources on the page.
    #[serde(alias="otherResponseBytes")]
    pub other_response_bytes: String,
    /// Number of response bytes for image resources on the page.
    #[serde(alias="imageResponseBytes")]
    pub image_response_bytes: String,
    /// Number of unique hosts referenced by the page.
    #[serde(alias="numberHosts")]
    pub number_hosts: i32,
    /// Number of uncompressed response bytes for JS resources on the page.
    #[serde(alias="javascriptResponseBytes")]
    pub javascript_response_bytes: String,
    /// Number of uncompressed response bytes for the main HTML document and all iframes on the page.
    #[serde(alias="htmlResponseBytes")]
    pub html_response_bytes: String,
    /// Number of uncompressed response bytes for CSS resources on the page.
    #[serde(alias="cssResponseBytes")]
    pub css_response_bytes: String,
    /// Number of JavaScript resources referenced by the page.
    #[serde(alias="numberJsResources")]
    pub number_js_resources: i32,
    /// Number of static (i.e. cacheable) resources on the page.
    #[serde(alias="numberStaticResources")]
    pub number_static_resources: i32,
}

impl NestedType for ResultPageStats {}
impl Part for ResultPageStats {}


/// The enum-like identifier for this rule. For instance "EnableKeepAlive" or "AvoidCssImport". Not localized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultFormattedResultsRuleResults {
    /// Localized name of the rule, intended for presentation to a user.
    #[serde(alias="localizedRuleName")]
    pub localized_rule_name: String,
    /// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
    #[serde(alias="urlBlocks")]
    pub url_blocks: Vec<ResultFormattedResultsRuleResultsUrlBlocks>,
    /// List of rule groups that this rule belongs to. Each entry in the list is one of "SPEED" or "USABILITY".
    pub groups: Vec<String>,
    /// The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal.
    #[serde(alias="ruleImpact")]
    pub rule_impact: f64,
    /// A brief summary description for the rule, indicating at a high level what should be done to follow the rule and what benefit can be gained by doing so.
    pub summary: PagespeedApiFormatStringV2,
}

impl NestedType for ResultFormattedResultsRuleResults {}
impl Part for ResultFormattedResultsRuleResults {}


/// The version of PageSpeed used to generate these results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultVersion {
    /// The major version number of PageSpeed used to generate these results.
    pub major: i32,
    /// The minor version number of PageSpeed used to generate these results.
    pub minor: i32,
}

impl NestedType for ResultVersion {}
impl Part for ResultVersion {}


/// List of arguments for the format string.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PagespeedApiFormatStringV2Args {
    /// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
    pub rects: Vec<PagespeedApiFormatStringV2ArgsRects>,
    /// The placeholder key for this arg, as a string.
    pub key: String,
    /// Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, DURATION, VERBATIM_STRING, PERCENTAGE, HYPERLINK, or SNAPSHOT_RECT.
    #[serde(alias="type")]
    pub type_: String,
    /// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
    pub secondary_rects: Vec<PagespeedApiFormatStringV2ArgsSecondaryRects>,
    /// Argument value, as a localized string.
    pub value: String,
}

impl NestedType for PagespeedApiFormatStringV2Args {}
impl Part for PagespeedApiFormatStringV2Args {}


/// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultFormattedResultsRuleResultsUrlBlocks {
    /// Heading to be displayed with the list of URLs.
    pub header: PagespeedApiFormatStringV2,
    /// List of entries that provide information about URLs in the url block. Optional.
    pub urls: Vec<ResultFormattedResultsRuleResultsUrlBlocksUrls>,
}

impl NestedType for ResultFormattedResultsRuleResultsUrlBlocks {}
impl Part for ResultFormattedResultsRuleResultsUrlBlocks {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PagespeedApiImageV2 {
    /// Width of screenshot in pixels.
    pub width: i32,
    /// Unique string key, if any, identifying this image.
    pub key: String,
    /// The region of the page that is captured by this image, with dimensions measured in CSS pixels.
    pub page_rect: PagespeedApiImageV2PageRect,
    /// Image data base64 encoded.
    pub data: String,
    /// Mime type of image data (e.g. "image/jpeg").
    pub mime_type: String,
    /// Height of screenshot in pixels.
    pub height: i32,
}

impl Part for PagespeedApiImageV2 {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [runpagespeed pagespeedapi](struct.PagespeedapiRunpagespeedCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultType {
    /// Kind of result.
    pub kind: String,
    /// Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error.
    #[serde(alias="responseCode")]
    pub response_code: i32,
    /// Base64-encoded screenshot of the page that was analyzed.
    pub screenshot: PagespeedApiImageV2,
    /// Title of the page, as displayed in the browser's title bar.
    pub title: String,
    /// A map with one entry for each rule group in these results.
    #[serde(alias="ruleGroups")]
    pub rule_groups: HashMap<String, ResultRuleGroups>,
    /// The version of PageSpeed used to generate these results.
    pub version: ResultVersion,
    /// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
    #[serde(alias="formattedResults")]
    pub formatted_results: ResultFormattedResults,
    /// List of rules that were specified in the request, but which the server did not know how to instantiate.
    #[serde(alias="invalidRules")]
    pub invalid_rules: Vec<String>,
    /// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
    #[serde(alias="pageStats")]
    pub page_stats: ResultPageStats,
    /// Canonicalized and final URL for the document, after following page redirects (if any).
    pub id: String,
}

impl ResponseResult for ResultType {}


/// The region of the page that is captured by this image, with dimensions measured in CSS pixels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PagespeedApiImageV2PageRect {
    /// The width of the rect.
    pub width: i32,
    /// The top coordinate of the rect, in page coordinates.
    pub top: i32,
    /// The height of the rect.
    pub height: i32,
    /// The left coordinate of the rect, in page coordinates.
    pub left: i32,
}

impl NestedType for PagespeedApiImageV2PageRect {}
impl Part for PagespeedApiImageV2PageRect {}


/// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultFormattedResults {
    /// The locale of the formattedResults, e.g. "en_US".
    pub locale: String,
    /// Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server.
    #[serde(alias="ruleResults")]
    pub rule_results: HashMap<String, ResultFormattedResultsRuleResults>,
}

impl NestedType for ResultFormattedResults {}
impl Part for ResultFormattedResults {}


/// The name of this rule group: one of "SPEED" or "USABILITY".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResultRuleGroups {
    /// The score (0-100) for this rule group, which indicates how much better a page could be in that category (e.g. how much faster, or how much more usable). A high score indicates little room for improvement, while a lower score indicates more room for improvement.
    pub score: i32,
}

impl NestedType for ResultRuleGroups {}
impl Part for ResultRuleGroups {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *pagespeedapi* resources.
/// It is not used directly, but through the `Pagespeedonline` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_pagespeedonline2 as pagespeedonline2;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use pagespeedonline2::Pagespeedonline;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Pagespeedonline::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Pagespeedonline<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for PagespeedapiMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> PagespeedapiMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    /// 
    /// # Arguments
    ///
    /// * `url` - The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _url: url.to_string(),
            _strategy: Default::default(),
            _screenshot: Default::default(),
            _rule: Default::default(),
            _locale: Default::default(),
            _filter_third_party_resources: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
///
/// A builder for the *runpagespeed* method supported by a *pagespeedapi* resource.
/// It is not used directly, but through a `PagespeedapiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_pagespeedonline2 as pagespeedonline2;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use pagespeedonline2::Pagespeedonline;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Pagespeedonline::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .strategy("ea")
///              .screenshot(false)
///              .add_rule("justo")
///              .locale("justo")
///              .filter_third_party_resources(true)
///              .doit();
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Pagespeedonline<C, NC, A>,
    _url: String,
    _strategy: Option<String>,
    _screenshot: Option<bool>,
    _rule: Vec<String>,
    _locale: Option<String>,
    _filter_third_party_resources: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for PagespeedapiRunpagespeedCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PagespeedapiRunpagespeedCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ResultType)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("url", self._url.to_string()));
        if let Some(value) = self._strategy {
            params.push(("strategy", value.to_string()));
        }
        if let Some(value) = self._screenshot {
            params.push(("screenshot", value.to_string()));
        }
        if self._rule.len() > 0 {
            let mut s = String::new();
            for f in self._rule.iter() {
                s.push_str(&("/".to_string() + &f.to_string()));
            }
            params.push(("rule", s));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._filter_third_party_resources {
            params.push(("filter_third_party_resources", value.to_string()));
        }
        for &field in ["alt", "url", "strategy", "screenshot", "rule", "locale", "filter_third_party_resources"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/pagespeedonline/v2/runPagespeed".to_string();
        
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


    /// Sets the *url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The URL to fetch and analyze
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
        self._url = new_value.to_string();
        self
    }
    /// Sets the *strategy* query property to the given value.
    ///
    /// 
    /// The analysis strategy to use
    pub fn strategy(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
        self._strategy = Some(new_value.to_string());
        self
    }
    /// Sets the *screenshot* query property to the given value.
    ///
    /// 
    /// Indicates if binary data containing a screenshot should be included
    pub fn screenshot(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
        self._screenshot = Some(new_value);
        self
    }
    /// Append the given value to the *rule* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// 
    /// A PageSpeed rule to run; if none are given, all rules are run
    pub fn add_rule(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
        self._rule.push(new_value.to_string());
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// The locale used to localize formatted results
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *filter_third_party_resources* query property to the given value.
    ///
    /// 
    /// Indicates if third party resources should be filtered out before PageSpeed analysis.
    pub fn filter_third_party_resources(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
        self._filter_third_party_resources = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PagespeedapiRunpagespeedCall<'a, C, NC, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, C, NC, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


