#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// Manage your Ad Exchange buyer account configuration
    AdexchangeBuyer,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AdexchangeBuyer => "https://www.googleapis.com/auth/adexchange.buyer",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::AdexchangeBuyer
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all AdExchangeBuyer related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::PretargetingConfig;
/// use adexchangebuyer1d3::{Result, Error};
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = PretargetingConfig::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pretargeting_config().patch(req, -28, -27)
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
pub struct AdExchangeBuyer<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for AdExchangeBuyer<C> {}

impl<'a, C> AdExchangeBuyer<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> AdExchangeBuyer<C> {
        AdExchangeBuyer {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://www.googleapis.com/adexchangebuyer/v1.3/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C> {
        AccountMethods { hub: self }
    }
    pub fn billing_info(&'a self) -> BillingInfoMethods<'a, C> {
        BillingInfoMethods { hub: self }
    }
    pub fn budget(&'a self) -> BudgetMethods<'a, C> {
        BudgetMethods { hub: self }
    }
    pub fn creatives(&'a self) -> CreativeMethods<'a, C> {
        CreativeMethods { hub: self }
    }
    pub fn direct_deals(&'a self) -> DirectDealMethods<'a, C> {
        DirectDealMethods { hub: self }
    }
    pub fn performance_report(&'a self) -> PerformanceReportMethods<'a, C> {
        PerformanceReportMethods { hub: self }
    }
    pub fn pretargeting_config(&'a self) -> PretargetingConfigMethods<'a, C> {
        PretargetingConfigMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/adexchangebuyer/v1.3/`.
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    /// Your bidder locations that have distinct URLs.
    #[serde(rename = "bidderLocation")]
    pub bidder_location: Option<Vec<AccountBidderLocation>>,
    /// The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this.
    #[serde(rename = "cookieMatchingNid")]
    pub cookie_matching_nid: Option<String>,
    /// The base URL used in cookie match requests.
    #[serde(rename = "cookieMatchingUrl")]
    pub cookie_matching_url: Option<String>,
    /// Account id.
    pub id: Option<i32>,
    /// Resource type.
    pub kind: Option<String>,
    /// The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this.
    #[serde(rename = "maximumActiveCreatives")]
    pub maximum_active_creatives: Option<i32>,
    /// The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this.
    #[serde(rename = "maximumTotalQps")]
    pub maximum_total_qps: Option<i32>,
    /// The number of creatives that this account inserted or bid with in the last 30 days.
    #[serde(rename = "numberActiveCreatives")]
    pub number_active_creatives: Option<i32>,
}

impl common::RequestValue for Account {}
impl common::Resource for Account {}
impl common::ResponseResult for Account {}

/// An account feed lists Ad Exchange buyer accounts that the user has access to. Each entry in the feed corresponds to a single buyer account.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list accounts](AccountListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountsList {
    /// A list of accounts.
    pub items: Option<Vec<Account>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl common::ResponseResult for AccountsList {}

/// The configuration data for an Ad Exchange billing info.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get billing info](BillingInfoGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BillingInfo {
    /// Account id.
    #[serde(rename = "accountId")]
    pub account_id: Option<i32>,
    /// Account name.
    #[serde(rename = "accountName")]
    pub account_name: Option<String>,
    /// A list of adgroup IDs associated with this particular account. These IDs may show up as part of a realtime bidding BidRequest, which indicates a bid request for this account.
    #[serde(rename = "billingId")]
    pub billing_id: Option<Vec<String>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl common::ResponseResult for BillingInfo {}

/// A billing info feed lists Billing Info the Ad Exchange buyer account has access to. Each entry in the feed corresponds to a single billing info.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list billing info](BillingInfoListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BillingInfoList {
    /// A list of billing info relevant for your account.
    pub items: Option<Vec<BillingInfo>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl common::ResponseResult for BillingInfoList {}

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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Budget {
    /// The id of the account. This is required for get and update requests.
    #[serde(rename = "accountId")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// The billing id to determine which adgroup to provide budget information for. This is required for get and update requests.
    #[serde(rename = "billingId")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub billing_id: Option<i64>,
    /// The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests.
    #[serde(rename = "budgetAmount")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub budget_amount: Option<i64>,
    /// The currency code for the buyer. This cannot be altered here.
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
    /// The unique id that describes this item.
    pub id: Option<String>,
    /// The kind of the resource, i.e. "adexchangebuyer#budget".
    pub kind: Option<String>,
}

impl common::RequestValue for Budget {}
impl common::ResponseResult for Budget {}

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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Creative {
    /// The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set.
    #[serde(rename = "HTMLSnippet")]
    pub html_snippet: Option<String>,
    /// Account id.
    #[serde(rename = "accountId")]
    pub account_id: Option<i32>,
    /// no description provided
    #[serde(rename = "adTechnologyProviders")]
    pub ad_technology_providers: Option<CreativeAdTechnologyProviders>,
    /// Detected advertiser id, if any. Read-only. This field should not be set in requests.
    #[serde(rename = "advertiserId")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub advertiser_id: Option<Vec<i64>>,
    /// The name of the company being advertised in the creative.
    #[serde(rename = "advertiserName")]
    pub advertiser_name: Option<String>,
    /// The agency id for this creative.
    #[serde(rename = "agencyId")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp).
    #[serde(rename = "apiUploadTimestamp")]
    pub api_upload_timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// All attributes for the ads that may be shown from this snippet.
    pub attribute: Option<Vec<i32>>,
    /// A buyer-specific id identifying the creative in this ad.
    #[serde(rename = "buyerCreativeId")]
    pub buyer_creative_id: Option<String>,
    /// The set of destination urls for the snippet.
    #[serde(rename = "clickThroughUrl")]
    pub click_through_url: Option<Vec<String>>,
    /// Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests.
    pub corrections: Option<Vec<CreativeCorrections>>,
    /// The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests.
    #[serde(rename = "disapprovalReasons")]
    pub disapproval_reasons: Option<Vec<CreativeDisapprovalReasons>>,
    /// The filtering reasons for the creative. Read-only. This field should not be set in requests.
    #[serde(rename = "filteringReasons")]
    pub filtering_reasons: Option<CreativeFilteringReasons>,
    /// Ad height.
    pub height: Option<i32>,
    /// The set of urls to be called to record an impression.
    #[serde(rename = "impressionTrackingUrl")]
    pub impression_tracking_url: Option<Vec<String>>,
    /// Resource type.
    pub kind: Option<String>,
    /// If nativeAd is set, HTMLSnippet and videoURL should not be set.
    #[serde(rename = "nativeAd")]
    pub native_ad: Option<CreativeNativeAd>,
    /// Detected product categories, if any. Read-only. This field should not be set in requests.
    #[serde(rename = "productCategories")]
    pub product_categories: Option<Vec<i32>>,
    /// All restricted categories for the ads that may be shown from this snippet.
    #[serde(rename = "restrictedCategories")]
    pub restricted_categories: Option<Vec<i32>>,
    /// Detected sensitive categories, if any. Read-only. This field should not be set in requests.
    #[serde(rename = "sensitiveCategories")]
    pub sensitive_categories: Option<Vec<i32>>,
    /// Creative serving status. Read-only. This field should not be set in requests.
    pub status: Option<String>,
    /// All vendor types for the ads that may be shown from this snippet.
    #[serde(rename = "vendorType")]
    pub vendor_type: Option<Vec<i32>>,
    /// The version for this creative. Read-only. This field should not be set in requests.
    pub version: Option<i32>,
    /// The URL to fetch a video ad. If set, HTMLSnippet and the nativeAd should not be set.
    #[serde(rename = "videoURL")]
    pub video_url: Option<String>,
    /// Ad width.
    pub width: Option<i32>,
}

impl common::RequestValue for Creative {}
impl common::Resource for Creative {}
impl common::ResponseResult for Creative {}

/// The creatives feed lists the active creatives for the Ad Exchange buyer accounts that the user has access to. Each entry in the feed corresponds to a single creative.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list creatives](CreativeListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativesList {
    /// A list of creatives.
    pub items: Option<Vec<Creative>>,
    /// Resource type.
    pub kind: Option<String>,
    /// Continuation token used to page through creatives. To retrieve the next page of results, set the next request's "pageToken" value to this.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl common::ResponseResult for CreativesList {}

/// The configuration data for an Ad Exchange direct deal.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get direct deals](DirectDealGetCall) (response)
/// * [list direct deals](DirectDealListCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DirectDeal {
    /// The account id of the buyer this deal is for.
    #[serde(rename = "accountId")]
    pub account_id: Option<i32>,
    /// The name of the advertiser this deal is for.
    pub advertiser: Option<String>,
    /// Whether the publisher for this deal is eligible for alcohol ads.
    #[serde(rename = "allowsAlcohol")]
    pub allows_alcohol: Option<bool>,
    /// The account id that this deal was negotiated for. It is either the buyer or the client that this deal was negotiated on behalf of.
    #[serde(rename = "buyerAccountId")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub buyer_account_id: Option<i64>,
    /// The currency code that applies to the fixed_cpm value. If not set then assumed to be USD.
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
    /// The deal type such as programmatic reservation or fixed price and so on.
    #[serde(rename = "dealTier")]
    pub deal_tier: Option<String>,
    /// End time for when this deal stops being active. If not set then this deal is valid until manually disabled by the publisher. In seconds since the epoch.
    #[serde(rename = "endTime")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub end_time: Option<i64>,
    /// The fixed price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the fixed price tier of buying (highest priority, pay exactly the configured fixed price).
    #[serde(rename = "fixedCpm")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub fixed_cpm: Option<i64>,
    /// Deal id.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// Resource type.
    pub kind: Option<String>,
    /// Deal name.
    pub name: Option<String>,
    /// The minimum price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the private exchange tier of buying (below fixed price priority, run as a second price auction).
    #[serde(rename = "privateExchangeMinCpm")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub private_exchange_min_cpm: Option<i64>,
    /// If true, the publisher has opted to have their blocks ignored when a creative is bid with for this deal.
    #[serde(rename = "publisherBlocksOverriden")]
    pub publisher_blocks_overriden: Option<bool>,
    /// The name of the publisher offering this direct deal.
    #[serde(rename = "sellerNetwork")]
    pub seller_network: Option<String>,
    /// Start time for when this deal becomes active. If not set then this deal is active immediately upon creation. In seconds since the epoch.
    #[serde(rename = "startTime")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub start_time: Option<i64>,
}

impl common::Resource for DirectDeal {}
impl common::ResponseResult for DirectDeal {}

/// A direct deals feed lists Direct Deals the Ad Exchange buyer account has access to. This includes direct deals set up for the buyer account as well as its merged stream seats.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list direct deals](DirectDealListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DirectDealsList {
    /// A list of direct deals relevant for your account.
    #[serde(rename = "directDeals")]
    pub direct_deals: Option<Vec<DirectDeal>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl common::ResponseResult for DirectDealsList {}

/// The configuration data for an Ad Exchange performance report list.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PerformanceReport {
    /// The number of bid responses with an ad.
    #[serde(rename = "bidRate")]
    pub bid_rate: Option<f64>,
    /// The number of bid requests sent to your bidder.
    #[serde(rename = "bidRequestRate")]
    pub bid_request_rate: Option<f64>,
    /// Rate of various prefiltering statuses per match. Please refer to the callout-status-codes.txt file for different statuses.
    #[serde(rename = "calloutStatusRate")]
    pub callout_status_rate: Option<Vec<serde_json::Value>>,
    /// Average QPS for cookie matcher operations.
    #[serde(rename = "cookieMatcherStatusRate")]
    pub cookie_matcher_status_rate: Option<Vec<serde_json::Value>>,
    /// Rate of ads with a given status. Please refer to the creative-status-codes.txt file for different statuses.
    #[serde(rename = "creativeStatusRate")]
    pub creative_status_rate: Option<Vec<serde_json::Value>>,
    /// The number of bid responses that were filtered due to a policy violation or other errors.
    #[serde(rename = "filteredBidRate")]
    pub filtered_bid_rate: Option<f64>,
    /// Average QPS for hosted match operations.
    #[serde(rename = "hostedMatchStatusRate")]
    pub hosted_match_status_rate: Option<Vec<serde_json::Value>>,
    /// The number of potential queries based on your pretargeting settings.
    #[serde(rename = "inventoryMatchRate")]
    pub inventory_match_rate: Option<f64>,
    /// Resource type.
    pub kind: Option<String>,
    /// The 50th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report.
    #[serde(rename = "latency50thPercentile")]
    pub latency50th_percentile: Option<f64>,
    /// The 85th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report.
    #[serde(rename = "latency85thPercentile")]
    pub latency85th_percentile: Option<f64>,
    /// The 95th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report.
    #[serde(rename = "latency95thPercentile")]
    pub latency95th_percentile: Option<f64>,
    /// Rate of various quota account statuses per quota check.
    #[serde(rename = "noQuotaInRegion")]
    pub no_quota_in_region: Option<f64>,
    /// Rate of various quota account statuses per quota check.
    #[serde(rename = "outOfQuota")]
    pub out_of_quota: Option<f64>,
    /// Average QPS for pixel match requests from clients.
    #[serde(rename = "pixelMatchRequests")]
    pub pixel_match_requests: Option<f64>,
    /// Average QPS for pixel match responses from clients.
    #[serde(rename = "pixelMatchResponses")]
    pub pixel_match_responses: Option<f64>,
    /// The configured quota limits for this account.
    #[serde(rename = "quotaConfiguredLimit")]
    pub quota_configured_limit: Option<f64>,
    /// The throttled quota limits for this account.
    #[serde(rename = "quotaThrottledLimit")]
    pub quota_throttled_limit: Option<f64>,
    /// The trading location of this data.
    pub region: Option<String>,
    /// The number of properly formed bid responses received by our servers within the deadline.
    #[serde(rename = "successfulRequestRate")]
    pub successful_request_rate: Option<f64>,
    /// The unix timestamp of the starting time of this performance data.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub timestamp: Option<i64>,
    /// The number of bid responses that were unsuccessful due to timeouts, incorrect formatting, etc.
    #[serde(rename = "unsuccessfulRequestRate")]
    pub unsuccessful_request_rate: Option<f64>,
}

impl common::Part for PerformanceReport {}

/// The configuration data for an Ad Exchange performance report list.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list performance report](PerformanceReportListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PerformanceReportList {
    /// Resource type.
    pub kind: Option<String>,
    /// A list of performance reports relevant for the account.
    #[serde(rename = "performanceReport")]
    pub performance_report: Option<Vec<PerformanceReport>>,
}

impl common::ResponseResult for PerformanceReportList {}

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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PretargetingConfig {
    /// The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically.
    #[serde(rename = "billingId")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub billing_id: Option<i64>,
    /// The config id; generated automatically. Leave this field blank for insert requests.
    #[serde(rename = "configId")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub config_id: Option<i64>,
    /// The name of the config. Must be unique. Required for all requests.
    #[serde(rename = "configName")]
    pub config_name: Option<String>,
    /// List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO.
    #[serde(rename = "creativeType")]
    pub creative_type: Option<Vec<String>>,
    /// Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions.
    pub dimensions: Option<Vec<PretargetingConfigDimensions>>,
    /// Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section.
    #[serde(rename = "excludedContentLabels")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub excluded_content_labels: Option<Vec<i64>>,
    /// Requests containing any of these geo criteria ids will not match.
    #[serde(rename = "excludedGeoCriteriaIds")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub excluded_geo_criteria_ids: Option<Vec<i64>>,
    /// Requests containing any of these placements will not match.
    #[serde(rename = "excludedPlacements")]
    pub excluded_placements: Option<Vec<PretargetingConfigExcludedPlacements>>,
    /// Requests containing any of these users list ids will not match.
    #[serde(rename = "excludedUserLists")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub excluded_user_lists: Option<Vec<i64>>,
    /// Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section.
    #[serde(rename = "excludedVerticals")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub excluded_verticals: Option<Vec<i64>>,
    /// Requests containing any of these geo criteria ids will match.
    #[serde(rename = "geoCriteriaIds")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub geo_criteria_ids: Option<Vec<i64>>,
    /// Whether this config is active. Required for all requests.
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
    /// The kind of the resource, i.e. "adexchangebuyer#pretargetingConfig".
    pub kind: Option<String>,
    /// Request containing any of these language codes will match.
    pub languages: Option<Vec<String>>,
    /// The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed).
    #[serde(rename = "maximumQps")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub maximum_qps: Option<i64>,
    /// Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section.
    #[serde(rename = "mobileCarriers")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub mobile_carriers: Option<Vec<i64>>,
    /// Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section.
    #[serde(rename = "mobileDevices")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub mobile_devices: Option<Vec<i64>>,
    /// Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section.
    #[serde(rename = "mobileOperatingSystemVersions")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub mobile_operating_system_versions: Option<Vec<i64>>,
    /// Requests containing any of these placements will match.
    pub placements: Option<Vec<PretargetingConfigPlacements>>,
    /// Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET.
    pub platforms: Option<Vec<String>>,
    /// Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section.
    #[serde(rename = "supportedCreativeAttributes")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub supported_creative_attributes: Option<Vec<i64>>,
    /// Requests containing any of these user list ids will match.
    #[serde(rename = "userLists")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub user_lists: Option<Vec<i64>>,
    /// Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section.
    #[serde(rename = "vendorTypes")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub vendor_types: Option<Vec<i64>>,
    /// Requests containing any of these vertical ids will match.
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub verticals: Option<Vec<i64>>,
}

impl common::RequestValue for PretargetingConfig {}
impl common::ResponseResult for PretargetingConfig {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list pretargeting config](PretargetingConfigListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PretargetingConfigList {
    /// A list of pretargeting configs
    pub items: Option<Vec<PretargetingConfig>>,
    /// Resource type.
    pub kind: Option<String>,
}

impl common::ResponseResult for PretargetingConfigList {}

/// Your bidder locations that have distinct URLs.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountBidderLocation {
    /// The maximum queries per second the Ad Exchange will send.
    #[serde(rename = "maximumQps")]
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

impl common::NestedType for AccountBidderLocation {}
impl common::Part for AccountBidderLocation {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeAdTechnologyProviders {
    /// The detected ad technology provider IDs for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider. If this creative contains provider IDs that are outside of those listed in the `BidRequest.adslot.consented_providers_settings.consented_providers` field on the  Authorized Buyers Real-Time Bidding protocol or the `BidRequest.user.ext.consented_providers_settings.consented_providers` field on the OpenRTB protocol, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the "Third-party Ad Technology Vendors" section of Authorized Buyers Program Guidelines.
    #[serde(rename = "detectedProviderIds")]
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    pub detected_provider_ids: Option<Vec<i64>>,
    /// Whether the creative contains an unidentified ad technology provider. If true, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the "Third-party Ad Technology Vendors" section of Authorized Buyers Program Guidelines.
    #[serde(rename = "hasUnidentifiedProvider")]
    pub has_unidentified_provider: Option<bool>,
}

impl common::NestedType for CreativeAdTechnologyProviders {}
impl common::Part for CreativeAdTechnologyProviders {}

/// Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeCorrections {
    /// Additional details about the correction.
    pub details: Option<Vec<String>>,
    /// The type of correction that was applied to the creative.
    pub reason: Option<String>,
}

impl common::NestedType for CreativeCorrections {}
impl common::Part for CreativeCorrections {}

/// The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeDisapprovalReasons {
    /// Additional details about the reason for disapproval.
    pub details: Option<Vec<String>>,
    /// The categorized reason for disapproval.
    pub reason: Option<String>,
}

impl common::NestedType for CreativeDisapprovalReasons {}
impl common::Part for CreativeDisapprovalReasons {}

/// The filtering reasons for the creative. Read-only. This field should not be set in requests.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeFilteringReasons {
    /// The date in ISO 8601 format for the data. The data is collected from 00:00:00 to 23:59:59 in PST.
    pub date: Option<String>,
    /// The filtering reasons.
    pub reasons: Option<Vec<CreativeFilteringReasonsReasons>>,
}

impl common::NestedType for CreativeFilteringReasons {}
impl common::Part for CreativeFilteringReasons {}

/// The filtering reasons.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeFilteringReasonsReasons {
    /// The number of times the creative was filtered for the status. The count is aggregated across all publishers on the exchange.
    #[serde(rename = "filteringCount")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub filtering_count: Option<i64>,
    /// The filtering status code. Please refer to the creative-status-codes.txt file for different statuses.
    #[serde(rename = "filteringStatus")]
    pub filtering_status: Option<i32>,
}

impl common::NestedType for CreativeFilteringReasonsReasons {}
impl common::Part for CreativeFilteringReasonsReasons {}

/// If nativeAd is set, HTMLSnippet and videoURL should not be set.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeNativeAd {
    /// no description provided
    pub advertiser: Option<String>,
    /// The app icon, for app download ads.
    #[serde(rename = "appIcon")]
    pub app_icon: Option<CreativeNativeAdAppIcon>,
    /// A long description of the ad.
    pub body: Option<String>,
    /// A label for the button that the user is supposed to click.
    #[serde(rename = "callToAction")]
    pub call_to_action: Option<String>,
    /// The URL to use for click tracking.
    #[serde(rename = "clickTrackingUrl")]
    pub click_tracking_url: Option<String>,
    /// A short title for the ad.
    pub headline: Option<String>,
    /// A large image.
    pub image: Option<CreativeNativeAdImage>,
    /// The URLs are called when the impression is rendered.
    #[serde(rename = "impressionTrackingUrl")]
    pub impression_tracking_url: Option<Vec<String>>,
    /// A smaller image, for the advertiser logo.
    pub logo: Option<CreativeNativeAdLogo>,
    /// The price of the promoted app including the currency info.
    pub price: Option<String>,
    /// The app rating in the app store. Must be in the range [0-5].
    #[serde(rename = "starRating")]
    pub star_rating: Option<f64>,
}

impl common::NestedType for CreativeNativeAd {}
impl common::Part for CreativeNativeAd {}

/// The app icon, for app download ads.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeNativeAdAppIcon {
    /// no description provided
    pub height: Option<i32>,
    /// no description provided
    pub url: Option<String>,
    /// no description provided
    pub width: Option<i32>,
}

impl common::NestedType for CreativeNativeAdAppIcon {}
impl common::Part for CreativeNativeAdAppIcon {}

/// A large image.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeNativeAdImage {
    /// no description provided
    pub height: Option<i32>,
    /// no description provided
    pub url: Option<String>,
    /// no description provided
    pub width: Option<i32>,
}

impl common::NestedType for CreativeNativeAdImage {}
impl common::Part for CreativeNativeAdImage {}

/// A smaller image, for the advertiser logo.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreativeNativeAdLogo {
    /// no description provided
    pub height: Option<i32>,
    /// no description provided
    pub url: Option<String>,
    /// no description provided
    pub width: Option<i32>,
}

impl common::NestedType for CreativeNativeAdLogo {}
impl common::Part for CreativeNativeAdLogo {}

/// Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PretargetingConfigDimensions {
    /// Height in pixels.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub height: Option<i64>,
    /// Width in pixels.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub width: Option<i64>,
}

impl common::NestedType for PretargetingConfigDimensions {}
impl common::Part for PretargetingConfigDimensions {}

/// Requests containing any of these placements will not match.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PretargetingConfigExcludedPlacements {
    /// The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement.
    pub token: Option<String>,
    /// The type of the placement.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl common::NestedType for PretargetingConfigExcludedPlacements {}
impl common::Part for PretargetingConfigExcludedPlacements {}

/// Requests containing any of these placements will match.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PretargetingConfigPlacements {
    /// The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement.
    pub token: Option<String>,
    /// The type of the placement.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl common::NestedType for PretargetingConfigPlacements {}
impl common::Part for PretargetingConfigPlacements {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
///
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
}

impl<'a, C> common::MethodsBuilder for AccountMethods<'a, C> {}

impl<'a, C> AccountMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one account by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The account id
    pub fn get(&self, id: i32) -> AccountGetCall<'a, C> {
        AccountGetCall {
            hub: self.hub,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the authenticated user's list of accounts.
    pub fn list(&self) -> AccountListCall<'a, C> {
        AccountListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account. This method supports patch semantics.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The account id
    pub fn patch(&self, request: Account, id: i32) -> AccountPatchCall<'a, C> {
        AccountPatchCall {
            hub: self.hub,
            _request: request,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The account id
    pub fn update(&self, request: Account, id: i32) -> AccountUpdateCall<'a, C> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *billingInfo* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
///
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.billing_info();
/// # }
/// ```
pub struct BillingInfoMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
}

impl<'a, C> common::MethodsBuilder for BillingInfoMethods<'a, C> {}

impl<'a, C> BillingInfoMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the billing information for one account specified by account ID.
    ///
    /// # Arguments
    ///
    /// * `accountId` - The account id.
    pub fn get(&self, account_id: i32) -> BillingInfoGetCall<'a, C> {
        BillingInfoGetCall {
            hub: self.hub,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of billing information for all accounts of the authenticated user.
    pub fn list(&self) -> BillingInfoListCall<'a, C> {
        BillingInfoListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *budget* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
///
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.budget();
/// # }
/// ```
pub struct BudgetMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
}

impl<'a, C> common::MethodsBuilder for BudgetMethods<'a, C> {}

impl<'a, C> BudgetMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the budget information for the adgroup specified by the accountId and billingId.
    ///
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the budget information for.
    /// * `billingId` - The billing id to get the budget information for.
    pub fn get(&self, account_id: i64, billing_id: i64) -> BudgetGetCall<'a, C> {
        BudgetGetCall {
            hub: self.hub,
            _account_id: account_id,
            _billing_id: billing_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request. This method supports patch semantics.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id associated with the budget being updated.
    /// * `billingId` - The billing id associated with the budget being updated.
    pub fn patch(
        &self,
        request: Budget,
        account_id: i64,
        billing_id: i64,
    ) -> BudgetPatchCall<'a, C> {
        BudgetPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _billing_id: billing_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id associated with the budget being updated.
    /// * `billingId` - The billing id associated with the budget being updated.
    pub fn update(
        &self,
        request: Budget,
        account_id: i64,
        billing_id: i64,
    ) -> BudgetUpdateCall<'a, C> {
        BudgetUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _billing_id: billing_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *creative* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
///
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.creatives();
/// # }
/// ```
pub struct CreativeMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
}

impl<'a, C> common::MethodsBuilder for CreativeMethods<'a, C> {}

impl<'a, C> CreativeMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the status for a single creative. A creative will be available 30-40 minutes after submission.
    ///
    /// # Arguments
    ///
    /// * `accountId` - The id for the account that will serve this creative.
    /// * `buyerCreativeId` - The buyer-specific id for this creative.
    pub fn get(&self, account_id: i32, buyer_creative_id: &str) -> CreativeGetCall<'a, C> {
        CreativeGetCall {
            hub: self.hub,
            _account_id: account_id,
            _buyer_creative_id: buyer_creative_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Submit a new creative.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Creative) -> CreativeInsertCall<'a, C> {
        CreativeInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of the authenticated user's active creatives. A creative will be available 30-40 minutes after submission.
    pub fn list(&self) -> CreativeListCall<'a, C> {
        CreativeListCall {
            hub: self.hub,
            _status_filter: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _buyer_creative_id: Default::default(),
            _account_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *directDeal* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
///
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.direct_deals();
/// # }
/// ```
pub struct DirectDealMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
}

impl<'a, C> common::MethodsBuilder for DirectDealMethods<'a, C> {}

impl<'a, C> DirectDealMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one direct deal by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The direct deal id
    pub fn get(&self, id: i64) -> DirectDealGetCall<'a, C> {
        DirectDealGetCall {
            hub: self.hub,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the authenticated user's list of direct deals.
    pub fn list(&self) -> DirectDealListCall<'a, C> {
        DirectDealListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *performanceReport* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
///
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.performance_report();
/// # }
/// ```
pub struct PerformanceReportMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
}

impl<'a, C> common::MethodsBuilder for PerformanceReportMethods<'a, C> {}

impl<'a, C> PerformanceReportMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the authenticated user's list of performance metrics.
    ///
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the reports.
    /// * `endDateTime` - The end time of the report in ISO 8601 timestamp format using UTC.
    /// * `startDateTime` - The start time of the report in ISO 8601 timestamp format using UTC.
    pub fn list(
        &self,
        account_id: i64,
        end_date_time: &str,
        start_date_time: &str,
    ) -> PerformanceReportListCall<'a, C> {
        PerformanceReportListCall {
            hub: self.hub,
            _account_id: account_id,
            _end_date_time: end_date_time.to_string(),
            _start_date_time: start_date_time.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *pretargetingConfig* resources.
/// It is not used directly, but through the [`AdExchangeBuyer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
///
/// # async fn dox() {
/// use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdExchangeBuyer::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.pretargeting_config();
/// # }
/// ```
pub struct PretargetingConfigMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
}

impl<'a, C> common::MethodsBuilder for PretargetingConfigMethods<'a, C> {}

impl<'a, C> PretargetingConfigMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing pretargeting config.
    ///
    /// # Arguments
    ///
    /// * `accountId` - The account id to delete the pretargeting config for.
    /// * `configId` - The specific id of the configuration to delete.
    pub fn delete(&self, account_id: i64, config_id: i64) -> PretargetingConfigDeleteCall<'a, C> {
        PretargetingConfigDeleteCall {
            hub: self.hub,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific pretargeting configuration
    ///
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the pretargeting config for.
    /// * `configId` - The specific id of the configuration to retrieve.
    pub fn get(&self, account_id: i64, config_id: i64) -> PretargetingConfigGetCall<'a, C> {
        PretargetingConfigGetCall {
            hub: self.hub,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new pretargeting configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id to insert the pretargeting config for.
    pub fn insert(
        &self,
        request: PretargetingConfig,
        account_id: i64,
    ) -> PretargetingConfigInsertCall<'a, C> {
        PretargetingConfigInsertCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of the authenticated user's pretargeting configurations.
    ///
    /// # Arguments
    ///
    /// * `accountId` - The account id to get the pretargeting configs for.
    pub fn list(&self, account_id: i64) -> PretargetingConfigListCall<'a, C> {
        PretargetingConfigListCall {
            hub: self.hub,
            _account_id: account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing pretargeting config. This method supports patch semantics.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id to update the pretargeting config for.
    /// * `configId` - The specific id of the configuration to update.
    pub fn patch(
        &self,
        request: PretargetingConfig,
        account_id: i64,
        config_id: i64,
    ) -> PretargetingConfigPatchCall<'a, C> {
        PretargetingConfigPatchCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing pretargeting config.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `accountId` - The account id to update the pretargeting config for.
    /// * `configId` - The specific id of the configuration to update.
    pub fn update(
        &self,
        request: PretargetingConfig,
        account_id: i64,
        config_id: i64,
    ) -> PretargetingConfigUpdateCall<'a, C> {
        PretargetingConfigUpdateCall {
            hub: self.hub,
            _request: request,
            _account_id: account_id,
            _config_id: config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Gets one account by ID.
///
/// A builder for the *get* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().get(-8)
///              .doit().await;
/// # }
/// ```
pub struct AccountGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _id: i32,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountGetCall<'a, C> {}

impl<'a, C> AccountGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Account)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.accounts.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("id", self._id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "accounts/{id}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{id}", "id")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["id"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The account id
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: i32) -> AccountGetCall<'a, C> {
        self._id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> AccountGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Retrieves the authenticated user's list of accounts.
///
/// A builder for the *list* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().list()
///              .doit().await;
/// # }
/// ```
pub struct AccountListCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountListCall<'a, C> {}

impl<'a, C> AccountListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, AccountsList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.accounts.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(2 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "accounts";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> AccountListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates an existing account. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::Account;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Account::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().patch(req, -80)
///              .doit().await;
/// # }
/// ```
pub struct AccountPatchCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: Account,
    _id: i32,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountPatchCall<'a, C> {}

impl<'a, C> AccountPatchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Account)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.accounts.patch",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("id", self._id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "accounts/{id}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{id}", "id")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["id"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Account) -> AccountPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The account id
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: i32) -> AccountPatchCall<'a, C> {
        self._id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> AccountPatchCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountPatchCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountPatchCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountPatchCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountPatchCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates an existing account.
///
/// A builder for the *update* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::Account;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Account::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().update(req, -2)
///              .doit().await;
/// # }
/// ```
pub struct AccountUpdateCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: Account,
    _id: i32,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountUpdateCall<'a, C> {}

impl<'a, C> AccountUpdateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Account)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.accounts.update",
            http_method: hyper::Method::PUT,
        });

        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("id", self._id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "accounts/{id}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{id}", "id")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["id"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Account) -> AccountUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The account id
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: i32) -> AccountUpdateCall<'a, C> {
        self._id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> AccountUpdateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountUpdateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountUpdateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountUpdateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountUpdateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns the billing information for one account specified by account ID.
///
/// A builder for the *get* method supported by a *billingInfo* resource.
/// It is not used directly, but through a [`BillingInfoMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.billing_info().get(-59)
///              .doit().await;
/// # }
/// ```
pub struct BillingInfoGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _account_id: i32,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BillingInfoGetCall<'a, C> {}

impl<'a, C> BillingInfoGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, BillingInfo)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.billingInfo.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "billinginfo/{accountId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The account id.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i32) -> BillingInfoGetCall<'a, C> {
        self._account_id = new_value;
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
    ) -> BillingInfoGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BillingInfoGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BillingInfoGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BillingInfoGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BillingInfoGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Retrieves a list of billing information for all accounts of the authenticated user.
///
/// A builder for the *list* method supported by a *billingInfo* resource.
/// It is not used directly, but through a [`BillingInfoMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.billing_info().list()
///              .doit().await;
/// # }
/// ```
pub struct BillingInfoListCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BillingInfoListCall<'a, C> {}

impl<'a, C> BillingInfoListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, BillingInfoList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.billingInfo.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(2 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "billinginfo";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
    ) -> BillingInfoListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BillingInfoListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BillingInfoListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BillingInfoListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BillingInfoListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns the budget information for the adgroup specified by the accountId and billingId.
///
/// A builder for the *get* method supported by a *budget* resource.
/// It is not used directly, but through a [`BudgetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.budget().get(-52, -20)
///              .doit().await;
/// # }
/// ```
pub struct BudgetGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _account_id: i64,
    _billing_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BudgetGetCall<'a, C> {}

impl<'a, C> BudgetGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Budget)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.budget.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId", "billingId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("billingId", self._billing_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "billinginfo/{accountId}/{billingId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{billingId}", "billingId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["billingId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The account id to get the budget information for.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> BudgetGetCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The billing id to get the budget information for.
    ///
    /// Sets the *billing id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn billing_id(mut self, new_value: i64) -> BudgetGetCall<'a, C> {
        self._billing_id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> BudgetGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BudgetGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BudgetGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BudgetGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BudgetGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *budget* resource.
/// It is not used directly, but through a [`BudgetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::Budget;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Budget::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.budget().patch(req, -55, -62)
///              .doit().await;
/// # }
/// ```
pub struct BudgetPatchCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: Budget,
    _account_id: i64,
    _billing_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BudgetPatchCall<'a, C> {}

impl<'a, C> BudgetPatchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Budget)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.budget.patch",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "accountId", "billingId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("billingId", self._billing_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "billinginfo/{accountId}/{billingId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{billingId}", "billingId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["billingId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Budget) -> BudgetPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The account id associated with the budget being updated.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> BudgetPatchCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The billing id associated with the budget being updated.
    ///
    /// Sets the *billing id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn billing_id(mut self, new_value: i64) -> BudgetPatchCall<'a, C> {
        self._billing_id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> BudgetPatchCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BudgetPatchCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BudgetPatchCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BudgetPatchCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BudgetPatchCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request.
///
/// A builder for the *update* method supported by a *budget* resource.
/// It is not used directly, but through a [`BudgetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::Budget;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Budget::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.budget().update(req, -51, -12)
///              .doit().await;
/// # }
/// ```
pub struct BudgetUpdateCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: Budget,
    _account_id: i64,
    _billing_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for BudgetUpdateCall<'a, C> {}

impl<'a, C> BudgetUpdateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Budget)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.budget.update",
            http_method: hyper::Method::PUT,
        });

        for &field in ["alt", "accountId", "billingId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("billingId", self._billing_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "billinginfo/{accountId}/{billingId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{billingId}", "billingId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["billingId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Budget) -> BudgetUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The account id associated with the budget being updated.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> BudgetUpdateCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The billing id associated with the budget being updated.
    ///
    /// Sets the *billing id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn billing_id(mut self, new_value: i64) -> BudgetUpdateCall<'a, C> {
        self._billing_id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> BudgetUpdateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> BudgetUpdateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> BudgetUpdateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> BudgetUpdateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> BudgetUpdateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets the status for a single creative. A creative will be available 30-40 minutes after submission.
///
/// A builder for the *get* method supported by a *creative* resource.
/// It is not used directly, but through a [`CreativeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.creatives().get(-75, "buyerCreativeId")
///              .doit().await;
/// # }
/// ```
pub struct CreativeGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _account_id: i32,
    _buyer_creative_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for CreativeGetCall<'a, C> {}

impl<'a, C> CreativeGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Creative)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.creatives.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId", "buyerCreativeId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("buyerCreativeId", self._buyer_creative_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "creatives/{accountId}/{buyerCreativeId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{accountId}", "accountId"),
            ("{buyerCreativeId}", "buyerCreativeId"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["buyerCreativeId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The id for the account that will serve this creative.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i32) -> CreativeGetCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The buyer-specific id for this creative.
    ///
    /// Sets the *buyer creative id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn buyer_creative_id(mut self, new_value: &str) -> CreativeGetCall<'a, C> {
        self._buyer_creative_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> CreativeGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CreativeGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CreativeGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CreativeGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CreativeGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Submit a new creative.
///
/// A builder for the *insert* method supported by a *creative* resource.
/// It is not used directly, but through a [`CreativeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::Creative;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Creative::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.creatives().insert(req)
///              .doit().await;
/// # }
/// ```
pub struct CreativeInsertCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: Creative,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for CreativeInsertCall<'a, C> {}

impl<'a, C> CreativeInsertCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Creative)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.creatives.insert",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "creatives";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Creative) -> CreativeInsertCall<'a, C> {
        self._request = new_value;
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
    ) -> CreativeInsertCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CreativeInsertCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CreativeInsertCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CreativeInsertCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CreativeInsertCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Retrieves a list of the authenticated user's active creatives. A creative will be available 30-40 minutes after submission.
///
/// A builder for the *list* method supported by a *creative* resource.
/// It is not used directly, but through a [`CreativeMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.creatives().list()
///              .status_filter("ea")
///              .page_token("ipsum")
///              .max_results(13)
///              .add_buyer_creative_id("amet")
///              .add_account_id(-20)
///              .doit().await;
/// # }
/// ```
pub struct CreativeListCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _status_filter: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _buyer_creative_id: Vec<String>,
    _account_id: Vec<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for CreativeListCall<'a, C> {}

impl<'a, C> CreativeListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, CreativesList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.creatives.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "statusFilter",
            "pageToken",
            "maxResults",
            "buyerCreativeId",
            "accountId",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        if let Some(value) = self._status_filter.as_ref() {
            params.push("statusFilter", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if !self._buyer_creative_id.is_empty() {
            for f in self._buyer_creative_id.iter() {
                params.push("buyerCreativeId", f);
            }
        }
        if !self._account_id.is_empty() {
            for f in self._account_id.iter() {
                params.push("accountId", f.to_string());
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "creatives";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// When specified, only creatives having the given status are returned.
    ///
    /// Sets the *status filter* query property to the given value.
    pub fn status_filter(mut self, new_value: &str) -> CreativeListCall<'a, C> {
        self._status_filter = Some(new_value.to_string());
        self
    }
    /// A continuation token, used to page through ad clients. To retrieve the next page, set this parameter to the value of "nextPageToken" from the previous response. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CreativeListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of entries returned on one result page. If not set, the default is 100. Optional.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> CreativeListCall<'a, C> {
        self._max_results = Some(new_value);
        self
    }
    /// When specified, only creatives for the given buyer creative ids are returned.
    ///
    /// Append the given value to the *buyer creative id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_buyer_creative_id(mut self, new_value: &str) -> CreativeListCall<'a, C> {
        self._buyer_creative_id.push(new_value.to_string());
        self
    }
    /// When specified, only creatives for the given account ids are returned.
    ///
    /// Append the given value to the *account id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_account_id(mut self, new_value: i32) -> CreativeListCall<'a, C> {
        self._account_id.push(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> CreativeListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CreativeListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CreativeListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CreativeListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CreativeListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets one direct deal by ID.
///
/// A builder for the *get* method supported by a *directDeal* resource.
/// It is not used directly, but through a [`DirectDealMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.direct_deals().get(-50)
///              .doit().await;
/// # }
/// ```
pub struct DirectDealGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for DirectDealGetCall<'a, C> {}

impl<'a, C> DirectDealGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, DirectDeal)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.directDeals.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("id", self._id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "directdeals/{id}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{id}", "id")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["id"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The direct deal id
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: i64) -> DirectDealGetCall<'a, C> {
        self._id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> DirectDealGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DirectDealGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DirectDealGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DirectDealGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DirectDealGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Retrieves the authenticated user's list of direct deals.
///
/// A builder for the *list* method supported by a *directDeal* resource.
/// It is not used directly, but through a [`DirectDealMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.direct_deals().list()
///              .doit().await;
/// # }
/// ```
pub struct DirectDealListCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for DirectDealListCall<'a, C> {}

impl<'a, C> DirectDealListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, DirectDealsList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.directDeals.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(2 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "directdeals";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
    ) -> DirectDealListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DirectDealListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DirectDealListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DirectDealListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DirectDealListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Retrieves the authenticated user's list of performance metrics.
///
/// A builder for the *list* method supported by a *performanceReport* resource.
/// It is not used directly, but through a [`PerformanceReportMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.performance_report().list(-93, "endDateTime", "startDateTime")
///              .page_token("rebum.")
///              .max_results(44)
///              .doit().await;
/// # }
/// ```
pub struct PerformanceReportListCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _account_id: i64,
    _end_date_time: String,
    _start_date_time: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PerformanceReportListCall<'a, C> {}

impl<'a, C> PerformanceReportListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PerformanceReportList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.performanceReport.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "accountId",
            "endDateTime",
            "startDateTime",
            "pageToken",
            "maxResults",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("endDateTime", self._end_date_time);
        params.push("startDateTime", self._start_date_time);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "performancereport";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The account id to get the reports.
    ///
    /// Sets the *account id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> PerformanceReportListCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The end time of the report in ISO 8601 timestamp format using UTC.
    ///
    /// Sets the *end date time* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn end_date_time(mut self, new_value: &str) -> PerformanceReportListCall<'a, C> {
        self._end_date_time = new_value.to_string();
        self
    }
    /// The start time of the report in ISO 8601 timestamp format using UTC.
    ///
    /// Sets the *start date time* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn start_date_time(mut self, new_value: &str) -> PerformanceReportListCall<'a, C> {
        self._start_date_time = new_value.to_string();
        self
    }
    /// A continuation token, used to page through performance reports. To retrieve the next page, set this parameter to the value of "nextPageToken" from the previous response. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PerformanceReportListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of entries returned on one result page. If not set, the default is 100. Optional.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PerformanceReportListCall<'a, C> {
        self._max_results = Some(new_value);
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
    ) -> PerformanceReportListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PerformanceReportListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PerformanceReportListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PerformanceReportListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PerformanceReportListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Deletes an existing pretargeting config.
///
/// A builder for the *delete* method supported by a *pretargetingConfig* resource.
/// It is not used directly, but through a [`PretargetingConfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pretargeting_config().delete(-50, -50)
///              .doit().await;
/// # }
/// ```
pub struct PretargetingConfigDeleteCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _account_id: i64,
    _config_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PretargetingConfigDeleteCall<'a, C> {}

impl<'a, C> PretargetingConfigDeleteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<common::Response> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.pretargetingConfig.delete",
            http_method: hyper::Method::DELETE,
        });

        for &field in ["accountId", "configId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("configId", self._config_id.to_string());

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "pretargetingconfigs/{accountId}/{configId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{configId}", "configId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["configId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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
                    let response = common::Response::from_parts(parts, body);

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The account id to delete the pretargeting config for.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> PretargetingConfigDeleteCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The specific id of the configuration to delete.
    ///
    /// Sets the *config id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn config_id(mut self, new_value: i64) -> PretargetingConfigDeleteCall<'a, C> {
        self._config_id = new_value;
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
    ) -> PretargetingConfigDeleteCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PretargetingConfigDeleteCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PretargetingConfigDeleteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PretargetingConfigDeleteCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PretargetingConfigDeleteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets a specific pretargeting configuration
///
/// A builder for the *get* method supported by a *pretargetingConfig* resource.
/// It is not used directly, but through a [`PretargetingConfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pretargeting_config().get(-7, -62)
///              .doit().await;
/// # }
/// ```
pub struct PretargetingConfigGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _account_id: i64,
    _config_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PretargetingConfigGetCall<'a, C> {}

impl<'a, C> PretargetingConfigGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PretargetingConfig)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.pretargetingConfig.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId", "configId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("configId", self._config_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "pretargetingconfigs/{accountId}/{configId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{configId}", "configId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["configId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The account id to get the pretargeting config for.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> PretargetingConfigGetCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The specific id of the configuration to retrieve.
    ///
    /// Sets the *config id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn config_id(mut self, new_value: i64) -> PretargetingConfigGetCall<'a, C> {
        self._config_id = new_value;
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
    ) -> PretargetingConfigGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PretargetingConfigGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PretargetingConfigGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PretargetingConfigGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PretargetingConfigGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Inserts a new pretargeting configuration.
///
/// A builder for the *insert* method supported by a *pretargetingConfig* resource.
/// It is not used directly, but through a [`PretargetingConfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::PretargetingConfig;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = PretargetingConfig::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pretargeting_config().insert(req, -17)
///              .doit().await;
/// # }
/// ```
pub struct PretargetingConfigInsertCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: PretargetingConfig,
    _account_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PretargetingConfigInsertCall<'a, C> {}

impl<'a, C> PretargetingConfigInsertCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PretargetingConfig)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.pretargetingConfig.insert",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "pretargetingconfigs/{accountId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: PretargetingConfig) -> PretargetingConfigInsertCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The account id to insert the pretargeting config for.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> PretargetingConfigInsertCall<'a, C> {
        self._account_id = new_value;
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
    ) -> PretargetingConfigInsertCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PretargetingConfigInsertCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PretargetingConfigInsertCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PretargetingConfigInsertCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PretargetingConfigInsertCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Retrieves a list of the authenticated user's pretargeting configurations.
///
/// A builder for the *list* method supported by a *pretargetingConfig* resource.
/// It is not used directly, but through a [`PretargetingConfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pretargeting_config().list(-99)
///              .doit().await;
/// # }
/// ```
pub struct PretargetingConfigListCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _account_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PretargetingConfigListCall<'a, C> {}

impl<'a, C> PretargetingConfigListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PretargetingConfigList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.pretargetingConfig.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "pretargetingconfigs/{accountId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

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

    /// The account id to get the pretargeting configs for.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> PretargetingConfigListCall<'a, C> {
        self._account_id = new_value;
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
    ) -> PretargetingConfigListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PretargetingConfigListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PretargetingConfigListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PretargetingConfigListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PretargetingConfigListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates an existing pretargeting config. This method supports patch semantics.
///
/// A builder for the *patch* method supported by a *pretargetingConfig* resource.
/// It is not used directly, but through a [`PretargetingConfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::PretargetingConfig;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = PretargetingConfig::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pretargeting_config().patch(req, -56, -25)
///              .doit().await;
/// # }
/// ```
pub struct PretargetingConfigPatchCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: PretargetingConfig,
    _account_id: i64,
    _config_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PretargetingConfigPatchCall<'a, C> {}

impl<'a, C> PretargetingConfigPatchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PretargetingConfig)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.pretargetingConfig.patch",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "accountId", "configId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("configId", self._config_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "pretargetingconfigs/{accountId}/{configId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{configId}", "configId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["configId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: PretargetingConfig) -> PretargetingConfigPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The account id to update the pretargeting config for.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> PretargetingConfigPatchCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The specific id of the configuration to update.
    ///
    /// Sets the *config id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn config_id(mut self, new_value: i64) -> PretargetingConfigPatchCall<'a, C> {
        self._config_id = new_value;
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
    ) -> PretargetingConfigPatchCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PretargetingConfigPatchCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PretargetingConfigPatchCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PretargetingConfigPatchCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PretargetingConfigPatchCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates an existing pretargeting config.
///
/// A builder for the *update* method supported by a *pretargetingConfig* resource.
/// It is not used directly, but through a [`PretargetingConfigMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adexchangebuyer1d3 as adexchangebuyer1d3;
/// use adexchangebuyer1d3::api::PretargetingConfig;
/// # async fn dox() {
/// # use adexchangebuyer1d3::{AdExchangeBuyer, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdExchangeBuyer::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = PretargetingConfig::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pretargeting_config().update(req, -86, -43)
///              .doit().await;
/// # }
/// ```
pub struct PretargetingConfigUpdateCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdExchangeBuyer<C>,
    _request: PretargetingConfig,
    _account_id: i64,
    _config_id: i64,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PretargetingConfigUpdateCall<'a, C> {}

impl<'a, C> PretargetingConfigUpdateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PretargetingConfig)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adexchangebuyer.pretargetingConfig.update",
            http_method: hyper::Method::PUT,
        });

        for &field in ["alt", "accountId", "configId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("accountId", self._account_id.to_string());
        params.push("configId", self._config_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "pretargetingconfigs/{accountId}/{configId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::AdexchangeBuyer.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{configId}", "configId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["configId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

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

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: PretargetingConfig) -> PretargetingConfigUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The account id to update the pretargeting config for.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: i64) -> PretargetingConfigUpdateCall<'a, C> {
        self._account_id = new_value;
        self
    }
    /// The specific id of the configuration to update.
    ///
    /// Sets the *config id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn config_id(mut self, new_value: i64) -> PretargetingConfigUpdateCall<'a, C> {
        self._config_id = new_value;
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
    ) -> PretargetingConfigUpdateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PretargetingConfigUpdateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::AdexchangeBuyer`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PretargetingConfigUpdateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PretargetingConfigUpdateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PretargetingConfigUpdateCall<'a, C> {
        self._scopes.clear();
        self
    }
}
