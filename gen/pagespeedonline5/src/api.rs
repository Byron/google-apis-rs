use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

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
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use pagespeedonline5::PagespeedInsights;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PagespeedInsights::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("gubergren")
///              .utm_campaign("eos")
///              .strategy("dolor")
///              .locale("ea")
///              .add_category("ipsum")
///              .captcha_token("invidunt")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
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
pub struct PagespeedInsights<> {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for PagespeedInsights<> {}

impl<'a, > PagespeedInsights<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> PagespeedInsights<> {
        PagespeedInsights {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/2.0.5".to_string(),
            _base_url: "https://pagespeedonline.googleapis.com/".to_string(),
            _root_url: "https://pagespeedonline.googleapis.com/".to_string(),
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a> {
        PagespeedapiMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.5`.
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
/// A light reference to an audit by id, used to group and weight audits in a given category.
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

impl client::Part for AuditRefs {}


/// A proportion of data in the total distribution, bucketed by a min/max percentage. Each bucket's range is bounded by min <= x < max, In millisecond.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// Upper bound for a bucket's range.
    pub max: Option<i32>,
    /// Lower bound for a bucket's range.
    pub min: Option<i32>,
    /// The proportion of data in this bucket.
    pub proportion: Option<f64>,
}

impl client::Part for Bucket {}


/// The categories in a Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Categories {
    /// The accessibility category, containing all accessibility related audits.
    pub accessibility: Option<LighthouseCategoryV5>,
    /// The best practices category, containing all best practices related audits.
    #[serde(rename="best-practices")]
    pub best_practices: Option<LighthouseCategoryV5>,
    /// The performance category, containing all performance related audits.
    pub performance: Option<LighthouseCategoryV5>,
    /// The Progressive-Web-App (PWA) category, containing all pwa related audits.
    pub pwa: Option<LighthouseCategoryV5>,
    /// The Search-Engine-Optimization (SEO) category, containing all seo related audits.
    pub seo: Option<LighthouseCategoryV5>,
}

impl client::Part for Categories {}


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

impl client::Part for CategoryGroupV5 {}


/// Message containing the configuration settings for the Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigSettings {
    /// How Lighthouse was run, e.g. from the Chrome extension or from the npm module.
    pub channel: Option<String>,
    /// The form factor the emulation should use. This field is deprecated, form_factor should be used instead.
    #[serde(rename="emulatedFormFactor")]
    pub emulated_form_factor: Option<String>,
    /// How Lighthouse should interpret this run in regards to scoring performance metrics and skipping mobile-only tests in desktop.
    #[serde(rename="formFactor")]
    pub form_factor: Option<String>,
    /// The locale setting.
    pub locale: Option<String>,
    /// List of categories of audits the run should conduct.
    #[serde(rename="onlyCategories")]
    pub only_categories: Option<String>,
}

impl client::Part for ConfigSettings {}


/// Message containing environment configuration for a Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// The benchmark index number that indicates rough device class.
    #[serde(rename="benchmarkIndex")]
    pub benchmark_index: Option<f64>,
    /// The user agent string of the version of Chrome used.
    #[serde(rename="hostUserAgent")]
    pub host_user_agent: Option<String>,
    /// The user agent string that was sent over the network.
    #[serde(rename="networkUserAgent")]
    pub network_user_agent: Option<String>,
}

impl client::Part for Environment {}


/// Message containing the i18n data for the LHR - Version 1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18n {
    /// Internationalized strings that are formatted to the locale in configSettings.
    #[serde(rename="rendererFormattedStrings")]
    pub renderer_formatted_strings: Option<RendererFormattedStrings>,
}

impl client::Part for I18n {}


/// An audit's result object in a Lighthouse result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseAuditResultV5 {
    /// The description of the audit.
    pub description: Option<String>,
    /// Freeform details section of the audit.
    pub details: Option<HashMap<String, String>>,
    /// The value that should be displayed on the UI for this audit.
    #[serde(rename="displayValue")]
    pub display_value: Option<String>,
    /// An error message from a thrown error inside the audit.
    #[serde(rename="errorMessage")]
    pub error_message: Option<String>,
    /// An explanation of the errors in the audit.
    pub explanation: Option<String>,
    /// The audit's id.
    pub id: Option<String>,
    /// A numeric value that has a meaning specific to the audit, e.g. the number of nodes in the DOM or the timestamp of a specific load event. More information can be found in the audit details, if present.
    #[serde(rename="numericValue")]
    pub numeric_value: Option<f64>,
    /// The score of the audit, can be null.
    pub score: Option<String>,
    /// The enumerated score display mode.
    #[serde(rename="scoreDisplayMode")]
    pub score_display_mode: Option<String>,
    /// The human readable title.
    pub title: Option<String>,
    /// Possible warnings that occurred in the audit, can be null.
    pub warnings: Option<String>,
}

impl client::Part for LighthouseAuditResultV5 {}


/// A Lighthouse category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseCategoryV5 {
    /// An array of references to all the audit members of this category.
    #[serde(rename="auditRefs")]
    pub audit_refs: Option<Vec<AuditRefs>>,
    /// A more detailed description of the category and its importance.
    pub description: Option<String>,
    /// The string identifier of the category.
    pub id: Option<String>,
    /// A description for the manual audits in the category.
    #[serde(rename="manualDescription")]
    pub manual_description: Option<String>,
    /// The overall score of the category, the weighted average of all its audits. (The category's score, can be null.)
    pub score: Option<String>,
    /// The human-friendly name of the category.
    pub title: Option<String>,
}

impl client::Part for LighthouseCategoryV5 {}


/// The Lighthouse result object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseResultV5 {
    /// Map of audits in the LHR.
    pub audits: Option<HashMap<String, LighthouseAuditResultV5>>,
    /// Map of categories in the LHR.
    pub categories: Option<Categories>,
    /// Map of category groups in the LHR.
    #[serde(rename="categoryGroups")]
    pub category_groups: Option<HashMap<String, CategoryGroupV5>>,
    /// The configuration settings for this LHR.
    #[serde(rename="configSettings")]
    pub config_settings: Option<ConfigSettings>,
    /// Environment settings that were used when making this LHR.
    pub environment: Option<Environment>,
    /// The time that this run was fetched.
    #[serde(rename="fetchTime")]
    pub fetch_time: Option<String>,
    /// The final resolved url that was audited.
    #[serde(rename="finalUrl")]
    pub final_url: Option<String>,
    /// The internationalization strings that are required to render the LHR.
    pub i18n: Option<I18n>,
    /// The lighthouse version that was used to generate this LHR.
    #[serde(rename="lighthouseVersion")]
    pub lighthouse_version: Option<String>,
    /// The original requested url.
    #[serde(rename="requestedUrl")]
    pub requested_url: Option<String>,
    /// List of all run warnings in the LHR. Will always output to at least `[]`.
    #[serde(rename="runWarnings")]
    pub run_warnings: Option<Vec<String>>,
    /// A top-level error message that, if present, indicates a serious enough problem that this Lighthouse result may need to be discarded.
    #[serde(rename="runtimeError")]
    pub runtime_error: Option<RuntimeError>,
    /// The Stack Pack advice strings.
    #[serde(rename="stackPacks")]
    pub stack_packs: Option<Vec<StackPack>>,
    /// Timing information for this LHR.
    pub timing: Option<Timing>,
    /// The user agent that was used to run this LHR.
    #[serde(rename="userAgent")]
    pub user_agent: Option<String>,
}

impl client::Part for LighthouseResultV5 {}


/// The CrUX loading experience object that contains CrUX data breakdowns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiLoadingExperienceV5 {
    /// The url, pattern or origin which the metrics are on.
    pub id: Option<String>,
    /// The requested URL, which may differ from the resolved "id".
    pub initial_url: Option<String>,
    /// The map of .
    pub metrics: Option<HashMap<String, UserPageLoadMetricV5>>,
    /// True if the result is an origin fallback from a page, false otherwise.
    pub origin_fallback: Option<bool>,
    /// The human readable speed "category" of the id.
    pub overall_category: Option<String>,
}

impl client::Part for PagespeedApiLoadingExperienceV5 {}


/// The Pagespeed API response object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [runpagespeed pagespeedapi](PagespeedapiRunpagespeedCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV5 {
    /// The UTC timestamp of this analysis.
    #[serde(rename="analysisUTCTimestamp")]
    pub analysis_utc_timestamp: Option<String>,
    /// The captcha verify result
    #[serde(rename="captchaResult")]
    pub captcha_result: Option<String>,
    /// Canonicalized and final URL for the document, after following page redirects (if any).
    pub id: Option<String>,
    /// Kind of result.
    pub kind: Option<String>,
    /// Lighthouse response for the audit url as an object.
    #[serde(rename="lighthouseResult")]
    pub lighthouse_result: Option<LighthouseResultV5>,
    /// Metrics of end users' page loading experience.
    #[serde(rename="loadingExperience")]
    pub loading_experience: Option<PagespeedApiLoadingExperienceV5>,
    /// Metrics of the aggregated page loading experience of the origin
    #[serde(rename="originLoadingExperience")]
    pub origin_loading_experience: Option<PagespeedApiLoadingExperienceV5>,
    /// The version of PageSpeed used to generate these results.
    pub version: Option<PagespeedVersion>,
}

impl client::ResponseResult for PagespeedApiPagespeedResponseV5 {}


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

impl client::Part for PagespeedVersion {}


/// Message holding the formatted strings used in the renderer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RendererFormattedStrings {
    /// The tooltip text on an expandable chevron icon.
    #[serde(rename="auditGroupExpandTooltip")]
    pub audit_group_expand_tooltip: Option<String>,
    /// The label for the initial request in a critical request chain.
    #[serde(rename="crcInitialNavigation")]
    pub crc_initial_navigation: Option<String>,
    /// The label for values shown in the summary of critical request chains.
    #[serde(rename="crcLongestDurationLabel")]
    pub crc_longest_duration_label: Option<String>,
    /// The label shown next to an audit or metric that has had an error.
    #[serde(rename="errorLabel")]
    pub error_label: Option<String>,
    /// The error string shown next to an erroring audit.
    #[serde(rename="errorMissingAuditInfo")]
    pub error_missing_audit_info: Option<String>,
    /// The title of the lab data performance category.
    #[serde(rename="labDataTitle")]
    pub lab_data_title: Option<String>,
    /// The disclaimer shown under performance explaning that the network can vary.
    #[serde(rename="lsPerformanceCategoryDescription")]
    pub ls_performance_category_description: Option<String>,
    /// The heading shown above a list of audits that were not computerd in the run.
    #[serde(rename="manualAuditsGroupTitle")]
    pub manual_audits_group_title: Option<String>,
    /// The heading shown above a list of audits that do not apply to a page.
    #[serde(rename="notApplicableAuditsGroupTitle")]
    pub not_applicable_audits_group_title: Option<String>,
    /// The heading for the estimated page load savings opportunity of an audit.
    #[serde(rename="opportunityResourceColumnLabel")]
    pub opportunity_resource_column_label: Option<String>,
    /// The heading for the estimated page load savings of opportunity audits.
    #[serde(rename="opportunitySavingsColumnLabel")]
    pub opportunity_savings_column_label: Option<String>,
    /// The heading that is shown above a list of audits that are passing.
    #[serde(rename="passedAuditsGroupTitle")]
    pub passed_audits_group_title: Option<String>,
    /// The label that explains the score gauges scale (0-49, 50-89, 90-100).
    #[serde(rename="scorescaleLabel")]
    pub scorescale_label: Option<String>,
    /// The label shown preceding important warnings that may have invalidated an entire report.
    #[serde(rename="toplevelWarningsMessage")]
    pub toplevel_warnings_message: Option<String>,
    /// The disclaimer shown below a performance metric value.
    #[serde(rename="varianceDisclaimer")]
    pub variance_disclaimer: Option<String>,
    /// The label shown above a bulleted list of warnings.
    #[serde(rename="warningHeader")]
    pub warning_header: Option<String>,
}

impl client::Part for RendererFormattedStrings {}


/// Message containing a runtime error config.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeError {
    /// The enumerated Lighthouse Error code.
    pub code: Option<String>,
    /// A human readable message explaining the error code.
    pub message: Option<String>,
}

impl client::Part for RuntimeError {}


/// Message containing Stack Pack information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackPack {
    /// The stack pack advice strings.
    pub descriptions: Option<HashMap<String, String>>,
    /// The stack pack icon data uri.
    #[serde(rename="iconDataURL")]
    pub icon_data_url: Option<String>,
    /// The stack pack id.
    pub id: Option<String>,
    /// The stack pack title.
    pub title: Option<String>,
}

impl client::Part for StackPack {}


/// Message containing the performance timing data for the Lighthouse run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Timing {
    /// The total duration of Lighthouse's run.
    pub total: Option<f64>,
}

impl client::Part for Timing {}


/// A CrUX metric object for a single metric and form factor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPageLoadMetricV5 {
    /// The category of the specific time metric.
    pub category: Option<String>,
    /// Metric distributions. Proportions should sum up to 1.
    pub distributions: Option<Vec<Bucket>>,
    /// Identifies the form factor of the metric being collected.
    #[serde(rename="formFactor")]
    pub form_factor: Option<String>,
    /// The median number of the metric, in millisecond.
    pub median: Option<i32>,
    /// Identifies the type of the metric.
    #[serde(rename="metricId")]
    pub metric_id: Option<String>,
    /// We use this field to store certain percentile value for this metric. For v4, this field contains pc50. For v5, this field contains pc90.
    pub percentile: Option<i32>,
}

impl client::Part for UserPageLoadMetricV5 {}



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
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use pagespeedonline5::PagespeedInsights;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PagespeedInsights::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a>
    where  {

    hub: &'a PagespeedInsights<>,
}

impl<'a> client::MethodsBuilder for PagespeedapiMethods<'a> {}

impl<'a> PagespeedapiMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    /// 
    /// # Arguments
    ///
    /// * `url` - Required. The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _url: url.to_string(),
            _utm_source: Default::default(),
            _utm_campaign: Default::default(),
            _strategy: Default::default(),
            _locale: Default::default(),
            _category: Default::default(),
            _captcha_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
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
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use pagespeedonline5::PagespeedInsights;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PagespeedInsights::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("duo")
///              .utm_campaign("ipsum")
///              .strategy("sed")
///              .locale("ut")
///              .add_category("gubergren")
///              .captcha_token("rebum.")
///              .doit().await;
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a>
    where  {

    hub: &'a PagespeedInsights<>,
    _url: String,
    _utm_source: Option<String>,
    _utm_campaign: Option<String>,
    _strategy: Option<String>,
    _locale: Option<String>,
    _category: Vec<String>,
    _captcha_token: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for PagespeedapiRunpagespeedCall<'a> {}

impl<'a> PagespeedapiRunpagespeedCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, PagespeedApiPagespeedResponseV5)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
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
        if let Some(value) = self._captcha_token {
            params.push(("captchaToken", value.to_string()));
        }
        for &field in ["alt", "url", "utm_source", "utm_campaign", "strategy", "locale", "category", "captchaToken"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
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


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The URL to fetch and analyze
    ///
    /// Sets the *url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a> {
        self._url = new_value.to_string();
        self
    }
    /// Campaign source for analytics.
    ///
    /// Sets the *utm_source* query property to the given value.
    pub fn utm_source(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a> {
        self._utm_source = Some(new_value.to_string());
        self
    }
    /// Campaign name for analytics.
    ///
    /// Sets the *utm_campaign* query property to the given value.
    pub fn utm_campaign(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a> {
        self._utm_campaign = Some(new_value.to_string());
        self
    }
    /// The analysis strategy (desktop or mobile) to use, and desktop is the default
    ///
    /// Sets the *strategy* query property to the given value.
    pub fn strategy(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a> {
        self._strategy = Some(new_value.to_string());
        self
    }
    /// The locale used to localize formatted results
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// A Lighthouse category to run; if none are given, only Performance category will be run
    ///
    /// Append the given value to the *category* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_category(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a> {
        self._category.push(new_value.to_string());
        self
    }
    /// The captcha token passed when filling out a captcha.
    ///
    /// Sets the *captcha token* query property to the given value.
    pub fn captcha_token(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a> {
        self._captcha_token = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PagespeedapiRunpagespeedCall<'a> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> PagespeedapiRunpagespeedCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


