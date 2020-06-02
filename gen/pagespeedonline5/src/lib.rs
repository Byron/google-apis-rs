// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Pagespeed Insights* crate version *1.0.13+20200318*, where *20200318* is the exact revision of the *pagespeedonline:v5* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.
//! 
//! Everything else about the *Pagespeed Insights* *v5* API can be found at the
//! [official documentation site](https://developers.google.com/speed/docs/insights/v5/about).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/pagespeedonline5).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.PagespeedInsights.html) ... 
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
//! * **[Hub](struct.PagespeedInsights.html)**
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
//! use pagespeedonline5::PagespeedInsights;
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
//! let mut hub = PagespeedInsights::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.pagespeedapi().runpagespeed()
//!              .utm_source("accusam")
//!              .utm_campaign("takimata")
//!              .url("justo")
//!              .strategy("amet.")
//!              .locale("erat")
//!              .add_category("labore")
//!              .captcha_token("sea")
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

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Associate you with your personal info on Google
    Openid,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Openid => "openid",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Openid
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all PagespeedInsights related resource activities
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
/// use pagespeedonline5::PagespeedInsights;
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
/// let mut hub = PagespeedInsights::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed()
///              .utm_source("nonumy")
///              .utm_campaign("dolores")
///              .url("gubergren")
///              .strategy("sadipscing")
///              .locale("aliquyam")
///              .add_category("ea")
///              .captcha_token("no")
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
pub struct PagespeedInsights<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for PagespeedInsights<C, A> {}

impl<'a, C, A> PagespeedInsights<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> PagespeedInsights<C, A> {
        PagespeedInsights {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.13".to_string(),
            _base_url: "https://pagespeedonline.googleapis.com/".to_string(),
            _root_url: "https://pagespeedonline.googleapis.com/".to_string(),
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a, C, A> {
        PagespeedapiMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.13`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://pagespeedonline.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://pagespeedonline.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The Pagespeed API response object.
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
    /// Canonicalized and final URL for the document, after following page
    /// redirects (if any).
    pub id: Option<String>,
    /// Metrics of end users' page loading experience.
    #[serde(rename="loadingExperience")]
    pub loading_experience: Option<PagespeedApiLoadingExperienceV5>,
    /// The version of PageSpeed used to generate these results.
    pub version: Option<PagespeedVersion>,
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


/// Message containing the configuration settings for the Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigSettings {
    /// The locale setting.
    pub locale: Option<String>,
    /// List of categories of audits the run should conduct.
    #[serde(rename="onlyCategories")]
    pub only_categories: Option<String>,
    /// The form factor the emulation should use.
    #[serde(rename="emulatedFormFactor")]
    pub emulated_form_factor: Option<String>,
    /// How Lighthouse was run, e.g. from the Chrome extension or from the npm
    /// module.
    pub channel: Option<String>,
}

impl Part for ConfigSettings {}


/// Message containing a runtime error config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeError {
    /// A human readable message explaining the error code.
    pub message: Option<String>,
    /// The enumerated Lighthouse Error code.
    pub code: Option<String>,
}

impl Part for RuntimeError {}


/// Message holding the formatted strings used in the renderer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RendererFormattedStrings {
    /// The heading that is shown above a list of audits that are passing.
    #[serde(rename="passedAuditsGroupTitle")]
    pub passed_audits_group_title: Option<String>,
    /// The tooltip text on an expandable chevron icon.
    #[serde(rename="auditGroupExpandTooltip")]
    pub audit_group_expand_tooltip: Option<String>,
    /// The heading for the estimated page load savings of opportunity audits.
    #[serde(rename="opportunitySavingsColumnLabel")]
    pub opportunity_savings_column_label: Option<String>,
    /// The error string shown next to an erroring audit.
    #[serde(rename="errorMissingAuditInfo")]
    pub error_missing_audit_info: Option<String>,
    /// The disclaimer shown under performance explaning that the network can
    /// vary.
    #[serde(rename="lsPerformanceCategoryDescription")]
    pub ls_performance_category_description: Option<String>,
    /// The heading shown above a list of audits that were not computerd in the
    /// run.
    #[serde(rename="manualAuditsGroupTitle")]
    pub manual_audits_group_title: Option<String>,
    /// The heading shown above a list of audits that do not apply to a page.
    #[serde(rename="notApplicableAuditsGroupTitle")]
    pub not_applicable_audits_group_title: Option<String>,
    /// The label that explains the score gauges scale (0-49, 50-89, 90-100).
    #[serde(rename="scorescaleLabel")]
    pub scorescale_label: Option<String>,
    /// The disclaimer shown below a performance metric value.
    #[serde(rename="varianceDisclaimer")]
    pub variance_disclaimer: Option<String>,
    /// The title of the lab data performance category.
    #[serde(rename="labDataTitle")]
    pub lab_data_title: Option<String>,
    /// The label shown above a bulleted list of warnings.
    #[serde(rename="warningHeader")]
    pub warning_header: Option<String>,
    /// The label shown preceding important warnings that may have invalidated
    /// an entire report.
    #[serde(rename="toplevelWarningsMessage")]
    pub toplevel_warnings_message: Option<String>,
    /// The label for values shown in the summary of critical request chains.
    #[serde(rename="crcLongestDurationLabel")]
    pub crc_longest_duration_label: Option<String>,
    /// The label shown next to an audit or metric that has had an error.
    #[serde(rename="errorLabel")]
    pub error_label: Option<String>,
    /// The heading for the estimated page load savings opportunity of an
    /// audit.
    #[serde(rename="opportunityResourceColumnLabel")]
    pub opportunity_resource_column_label: Option<String>,
    /// The label for the initial request in a critical request chain.
    #[serde(rename="crcInitialNavigation")]
    pub crc_initial_navigation: Option<String>,
}

impl Part for RendererFormattedStrings {}


/// A proportion of data in the total distribution, bucketed by a min/max
/// percentage. Each bucket's range is bounded by min <= x < max, In
/// millisecond.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// Upper bound for a bucket's range.
    pub max: Option<i32>,
    /// The proportion of data in this bucket.
    pub proportion: Option<f64>,
    /// Lower bound for a bucket's range.
    pub min: Option<i32>,
}

impl Part for Bucket {}


/// The Pagespeed Version object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedVersion {
    /// The major version number of PageSpeed used to generate these results.
    pub major: Option<String>,
    /// The minor version number of PageSpeed used to generate these results.
    pub minor: Option<String>,
}

impl Part for PagespeedVersion {}


/// Message containing environment configuration for a Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
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

impl Part for Environment {}


/// Message containing the i18n data for the LHR - Version 1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18n {
    /// Internationalized strings that are formatted to the locale in
    /// configSettings.
    #[serde(rename="rendererFormattedStrings")]
    pub renderer_formatted_strings: Option<RendererFormattedStrings>,
}

impl Part for I18n {}


/// A Lighthouse category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseCategoryV5 {
    /// A description for the manual audits in the category.
    #[serde(rename="manualDescription")]
    pub manual_description: Option<String>,
    /// The overall score of the category, the weighted average of all its audits.
    /// (The category's score, can be null.)
    pub score: Option<String>,
    /// A more detailed description of the category and its importance.
    pub description: Option<String>,
    /// The human-friendly name of the category.
    pub title: Option<String>,
    /// An array of references to all the audit members of this category.
    #[serde(rename="auditRefs")]
    pub audit_refs: Option<Vec<AuditRefs>>,
    /// The string identifier of the category.
    pub id: Option<String>,
}

impl Part for LighthouseCategoryV5 {}


/// Message containing a category
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoryGroupV5 {
    /// The description of what the category is grouping
    pub description: Option<String>,
    /// The human readable title of the group
    pub title: Option<String>,
}

impl Part for CategoryGroupV5 {}


/// The Lighthouse result object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5 {
    /// Environment settings that were used when making this LHR.
    pub environment: Option<Environment>,
    /// The time that this run was fetched.
    #[serde(rename="fetchTime")]
    pub fetch_time: Option<String>,
    /// Map of category groups in the LHR.
    #[serde(rename="categoryGroups")]
    pub category_groups: Option<HashMap<String, CategoryGroupV5>>,
    /// The configuration settings for this LHR.
    #[serde(rename="configSettings")]
    pub config_settings: Option<ConfigSettings>,
    /// A top-level error message that, if present, indicates a serious enough
    /// problem that this Lighthouse result may need to be discarded.
    #[serde(rename="runtimeError")]
    pub runtime_error: Option<RuntimeError>,
    /// The final resolved url that was audited.
    #[serde(rename="finalUrl")]
    pub final_url: Option<String>,
    /// The lighthouse version that was used to generate this LHR.
    #[serde(rename="lighthouseVersion")]
    pub lighthouse_version: Option<String>,
    /// The Stack Pack advice strings.
    #[serde(rename="stackPacks")]
    pub stack_packs: Option<Vec<StackPack>>,
    /// The internationalization strings that are required to render the LHR.
    pub i18n: Option<I18n>,
    /// List of all run warnings in the LHR.  Will always output to at least `[]`.
    #[serde(rename="runWarnings")]
    pub run_warnings: Option<Vec<String>>,
    /// Timing information for this LHR.
    pub timing: Option<Timing>,
    /// The user agent that was used to run this LHR.
    #[serde(rename="userAgent")]
    pub user_agent: Option<String>,
    /// Map of audits in the LHR.
    pub audits: Option<HashMap<String, LighthouseAuditResultV5>>,
    /// Map of categories in the LHR.
    pub categories: Option<Categories>,
    /// The original requested url.
    #[serde(rename="requestedUrl")]
    pub requested_url: Option<String>,
}

impl Part for LighthouseResultV5 {}


/// A light reference to an audit by id, used to group and weight audits in a
/// given category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditRefs {
    /// The category group that the audit belongs to (optional).
    pub group: Option<String>,
    /// The audit ref id.
    pub id: Option<String>,
    /// The weight this audit's score has on the overall category score.
    pub weight: Option<f64>,
}

impl Part for AuditRefs {}


/// The CrUX loading experience object that contains CrUX data breakdowns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiLoadingExperienceV5 {
    /// The map of <metrics, data>.
    pub metrics: Option<HashMap<String, UserPageLoadMetricV5>>,
    /// The url, pattern or origin which the metrics are on.
    pub id: Option<String>,
    /// The human readable speed "category" of the id.
    pub overall_category: Option<String>,
    /// The requested URL, which may differ from the resolved "id".
    pub initial_url: Option<String>,
}

impl Part for PagespeedApiLoadingExperienceV5 {}


/// A CrUX metric object for a single metric and form factor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPageLoadMetricV5 {
    /// The category of the specific time metric.
    pub category: Option<String>,
    /// Identifies the form factor of the metric being collected.
    #[serde(rename="formFactor")]
    pub form_factor: Option<String>,
    /// We use this field to store certain percentile value for this metric.
    /// For v4, this field contains pc50.
    /// For v5, this field contains pc90.
    pub percentile: Option<i32>,
    /// Metric distributions. Proportions should sum up to 1.
    pub distributions: Option<Vec<Bucket>>,
    /// Identifies the type of the metric.
    #[serde(rename="metricId")]
    pub metric_id: Option<String>,
    /// The median number of the metric, in millisecond.
    pub median: Option<i32>,
}

impl Part for UserPageLoadMetricV5 {}


/// Message containing the performance timing data for the Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Timing {
    /// The total duration of Lighthouse's run.
    pub total: Option<f64>,
}

impl Part for Timing {}


/// An audit's result object in a Lighthouse result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseAuditResultV5 {
    /// A numeric value that has a meaning specific to the audit, e.g. the number
    /// of nodes in the DOM or the timestamp of a specific load event. More
    /// information can be found in the audit details, if present.
    #[serde(rename="numericValue")]
    pub numeric_value: Option<f64>,
    /// The description of the audit.
    pub description: Option<String>,
    /// The human readable title.
    pub title: Option<String>,
    /// An explanation of the errors in the audit.
    pub explanation: Option<String>,
    /// An error message from a thrown error inside the audit.
    #[serde(rename="errorMessage")]
    pub error_message: Option<String>,
    /// The score of the audit, can be null.
    pub score: Option<String>,
    /// Freeform details section of the audit.
    pub details: Option<HashMap<String, String>>,
    /// Possible warnings that occurred in the audit, can be null.
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


/// Message containing Stack Pack information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackPack {
    /// The stack pack id.
    pub id: Option<String>,
    /// The stack pack advice strings.
    pub descriptions: Option<HashMap<String, String>>,
    /// The stack pack icon data uri.
    #[serde(rename="iconDataURL")]
    pub icon_data_url: Option<String>,
    /// The stack pack title.
    pub title: Option<String>,
}

impl Part for StackPack {}


/// The categories in a Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Categories {
    /// The Search-Engine-Optimization (SEO) category, containing all seo related
    /// audits.
    pub seo: Option<LighthouseCategoryV5>,
    /// The best practices category, containing all best practices related
    /// audits.
    #[serde(rename="best-practices")]
    pub best_practices: Option<LighthouseCategoryV5>,
    /// The performance category, containing all performance related audits.
    pub performance: Option<LighthouseCategoryV5>,
    /// The accessibility category, containing all accessibility related audits.
    pub accessibility: Option<LighthouseCategoryV5>,
    /// The Progressive-Web-App (PWA) category, containing all pwa related
    /// audits.
    pub pwa: Option<LighthouseCategoryV5>,
}

impl Part for Categories {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *pagespeedapi* resources.
/// It is not used directly, but through the `PagespeedInsights` hub.
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
/// use pagespeedonline5::PagespeedInsights;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PagespeedInsights::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PagespeedInsights<C, A>,
}

impl<'a, C, A> MethodsBuilder for PagespeedapiMethods<'a, C, A> {}

impl<'a, C, A> PagespeedapiMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns
    /// PageSpeed scores, a list of suggestions to make that page faster, and other
    /// information.
    pub fn runpagespeed(&self) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _utm_source: Default::default(),
            _utm_campaign: Default::default(),
            _url: Default::default(),
            _strategy: Default::default(),
            _locale: Default::default(),
            _category: Default::default(),
            _captcha_token: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Runs PageSpeed analysis on the page at the specified URL, and returns
/// PageSpeed scores, a list of suggestions to make that page faster, and other
/// information.
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
/// # use pagespeedonline5::PagespeedInsights;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PagespeedInsights::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed()
///              .utm_source("justo")
///              .utm_campaign("justo")
///              .url("et")
///              .strategy("et")
///              .locale("diam")
///              .add_category("ipsum")
///              .captcha_token("Lorem")
///              .doit();
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PagespeedInsights<C, A>,
    _utm_source: Option<String>,
    _utm_campaign: Option<String>,
    _url: Option<String>,
    _strategy: Option<String>,
    _locale: Option<String>,
    _category: Vec<String>,
    _captcha_token: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PagespeedapiRunpagespeedCall<'a, C, A> {}

impl<'a, C, A> PagespeedapiRunpagespeedCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PagespeedApiPagespeedResponseV5)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._utm_source {
            params.push(("utm_source", value.to_string()));
        }
        if let Some(value) = self._utm_campaign {
            params.push(("utm_campaign", value.to_string()));
        }
        if let Some(value) = self._url {
            params.push(("url", value.to_string()));
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
        if let Some(value) = self._captcha_token {
            params.push(("captchaToken", value.to_string()));
        }
        for &field in ["alt", "utm_source", "utm_campaign", "url", "strategy", "locale", "category", "captchaToken"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "pagespeedonline/v5/runPagespeed";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Openid.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();



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
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
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
    /// The URL to fetch and analyze
    ///
    /// Sets the *url* query property to the given value.
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._url = Some(new_value.to_string());
        self
    }
    /// The analysis strategy (desktop or mobile) to use, and desktop is the
    /// default
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
    /// A Lighthouse category to run; if none are given, only Performance category
    /// will be run
    ///
    /// Append the given value to the *category* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_category(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._category.push(new_value.to_string());
        self
    }
    /// The captcha token passed when filling out a captcha.
    ///
    /// Sets the *captcha token* query property to the given value.
    pub fn captcha_token(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._captcha_token = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> PagespeedapiRunpagespeedCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Openid`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PagespeedapiRunpagespeedCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


