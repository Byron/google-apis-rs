use super::*;
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
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
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


