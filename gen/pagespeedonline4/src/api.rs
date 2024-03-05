use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

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
/// extern crate google_pagespeedonline4 as pagespeedonline4;
/// use pagespeedonline4::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use pagespeedonline4::{Pagespeedonline, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Pagespeedonline::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("ut")
///              .utm_campaign("gubergren")
///              .strategy("rebum.")
///              .snapshots(true)
///              .screenshot(true)
///              .add_rule("ipsum")
///              .locale("est")
///              .filter_third_party_resources(true)
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
#[derive(Clone)]
pub struct Pagespeedonline<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Pagespeedonline<S> {}

impl<'a, S> Pagespeedonline<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Pagespeedonline<S> {
        Pagespeedonline {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://www.googleapis.com/pagespeedonline/v4/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a, S> {
        PagespeedapiMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/pagespeedonline/v4/`.
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4 {
    /// List of arguments for the format string.
    
    pub args: Option<Vec<PagespeedApiFormatStringV4Args>>,
    /// A localized format string with {{FOO}} placeholders, where 'FOO' is the key of the argument whose value should be substituted. For HYPERLINK arguments, the format string will instead contain {{BEGIN_FOO}} and {{END_FOO}} for the argument with key 'FOO'.
    
    pub format: Option<String>,
}

impl client::Part for PagespeedApiFormatStringV4 {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiImageV4 {
    /// Image data base64 encoded.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Height of screenshot in pixels.
    
    pub height: Option<i32>,
    /// Unique string key, if any, identifying this image.
    
    pub key: Option<String>,
    /// Mime type of image data (e.g. "image/jpeg").
    
    pub mime_type: Option<String>,
    /// no description provided
    
    pub page_rect: Option<PagespeedApiImageV4PageRect>,
    /// Width of screenshot in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for PagespeedApiImageV4 {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [runpagespeed pagespeedapi](PagespeedapiRunpagespeedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4 {
    /// The captcha verify result
    #[serde(rename="captchaResult")]
    
    pub captcha_result: Option<String>,
    /// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename="formattedResults")]
    
    pub formatted_results: Option<PagespeedApiPagespeedResponseV4FormattedResults>,
    /// Canonicalized and final URL for the document, after following page redirects (if any).
    
    pub id: Option<String>,
    /// List of rules that were specified in the request, but which the server did not know how to instantiate.
    #[serde(rename="invalidRules")]
    
    pub invalid_rules: Option<Vec<String>>,
    /// Kind of result.
    
    pub kind: Option<String>,
    /// Metrics of end users' page loading experience.
    #[serde(rename="loadingExperience")]
    
    pub loading_experience: Option<PagespeedApiPagespeedResponseV4LoadingExperience>,
    /// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
    #[serde(rename="pageStats")]
    
    pub page_stats: Option<PagespeedApiPagespeedResponseV4PageStats>,
    /// Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error.
    #[serde(rename="responseCode")]
    
    pub response_code: Option<i32>,
    /// A map with one entry for each rule group in these results.
    #[serde(rename="ruleGroups")]
    
    pub rule_groups: Option<HashMap<String, PagespeedApiPagespeedResponseV4RuleGroups>>,
    /// Base64-encoded screenshot of the page that was analyzed.
    
    pub screenshot: Option<PagespeedApiImageV4>,
    /// Additional base64-encoded screenshots of the page, in various partial render states.
    
    pub snapshots: Option<Vec<PagespeedApiImageV4>>,
    /// Title of the page, as displayed in the browser's title bar.
    
    pub title: Option<String>,
    /// The version of PageSpeed used to generate these results.
    
    pub version: Option<PagespeedApiPagespeedResponseV4Version>,
}

impl client::ResponseResult for PagespeedApiPagespeedResponseV4 {}


/// List of arguments for the format string.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4Args {
    /// The placeholder key for this arg, as a string.
    
    pub key: Option<String>,
    /// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
    
    pub rects: Option<Vec<PagespeedApiFormatStringV4ArgsRects>>,
    /// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
    
    pub secondary_rects: Option<Vec<PagespeedApiFormatStringV4ArgsSecondaryRects>>,
    /// Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, DURATION, VERBATIM_STRING, PERCENTAGE, HYPERLINK, or SNAPSHOT_RECT.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Argument value, as a localized string.
    
    pub value: Option<String>,
}

impl client::NestedType for PagespeedApiFormatStringV4Args {}
impl client::Part for PagespeedApiFormatStringV4Args {}


/// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4ArgsRects {
    /// no description provided
    
    pub height: Option<i32>,
    /// no description provided
    
    pub left: Option<i32>,
    /// no description provided
    
    pub top: Option<i32>,
    /// no description provided
    
    pub width: Option<i32>,
}

impl client::NestedType for PagespeedApiFormatStringV4ArgsRects {}
impl client::Part for PagespeedApiFormatStringV4ArgsRects {}


/// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4ArgsSecondaryRects {
    /// no description provided
    
    pub height: Option<i32>,
    /// no description provided
    
    pub left: Option<i32>,
    /// no description provided
    
    pub top: Option<i32>,
    /// no description provided
    
    pub width: Option<i32>,
}

impl client::NestedType for PagespeedApiFormatStringV4ArgsSecondaryRects {}
impl client::Part for PagespeedApiFormatStringV4ArgsSecondaryRects {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiImageV4PageRect {
    /// no description provided
    
    pub height: Option<i32>,
    /// no description provided
    
    pub left: Option<i32>,
    /// no description provided
    
    pub top: Option<i32>,
    /// no description provided
    
    pub width: Option<i32>,
}

impl client::NestedType for PagespeedApiImageV4PageRect {}
impl client::Part for PagespeedApiImageV4PageRect {}


/// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResults {
    /// The locale of the formattedResults, e.g. "en_US".
    
    pub locale: Option<String>,
    /// Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename="ruleResults")]
    
    pub rule_results: Option<HashMap<String, PagespeedApiPagespeedResponseV4FormattedResultsRuleResults>>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4FormattedResults {}
impl client::Part for PagespeedApiPagespeedResponseV4FormattedResults {}


/// The enum-like identifier for this rule. For instance "EnableKeepAlive" or "AvoidCssImport". Not localized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResults {
    /// Whether this rule is in 'beta'. Rules in beta are new rules that are being tested, which do not impact the overall score.
    
    pub beta: Option<bool>,
    /// List of rule groups that this rule belongs to. Each entry in the list is one of "SPEED", "USABILITY", or "SECURITY".
    
    pub groups: Option<Vec<String>>,
    /// Localized name of the rule, intended for presentation to a user.
    #[serde(rename="localizedRuleName")]
    
    pub localized_rule_name: Option<String>,
    /// The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal.
    #[serde(rename="ruleImpact")]
    
    pub rule_impact: Option<f64>,
    /// A brief summary description for the rule, indicating at a high level what should be done to follow the rule and what benefit can be gained by doing so.
    
    pub summary: Option<PagespeedApiFormatStringV4>,
    /// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
    #[serde(rename="urlBlocks")]
    
    pub url_blocks: Option<Vec<PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks>>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4FormattedResultsRuleResults {}
impl client::Part for PagespeedApiPagespeedResponseV4FormattedResultsRuleResults {}


/// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks {
    /// Heading to be displayed with the list of URLs.
    
    pub header: Option<PagespeedApiFormatStringV4>,
    /// List of entries that provide information about URLs in the url block. Optional.
    
    pub urls: Option<Vec<PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls>>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks {}
impl client::Part for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks {}


/// List of entries that provide information about URLs in the url block. Optional.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls {
    /// List of entries that provide additional details about a single URL. Optional.
    
    pub details: Option<Vec<PagespeedApiFormatStringV4>>,
    /// A format string that gives information about the URL, and a list of arguments for that format string.
    
    pub result: Option<PagespeedApiFormatStringV4>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls {}
impl client::Part for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls {}


/// Metrics of end users' page loading experience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4LoadingExperience {
    /// The url, pattern or origin which the metrics are on.
    
    pub id: Option<String>,
    /// no description provided
    
    pub initial_url: Option<String>,
    /// no description provided
    
    pub metrics: Option<HashMap<String, PagespeedApiPagespeedResponseV4LoadingExperienceMetrics>>,
    /// no description provided
    
    pub overall_category: Option<String>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4LoadingExperience {}
impl client::Part for PagespeedApiPagespeedResponseV4LoadingExperience {}


/// The type of the metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4LoadingExperienceMetrics {
    /// no description provided
    
    pub category: Option<String>,
    /// no description provided
    
    pub distributions: Option<Vec<PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions>>,
    /// no description provided
    
    pub median: Option<i32>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4LoadingExperienceMetrics {}
impl client::Part for PagespeedApiPagespeedResponseV4LoadingExperienceMetrics {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions {
    /// no description provided
    
    pub max: Option<i32>,
    /// no description provided
    
    pub min: Option<i32>,
    /// no description provided
    
    pub proportion: Option<f64>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions {}
impl client::Part for PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions {}


/// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4PageStats {
    /// Content management system (CMS) used for the page.
    
    pub cms: Option<String>,
    /// Number of uncompressed response bytes for CSS resources on the page.
    #[serde(rename="cssResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub css_response_bytes: Option<i64>,
    /// Number of response bytes for flash resources on the page.
    #[serde(rename="flashResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub flash_response_bytes: Option<i64>,
    /// Number of uncompressed response bytes for the main HTML document and all iframes on the page.
    #[serde(rename="htmlResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub html_response_bytes: Option<i64>,
    /// Number of response bytes for image resources on the page.
    #[serde(rename="imageResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub image_response_bytes: Option<i64>,
    /// Number of uncompressed response bytes for JS resources on the page.
    #[serde(rename="javascriptResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub javascript_response_bytes: Option<i64>,
    /// The needed round trips to load render blocking resources
    #[serde(rename="numRenderBlockingRoundTrips")]
    
    pub num_render_blocking_round_trips: Option<i32>,
    /// The needed round trips to load the full page
    #[serde(rename="numTotalRoundTrips")]
    
    pub num_total_round_trips: Option<i32>,
    /// Number of CSS resources referenced by the page.
    #[serde(rename="numberCssResources")]
    
    pub number_css_resources: Option<i32>,
    /// Number of unique hosts referenced by the page.
    #[serde(rename="numberHosts")]
    
    pub number_hosts: Option<i32>,
    /// Number of JavaScript resources referenced by the page.
    #[serde(rename="numberJsResources")]
    
    pub number_js_resources: Option<i32>,
    /// Number of HTTP resources loaded by the page.
    #[serde(rename="numberResources")]
    
    pub number_resources: Option<i32>,
    /// Number of roboted resources.
    #[serde(rename="numberRobotedResources")]
    
    pub number_roboted_resources: Option<i32>,
    /// Number of static (i.e. cacheable) resources on the page.
    #[serde(rename="numberStaticResources")]
    
    pub number_static_resources: Option<i32>,
    /// Number of transient-failed resources.
    #[serde(rename="numberTransientFetchFailureResources")]
    
    pub number_transient_fetch_failure_resources: Option<i32>,
    /// Number of response bytes for other resources on the page.
    #[serde(rename="otherResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub other_response_bytes: Option<i64>,
    /// Number of over-the-wire bytes, uses the default gzip compression strategy as an estimation.
    #[serde(rename="overTheWireResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub over_the_wire_response_bytes: Option<i64>,
    /// List of roboted urls.
    #[serde(rename="robotedUrls")]
    
    pub roboted_urls: Option<Vec<String>>,
    /// Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page.
    #[serde(rename="textResponseBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub text_response_bytes: Option<i64>,
    /// Total size of all request bytes sent by the page.
    #[serde(rename="totalRequestBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_request_bytes: Option<i64>,
    /// List of transient fetch failure urls.
    #[serde(rename="transientFetchFailureUrls")]
    
    pub transient_fetch_failure_urls: Option<Vec<String>>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4PageStats {}
impl client::Part for PagespeedApiPagespeedResponseV4PageStats {}


/// The name of this rule group: one of "SPEED", "USABILITY", or "SECURITY".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4RuleGroups {
    /// no description provided
    
    pub pass: Option<bool>,
    /// The score (0-100) for this rule group, which indicates how much better a page could be in that category (e.g. how much faster, or how much more usable, or how much more secure). A high score indicates little room for improvement, while a lower score indicates more room for improvement.
    
    pub score: Option<i32>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4RuleGroups {}
impl client::Part for PagespeedApiPagespeedResponseV4RuleGroups {}


/// The version of PageSpeed used to generate these results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4Version {
    /// The major version number of PageSpeed used to generate these results.
    
    pub major: Option<i32>,
    /// The minor version number of PageSpeed used to generate these results.
    
    pub minor: Option<i32>,
}

impl client::NestedType for PagespeedApiPagespeedResponseV4Version {}
impl client::Part for PagespeedApiPagespeedResponseV4Version {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *pagespeedapi* resources.
/// It is not used directly, but through the [`Pagespeedonline`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pagespeedonline4 as pagespeedonline4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use pagespeedonline4::{Pagespeedonline, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Pagespeedonline::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, S>
    where S: 'a {

    hub: &'a Pagespeedonline<S>,
}

impl<'a, S> client::MethodsBuilder for PagespeedapiMethods<'a, S> {}

impl<'a, S> PagespeedapiMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    /// 
    /// # Arguments
    ///
    /// * `url` - The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _url: url.to_string(),
            _utm_source: Default::default(),
            _utm_campaign: Default::default(),
            _strategy: Default::default(),
            _snapshots: Default::default(),
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
/// It is not used directly, but through a [`PagespeedapiMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_pagespeedonline4 as pagespeedonline4;
/// # async fn dox() {
/// # use std::default::Default;
/// # use pagespeedonline4::{Pagespeedonline, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Pagespeedonline::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("dolor")
///              .utm_campaign("Lorem")
///              .strategy("eos")
///              .snapshots(false)
///              .screenshot(true)
///              .add_rule("duo")
///              .locale("sed")
///              .filter_third_party_resources(true)
///              .doit().await;
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, S>
    where S: 'a {

    hub: &'a Pagespeedonline<S>,
    _url: String,
    _utm_source: Option<String>,
    _utm_campaign: Option<String>,
    _strategy: Option<String>,
    _snapshots: Option<bool>,
    _screenshot: Option<bool>,
    _rule: Vec<String>,
    _locale: Option<String>,
    _filter_third_party_resources: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for PagespeedapiRunpagespeedCall<'a, S> {}

impl<'a, S> PagespeedapiRunpagespeedCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, PagespeedApiPagespeedResponseV4)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "url", "utm_source", "utm_campaign", "strategy", "snapshots", "screenshot", "rule", "locale", "filter_third_party_resources"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("url", self._url);
        if let Some(value) = self._utm_source.as_ref() {
            params.push("utm_source", value);
        }
        if let Some(value) = self._utm_campaign.as_ref() {
            params.push("utm_campaign", value);
        }
        if let Some(value) = self._strategy.as_ref() {
            params.push("strategy", value);
        }
        if let Some(value) = self._snapshots.as_ref() {
            params.push("snapshots", value.to_string());
        }
        if let Some(value) = self._screenshot.as_ref() {
            params.push("screenshot", value.to_string());
        }
        if self._rule.len() > 0 {
            for f in self._rule.iter() {
                params.push("rule", f);
            }
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if let Some(value) = self._filter_third_party_resources.as_ref() {
            params.push("filter_third_party_resources", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "runPagespeed";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = params.parse_with_url(&url);



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// The URL to fetch and analyze
    ///
    /// Sets the *url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._url = new_value.to_string();
        self
    }
    /// Campaign source for analytics.
    ///
    /// Sets the *utm_source* query property to the given value.
    pub fn utm_source(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._utm_source = Some(new_value.to_string());
        self
    }
    /// Campaign name for analytics.
    ///
    /// Sets the *utm_campaign* query property to the given value.
    pub fn utm_campaign(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._utm_campaign = Some(new_value.to_string());
        self
    }
    /// The analysis strategy (desktop or mobile) to use, and desktop is the default
    ///
    /// Sets the *strategy* query property to the given value.
    pub fn strategy(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._strategy = Some(new_value.to_string());
        self
    }
    /// Indicates if binary data containing snapshot images should be included
    ///
    /// Sets the *snapshots* query property to the given value.
    pub fn snapshots(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._snapshots = Some(new_value);
        self
    }
    /// Indicates if binary data containing a screenshot should be included
    ///
    /// Sets the *screenshot* query property to the given value.
    pub fn screenshot(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._screenshot = Some(new_value);
        self
    }
    /// A PageSpeed rule to run; if none are given, all rules are run
    ///
    /// Append the given value to the *rule* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_rule(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._rule.push(new_value.to_string());
        self
    }
    /// The locale used to localize formatted results
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Indicates if third party resources should be filtered out before PageSpeed analysis.
    ///
    /// Sets the *filter_third_party_resources* query property to the given value.
    pub fn filter_third_party_resources(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._filter_third_party_resources = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PagespeedapiRunpagespeedCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


