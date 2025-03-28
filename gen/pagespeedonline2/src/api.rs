#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

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
/// extern crate google_pagespeedonline2 as pagespeedonline2;
/// use pagespeedonline2::{Result, Error};
/// # async fn dox() {
/// use pagespeedonline2::{Pagespeedonline, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Pagespeedonline::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .strategy("gubergren")
///              .screenshot(false)
///              .add_rule("dolor")
///              .locale("ea")
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
pub struct Pagespeedonline<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for Pagespeedonline<C> {}

impl<'a, C> Pagespeedonline<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> Pagespeedonline<C> {
        Pagespeedonline {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://www.googleapis.com/pagespeedonline/v2/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a, C> {
        PagespeedapiMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/pagespeedonline/v2/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PagespeedApiFormatStringV2 {
    /// List of arguments for the format string.
    pub args: Option<Vec<PagespeedApiFormatStringV2Args>>,
    /// A localized format string with {{FOO}} placeholders, where 'FOO' is the key of the argument whose value should be substituted. For HYPERLINK arguments, the format string will instead contain {{BEGIN_FOO}} and {{END_FOO}} for the argument with key 'FOO'.
    pub format: Option<String>,
}

impl common::Part for PagespeedApiFormatStringV2 {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PagespeedApiImageV2 {
    /// Image data base64 encoded.
    #[serde_as(as = "Option<common::serde::standard_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Height of screenshot in pixels.
    pub height: Option<i32>,
    /// Unique string key, if any, identifying this image.
    pub key: Option<String>,
    /// Mime type of image data (e.g. "image/jpeg").
    pub mime_type: Option<String>,
    /// The region of the page that is captured by this image, with dimensions measured in CSS pixels.
    pub page_rect: Option<PagespeedApiImageV2PageRect>,
    /// Width of screenshot in pixels.
    pub width: Option<i32>,
}

impl common::Part for PagespeedApiImageV2 {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [runpagespeed pagespeedapi](PagespeedapiRunpagespeedCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Result {
    /// The captcha verify result
    #[serde(rename = "captchaResult")]
    pub captcha_result: Option<String>,
    /// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename = "formattedResults")]
    pub formatted_results: Option<ResultFormattedResults>,
    /// Canonicalized and final URL for the document, after following page redirects (if any).
    pub id: Option<String>,
    /// List of rules that were specified in the request, but which the server did not know how to instantiate.
    #[serde(rename = "invalidRules")]
    pub invalid_rules: Option<Vec<String>>,
    /// Kind of result.
    pub kind: Option<String>,
    /// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
    #[serde(rename = "pageStats")]
    pub page_stats: Option<ResultPageStats>,
    /// Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error.
    #[serde(rename = "responseCode")]
    pub response_code: Option<i32>,
    /// A map with one entry for each rule group in these results.
    #[serde(rename = "ruleGroups")]
    pub rule_groups: Option<HashMap<String, ResultRuleGroups>>,
    /// Base64-encoded screenshot of the page that was analyzed.
    pub screenshot: Option<PagespeedApiImageV2>,
    /// Title of the page, as displayed in the browser's title bar.
    pub title: Option<String>,
    /// The version of PageSpeed used to generate these results.
    pub version: Option<ResultVersion>,
}

impl common::ResponseResult for Result {}

/// List of arguments for the format string.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PagespeedApiFormatStringV2Args {
    /// The placeholder key for this arg, as a string.
    pub key: Option<String>,
    /// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
    pub rects: Option<Vec<PagespeedApiFormatStringV2ArgsRects>>,
    /// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
    pub secondary_rects: Option<Vec<PagespeedApiFormatStringV2ArgsSecondaryRects>>,
    /// Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, DURATION, VERBATIM_STRING, PERCENTAGE, HYPERLINK, or SNAPSHOT_RECT.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Argument value, as a localized string.
    pub value: Option<String>,
}

impl common::NestedType for PagespeedApiFormatStringV2Args {}
impl common::Part for PagespeedApiFormatStringV2Args {}

/// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PagespeedApiFormatStringV2ArgsRects {
    /// The height of the rect.
    pub height: Option<i32>,
    /// The left coordinate of the rect, in page coordinates.
    pub left: Option<i32>,
    /// The top coordinate of the rect, in page coordinates.
    pub top: Option<i32>,
    /// The width of the rect.
    pub width: Option<i32>,
}

impl common::NestedType for PagespeedApiFormatStringV2ArgsRects {}
impl common::Part for PagespeedApiFormatStringV2ArgsRects {}

/// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PagespeedApiFormatStringV2ArgsSecondaryRects {
    /// The height of the rect.
    pub height: Option<i32>,
    /// The left coordinate of the rect, in page coordinates.
    pub left: Option<i32>,
    /// The top coordinate of the rect, in page coordinates.
    pub top: Option<i32>,
    /// The width of the rect.
    pub width: Option<i32>,
}

impl common::NestedType for PagespeedApiFormatStringV2ArgsSecondaryRects {}
impl common::Part for PagespeedApiFormatStringV2ArgsSecondaryRects {}

/// The region of the page that is captured by this image, with dimensions measured in CSS pixels.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PagespeedApiImageV2PageRect {
    /// The height of the rect.
    pub height: Option<i32>,
    /// The left coordinate of the rect, in page coordinates.
    pub left: Option<i32>,
    /// The top coordinate of the rect, in page coordinates.
    pub top: Option<i32>,
    /// The width of the rect.
    pub width: Option<i32>,
}

impl common::NestedType for PagespeedApiImageV2PageRect {}
impl common::Part for PagespeedApiImageV2PageRect {}

/// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultFormattedResults {
    /// The locale of the formattedResults, e.g. "en_US".
    pub locale: Option<String>,
    /// Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename = "ruleResults")]
    pub rule_results: Option<HashMap<String, ResultFormattedResultsRuleResults>>,
}

impl common::NestedType for ResultFormattedResults {}
impl common::Part for ResultFormattedResults {}

/// The enum-like identifier for this rule. For instance "EnableKeepAlive" or "AvoidCssImport". Not localized.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultFormattedResultsRuleResults {
    /// List of rule groups that this rule belongs to. Each entry in the list is one of "SPEED" or "USABILITY".
    pub groups: Option<Vec<String>>,
    /// Localized name of the rule, intended for presentation to a user.
    #[serde(rename = "localizedRuleName")]
    pub localized_rule_name: Option<String>,
    /// The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal.
    #[serde(rename = "ruleImpact")]
    pub rule_impact: Option<f64>,
    /// A brief summary description for the rule, indicating at a high level what should be done to follow the rule and what benefit can be gained by doing so.
    pub summary: Option<PagespeedApiFormatStringV2>,
    /// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
    #[serde(rename = "urlBlocks")]
    pub url_blocks: Option<Vec<ResultFormattedResultsRuleResultsUrlBlocks>>,
}

impl common::NestedType for ResultFormattedResultsRuleResults {}
impl common::Part for ResultFormattedResultsRuleResults {}

/// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultFormattedResultsRuleResultsUrlBlocks {
    /// Heading to be displayed with the list of URLs.
    pub header: Option<PagespeedApiFormatStringV2>,
    /// List of entries that provide information about URLs in the url block. Optional.
    pub urls: Option<Vec<ResultFormattedResultsRuleResultsUrlBlocksUrls>>,
}

impl common::NestedType for ResultFormattedResultsRuleResultsUrlBlocks {}
impl common::Part for ResultFormattedResultsRuleResultsUrlBlocks {}

/// List of entries that provide information about URLs in the url block. Optional.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultFormattedResultsRuleResultsUrlBlocksUrls {
    /// List of entries that provide additional details about a single URL. Optional.
    pub details: Option<Vec<PagespeedApiFormatStringV2>>,
    /// A format string that gives information about the URL, and a list of arguments for that format string.
    pub result: Option<PagespeedApiFormatStringV2>,
}

impl common::NestedType for ResultFormattedResultsRuleResultsUrlBlocksUrls {}
impl common::Part for ResultFormattedResultsRuleResultsUrlBlocksUrls {}

/// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultPageStats {
    /// Number of uncompressed response bytes for CSS resources on the page.
    #[serde(rename = "cssResponseBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub css_response_bytes: Option<i64>,
    /// Number of response bytes for flash resources on the page.
    #[serde(rename = "flashResponseBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub flash_response_bytes: Option<i64>,
    /// Number of uncompressed response bytes for the main HTML document and all iframes on the page.
    #[serde(rename = "htmlResponseBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub html_response_bytes: Option<i64>,
    /// Number of response bytes for image resources on the page.
    #[serde(rename = "imageResponseBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub image_response_bytes: Option<i64>,
    /// Number of uncompressed response bytes for JS resources on the page.
    #[serde(rename = "javascriptResponseBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub javascript_response_bytes: Option<i64>,
    /// Number of CSS resources referenced by the page.
    #[serde(rename = "numberCssResources")]
    pub number_css_resources: Option<i32>,
    /// Number of unique hosts referenced by the page.
    #[serde(rename = "numberHosts")]
    pub number_hosts: Option<i32>,
    /// Number of JavaScript resources referenced by the page.
    #[serde(rename = "numberJsResources")]
    pub number_js_resources: Option<i32>,
    /// Number of HTTP resources loaded by the page.
    #[serde(rename = "numberResources")]
    pub number_resources: Option<i32>,
    /// Number of static (i.e. cacheable) resources on the page.
    #[serde(rename = "numberStaticResources")]
    pub number_static_resources: Option<i32>,
    /// Number of response bytes for other resources on the page.
    #[serde(rename = "otherResponseBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub other_response_bytes: Option<i64>,
    /// Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page.
    #[serde(rename = "textResponseBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub text_response_bytes: Option<i64>,
    /// Total size of all request bytes sent by the page.
    #[serde(rename = "totalRequestBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub total_request_bytes: Option<i64>,
}

impl common::NestedType for ResultPageStats {}
impl common::Part for ResultPageStats {}

/// The name of this rule group: one of "SPEED" or "USABILITY".
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultRuleGroups {
    /// The score (0-100) for this rule group, which indicates how much better a page could be in that category (e.g. how much faster, or how much more usable). A high score indicates little room for improvement, while a lower score indicates more room for improvement.
    pub score: Option<i32>,
}

impl common::NestedType for ResultRuleGroups {}
impl common::Part for ResultRuleGroups {}

/// The version of PageSpeed used to generate these results.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultVersion {
    /// The major version number of PageSpeed used to generate these results.
    pub major: Option<i32>,
    /// The minor version number of PageSpeed used to generate these results.
    pub minor: Option<i32>,
}

impl common::NestedType for ResultVersion {}
impl common::Part for ResultVersion {}

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
/// extern crate google_pagespeedonline2 as pagespeedonline2;
///
/// # async fn dox() {
/// use pagespeedonline2::{Pagespeedonline, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Pagespeedonline::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Pagespeedonline<C>,
}

impl<'a, C> common::MethodsBuilder for PagespeedapiMethods<'a, C> {}

impl<'a, C> PagespeedapiMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a, C> {
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
/// It is not used directly, but through a [`PagespeedapiMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_pagespeedonline2 as pagespeedonline2;
/// # async fn dox() {
/// # use pagespeedonline2::{Pagespeedonline, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Pagespeedonline::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .strategy("amet")
///              .screenshot(true)
///              .add_rule("sed")
///              .locale("ut")
///              .filter_third_party_resources(true)
///              .doit().await;
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, C>
where
    C: 'a,
{
    hub: &'a Pagespeedonline<C>,
    _url: String,
    _strategy: Option<String>,
    _screenshot: Option<bool>,
    _rule: Vec<String>,
    _locale: Option<String>,
    _filter_third_party_resources: Option<bool>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for PagespeedapiRunpagespeedCall<'a, C> {}

impl<'a, C> PagespeedapiRunpagespeedCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Result)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "pagespeedonline.pagespeedapi.runpagespeed",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "url",
            "strategy",
            "screenshot",
            "rule",
            "locale",
            "filter_third_party_resources",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("url", self._url);
        if let Some(value) = self._strategy.as_ref() {
            params.push("strategy", value);
        }
        if let Some(value) = self._screenshot.as_ref() {
            params.push("screenshot", value.to_string());
        }
        if !self._rule.is_empty() {
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
                return Err(common::Error::MissingAPIKey);
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
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
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
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C> {
        self._url = new_value.to_string();
        self
    }
    /// The analysis strategy to use
    ///
    /// Sets the *strategy* query property to the given value.
    pub fn strategy(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C> {
        self._strategy = Some(new_value.to_string());
        self
    }
    /// Indicates if binary data containing a screenshot should be included
    ///
    /// Sets the *screenshot* query property to the given value.
    pub fn screenshot(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, C> {
        self._screenshot = Some(new_value);
        self
    }
    /// A PageSpeed rule to run; if none are given, all rules are run
    ///
    /// Append the given value to the *rule* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_rule(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C> {
        self._rule.push(new_value.to_string());
        self
    }
    /// The locale used to localize formatted results
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Indicates if third party resources should be filtered out before PageSpeed analysis.
    ///
    /// Sets the *filter_third_party_resources* query property to the given value.
    pub fn filter_third_party_resources(
        mut self,
        new_value: bool,
    ) -> PagespeedapiRunpagespeedCall<'a, C> {
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
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> PagespeedapiRunpagespeedCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}
