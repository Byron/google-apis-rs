use super::*;
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


/// There is no detailed description.
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


/// There is no detailed description.
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
    /// [Optional; Default is "auto"] How data is aggregated. If aggregated by property, all data for the same property is aggregated; if aggregated by page, all data is aggregated by canonical URI. If you filter or group by page, choose AUTO; otherwise you can aggregate either by property or by page, depending on how you want your data calculated; see  the help documentation to learn how data is calculated differently by site versus by page.
    /// 
    /// Note: If you group or filter by page, you cannot aggregate by property.
    /// 
    /// If you specify any value other than AUTO, the aggregation type in the result will match the requested type, or if you request an invalid type, you will get an error. The API will never change your aggregation type if the requested type is invalid.
    #[serde(rename="aggregationType")]
    
    pub aggregation_type: Option<String>,
    /// [Optional] If "all" (case-insensitive), data will include fresh data. If "final" (case-insensitive) or if this parameter is omitted, the returned data will include only finalized data.
    #[serde(rename="dataState")]
    
    pub data_state: Option<String>,
    /// [Optional] Zero or more filters to apply to the dimension grouping values; for example, 'query contains "buy"' to see only data where the query string contains the substring "buy" (not case-sensitive). You can filter by a dimension without grouping by it.
    #[serde(rename="dimensionFilterGroups")]
    
    pub dimension_filter_groups: Option<Vec<ApiDimensionFilterGroup>>,
    /// [Optional] Zero or more dimensions to group results by. Dimensions are the group-by values in the Search Analytics page. Dimensions are combined to create a unique row key for each row. Results are grouped in the order that you supply these dimensions.
    
    pub dimensions: Option<Vec<String>>,
    /// [Required] End date of the requested date range, in YYYY-MM-DD format, in PST (UTC - 8:00). Must be greater than or equal to the start date. This value is included in the range.
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// [Optional; Default is 1000] The maximum number of rows to return. Must be a number from 1 to 5,000 (inclusive).
    #[serde(rename="rowLimit")]
    
    pub row_limit: Option<i32>,
    /// [Optional; Default is "web"] The search type to filter for.
    #[serde(rename="searchType")]
    
    pub search_type: Option<String>,
    /// [Required] Start date of the requested date range, in YYYY-MM-DD format, in PST time (UTC - 8:00). Must be less than or equal to the end date. This value is included in the range.
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
    /// [Optional; Default is 0] Zero-based index of the first row in the response. Must be a non-negative number.
    #[serde(rename="startRow")]
    
    pub start_row: Option<i32>,
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
    /// Contains detailed information about a specific URL submitted as a sitemap.
    
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
    /// Contains permission level information about a Search Console site. For more information, see Permissions in Search Console.
    #[serde(rename="siteEntry")]
    
    pub site_entry: Option<Vec<WmxSite>>,
}

impl client::ResponseResult for SitesListResponse {}


/// Contains permission level information about a Search Console site. For more information, see  Permissions in Search Console.
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


/// Contains detailed information about a specific URL submitted as a sitemap.
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
    /// The type of the sitemap. For example: rssFeed.
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
    /// The number of URLs from the sitemap that were indexed (of the content type).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub indexed: Option<i64>,
    /// The number of URLs in the sitemap (of the content type).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub submitted: Option<i64>,
    /// The specific type of content in this sitemap. For example: web.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for WmxSitemapContent {}


