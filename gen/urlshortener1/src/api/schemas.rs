use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsSnapshot {
    /// Top browsers, e.g. "Chrome"; sorted by (descending) click counts. Only present if this data is available.
    
    pub browsers: Option<Vec<StringCount>>,
    /// Top countries (expressed as country codes), e.g. "US" or "DE"; sorted by (descending) click counts. Only present if this data is available.
    
    pub countries: Option<Vec<StringCount>>,
    /// Number of clicks on all goo.gl short URLs pointing to this long URL.
    #[serde(rename="longUrlClicks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub long_url_clicks: Option<i64>,
    /// Top platforms or OSes, e.g. "Windows"; sorted by (descending) click counts. Only present if this data is available.
    
    pub platforms: Option<Vec<StringCount>>,
    /// Top referring hosts, e.g. "www.google.com"; sorted by (descending) click counts. Only present if this data is available.
    
    pub referrers: Option<Vec<StringCount>>,
    /// Number of clicks on this short URL.
    #[serde(rename="shortUrlClicks")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub short_url_clicks: Option<i64>,
}

impl client::Part for AnalyticsSnapshot {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    /// Click analytics over all time.
    #[serde(rename="allTime")]
    
    pub all_time: Option<AnalyticsSnapshot>,
    /// Click analytics over the last day.
    
    pub day: Option<AnalyticsSnapshot>,
    /// Click analytics over the last month.
    
    pub month: Option<AnalyticsSnapshot>,
    /// Click analytics over the last two hours.
    #[serde(rename="twoHours")]
    
    pub two_hours: Option<AnalyticsSnapshot>,
    /// Click analytics over the last week.
    
    pub week: Option<AnalyticsSnapshot>,
}

impl client::Part for AnalyticsSummary {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringCount {
    /// Number of clicks for this top entry, e.g. for this particular country or browser.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Label assigned to this top entry, e.g. "US" or "Chrome".
    
    pub id: Option<String>,
}

impl client::Part for StringCount {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get url](UrlGetCall) (response)
/// * [insert url](UrlInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    /// A summary of the click analytics for the short and long URL. Might not be present if not requested or currently unavailable.
    
    pub analytics: Option<AnalyticsSummary>,
    /// Time the short URL was created; ISO 8601 representation using the yyyy-MM-dd'T'HH:mm:ss.SSSZZ format, e.g. "2010-10-14T19:01:24.944+00:00".
    
    pub created: Option<String>,
    /// Short URL, e.g. "http://goo.gl/l6MS".
    
    pub id: Option<String>,
    /// The fixed string "urlshortener#url".
    
    pub kind: Option<String>,
    /// Long URL, e.g. "http://www.google.com/". Might not be present if the status is "REMOVED".
    #[serde(rename="longUrl")]
    
    pub long_url: Option<String>,
    /// Status of the target URL. Possible values: "OK", "MALWARE", "PHISHING", or "REMOVED". A URL might be marked "REMOVED" if it was flagged as spam, for example.
    
    pub status: Option<String>,
}

impl client::RequestValue for Url {}
impl client::ResponseResult for Url {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list url](UrlListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlHistory {
    /// A list of URL resources.
    
    pub items: Option<Vec<Url>>,
    /// Number of items returned with each full "page" of results. Note that the last page could have fewer items than the "itemsPerPage" value.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// The fixed string "urlshortener#urlHistory".
    
    pub kind: Option<String>,
    /// A token to provide to get the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of short URLs associated with this user (may be approximate).
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for UrlHistory {}


