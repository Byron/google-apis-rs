// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *pagespeedonline* crate version *1.0.9+20190507*, where *20190507* is the exact revision of the *pagespeedonline:v5* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.9*.
//! 
//! Everything else about the *pagespeedonline* *v5* API can be found at the
//! [official documentation site](https://developers.google.com/speed/docs/insights/v5/get-started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/pagespeedonline5).
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
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
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
//! google-pagespeedonline5 = "*"
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
//! extern crate google_pagespeedonline5 as pagespeedonline5;
//! use pagespeedonline5::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use pagespeedonline5::Pagespeedonline;
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
//! let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.pagespeedapi().runpagespeed("url")
//!              .utm_source("accusam")
//!              .utm_campaign("takimata")
//!              .strategy("justo")
//!              .locale("amet.")
//!              .add_category("erat")
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

pub use cmn::*;


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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_pagespeedonline5 as pagespeedonline5;
/// use pagespeedonline5::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use pagespeedonline5::Pagespeedonline;
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
/// let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("sea")
///              .utm_campaign("nonumy")
///              .strategy("dolores")
///              .locale("gubergren")
///              .add_category("sadipscing")
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
pub struct Pagespeedonline<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Pagespeedonline<C, A> {}

impl<'a, C, A> Pagespeedonline<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Pagespeedonline<C, A> {
        Pagespeedonline {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.9".to_string(),
            _base_url: "https://www.googleapis.com/pagespeedonline/v5/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a, C, A> {
        PagespeedapiMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.9`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/pagespeedonline/v5/`.
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
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [runpagespeed pagespeedapi](struct.PagespeedapiRunpagespeedCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV5 {
    /// Kind of result.
    pub kind: Option<String>,
    /// The captcha verify result
    #[serde(rename="captchaResult")]
    pub captcha_result: Option<String>,
    /// Canonicalized and final URL for the document, after following page redirects (if any).
    pub id: Option<String>,
    /// Metrics of end users' page loading experience.
    #[serde(rename="loadingExperience")]
    pub loading_experience: Option<PagespeedApiLoadingExperienceV5>,
    /// The version of PageSpeed used to generate these results.
    pub version: Option<PagespeedApiPagespeedResponseV5Version>,
    /// The UTC timestamp of this analysis.
    #[serde(rename="analysisUTCTimestamp")]
    pub analysis_utc_timestamp: Option<String>,
    /// Lighthouse response for the audit url as an object.
    #[serde(rename="lighthouseResult")]
    pub lighthouse_result: Option<LighthouseResultV5>,
    /// Metrics of the aggregated page loading experience of the origin
    #[serde(rename="originLoadingExperience")]
    pub origin_loading_experience: Option<PagespeedApiLoadingExperienceV5>,
}

impl ResponseResult for PagespeedApiPagespeedResponseV5 {}


/// Timing information for this LHR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5Timing {
    /// The total duration of Lighthouse's run.
    pub total: Option<f64>,
}

impl NestedType for LighthouseResultV5Timing {}
impl Part for LighthouseResultV5Timing {}


/// Map of categories in the LHR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5Categories {
    /// The Search-Engine-Optimization (SEO) category, containing all seo related audits.
    pub seo: Option<LighthouseCategoryV5>,
    /// The best practices category, containing all web best practice related audits.
    #[serde(rename="best-practices")]
    pub best_practices: Option<LighthouseCategoryV5>,
    /// The performance category, containing all performance related audits.
    pub performance: Option<LighthouseCategoryV5>,
    /// The accessibility category, containing all accessibility related audits.
    pub accessibility: Option<LighthouseCategoryV5>,
    /// The Progressive-Web-App (PWA) category, containing all pwa related audits.
    pub pwa: Option<LighthouseCategoryV5>,
}

impl NestedType for LighthouseResultV5Categories {}
impl Part for LighthouseResultV5Categories {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseCategoryV5 {
    /// A description for the manual audits in the category.
    #[serde(rename="manualDescription")]
    pub manual_description: Option<String>,
    /// no description provided
    pub score: Option<String>,
    /// A more detailed description of the category and its importance.
    pub description: Option<String>,
    /// The human-friendly name of the category.
    pub title: Option<String>,
    /// An array of references to all the audit members of this category.
    #[serde(rename="auditRefs")]
    pub audit_refs: Option<Vec<LighthouseCategoryV5AuditRefs>>,
    /// The string identifier of the category.
    pub id: Option<String>,
}

impl Part for LighthouseCategoryV5 {}


/// Environment settings that were used when making this LHR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5Environment {
    /// The benchmark index number that indicates rough device class.
    #[serde(rename="benchmarkIndex")]
    pub benchmark_index: Option<f64>,
    /// The user agent string that was sent over the network.
    #[serde(rename="networkUserAgent")]
    pub network_user_agent: Option<String>,
    /// The user agent string of the version of Chrome used.
    #[serde(rename="hostUserAgent")]
    pub host_user_agent: Option<String>,
}

impl NestedType for LighthouseResultV5Environment {}
impl Part for LighthouseResultV5Environment {}


/// Internationalized strings that are formatted to the locale in configSettings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5I18nRendererFormattedStrings {
    /// The heading that is shown above a list of audits that are passing.
    #[serde(rename="passedAuditsGroupTitle")]
    pub passed_audits_group_title: Option<String>,
    /// The tooltip text on an expandable chevron icon.
    #[serde(rename="auditGroupExpandTooltip")]
    pub audit_group_expand_tooltip: Option<String>,
    /// The disclaimer shown below a performance metric value.
    #[serde(rename="varianceDisclaimer")]
    pub variance_disclaimer: Option<String>,
    /// The heading for the estimated page load savings of opportunity audits.
    #[serde(rename="opportunitySavingsColumnLabel")]
    pub opportunity_savings_column_label: Option<String>,
    /// The error string shown next to an erroring audit.
    #[serde(rename="errorMissingAuditInfo")]
    pub error_missing_audit_info: Option<String>,
    /// The disclaimer shown under performance explaning that the network can vary.
    #[serde(rename="lsPerformanceCategoryDescription")]
    pub ls_performance_category_description: Option<String>,
    /// The heading shown above a list of audits that were not computerd in the run.
    #[serde(rename="manualAuditsGroupTitle")]
    pub manual_audits_group_title: Option<String>,
    /// The heading shown above a list of audits that do not apply to a page.
    #[serde(rename="notApplicableAuditsGroupTitle")]
    pub not_applicable_audits_group_title: Option<String>,
    /// The label that explains the score gauges scale (0-49, 50-89, 90-100).
    #[serde(rename="scorescaleLabel")]
    pub scorescale_label: Option<String>,
    /// The label shown above a bulleted list of warnings.
    #[serde(rename="warningHeader")]
    pub warning_header: Option<String>,
    /// The title of the lab data performance category.
    #[serde(rename="labDataTitle")]
    pub lab_data_title: Option<String>,
    /// The label shown preceding important warnings that may have invalidated an entire report.
    #[serde(rename="toplevelWarningsMessage")]
    pub toplevel_warnings_message: Option<String>,
    /// The label for values shown in the summary of critical request chains.
    #[serde(rename="crcLongestDurationLabel")]
    pub crc_longest_duration_label: Option<String>,
    /// The label shown next to an audit or metric that has had an error.
    #[serde(rename="errorLabel")]
    pub error_label: Option<String>,
    /// The heading for the estimated page load savings opportunity of an audit.
    #[serde(rename="opportunityResourceColumnLabel")]
    pub opportunity_resource_column_label: Option<String>,
    /// The label for the initial request in a critical request chain.
    #[serde(rename="crcInitialNavigation")]
    pub crc_initial_navigation: Option<String>,
}

impl NestedType for LighthouseResultV5I18nRendererFormattedStrings {}
impl Part for LighthouseResultV5I18nRendererFormattedStrings {}


/// A grouping contained in a category that groups similar audits together.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5CategoryGroups {
    /// An optional human readable description of the category group.
    pub description: Option<String>,
    /// The title of the category group.
    pub title: Option<String>,
}

impl NestedType for LighthouseResultV5CategoryGroups {}
impl Part for LighthouseResultV5CategoryGroups {}


/// The configuration settings for this LHR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5ConfigSettings {
    /// The locale setting.
    pub locale: Option<String>,
    /// no description provided
    #[serde(rename="onlyCategories")]
    pub only_categories: Option<String>,
    /// The form factor the emulation should use.
    #[serde(rename="emulatedFormFactor")]
    pub emulated_form_factor: Option<String>,
}

impl NestedType for LighthouseResultV5ConfigSettings {}
impl Part for LighthouseResultV5ConfigSettings {}


/// The internationalization strings that are required to render the LHR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5I18n {
    /// Internationalized strings that are formatted to the locale in configSettings.
    #[serde(rename="rendererFormattedStrings")]
    pub renderer_formatted_strings: Option<LighthouseResultV5I18nRendererFormattedStrings>,
}

impl NestedType for LighthouseResultV5I18n {}
impl Part for LighthouseResultV5I18n {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5 {
    /// Environment settings that were used when making this LHR.
    pub environment: Option<LighthouseResultV5Environment>,
    /// The time that this run was fetched.
    #[serde(rename="fetchTime")]
    pub fetch_time: Option<String>,
    /// Map of category groups in the LHR.
    #[serde(rename="categoryGroups")]
    pub category_groups: Option<HashMap<String, LighthouseResultV5CategoryGroups>>,
    /// The configuration settings for this LHR.
    #[serde(rename="configSettings")]
    pub config_settings: Option<LighthouseResultV5ConfigSettings>,
    /// A top-level error message that, if present, indicates a serious enough problem that this Lighthouse result may need to be discarded.
    #[serde(rename="runtimeError")]
    pub runtime_error: Option<LighthouseResultV5RuntimeError>,
    /// The final resolved url that was audited.
    #[serde(rename="finalUrl")]
    pub final_url: Option<String>,
    /// The lighthouse version that was used to generate this LHR.
    #[serde(rename="lighthouseVersion")]
    pub lighthouse_version: Option<String>,
    /// The Stack Pack advice strings.
    #[serde(rename="stackPacks")]
    pub stack_packs: Option<Vec<LighthouseResultV5StackPacks>>,
    /// The internationalization strings that are required to render the LHR.
    pub i18n: Option<LighthouseResultV5I18n>,
    /// List of all run warnings in the LHR. Will always output to at least `[]`.
    #[serde(rename="runWarnings")]
    pub run_warnings: Option<Vec<String>>,
    /// Timing information for this LHR.
    pub timing: Option<LighthouseResultV5Timing>,
    /// The user agent that was used to run this LHR.
    #[serde(rename="userAgent")]
    pub user_agent: Option<String>,
    /// Map of audits in the LHR.
    pub audits: Option<HashMap<String, LighthouseAuditResultV5>>,
    /// Map of categories in the LHR.
    pub categories: Option<LighthouseResultV5Categories>,
    /// The original requested url.
    #[serde(rename="requestedUrl")]
    pub requested_url: Option<String>,
}

impl Part for LighthouseResultV5 {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiLoadingExperienceV5MetricsDistributions {
    /// no description provided
    pub max: Option<i32>,
    /// no description provided
    pub proportion: Option<f64>,
    /// no description provided
    pub min: Option<i32>,
}

impl NestedType for PagespeedApiLoadingExperienceV5MetricsDistributions {}
impl Part for PagespeedApiLoadingExperienceV5MetricsDistributions {}


/// A top-level error message that, if present, indicates a serious enough problem that this Lighthouse result may need to be discarded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5RuntimeError {
    /// A human readable message explaining the error code.
    pub message: Option<String>,
    /// The enumerated Lighthouse Error code.
    pub code: Option<String>,
}

impl NestedType for LighthouseResultV5RuntimeError {}
impl Part for LighthouseResultV5RuntimeError {}


/// An array of references to all the audit members of this category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseCategoryV5AuditRefs {
    /// The category group that the audit belongs to (optional).
    pub group: Option<String>,
    /// The audit ref id.
    pub id: Option<String>,
    /// The weight this audit's score has on the overall category score.
    pub weight: Option<f64>,
}

impl NestedType for LighthouseCategoryV5AuditRefs {}
impl Part for LighthouseCategoryV5AuditRefs {}


/// The version of PageSpeed used to generate these results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV5Version {
    /// The major version number of PageSpeed used to generate these results.
    pub major: Option<i32>,
    /// The minor version number of PageSpeed used to generate these results.
    pub minor: Option<i32>,
}

impl NestedType for PagespeedApiPagespeedResponseV5Version {}
impl Part for PagespeedApiPagespeedResponseV5Version {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiLoadingExperienceV5 {
    /// no description provided
    pub metrics: Option<HashMap<String, PagespeedApiLoadingExperienceV5Metrics>>,
    /// The url, pattern or origin which the metrics are on.
    pub id: Option<String>,
    /// no description provided
    pub overall_category: Option<String>,
    /// no description provided
    pub initial_url: Option<String>,
}

impl Part for PagespeedApiLoadingExperienceV5 {}


/// The Stack Pack advice strings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5StackPacks {
    /// The stack pack advice strings.
    pub descriptions: Option<HashMap<String, String>>,
    /// The stack pack id.
    pub id: Option<String>,
    /// The stack pack icon data uri.
    #[serde(rename="iconDataURL")]
    pub icon_data_url: Option<String>,
    /// The stack pack title.
    pub title: Option<String>,
}

impl NestedType for LighthouseResultV5StackPacks {}
impl Part for LighthouseResultV5StackPacks {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseAuditResultV5 {
    /// The description of the audit.
    pub description: Option<String>,
    /// The human readable title.
    pub title: Option<String>,
    /// An explanation of the errors in the audit.
    pub explanation: Option<String>,
    /// An error message from a thrown error inside the audit.
    #[serde(rename="errorMessage")]
    pub error_message: Option<String>,
    /// no description provided
    pub score: Option<String>,
    /// Freeform details section of the audit.
    pub details: Option<HashMap<String, String>>,
    /// no description provided
    pub warnings: Option<String>,
    /// The value that should be displayed on the UI for this audit.
    #[serde(rename="displayValue")]
    pub display_value: Option<String>,
    /// The enumerated score display mode.
    #[serde(rename="scoreDisplayMode")]
    pub score_display_mode: Option<String>,
    /// The audit's id.
    pub id: Option<String>,
}

impl Part for LighthouseAuditResultV5 {}


/// The type of the metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiLoadingExperienceV5Metrics {
    /// no description provided
    pub category: Option<String>,
    /// no description provided
    pub percentile: Option<i32>,
    /// no description provided
    pub distributions: Option<Vec<PagespeedApiLoadingExperienceV5MetricsDistributions>>,
}

impl NestedType for PagespeedApiLoadingExperienceV5Metrics {}
impl Part for PagespeedApiLoadingExperienceV5Metrics {}



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
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_pagespeedonline5 as pagespeedonline5;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use pagespeedonline5::Pagespeedonline;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Pagespeedonline<C, A>,
}

impl<'a, C, A> MethodsBuilder for PagespeedapiMethods<'a, C, A> {}

impl<'a, C, A> PagespeedapiMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    /// 
    /// # Arguments
    ///
    /// * `url` - The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _url: url.to_string(),
            _utm_source: Default::default(),
            _utm_campaign: Default::default(),
            _strategy: Default::default(),
            _locale: Default::default(),
            _category: Default::default(),
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
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_pagespeedonline5 as pagespeedonline5;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use pagespeedonline5::Pagespeedonline;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("ea")
///              .utm_campaign("no")
///              .strategy("justo")
///              .locale("justo")
///              .add_category("et")
///              .doit();
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Pagespeedonline<C, A>,
    _url: String,
    _utm_source: Option<String>,
    _utm_campaign: Option<String>,
    _strategy: Option<String>,
    _locale: Option<String>,
    _category: Vec<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for PagespeedapiRunpagespeedCall<'a, C, A> {}

impl<'a, C, A> PagespeedapiRunpagespeedCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PagespeedApiPagespeedResponseV5)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("url", self._url.to_string()));
        if let Some(value) = self._utm_source {
            params.push(("utm_source", value.to_string()));
        }
        if let Some(value) = self._utm_campaign {
            params.push(("utm_campaign", value.to_string()));
        }
        if let Some(value) = self._strategy {
            params.push(("strategy", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if self._category.len() > 0 {
            for f in self._category.iter() {
                params.push(("category", f.to_string()));
            }
        }
        for &field in ["alt", "url", "utm_source", "utm_campaign", "strategy", "locale", "category"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "runPagespeed";
        
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


    /// The URL to fetch and analyze
    ///
    /// Sets the *url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._url = new_value.to_string();
        self
    }
    /// Campaign source for analytics.
    ///
    /// Sets the *utm_source* query property to the given value.
    pub fn utm_source(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._utm_source = Some(new_value.to_string());
        self
    }
    /// Campaign name for analytics.
    ///
    /// Sets the *utm_campaign* query property to the given value.
    pub fn utm_campaign(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._utm_campaign = Some(new_value.to_string());
        self
    }
    /// The analysis strategy (desktop or mobile) to use, and desktop is the default
    ///
    /// Sets the *strategy* query property to the given value.
    pub fn strategy(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._strategy = Some(new_value.to_string());
        self
    }
    /// The locale used to localize formatted results
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// A Lighthouse category to run; if none are given, only Performance category will be run
    ///
    /// Append the given value to the *category* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_category(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._category.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PagespeedapiRunpagespeedCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


