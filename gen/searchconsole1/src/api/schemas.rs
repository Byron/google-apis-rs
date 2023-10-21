use super::*;
/// AMP inspection result of the live page or the current information from Google's index, depending on whether you requested a live inspection or not.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AmpInspectionResult {
    /// Index status of the AMP URL.
    #[serde(rename="ampIndexStatusVerdict")]
    
    pub amp_index_status_verdict: Option<String>,
    /// URL of the AMP that was inspected. If the submitted URL is a desktop page that refers to an AMP version, the AMP version will be inspected.
    #[serde(rename="ampUrl")]
    
    pub amp_url: Option<String>,
    /// Whether or not the page blocks indexing through a noindex rule.
    #[serde(rename="indexingState")]
    
    pub indexing_state: Option<String>,
    /// A list of zero or more AMP issues found for the inspected URL.
    
    pub issues: Option<Vec<AmpIssue>>,
    /// Last time this AMP version was crawled by Google. Absent if the URL was never crawled successfully.
    #[serde(rename="lastCrawlTime")]
    
    pub last_crawl_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether or not Google could fetch the AMP.
    #[serde(rename="pageFetchState")]
    
    pub page_fetch_state: Option<String>,
    /// Whether or not the page is blocked to Google by a robots.txt rule.
    #[serde(rename="robotsTxtState")]
    
    pub robots_txt_state: Option<String>,
    /// The status of the most severe error on the page. If a page has both warnings and errors, the page status is error. Error status means the page cannot be shown in Search results.
    
    pub verdict: Option<String>,
}

impl client::Part for AmpInspectionResult {}


/// AMP issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AmpIssue {
    /// Brief description of this issue.
    #[serde(rename="issueMessage")]
    
    pub issue_message: Option<String>,
    /// Severity of this issue: WARNING or ERROR.
    
    pub severity: Option<String>,
}

impl client::Part for AmpIssue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiDataRow {
    /// no description provided
    
    pub clicks: Option<f64>,
    /// no description provided
    
    pub ctr: Option<f64>,
    /// no description provided
    
    pub impressions: Option<f64>,
    /// no description provided
    
    pub keys: Option<Vec<String>>,
    /// no description provided
    
    pub position: Option<f64>,
}

impl client::Part for ApiDataRow {}


/// A filter test to be applied to each row in the data set, where a match can return the row. Filters are string comparisons, and values and dimension names are not case-sensitive. Individual filters are either AND'ed or OR'ed within their parent filter group, according to the group's group type. You do not need to group by a specified dimension to filter against it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiDimensionFilter {
    /// no description provided
    
    pub dimension: Option<String>,
    /// no description provided
    
    pub expression: Option<String>,
    /// no description provided
    
    pub operator: Option<String>,
}

impl client::Part for ApiDimensionFilter {}


/// A set of dimension value filters to test against each row. Only rows that pass all filter groups will be returned. All results within a filter group are either AND'ed or OR'ed together, depending on the group type selected. All filter groups are AND'ed together.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApiDimensionFilterGroup {
    /// no description provided
    
    pub filters: Option<Vec<ApiDimensionFilter>>,
    /// no description provided
    #[serde(rename="groupType")]
    
    pub group_type: Option<String>,
}

impl client::Part for ApiDimensionFilterGroup {}


/// Blocked resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlockedResource {
    /// URL of the blocked resource.
    
    pub url: Option<String>,
}

impl client::Part for BlockedResource {}


/// Rich Results items grouped by type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectedItems {
    /// List of Rich Results items.
    
    pub items: Option<Vec<Item>>,
    /// Rich Results type
    #[serde(rename="richResultType")]
    
    pub rich_result_type: Option<String>,
}

impl client::Part for DetectedItems {}


/// Describe image data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// Image data in format determined by the mime type. Currently, the format will always be "image/png", but this might change in the future.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The mime-type of the image data.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for Image {}


/// Results of index status inspection for either the live page or the version in Google's index, depending on whether you requested a live inspection or not. For more information, see the [Index coverage report documentation](https://support.google.com/webmasters/answer/7440203).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexStatusInspectionResult {
    /// Could Google find and index the page. More details about page indexing appear in 'indexing_state'.
    #[serde(rename="coverageState")]
    
    pub coverage_state: Option<String>,
    /// Primary crawler that was used by Google to crawl your site.
    #[serde(rename="crawledAs")]
    
    pub crawled_as: Option<String>,
    /// The URL of the page that Google selected as canonical. If the page was not indexed, this field is absent.
    #[serde(rename="googleCanonical")]
    
    pub google_canonical: Option<String>,
    /// Whether or not the page blocks indexing through a noindex rule.
    #[serde(rename="indexingState")]
    
    pub indexing_state: Option<String>,
    /// Last time this URL was crawled by Google using the [primary crawler](https://support.google.com/webmasters/answer/7440203#primary_crawler). Absent if the URL was never crawled successfully.
    #[serde(rename="lastCrawlTime")]
    
    pub last_crawl_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether or not Google could retrieve the page from your server. Equivalent to ["page fetch"](https://support.google.com/webmasters/answer/9012289#index_coverage) in the URL inspection report.
    #[serde(rename="pageFetchState")]
    
    pub page_fetch_state: Option<String>,
    /// URLs that link to the inspected URL, directly and indirectly.
    #[serde(rename="referringUrls")]
    
    pub referring_urls: Option<Vec<String>>,
    /// Whether or not the page is blocked to Google by a robots.txt rule.
    #[serde(rename="robotsTxtState")]
    
    pub robots_txt_state: Option<String>,
    /// Any sitemaps that this URL was listed in, as known by Google. Not guaranteed to be an exhaustive list, especially if Google did not discover this URL through a sitemap. Absent if no sitemaps were found.
    
    pub sitemap: Option<Vec<String>>,
    /// The URL that your page or site [declares as canonical](https://developers.google.com/search/docs/advanced/crawling/consolidate-duplicate-urls?#define-canonical). If you did not declare a canonical URL, this field is absent.
    #[serde(rename="userCanonical")]
    
    pub user_canonical: Option<String>,
    /// High level verdict about whether the URL *is* indexed (indexed status), or *can be* indexed (live inspection).
    
    pub verdict: Option<String>,
}

impl client::Part for IndexStatusInspectionResult {}


/// Index inspection request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [index inspect url inspection](UrlInspectionIndexInspectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InspectUrlIndexRequest {
    /// Required. URL to inspect. Must be under the property specified in "site_url".
    #[serde(rename="inspectionUrl")]
    
    pub inspection_url: Option<String>,
    /// Optional. An [IETF BCP-47](https://en.wikipedia.org/wiki/IETF_language_tag) language code representing the requested language for translated issue messages, e.g. "en-US", "or "de-CH". Default value is "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required. The URL of the property as defined in Search Console. **Examples:** `http://www.example.com/` for a URL-prefix property, or `sc-domain:example.com` for a Domain property.
    #[serde(rename="siteUrl")]
    
    pub site_url: Option<String>,
}

impl client::RequestValue for InspectUrlIndexRequest {}


/// Index-Status inspection response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [index inspect url inspection](UrlInspectionIndexInspectCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InspectUrlIndexResponse {
    /// URL inspection results.
    #[serde(rename="inspectionResult")]
    
    pub inspection_result: Option<UrlInspectionResult>,
}

impl client::ResponseResult for InspectUrlIndexResponse {}


/// A specific rich result found on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    /// A list of zero or more rich result issues found for this instance.
    
    pub issues: Option<Vec<RichResultsIssue>>,
    /// The user-provided name of this item.
    
    pub name: Option<String>,
}

impl client::Part for Item {}


/// Mobile-friendly issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileFriendlyIssue {
    /// Rule violated.
    
    pub rule: Option<String>,
}

impl client::Part for MobileFriendlyIssue {}


/// Mobile-usability inspection results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileUsabilityInspectionResult {
    /// A list of zero or more mobile-usability issues detected for this URL.
    
    pub issues: Option<Vec<MobileUsabilityIssue>>,
    /// High-level mobile-usability inspection result for this URL.
    
    pub verdict: Option<String>,
}

impl client::Part for MobileUsabilityInspectionResult {}


/// Mobile-usability issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MobileUsabilityIssue {
    /// Mobile-usability issue type.
    #[serde(rename="issueType")]
    
    pub issue_type: Option<String>,
    /// Additional information regarding the issue.
    
    pub message: Option<String>,
    /// Not returned; reserved for future use.
    
    pub severity: Option<String>,
}

impl client::Part for MobileUsabilityIssue {}


/// Information about a resource with issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceIssue {
    /// Describes a blocked resource issue.
    #[serde(rename="blockedResource")]
    
    pub blocked_resource: Option<BlockedResource>,
}

impl client::Part for ResourceIssue {}


/// Rich-Results inspection result, including any rich results found at this URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RichResultsInspectionResult {
    /// A list of zero or more rich results detected on this page. Rich results that cannot even be parsed due to syntactic issues will not be listed here.
    #[serde(rename="detectedItems")]
    
    pub detected_items: Option<Vec<DetectedItems>>,
    /// High-level rich results inspection result for this URL.
    
    pub verdict: Option<String>,
}

impl client::Part for RichResultsInspectionResult {}


/// Severity and status of a single issue affecting a single rich result instance on a page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RichResultsIssue {
    /// Rich Results issue type.
    #[serde(rename="issueMessage")]
    
    pub issue_message: Option<String>,
    /// Severity of this issue: WARNING, or ERROR. Items with an issue of status ERROR cannot appear with rich result features in Google Search results.
    
    pub severity: Option<String>,
}

impl client::Part for RichResultsIssue {}


/// Mobile-friendly test request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [mobile friendly test run url testing tools](UrlTestingToolMobileFriendlyTestRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunMobileFriendlyTestRequest {
    /// Whether or not screenshot is requested. Default is false.
    #[serde(rename="requestScreenshot")]
    
    pub request_screenshot: Option<bool>,
    /// URL for inspection.
    
    pub url: Option<String>,
}

impl client::RequestValue for RunMobileFriendlyTestRequest {}


/// Mobile-friendly test response, including mobile-friendly issues and resource issues.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [mobile friendly test run url testing tools](UrlTestingToolMobileFriendlyTestRunCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunMobileFriendlyTestResponse {
    /// Test verdict, whether the page is mobile friendly or not.
    #[serde(rename="mobileFriendliness")]
    
    pub mobile_friendliness: Option<String>,
    /// List of mobile-usability issues.
    #[serde(rename="mobileFriendlyIssues")]
    
    pub mobile_friendly_issues: Option<Vec<MobileFriendlyIssue>>,
    /// Information about embedded resources issues.
    #[serde(rename="resourceIssues")]
    
    pub resource_issues: Option<Vec<ResourceIssue>>,
    /// Screenshot of the requested URL.
    
    pub screenshot: Option<Image>,
    /// Final state of the test, can be either complete or an error.
    #[serde(rename="testStatus")]
    
    pub test_status: Option<TestStatus>,
}

impl client::ResponseResult for RunMobileFriendlyTestResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query searchanalytics](SearchanalyticQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchAnalyticsQueryRequest {
    /// [Optional; Default is \"auto\"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see the help documentation to learn how data is calculated differently by site versus by page. **Note:** If you group or filter by page, you cannot aggregate by property. If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid.
    #[serde(rename="aggregationType")]
    
    pub aggregation_type: Option<String>,
    /// The data state to be fetched, can be full or all, the latter including full and partial data.
    #[serde(rename="dataState")]
    
    pub data_state: Option<String>,
    /// [Optional] Zero or more filters to apply to the dimension grouping values; for example, 'query contains \"buy\"' to see only data where the query string contains the substring \"buy\" (not case-sensitive). You can filter by a dimension without grouping by it.
    #[serde(rename="dimensionFilterGroups")]
    
    pub dimension_filter_groups: Option<Vec<ApiDimensionFilterGroup>>,
    /// [Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions.
    
    pub dimensions: Option<Vec<String>>,
    /// [Required] End date of the requested date range, in YYYY-MM-DD format, in PST (UTC - 8:00). Must be greater than or equal to the start date. This value is included in the range.
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// [Optional; Default is 1000] The maximum number of rows to return. Must be a number from 1 to 25,000 (inclusive).
    #[serde(rename="rowLimit")]
    
    pub row_limit: Option<i32>,
    /// [Optional; Default is \"web\"] The search type to filter for.
    #[serde(rename="searchType")]
    
    pub search_type: Option<String>,
    ///  [Required] Start date of the requested date range, in YYYY-MM-DD format, in PST time (UTC - 8:00). Must be less than or equal to the end date. This value is included in the range.
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
    /// [Optional; Default is 0] Zero-based index of the first row in the response. Must be a non-negative number.
    #[serde(rename="startRow")]
    
    pub start_row: Option<i32>,
    /// Optional. [Optional; Default is \"web\"] Type of report: search type, or either Discover or Gnews.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for SearchAnalyticsQueryRequest {}


/// A list of rows, one per result, grouped by key. Metrics in each row are aggregated for all data grouped by that key either by page or property, as specified by the aggregation type parameter.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query searchanalytics](SearchanalyticQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchAnalyticsQueryResponse {
    /// How the results were aggregated.
    #[serde(rename="responseAggregationType")]
    
    pub response_aggregation_type: Option<String>,
    /// A list of rows grouped by the key values in the order given in the query.
    
    pub rows: Option<Vec<ApiDataRow>>,
}

impl client::ResponseResult for SearchAnalyticsQueryResponse {}


/// List of sitemaps.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list sitemaps](SitemapListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SitemapsListResponse {
    /// Contains detailed information about a specific URL submitted as a [sitemap](https://support.google.com/webmasters/answer/156184).
    
    pub sitemap: Option<Vec<WmxSitemap>>,
}

impl client::ResponseResult for SitemapsListResponse {}


/// List of sites with access level information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list sites](SiteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SitesListResponse {
    /// Contains permission level information about a Search Console site. For more information, see [Permissions in Search Console](https://support.google.com/webmasters/answer/2451999).
    #[serde(rename="siteEntry")]
    
    pub site_entry: Option<Vec<WmxSite>>,
}

impl client::ResponseResult for SitesListResponse {}


/// Final state of the test, including error details if necessary.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestStatus {
    /// Error details if applicable.
    
    pub details: Option<String>,
    /// Status of the test.
    
    pub status: Option<String>,
}

impl client::Part for TestStatus {}


/// URL inspection result, including all inspection results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlInspectionResult {
    /// Result of the AMP analysis. Absent if the page is not an AMP page.
    #[serde(rename="ampResult")]
    
    pub amp_result: Option<AmpInspectionResult>,
    /// Result of the index status analysis.
    #[serde(rename="indexStatusResult")]
    
    pub index_status_result: Option<IndexStatusInspectionResult>,
    /// Link to Search Console URL inspection.
    #[serde(rename="inspectionResultLink")]
    
    pub inspection_result_link: Option<String>,
    /// Result of the Mobile usability analysis.
    #[serde(rename="mobileUsabilityResult")]
    
    pub mobile_usability_result: Option<MobileUsabilityInspectionResult>,
    /// Result of the Rich Results analysis. Absent if there are no rich results found.
    #[serde(rename="richResultsResult")]
    
    pub rich_results_result: Option<RichResultsInspectionResult>,
}

impl client::Part for UrlInspectionResult {}


/// Contains permission level information about a Search Console site. For more information, see [Permissions in Search Console](https://support.google.com/webmasters/answer/2451999).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get sites](SiteGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WmxSite {
    /// The user's permission level for the site.
    #[serde(rename="permissionLevel")]
    
    pub permission_level: Option<String>,
    /// The URL of the site.
    #[serde(rename="siteUrl")]
    
    pub site_url: Option<String>,
}

impl client::ResponseResult for WmxSite {}


/// Contains detailed information about a specific URL submitted as a [sitemap](https://support.google.com/webmasters/answer/156184).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get sitemaps](SitemapGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WmxSitemap {
    /// The various content types in the sitemap.
    
    pub contents: Option<Vec<WmxSitemapContent>>,
    /// Number of errors in the sitemap. These are issues with the sitemap itself that need to be fixed before it can be processed correctly.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub errors: Option<i64>,
    /// If true, the sitemap has not been processed.
    #[serde(rename="isPending")]
    
    pub is_pending: Option<bool>,
    /// If true, the sitemap is a collection of sitemaps.
    #[serde(rename="isSitemapsIndex")]
    
    pub is_sitemaps_index: Option<bool>,
    /// Date & time in which this sitemap was last downloaded. Date format is in RFC 3339 format (yyyy-mm-dd).
    #[serde(rename="lastDownloaded")]
    
    pub last_downloaded: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Date & time in which this sitemap was submitted. Date format is in RFC 3339 format (yyyy-mm-dd).
    #[serde(rename="lastSubmitted")]
    
    pub last_submitted: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The url of the sitemap.
    
    pub path: Option<String>,
    /// The type of the sitemap. For example: `rssFeed`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Number of warnings for the sitemap. These are generally non-critical issues with URLs in the sitemaps.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub warnings: Option<i64>,
}

impl client::ResponseResult for WmxSitemap {}


/// Information about the various content types in the sitemap.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WmxSitemapContent {
    /// *Deprecated; do not use.*
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub indexed: Option<i64>,
    /// The number of URLs in the sitemap (of the content type).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub submitted: Option<i64>,
    /// The specific type of content in this sitemap. For example: `web`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for WmxSitemapContent {}


