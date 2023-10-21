use super::*;
/// Configuration data for an Ad Exchange buyer account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get accounts](AccountGetCall) (response)
/// * [list accounts](AccountListCall) (none)
/// * [patch accounts](AccountPatchCall) (request|response)
/// * [update accounts](AccountUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Your bidder locations that have distinct URLs.
    #[serde(rename="bidderLocation")]
    
    pub bidder_location: Option<Vec<AccountBidderLocation>>,
    /// The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this.
    #[serde(rename="cookieMatchingNid")]
    
    pub cookie_matching_nid: Option<String>,
    /// The base URL used in cookie match requests.
    #[serde(rename="cookieMatchingUrl")]
    
    pub cookie_matching_url: Option<String>,
    /// Account id.
    
    pub id: Option<i32>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this.
    #[serde(rename="maximumActiveCreatives")]
    
    pub maximum_active_creatives: Option<i32>,
    /// The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this.
    #[serde(rename="maximumTotalQps")]
    
    pub maximum_total_qps: Option<i32>,
    /// The number of creatives that this account inserted or bid with in the last 30 days.
    #[serde(rename="numberActiveCreatives")]
    
    pub number_active_creatives: Option<i32>,
}

impl client::RequestValue for Account {}
impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// An account feed lists Ad Exchange buyer accounts that the user has access to. Each entry in the feed corresponds to a single buyer account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsList {
    /// A list of accounts.
    
    pub items: Option<Vec<Account>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountsList {}


/// The configuration data for an Ad Exchange billing info.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get billing info](BillingInfoGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BillingInfo {
    /// Account id.
    #[serde(rename="accountId")]
    
    pub account_id: Option<i32>,
    /// Account name.
    #[serde(rename="accountName")]
    
    pub account_name: Option<String>,
    /// A list of adgroup IDs associated with this particular account. These IDs may show up as part of a realtime bidding BidRequest, which indicates a bid request for this account.
    #[serde(rename="billingId")]
    
    pub billing_id: Option<Vec<String>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for BillingInfo {}


/// A billing info feed lists Billing Info the Ad Exchange buyer account has access to. Each entry in the feed corresponds to a single billing info.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list billing info](BillingInfoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BillingInfoList {
    /// A list of billing info relevant for your account.
    
    pub items: Option<Vec<BillingInfo>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for BillingInfoList {}


/// The configuration data for Ad Exchange RTB - Budget API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get budget](BudgetGetCall) (response)
/// * [patch budget](BudgetPatchCall) (request|response)
/// * [update budget](BudgetUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Budget {
    /// The id of the account. This is required for get and update requests.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// The billing id to determine which adgroup to provide budget information for. This is required for get and update requests.
    #[serde(rename="billingId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub billing_id: Option<i64>,
    /// The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests.
    #[serde(rename="budgetAmount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub budget_amount: Option<i64>,
    /// The currency code for the buyer. This cannot be altered here.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// The unique id that describes this item.
    
    pub id: Option<String>,
    /// The kind of the resource, i.e. "adexchangebuyer#budget".
    
    pub kind: Option<String>,
}

impl client::RequestValue for Budget {}
impl client::ResponseResult for Budget {}


/// A creative and its classification data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get creatives](CreativeGetCall) (response)
/// * [insert creatives](CreativeInsertCall) (request|response)
/// * [list creatives](CreativeListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Creative {
    /// The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set.
    #[serde(rename="HTMLSnippet")]
    
    pub html_snippet: Option<String>,
    /// Account id.
    #[serde(rename="accountId")]
    
    pub account_id: Option<i32>,
    /// no description provided
    #[serde(rename="adTechnologyProviders")]
    
    pub ad_technology_providers: Option<CreativeAdTechnologyProviders>,
    /// Detected advertiser id, if any. Read-only. This field should not be set in requests.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub advertiser_id: Option<Vec<i64>>,
    /// The name of the company being advertised in the creative.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// The agency id for this creative.
    #[serde(rename="agencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp).
    #[serde(rename="apiUploadTimestamp")]
    
    pub api_upload_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// All attributes for the ads that may be shown from this snippet.
    
    pub attribute: Option<Vec<i32>>,
    /// A buyer-specific id identifying the creative in this ad.
    #[serde(rename="buyerCreativeId")]
    
    pub buyer_creative_id: Option<String>,
    /// The set of destination urls for the snippet.
    #[serde(rename="clickThroughUrl")]
    
    pub click_through_url: Option<Vec<String>>,
    /// Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests.
    
    pub corrections: Option<Vec<CreativeCorrections>>,
    /// The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests.
    #[serde(rename="disapprovalReasons")]
    
    pub disapproval_reasons: Option<Vec<CreativeDisapprovalReasons>>,
    /// The filtering reasons for the creative. Read-only. This field should not be set in requests.
    #[serde(rename="filteringReasons")]
    
    pub filtering_reasons: Option<CreativeFilteringReasons>,
    /// Ad height.
    
    pub height: Option<i32>,
    /// The set of urls to be called to record an impression.
    #[serde(rename="impressionTrackingUrl")]
    
    pub impression_tracking_url: Option<Vec<String>>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// If nativeAd is set, HTMLSnippet and videoURL should not be set.
    #[serde(rename="nativeAd")]
    
    pub native_ad: Option<CreativeNativeAd>,
    /// Detected product categories, if any. Read-only. This field should not be set in requests.
    #[serde(rename="productCategories")]
    
    pub product_categories: Option<Vec<i32>>,
    /// All restricted categories for the ads that may be shown from this snippet.
    #[serde(rename="restrictedCategories")]
    
    pub restricted_categories: Option<Vec<i32>>,
    /// Detected sensitive categories, if any. Read-only. This field should not be set in requests.
    #[serde(rename="sensitiveCategories")]
    
    pub sensitive_categories: Option<Vec<i32>>,
    /// Creative serving status. Read-only. This field should not be set in requests.
    
    pub status: Option<String>,
    /// All vendor types for the ads that may be shown from this snippet.
    #[serde(rename="vendorType")]
    
    pub vendor_type: Option<Vec<i32>>,
    /// The version for this creative. Read-only. This field should not be set in requests.
    
    pub version: Option<i32>,
    /// The URL to fetch a video ad. If set, HTMLSnippet and the nativeAd should not be set.
    #[serde(rename="videoURL")]
    
    pub video_url: Option<String>,
    /// Ad width.
    
    pub width: Option<i32>,
}

impl client::RequestValue for Creative {}
impl client::Resource for Creative {}
impl client::ResponseResult for Creative {}


/// The creatives feed lists the active creatives for the Ad Exchange buyer accounts that the user has access to. Each entry in the feed corresponds to a single creative.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list creatives](CreativeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativesList {
    /// A list of creatives.
    
    pub items: Option<Vec<Creative>>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Continuation token used to page through creatives. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CreativesList {}


/// The configuration data for an Ad Exchange direct deal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get direct deals](DirectDealGetCall) (response)
/// * [list direct deals](DirectDealListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectDeal {
    /// The account id of the buyer this deal is for.
    #[serde(rename="accountId")]
    
    pub account_id: Option<i32>,
    /// The name of the advertiser this deal is for.
    
    pub advertiser: Option<String>,
    /// Whether the publisher for this deal is eligible for alcohol ads.
    #[serde(rename="allowsAlcohol")]
    
    pub allows_alcohol: Option<bool>,
    /// The account id that this deal was negotiated for. It is either the buyer or the client that this deal was negotiated on behalf of.
    #[serde(rename="buyerAccountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub buyer_account_id: Option<i64>,
    /// The currency code that applies to the fixed_cpm value. If not set then assumed to be USD.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// The deal type such as programmatic reservation or fixed price and so on.
    #[serde(rename="dealTier")]
    
    pub deal_tier: Option<String>,
    /// End time for when this deal stops being active. If not set then this deal is valid until manually disabled by the publisher. In seconds since the epoch.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<i64>,
    /// The fixed price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the fixed price tier of buying (highest priority, pay exactly the configured fixed price).
    #[serde(rename="fixedCpm")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub fixed_cpm: Option<i64>,
    /// Deal id.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Deal name.
    
    pub name: Option<String>,
    /// The minimum price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the private exchange tier of buying (below fixed price priority, run as a second price auction).
    #[serde(rename="privateExchangeMinCpm")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub private_exchange_min_cpm: Option<i64>,
    /// If true, the publisher has opted to have their blocks ignored when a creative is bid with for this deal.
    #[serde(rename="publisherBlocksOverriden")]
    
    pub publisher_blocks_overriden: Option<bool>,
    /// The name of the publisher offering this direct deal.
    #[serde(rename="sellerNetwork")]
    
    pub seller_network: Option<String>,
    /// Start time for when this deal becomes active. If not set then this deal is active immediately upon creation. In seconds since the epoch.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<i64>,
}

impl client::Resource for DirectDeal {}
impl client::ResponseResult for DirectDeal {}


/// A direct deals feed lists Direct Deals the Ad Exchange buyer account has access to. This includes direct deals set up for the buyer account as well as its merged stream seats.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list direct deals](DirectDealListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectDealsList {
    /// A list of direct deals relevant for your account.
    #[serde(rename="directDeals")]
    
    pub direct_deals: Option<Vec<DirectDeal>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DirectDealsList {}


/// The configuration data for an Ad Exchange performance report list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// The number of bid responses with an ad.
    #[serde(rename="bidRate")]
    
    pub bid_rate: Option<f64>,
    /// The number of bid requests sent to your bidder.
    #[serde(rename="bidRequestRate")]
    
    pub bid_request_rate: Option<f64>,
    /// Rate of various prefiltering statuses per match. Please refer to the callout-status-codes.txt file for different statuses.
    #[serde(rename="calloutStatusRate")]
    
    pub callout_status_rate: Option<Vec<json::Value>>,
    /// Average QPS for cookie matcher operations.
    #[serde(rename="cookieMatcherStatusRate")]
    
    pub cookie_matcher_status_rate: Option<Vec<json::Value>>,
    /// Rate of ads with a given status. Please refer to the creative-status-codes.txt file for different statuses.
    #[serde(rename="creativeStatusRate")]
    
    pub creative_status_rate: Option<Vec<json::Value>>,
    /// The number of bid responses that were filtered due to a policy violation or other errors.
    #[serde(rename="filteredBidRate")]
    
    pub filtered_bid_rate: Option<f64>,
    /// Average QPS for hosted match operations.
    #[serde(rename="hostedMatchStatusRate")]
    
    pub hosted_match_status_rate: Option<Vec<json::Value>>,
    /// The number of potential queries based on your pretargeting settings.
    #[serde(rename="inventoryMatchRate")]
    
    pub inventory_match_rate: Option<f64>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// The 50th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report.
    #[serde(rename="latency50thPercentile")]
    
    pub latency50th_percentile: Option<f64>,
    /// The 85th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report.
    #[serde(rename="latency85thPercentile")]
    
    pub latency85th_percentile: Option<f64>,
    /// The 95th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report.
    #[serde(rename="latency95thPercentile")]
    
    pub latency95th_percentile: Option<f64>,
    /// Rate of various quota account statuses per quota check.
    #[serde(rename="noQuotaInRegion")]
    
    pub no_quota_in_region: Option<f64>,
    /// Rate of various quota account statuses per quota check.
    #[serde(rename="outOfQuota")]
    
    pub out_of_quota: Option<f64>,
    /// Average QPS for pixel match requests from clients.
    #[serde(rename="pixelMatchRequests")]
    
    pub pixel_match_requests: Option<f64>,
    /// Average QPS for pixel match responses from clients.
    #[serde(rename="pixelMatchResponses")]
    
    pub pixel_match_responses: Option<f64>,
    /// The configured quota limits for this account.
    #[serde(rename="quotaConfiguredLimit")]
    
    pub quota_configured_limit: Option<f64>,
    /// The throttled quota limits for this account.
    #[serde(rename="quotaThrottledLimit")]
    
    pub quota_throttled_limit: Option<f64>,
    /// The trading location of this data.
    
    pub region: Option<String>,
    /// The number of properly formed bid responses received by our servers within the deadline.
    #[serde(rename="successfulRequestRate")]
    
    pub successful_request_rate: Option<f64>,
    /// The unix timestamp of the starting time of this performance data.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp: Option<i64>,
    /// The number of bid responses that were unsuccessful due to timeouts, incorrect formatting, etc.
    #[serde(rename="unsuccessfulRequestRate")]
    
    pub unsuccessful_request_rate: Option<f64>,
}

impl client::Part for PerformanceReport {}


/// The configuration data for an Ad Exchange performance report list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list performance report](PerformanceReportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceReportList {
    /// Resource type.
    
    pub kind: Option<String>,
    /// A list of performance reports relevant for the account.
    #[serde(rename="performanceReport")]
    
    pub performance_report: Option<Vec<PerformanceReport>>,
}

impl client::ResponseResult for PerformanceReportList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get pretargeting config](PretargetingConfigGetCall) (response)
/// * [insert pretargeting config](PretargetingConfigInsertCall) (request|response)
/// * [patch pretargeting config](PretargetingConfigPatchCall) (request|response)
/// * [update pretargeting config](PretargetingConfigUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PretargetingConfig {
    /// The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically.
    #[serde(rename="billingId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub billing_id: Option<i64>,
    /// The config id; generated automatically. Leave this field blank for insert requests.
    #[serde(rename="configId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub config_id: Option<i64>,
    /// The name of the config. Must be unique. Required for all requests.
    #[serde(rename="configName")]
    
    pub config_name: Option<String>,
    /// List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO.
    #[serde(rename="creativeType")]
    
    pub creative_type: Option<Vec<String>>,
    /// Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions.
    
    pub dimensions: Option<Vec<PretargetingConfigDimensions>>,
    /// Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section.
    #[serde(rename="excludedContentLabels")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub excluded_content_labels: Option<Vec<i64>>,
    /// Requests containing any of these geo criteria ids will not match.
    #[serde(rename="excludedGeoCriteriaIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub excluded_geo_criteria_ids: Option<Vec<i64>>,
    /// Requests containing any of these placements will not match.
    #[serde(rename="excludedPlacements")]
    
    pub excluded_placements: Option<Vec<PretargetingConfigExcludedPlacements>>,
    /// Requests containing any of these users list ids will not match.
    #[serde(rename="excludedUserLists")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub excluded_user_lists: Option<Vec<i64>>,
    /// Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section.
    #[serde(rename="excludedVerticals")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub excluded_verticals: Option<Vec<i64>>,
    /// Requests containing any of these geo criteria ids will match.
    #[serde(rename="geoCriteriaIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub geo_criteria_ids: Option<Vec<i64>>,
    /// Whether this config is active. Required for all requests.
    #[serde(rename="isActive")]
    
    pub is_active: Option<bool>,
    /// The kind of the resource, i.e. "adexchangebuyer#pretargetingConfig".
    
    pub kind: Option<String>,
    /// Request containing any of these language codes will match.
    
    pub languages: Option<Vec<String>>,
    /// The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed).
    #[serde(rename="maximumQps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_qps: Option<i64>,
    /// Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section.
    #[serde(rename="mobileCarriers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub mobile_carriers: Option<Vec<i64>>,
    /// Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section.
    #[serde(rename="mobileDevices")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub mobile_devices: Option<Vec<i64>>,
    /// Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section.
    #[serde(rename="mobileOperatingSystemVersions")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub mobile_operating_system_versions: Option<Vec<i64>>,
    /// Requests containing any of these placements will match.
    
    pub placements: Option<Vec<PretargetingConfigPlacements>>,
    /// Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET.
    
    pub platforms: Option<Vec<String>>,
    /// Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section.
    #[serde(rename="supportedCreativeAttributes")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub supported_creative_attributes: Option<Vec<i64>>,
    /// Requests containing any of these user list ids will match.
    #[serde(rename="userLists")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub user_lists: Option<Vec<i64>>,
    /// Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section.
    #[serde(rename="vendorTypes")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub vendor_types: Option<Vec<i64>>,
    /// Requests containing any of these vertical ids will match.
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub verticals: Option<Vec<i64>>,
}

impl client::RequestValue for PretargetingConfig {}
impl client::ResponseResult for PretargetingConfig {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list pretargeting config](PretargetingConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PretargetingConfigList {
    /// A list of pretargeting configs
    
    pub items: Option<Vec<PretargetingConfig>>,
    /// Resource type.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for PretargetingConfigList {}


/// Your bidder locations that have distinct URLs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountBidderLocation {
    /// The maximum queries per second the Ad Exchange will send.
    #[serde(rename="maximumQps")]
    
    pub maximum_qps: Option<i32>,
    /// The geographical region the Ad Exchange should send requests from. Only used by some quota systems, but always setting the value is recommended. Allowed values:  
    /// - ASIA 
    /// - EUROPE 
    /// - US_EAST 
    /// - US_WEST
    
    pub region: Option<String>,
    /// The URL to which the Ad Exchange will send bid requests.
    
    pub url: Option<String>,
}

impl client::NestedType for AccountBidderLocation {}
impl client::Part for AccountBidderLocation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeAdTechnologyProviders {
    /// The detected ad technology provider IDs for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider. If this creative contains provider IDs that are outside of those listed in the `BidRequest.adslot.consented_providers_settings.consented_providers` field on the  Authorized Buyers Real-Time Bidding protocol or the `BidRequest.user.ext.consented_providers_settings.consented_providers` field on the OpenRTB protocol, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the "Third-party Ad Technology Vendors" section of Authorized Buyers Program Guidelines.
    #[serde(rename="detectedProviderIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub detected_provider_ids: Option<Vec<i64>>,
    /// Whether the creative contains an unidentified ad technology provider. If true, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the "Third-party Ad Technology Vendors" section of Authorized Buyers Program Guidelines.
    #[serde(rename="hasUnidentifiedProvider")]
    
    pub has_unidentified_provider: Option<bool>,
}

impl client::NestedType for CreativeAdTechnologyProviders {}
impl client::Part for CreativeAdTechnologyProviders {}


/// Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeCorrections {
    /// Additional details about the correction.
    
    pub details: Option<Vec<String>>,
    /// The type of correction that was applied to the creative.
    
    pub reason: Option<String>,
}

impl client::NestedType for CreativeCorrections {}
impl client::Part for CreativeCorrections {}


/// The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeDisapprovalReasons {
    /// Additional details about the reason for disapproval.
    
    pub details: Option<Vec<String>>,
    /// The categorized reason for disapproval.
    
    pub reason: Option<String>,
}

impl client::NestedType for CreativeDisapprovalReasons {}
impl client::Part for CreativeDisapprovalReasons {}


/// The filtering reasons for the creative. Read-only. This field should not be set in requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeFilteringReasons {
    /// The date in ISO 8601 format for the data. The data is collected from 00:00:00 to 23:59:59 in PST.
    
    pub date: Option<String>,
    /// The filtering reasons.
    
    pub reasons: Option<Vec<CreativeFilteringReasonsReasons>>,
}

impl client::NestedType for CreativeFilteringReasons {}
impl client::Part for CreativeFilteringReasons {}


/// The filtering reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeFilteringReasonsReasons {
    /// The number of times the creative was filtered for the status. The count is aggregated across all publishers on the exchange.
    #[serde(rename="filteringCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub filtering_count: Option<i64>,
    /// The filtering status code. Please refer to the creative-status-codes.txt file for different statuses.
    #[serde(rename="filteringStatus")]
    
    pub filtering_status: Option<i32>,
}

impl client::NestedType for CreativeFilteringReasonsReasons {}
impl client::Part for CreativeFilteringReasonsReasons {}


/// If nativeAd is set, HTMLSnippet and videoURL should not be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeNativeAd {
    /// no description provided
    
    pub advertiser: Option<String>,
    /// The app icon, for app download ads.
    #[serde(rename="appIcon")]
    
    pub app_icon: Option<CreativeNativeAdAppIcon>,
    /// A long description of the ad.
    
    pub body: Option<String>,
    /// A label for the button that the user is supposed to click.
    #[serde(rename="callToAction")]
    
    pub call_to_action: Option<String>,
    /// The URL to use for click tracking.
    #[serde(rename="clickTrackingUrl")]
    
    pub click_tracking_url: Option<String>,
    /// A short title for the ad.
    
    pub headline: Option<String>,
    /// A large image.
    
    pub image: Option<CreativeNativeAdImage>,
    /// The URLs are called when the impression is rendered.
    #[serde(rename="impressionTrackingUrl")]
    
    pub impression_tracking_url: Option<Vec<String>>,
    /// A smaller image, for the advertiser logo.
    
    pub logo: Option<CreativeNativeAdLogo>,
    /// The price of the promoted app including the currency info.
    
    pub price: Option<String>,
    /// The app rating in the app store. Must be in the range [0-5].
    #[serde(rename="starRating")]
    
    pub star_rating: Option<f64>,
}

impl client::NestedType for CreativeNativeAd {}
impl client::Part for CreativeNativeAd {}


/// The app icon, for app download ads.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeNativeAdAppIcon {
    /// no description provided
    
    pub height: Option<i32>,
    /// no description provided
    
    pub url: Option<String>,
    /// no description provided
    
    pub width: Option<i32>,
}

impl client::NestedType for CreativeNativeAdAppIcon {}
impl client::Part for CreativeNativeAdAppIcon {}


/// A large image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeNativeAdImage {
    /// no description provided
    
    pub height: Option<i32>,
    /// no description provided
    
    pub url: Option<String>,
    /// no description provided
    
    pub width: Option<i32>,
}

impl client::NestedType for CreativeNativeAdImage {}
impl client::Part for CreativeNativeAdImage {}


/// A smaller image, for the advertiser logo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeNativeAdLogo {
    /// no description provided
    
    pub height: Option<i32>,
    /// no description provided
    
    pub url: Option<String>,
    /// no description provided
    
    pub width: Option<i32>,
}

impl client::NestedType for CreativeNativeAdLogo {}
impl client::Part for CreativeNativeAdLogo {}


/// Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PretargetingConfigDimensions {
    /// Height in pixels.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub height: Option<i64>,
    /// Width in pixels.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub width: Option<i64>,
}

impl client::NestedType for PretargetingConfigDimensions {}
impl client::Part for PretargetingConfigDimensions {}


/// Requests containing any of these placements will not match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PretargetingConfigExcludedPlacements {
    /// The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement.
    
    pub token: Option<String>,
    /// The type of the placement.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for PretargetingConfigExcludedPlacements {}
impl client::Part for PretargetingConfigExcludedPlacements {}


/// Requests containing any of these placements will match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PretargetingConfigPlacements {
    /// The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement.
    
    pub token: Option<String>,
    /// The type of the placement.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for PretargetingConfigPlacements {}
impl client::Part for PretargetingConfigPlacements {}


