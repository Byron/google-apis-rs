use super::*;
/// A request to activate a pretargeting configuration. Sets the configuration’s state to ACTIVE.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs activate bidders](BidderPretargetingConfigActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivatePretargetingConfigRequest { _never_set: Option<bool> }

impl client::RequestValue for ActivatePretargetingConfigRequest {}


/// The list of detected Ad Technology Providers for this creative. Bids placed for inventory that will serve to EEA or UK users are expected to comply with GDPR requirements. You must ensure that the creatives used in such bids should contain only user consented ad technology providers as indicated in the bid request. Google reserves the right to filter non-compliant bids. User consented ad technology providers can be found in the [Google Protocol](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) with the `BidRequest.adslot.consented_providers_settings` field, and can be found as an [OpenRTB extension](https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto) with the `BidRequest.user.ext.consented_providers_settings` and `BidRequest.user.ext.consent` fields. See https://support.google.com/authorizedbuyers/answer/9789378 for additional information about the Google TCF v2 integration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdTechnologyProviders {
    /// The detected IAB Global Vendor List (GVL) IDs for this creative. See the IAB Global Vendor List at https://vendor-list.consensu.org/v2/vendor-list.json for details about the vendors.
    #[serde(rename="detectedGvlIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub detected_gvl_ids: Option<Vec<i64>>,
    /// The detected [Google Ad Tech Providers (ATP)](https://support.google.com/admanager/answer/9012903) for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider.
    #[serde(rename="detectedProviderIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub detected_provider_ids: Option<Vec<i64>>,
    /// Domains of detected unidentified ad technology providers (if any). You must ensure that the creatives used in bids placed for inventory that will serve to EEA or UK users does not contain unidentified ad technology providers. Google reserves the right to filter non-compliant bids.
    #[serde(rename="unidentifiedProviderDomains")]
    
    pub unidentified_provider_domains: Option<Vec<String>>,
}

impl client::Part for AdTechnologyProviders {}


/// A request to start targeting the provided app IDs in a specific pretargeting configuration. The pretargeting configuration itself specifies how these apps are targeted. in PretargetingConfig.appTargeting.mobileAppTargeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs add targeted apps bidders](BidderPretargetingConfigAddTargetedAppCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddTargetedAppsRequest {
    /// A list of app IDs to target in the pretargeting configuration. These values will be added to the list of targeted app IDs in PretargetingConfig.appTargeting.mobileAppTargeting.values.
    #[serde(rename="appIds")]
    
    pub app_ids: Option<Vec<String>>,
    /// Required. The targeting mode that should be applied to the list of app IDs. If there are existing targeted app IDs, must be equal to the existing PretargetingConfig.appTargeting.mobileAppTargeting.targetingMode or a 400 bad request error will be returned.
    #[serde(rename="targetingMode")]
    
    pub targeting_mode: Option<AddTargetedAppsRequestTargetingModeEnum>,
}

impl client::RequestValue for AddTargetedAppsRequest {}


/// A request to start targeting the provided publishers in a specific pretargeting configuration. The pretargeting configuration itself specifies how these publishers are targeted in PretargetingConfig.publisherTargeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs add targeted publishers bidders](BidderPretargetingConfigAddTargetedPublisherCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddTargetedPublishersRequest {
    /// A list of publisher IDs to target in the pretargeting configuration. These values will be added to the list of targeted publisher IDs in PretargetingConfig.publisherTargeting.values. Publishers are identified by their publisher ID from ads.txt / app-ads.txt. See https://iabtechlab.com/ads-txt/ and https://iabtechlab.com/app-ads-txt/ for more details.
    #[serde(rename="publisherIds")]
    
    pub publisher_ids: Option<Vec<String>>,
    /// Required. The targeting mode that should be applied to the list of publisher IDs. If are existing publisher IDs, must be equal to the existing PretargetingConfig.publisherTargeting.targetingMode or a 400 bad request error will be returned.
    #[serde(rename="targetingMode")]
    
    pub targeting_mode: Option<AddTargetedPublishersRequestTargetingModeEnum>,
}

impl client::RequestValue for AddTargetedPublishersRequest {}


/// A request to start targeting the provided sites in a specific pretargeting configuration. The pretargeting configuration itself specifies how these sites are targeted in PretargetingConfig.webTargeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs add targeted sites bidders](BidderPretargetingConfigAddTargetedSiteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddTargetedSitesRequest {
    /// A list of site URLs to target in the pretargeting configuration. These values will be added to the list of targeted URLs in PretargetingConfig.webTargeting.values.
    
    pub sites: Option<Vec<String>>,
    /// Required. The targeting mode that should be applied to the list of site URLs. If there are existing targeted sites, must be equal to the existing PretargetingConfig.webTargeting.targetingMode or a 400 bad request error will be returned.
    #[serde(rename="targetingMode")]
    
    pub targeting_mode: Option<AddTargetedSitesRequestTargetingModeEnum>,
}

impl client::RequestValue for AddTargetedSitesRequest {}


/// Detected advertiser and brand information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvertiserAndBrand {
    /// See https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt for the list of possible values. Can be used to filter the response of the creatives.list method.
    #[serde(rename="advertiserId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub advertiser_id: Option<i64>,
    /// Advertiser name. Can be used to filter the response of the creatives.list method.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// Detected brand ID or zero if no brand has been detected. See https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt for the list of possible values. Can be used to filter the response of the creatives.list method.
    #[serde(rename="brandId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub brand_id: Option<i64>,
    /// Brand name. Can be used to filter the response of the creatives.list method.
    #[serde(rename="brandName")]
    
    pub brand_name: Option<String>,
}

impl client::Part for AdvertiserAndBrand {}


/// A subset of app inventory to target. Bid requests that match criteria in at least one of the specified dimensions will be sent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppTargeting {
    /// Lists of included and excluded mobile app categories as defined in https://developers.google.com/adwords/api/docs/appendix/mobileappcategories.csv.
    #[serde(rename="mobileAppCategoryTargeting")]
    
    pub mobile_app_category_targeting: Option<NumericTargetingDimension>,
    /// Targeted app IDs. App IDs can refer to those found in an app store or ones that are not published in an app store. A maximum of 30,000 app IDs can be targeted.
    #[serde(rename="mobileAppTargeting")]
    
    pub mobile_app_targeting: Option<StringTargetingDimension>,
}

impl client::Part for AppTargeting {}


/// A request to approve a batch of publisher connections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher connections batch approve bidders](BidderPublisherConnectionBatchApproveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchApprovePublisherConnectionsRequest {
    /// Required. The names of the publishers with which connections will be approved. In the pattern `bidders/{bidder}/publisherConnections/{publisher}` where `{bidder}` is the account ID of the bidder, and `{publisher}` is the ads.txt/app-ads.txt publisher ID.
    
    pub names: Option<Vec<String>>,
}

impl client::RequestValue for BatchApprovePublisherConnectionsRequest {}


/// A response for the request to approve a batch of publisher connections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher connections batch approve bidders](BidderPublisherConnectionBatchApproveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchApprovePublisherConnectionsResponse {
    /// The publisher connections that have been approved.
    #[serde(rename="publisherConnections")]
    
    pub publisher_connections: Option<Vec<PublisherConnection>>,
}

impl client::ResponseResult for BatchApprovePublisherConnectionsResponse {}


/// A request to reject a batch of publisher connections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher connections batch reject bidders](BidderPublisherConnectionBatchRejectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRejectPublisherConnectionsRequest {
    /// Required. The names of the publishers with whom connection will be rejected. In the pattern `bidders/{bidder}/publisherConnections/{publisher}` where `{bidder}` is the account ID of the bidder, and `{publisher}` is the ads.txt/app-ads.txt publisher ID.
    
    pub names: Option<Vec<String>>,
}

impl client::RequestValue for BatchRejectPublisherConnectionsRequest {}


/// A response for the request to reject a batch of publisher connections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher connections batch reject bidders](BidderPublisherConnectionBatchRejectCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRejectPublisherConnectionsResponse {
    /// The publisher connections that have been rejected.
    #[serde(rename="publisherConnections")]
    
    pub publisher_connections: Option<Vec<PublisherConnection>>,
}

impl client::ResponseResult for BatchRejectPublisherConnectionsResponse {}


/// Bidder settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives list bidders](BidderCreativeListCall) (none)
/// * [creatives watch bidders](BidderCreativeWatchCall) (none)
/// * [endpoints get bidders](BidderEndpointGetCall) (none)
/// * [endpoints list bidders](BidderEndpointListCall) (none)
/// * [endpoints patch bidders](BidderEndpointPatchCall) (none)
/// * [pretargeting configs activate bidders](BidderPretargetingConfigActivateCall) (none)
/// * [pretargeting configs add targeted apps bidders](BidderPretargetingConfigAddTargetedAppCall) (none)
/// * [pretargeting configs add targeted publishers bidders](BidderPretargetingConfigAddTargetedPublisherCall) (none)
/// * [pretargeting configs add targeted sites bidders](BidderPretargetingConfigAddTargetedSiteCall) (none)
/// * [pretargeting configs create bidders](BidderPretargetingConfigCreateCall) (none)
/// * [pretargeting configs delete bidders](BidderPretargetingConfigDeleteCall) (none)
/// * [pretargeting configs get bidders](BidderPretargetingConfigGetCall) (none)
/// * [pretargeting configs list bidders](BidderPretargetingConfigListCall) (none)
/// * [pretargeting configs patch bidders](BidderPretargetingConfigPatchCall) (none)
/// * [pretargeting configs remove targeted apps bidders](BidderPretargetingConfigRemoveTargetedAppCall) (none)
/// * [pretargeting configs remove targeted publishers bidders](BidderPretargetingConfigRemoveTargetedPublisherCall) (none)
/// * [pretargeting configs remove targeted sites bidders](BidderPretargetingConfigRemoveTargetedSiteCall) (none)
/// * [pretargeting configs suspend bidders](BidderPretargetingConfigSuspendCall) (none)
/// * [publisher connections batch approve bidders](BidderPublisherConnectionBatchApproveCall) (none)
/// * [publisher connections batch reject bidders](BidderPublisherConnectionBatchRejectCall) (none)
/// * [publisher connections get bidders](BidderPublisherConnectionGetCall) (none)
/// * [publisher connections list bidders](BidderPublisherConnectionListCall) (none)
/// * [get bidders](BidderGetCall) (response)
/// * [list bidders](BidderListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bidder {
    /// Output only. An option to bypass pretargeting for private auctions and preferred deals. When true, bid requests from these nonguaranteed deals will always be sent. When false, bid requests will be subject to regular pretargeting configurations. Programmatic Guaranteed deals will always be sent to the bidder, regardless of the value for this flag. Auction packages are not impacted by this value and are subject to the regular pretargeting configurations.
    #[serde(rename="bypassNonguaranteedDealsPretargeting")]
    
    pub bypass_nonguaranteed_deals_pretargeting: Option<bool>,
    /// Output only. The buyer's network ID used for cookie matching. This ID corresponds to the `google_nid` parameter in the URL used in cookie match requests. Refer to https://developers.google.com/authorized-buyers/rtb/cookie-guide for further information.
    #[serde(rename="cookieMatchingNetworkId")]
    
    pub cookie_matching_network_id: Option<String>,
    /// Output only. The base URL used in cookie match requests. Refer to https://developers.google.com/authorized-buyers/rtb/cookie-guide for further information.
    #[serde(rename="cookieMatchingUrl")]
    
    pub cookie_matching_url: Option<String>,
    /// Output only. The billing ID for the deals pretargeting config. This billing ID is sent on the bid request for guaranteed and nonguaranteed deals matched in pretargeting.
    #[serde(rename="dealsBillingId")]
    
    pub deals_billing_id: Option<String>,
    /// Output only. Name of the bidder resource that must follow the pattern `bidders/{bidderAccountId}`, where `{bidderAccountId}` is the account ID of the bidder whose information is to be received. One can get their account ID on the Authorized Buyers or Open Bidding UI, or by contacting their Google account manager.
    
    pub name: Option<String>,
}

impl client::Resource for Bidder {}
impl client::ResponseResult for Bidder {}


/// RTB Buyer account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives create buyers](BuyerCreativeCreateCall) (none)
/// * [creatives get buyers](BuyerCreativeGetCall) (none)
/// * [creatives list buyers](BuyerCreativeListCall) (none)
/// * [creatives patch buyers](BuyerCreativePatchCall) (none)
/// * [user lists close buyers](BuyerUserListCloseCall) (none)
/// * [user lists create buyers](BuyerUserListCreateCall) (none)
/// * [user lists get buyers](BuyerUserListGetCall) (none)
/// * [user lists get remarketing tag buyers](BuyerUserListGetRemarketingTagCall) (none)
/// * [user lists list buyers](BuyerUserListListCall) (none)
/// * [user lists open buyers](BuyerUserListOpenCall) (none)
/// * [user lists update buyers](BuyerUserListUpdateCall) (none)
/// * [get buyers](BuyerGetCall) (response)
/// * [get remarketing tag buyers](BuyerGetRemarketingTagCall) (none)
/// * [list buyers](BuyerListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Buyer {
    /// Output only. The number of creatives that this buyer submitted through the API or bid with in the last 30 days. This is counted against the maximum number of active creatives.
    #[serde(rename="activeCreativeCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub active_creative_count: Option<i64>,
    /// Output only. The name of the bidder resource that is responsible for receiving bidding traffic for this account. The bidder name must follow the pattern `bidders/{bidderAccountId}`, where `{bidderAccountId}` is the account ID of the bidder receiving traffic for this buyer.
    
    pub bidder: Option<String>,
    /// Output only. A list of billing IDs associated with this account. These IDs appear on: 1. A bid request, to signal which buyers are eligible to bid on a given opportunity, and which pretargeting configurations were matched for each eligible buyer. 2. The bid response, to attribute a winning impression to a specific account for billing, reporting, policy and publisher block enforcement.
    #[serde(rename="billingIds")]
    
    pub billing_ids: Option<Vec<String>>,
    /// Output only. The diplay name associated with this buyer account, as visible to sellers.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The maximum number of active creatives that this buyer can have.
    #[serde(rename="maximumActiveCreativeCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_active_creative_count: Option<i64>,
    /// Output only. Name of the buyer resource that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` is the account ID of the buyer account whose information is to be received. One can get their account ID on the Authorized Buyers or Open Bidding UI, or by contacting their Google account manager.
    
    pub name: Option<String>,
}

impl client::Resource for Buyer {}
impl client::ResponseResult for Buyer {}


/// A request to close a specified user list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user lists close buyers](BuyerUserListCloseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloseUserListRequest { _never_set: Option<bool> }

impl client::RequestValue for CloseUserListRequest {}


/// A creative and its classification data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives create buyers](BuyerCreativeCreateCall) (request|response)
/// * [creatives get buyers](BuyerCreativeGetCall) (response)
/// * [creatives patch buyers](BuyerCreativePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Creative {
    /// Output only. ID of the buyer account that this creative is owned by. Can be used to filter the response of the creatives.list method with equality and inequality check.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// The link to AdChoices destination page. This is only supported for native ads.
    #[serde(rename="adChoicesDestinationUrl")]
    
    pub ad_choices_destination_url: Option<String>,
    /// The name of the company being advertised in the creative. Can be used to filter the response of the creatives.list method.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// The agency ID for this creative.
    #[serde(rename="agencyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub agency_id: Option<i64>,
    /// Output only. The last update timestamp of the creative through the API.
    #[serde(rename="apiUpdateTime")]
    
    pub api_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The format of this creative. Can be used to filter the response of the creatives.list method.
    #[serde(rename="creativeFormat")]
    
    pub creative_format: Option<CreativeCreativeFormatEnum>,
    /// Buyer-specific creative ID that references this creative in bid responses. This field is Ignored in update operations. Can be used to filter the response of the creatives.list method. The maximum length of the creative ID is 128 bytes.
    #[serde(rename="creativeId")]
    
    pub creative_id: Option<String>,
    /// Output only. Top level status and detected attributes of a creative (for example domain, language, advertiser, product category, etc.) that affect whether (status) and where (context) a creative will be allowed to serve.
    #[serde(rename="creativeServingDecision")]
    
    pub creative_serving_decision: Option<CreativeServingDecision>,
    /// Output only. IDs of all of the deals with which this creative has been used in bidding. Can be used to filter the response of the creatives.list method.
    #[serde(rename="dealIds")]
    
    pub deal_ids: Option<Vec<String>>,
    /// All declared attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto") contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction.
    #[serde(rename="declaredAttributes")]
    
    pub declared_attributes: Option<Vec<CreativeDeclaredAttributesEnum>>,
    /// The set of declared destination URLs for the creative. Can be used to filter the response of the creatives.list method.
    #[serde(rename="declaredClickThroughUrls")]
    
    pub declared_click_through_urls: Option<Vec<String>>,
    /// All declared restricted categories for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method.
    #[serde(rename="declaredRestrictedCategories")]
    
    pub declared_restricted_categories: Option<Vec<CreativeDeclaredRestrictedCategoriesEnum>>,
    /// IDs for the declared ad technology vendors that may be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method.
    #[serde(rename="declaredVendorIds")]
    
    pub declared_vendor_ids: Option<Vec<i32>>,
    /// An HTML creative.
    
    pub html: Option<HtmlContent>,
    /// The set of URLs to be called to record an impression.
    #[serde(rename="impressionTrackingUrls")]
    
    pub impression_tracking_urls: Option<Vec<String>>,
    /// Output only. Name of the creative. Follows the pattern `buyers/{buyer}/creatives/{creative}`, where `{buyer}` represents the account ID of the buyer who owns the creative, and `{creative}` is the buyer-specific creative ID that references this creative in the bid response.
    
    pub name: Option<String>,
    /// A native creative.
    
    pub native: Option<NativeContent>,
    /// Experimental field that can be used during the [FLEDGE Origin Trial](https://developers.google.com/authorized-buyers/rtb/fledge-origin-trial). The URL to fetch an interest group ad used in [TURTLEDOVE on-device auction](https://github.com/WICG/turtledove/blob/main/FLEDGE.md#1-browsers-record-interest-groups"). This should be unique among all creatives for a given `accountId`.
    #[serde(rename="renderUrl")]
    
    pub render_url: Option<String>,
    /// All restricted categories for the ads that may be shown from this creative.
    #[serde(rename="restrictedCategories")]
    
    pub restricted_categories: Option<Vec<CreativeRestrictedCategoriesEnum>>,
    /// Output only. The version of the creative. Version for a new creative is 1 and it increments during subsequent creative updates.
    
    pub version: Option<i32>,
    /// A video creative.
    
    pub video: Option<VideoContent>,
}

impl client::RequestValue for Creative {}
impl client::ResponseResult for Creative {}


/// The dimensions of a creative. This applies to only HTML and Native creatives.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeDimensions {
    /// The height of the creative in pixels.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub height: Option<i64>,
    /// The width of the creative in pixels.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub width: Option<i64>,
}

impl client::Part for CreativeDimensions {}


/// Top level status and detected attributes of a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreativeServingDecision {
    /// The detected ad technology providers.
    #[serde(rename="adTechnologyProviders")]
    
    pub ad_technology_providers: Option<AdTechnologyProviders>,
    /// The policy compliance of this creative in China. When approved or disapproved, this applies to both deals and open auction in China. When pending review, this creative is allowed to serve for deals but not for open auction.
    #[serde(rename="chinaPolicyCompliance")]
    
    pub china_policy_compliance: Option<PolicyCompliance>,
    /// Policy compliance of this creative when bidding on Programmatic Guaranteed and Preferred Deals (outside of Russia and China).
    #[serde(rename="dealsPolicyCompliance")]
    
    pub deals_policy_compliance: Option<PolicyCompliance>,
    /// Detected advertisers and brands.
    #[serde(rename="detectedAdvertisers")]
    
    pub detected_advertisers: Option<Vec<AdvertiserAndBrand>>,
    /// Publisher-excludable attributes that were detected for this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction.
    #[serde(rename="detectedAttributes")]
    
    pub detected_attributes: Option<Vec<CreativeServingDecisionDetectedAttributesEnum>>,
    /// The set of detected destination URLs for the creative. Can be used to filter the response of the creatives.list method.
    #[serde(rename="detectedClickThroughUrls")]
    
    pub detected_click_through_urls: Option<Vec<String>>,
    /// The detected domains for this creative.
    #[serde(rename="detectedDomains")]
    
    pub detected_domains: Option<Vec<String>>,
    /// The detected languages for this creative. The order is arbitrary. The codes are 2 or 5 characters and are documented at https://developers.google.com/adwords/api/docs/appendix/languagecodes. Can be used to filter the response of the creatives.list method.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<String>>,
    /// Detected product categories, if any. See the ad-product-categories.txt file in the technical documentation for a list of IDs. Can be used to filter the response of the creatives.list method.
    #[serde(rename="detectedProductCategories")]
    
    pub detected_product_categories: Option<Vec<i32>>,
    /// Detected sensitive categories, if any. Can be used to filter the response of the creatives.list method. See the ad-sensitive-categories.txt file in the technical documentation for a list of IDs. You should use these IDs along with the excluded-sensitive-category field in the bid request to filter your bids.
    #[serde(rename="detectedSensitiveCategories")]
    
    pub detected_sensitive_categories: Option<Vec<i32>>,
    /// IDs of the ad technology vendors that were detected to be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method. If the `allowed_vendor_type` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) does not contain one of the vendor type IDs that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction.
    #[serde(rename="detectedVendorIds")]
    
    pub detected_vendor_ids: Option<Vec<i32>>,
    /// The last time the creative status was updated. Can be used to filter the response of the creatives.list method.
    #[serde(rename="lastStatusUpdate")]
    
    pub last_status_update: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Policy compliance of this creative when bidding in open auction, private auction, or auction packages (outside of Russia and China).
    #[serde(rename="networkPolicyCompliance")]
    
    pub network_policy_compliance: Option<PolicyCompliance>,
    /// Policy compliance of this creative when bidding in Open Bidding (outside of Russia and China). For the list of platform policies, see: https://support.google.com/platformspolicy/answer/3013851.
    #[serde(rename="platformPolicyCompliance")]
    
    pub platform_policy_compliance: Option<PolicyCompliance>,
    /// The policy compliance of this creative in Russia. When approved or disapproved, this applies to both deals and open auction in Russia. When pending review, this creative is allowed to serve for deals but not for open auction.
    #[serde(rename="russiaPolicyCompliance")]
    
    pub russia_policy_compliance: Option<PolicyCompliance>,
}

impl client::Part for CreativeServingDecision {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// Evidence that the creative's destination URL was not crawlable by Google.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestinationNotCrawlableEvidence {
    /// Approximate time of the crawl.
    #[serde(rename="crawlTime")]
    
    pub crawl_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Destination URL that was attempted to be crawled.
    #[serde(rename="crawledUrl")]
    
    pub crawled_url: Option<String>,
    /// Reason of destination not crawlable.
    
    pub reason: Option<DestinationNotCrawlableEvidenceReasonEnum>,
}

impl client::Part for DestinationNotCrawlableEvidence {}


/// Evidence of the creative's destination URL not functioning properly or having been incorrectly set up.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestinationNotWorkingEvidence {
    /// DNS lookup errors.
    #[serde(rename="dnsError")]
    
    pub dns_error: Option<DestinationNotWorkingEvidenceDnsErrorEnum>,
    /// The full non-working URL.
    #[serde(rename="expandedUrl")]
    
    pub expanded_url: Option<String>,
    /// HTTP error code (for example, 404 or 5xx)
    #[serde(rename="httpError")]
    
    pub http_error: Option<i32>,
    /// Page was crawled successfully, but was detected as either a page with no content or an error page.
    #[serde(rename="invalidPage")]
    
    pub invalid_page: Option<DestinationNotWorkingEvidenceInvalidPageEnum>,
    /// Approximate time when the ad destination was last checked.
    #[serde(rename="lastCheckTime")]
    
    pub last_check_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Platform of the non-working URL.
    
    pub platform: Option<DestinationNotWorkingEvidencePlatformEnum>,
    /// HTTP redirect chain error.
    #[serde(rename="redirectionError")]
    
    pub redirection_error: Option<DestinationNotWorkingEvidenceRedirectionErrorEnum>,
    /// Rejected because of malformed URLs or invalid requests.
    #[serde(rename="urlRejected")]
    
    pub url_rejected: Option<DestinationNotWorkingEvidenceUrlRejectedEnum>,
}

impl client::Part for DestinationNotWorkingEvidence {}


/// The full landing page URL of the destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestinationUrlEvidence {
    /// The full landing page URL of the destination.
    #[serde(rename="destinationUrl")]
    
    pub destination_url: Option<String>,
}

impl client::Part for DestinationUrlEvidence {}


/// Number of HTTP calls made by a creative, broken down by domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainCallEvidence {
    /// Breakdown of the most frequent domains called through HTTP by the creative.
    #[serde(rename="topHttpCallDomains")]
    
    pub top_http_call_domains: Option<Vec<DomainCalls>>,
    /// The total number of HTTP calls made by the creative, including but not limited to the number of calls in the top_http_call_domains.
    #[serde(rename="totalHttpCallCount")]
    
    pub total_http_call_count: Option<i32>,
}

impl client::Part for DomainCallEvidence {}


/// The number of HTTP calls made to the given domain.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DomainCalls {
    /// The domain name.
    
    pub domain: Option<String>,
    /// Number of HTTP calls made to the domain.
    #[serde(rename="httpCallCount")]
    
    pub http_call_count: Option<i32>,
}

impl client::Part for DomainCalls {}


/// Total download size and URL-level download size breakdown for resources in a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadSizeEvidence {
    /// Download size broken down by URLs with the top download size.
    #[serde(rename="topUrlDownloadSizeBreakdowns")]
    
    pub top_url_download_size_breakdowns: Option<Vec<UrlDownloadSize>>,
    /// Total download size (in kilobytes) for all the resources in the creative.
    #[serde(rename="totalDownloadSizeKb")]
    
    pub total_download_size_kb: Option<i32>,
}

impl client::Part for DownloadSizeEvidence {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs delete bidders](BidderPretargetingConfigDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Bidder endpoint that receives bid requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [endpoints get bidders](BidderEndpointGetCall) (response)
/// * [endpoints patch bidders](BidderEndpointPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    /// The protocol that the bidder endpoint is using.
    #[serde(rename="bidProtocol")]
    
    pub bid_protocol: Option<EndpointBidProtocolEnum>,
    /// The maximum number of queries per second allowed to be sent to this server.
    #[serde(rename="maximumQps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_qps: Option<i64>,
    /// Output only. Name of the endpoint resource that must follow the pattern `bidders/{bidderAccountId}/endpoints/{endpointId}`, where {bidderAccountId} is the account ID of the bidder who operates this endpoint, and {endpointId} is a unique ID assigned by the server.
    
    pub name: Option<String>,
    /// The trading location that bid requests should be sent from. See https://developers.google.com/authorized-buyers/rtb/peer-guide#trading-locations for further information.
    #[serde(rename="tradingLocation")]
    
    pub trading_location: Option<EndpointTradingLocationEnum>,
    /// Output only. The URL that bid requests should be sent to.
    
    pub url: Option<String>,
}

impl client::RequestValue for Endpoint {}
impl client::ResponseResult for Endpoint {}


/// Response for a request to get remarketing tag.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user lists get remarketing tag buyers](BuyerUserListGetRemarketingTagCall) (response)
/// * [get remarketing tag buyers](BuyerGetRemarketingTagCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetRemarketingTagResponse {
    /// A HTML tag that can be placed on the advertiser’s page to add users to a user list. For more information and code samples on using snippet on your website refer to [Tag your site for remarketing](https://support.google.com/google-ads/answer/2476688).
    
    pub snippet: Option<String>,
}

impl client::ResponseResult for GetRemarketingTagResponse {}


/// HTML content for a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HtmlContent {
    /// The height of the HTML snippet in pixels. Can be used to filter the response of the creatives.list method.
    
    pub height: Option<i32>,
    /// The HTML snippet that displays the ad when inserted in the web page.
    
    pub snippet: Option<String>,
    /// The width of the HTML snippet in pixels. Can be used to filter the response of the creatives.list method.
    
    pub width: Option<i32>,
}

impl client::Part for HtmlContent {}


/// HTTP calls made by a creative that resulted in policy violations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpCallEvidence {
    /// URLs of HTTP calls made by the creative.
    
    pub urls: Option<Vec<String>>,
}

impl client::Part for HttpCallEvidence {}


/// Evidence for HTTP cookie-related policy violations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpCookieEvidence {
    /// Names of cookies that violate Google policies. For TOO_MANY_COOKIES policy, this will be the cookie names of top domains with the largest number of cookies. For other policies, this will be all the cookie names that violate the policy.
    #[serde(rename="cookieNames")]
    
    pub cookie_names: Option<Vec<String>>,
    /// The largest number of cookies set by a creative. If this field is set, cookie_names above will be set to the cookie names of top domains with the largest number of cookies. This field will only be set for TOO_MANY_COOKIES policy.
    #[serde(rename="maxCookieCount")]
    
    pub max_cookie_count: Option<i32>,
}

impl client::Part for HttpCookieEvidence {}


/// An image resource. You may provide a larger image than was requested, so long as the aspect ratio is preserved.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// Image height in pixels.
    
    pub height: Option<i32>,
    /// The URL of the image.
    
    pub url: Option<String>,
    /// Image width in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for Image {}


/// A response containing bidders.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list bidders](BidderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBiddersResponse {
    /// List of bidders.
    
    pub bidders: Option<Vec<Bidder>>,
    /// A token which can be passed to a subsequent call to the `ListBidders` method to retrieve the next page of results in ListBiddersRequest.pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBiddersResponse {}


/// A response containing buyer account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list buyers](BuyerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBuyersResponse {
    /// List of buyers.
    
    pub buyers: Option<Vec<Buyer>>,
    /// A token which can be passed to a subsequent call to the `ListBuyers` method to retrieve the next page of results in ListBuyersRequest.pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBuyersResponse {}


/// A response for listing creatives.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives list bidders](BidderCreativeListCall) (response)
/// * [creatives list buyers](BuyerCreativeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCreativesResponse {
    /// The list of creatives.
    
    pub creatives: Option<Vec<Creative>>,
    /// A token to retrieve the next page of results. Pass this value in the ListCreativesRequest.pageToken field in the subsequent call to the `ListCreatives` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCreativesResponse {}


/// A response containing bidder endpoints.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [endpoints list bidders](BidderEndpointListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEndpointsResponse {
    /// List of bidder endpoints.
    
    pub endpoints: Option<Vec<Endpoint>>,
    /// A token which can be passed to a subsequent call to the `ListEndpoints` method to retrieve the next page of results in ListEndpointsRequest.pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListEndpointsResponse {}


/// A response containing pretargeting configurations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs list bidders](BidderPretargetingConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPretargetingConfigsResponse {
    /// A token which can be passed to a subsequent call to the `ListPretargetingConfigs` method to retrieve the next page of results in ListPretargetingConfigsRequest.pageToken.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of pretargeting configurations.
    #[serde(rename="pretargetingConfigs")]
    
    pub pretargeting_configs: Option<Vec<PretargetingConfig>>,
}

impl client::ResponseResult for ListPretargetingConfigsResponse {}


/// A response to a request for listing publisher connections.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher connections list bidders](BidderPublisherConnectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPublisherConnectionsResponse {
    /// A token to retrieve the next page of results. Pass this value in the ListPublisherConnectionsRequest.pageToken field in the subsequent call to the `ListPublisherConnections` method to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of publisher connections.
    #[serde(rename="publisherConnections")]
    
    pub publisher_connections: Option<Vec<PublisherConnection>>,
}

impl client::ResponseResult for ListPublisherConnectionsResponse {}


/// The list user list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user lists list buyers](BuyerUserListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserListsResponse {
    /// The continuation page token to send back to the server in a subsequent request. Due to a currently known issue, it is recommended that the caller keep invoking the list method till the time a next page token is not returned (even if the result set is empty).
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of user lists from the search.
    #[serde(rename="userLists")]
    
    pub user_lists: Option<Vec<UserList>>,
}

impl client::ResponseResult for ListUserListsResponse {}


/// Information about each media file in the VAST.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaFile {
    /// Bitrate of the video file, in Kbps. Can be used to filter the response of the creatives.list method.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bitrate: Option<i64>,
    /// The MIME type of this media file. Can be used to filter the response of the creatives.list method.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<MediaFileMimeTypeEnum>,
}

impl client::Part for MediaFile {}


/// Native content for a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NativeContent {
    /// The name of the advertiser or sponsor, to be displayed in the ad creative.
    #[serde(rename="advertiserName")]
    
    pub advertiser_name: Option<String>,
    /// The app icon, for app download ads.
    #[serde(rename="appIcon")]
    
    pub app_icon: Option<Image>,
    /// A long description of the ad.
    
    pub body: Option<String>,
    /// A label for the button that the user is supposed to click.
    #[serde(rename="callToAction")]
    
    pub call_to_action: Option<String>,
    /// The URL that the browser/SDK will load when the user clicks the ad.
    #[serde(rename="clickLinkUrl")]
    
    pub click_link_url: Option<String>,
    /// The URL to use for click tracking.
    #[serde(rename="clickTrackingUrl")]
    
    pub click_tracking_url: Option<String>,
    /// A short title for the ad.
    
    pub headline: Option<String>,
    /// A large image.
    
    pub image: Option<Image>,
    /// A smaller image, for the advertiser's logo.
    
    pub logo: Option<Image>,
    /// The price of the promoted app including currency info.
    #[serde(rename="priceDisplayText")]
    
    pub price_display_text: Option<String>,
    /// The app rating in the app store. Must be in the range [0-5].
    #[serde(rename="starRating")]
    
    pub star_rating: Option<f64>,
    /// The URL to fetch a native video ad.
    #[serde(rename="videoUrl")]
    
    pub video_url: Option<String>,
    /// The contents of a VAST document for a native video ad.
    #[serde(rename="videoVastXml")]
    
    pub video_vast_xml: Option<String>,
}

impl client::Part for NativeContent {}


/// Generic targeting used for targeting dimensions that contain a list of included and excluded numeric IDs used in app, user list, geo, and vertical id targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumericTargetingDimension {
    /// The IDs excluded in a configuration.
    #[serde(rename="excludedIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub excluded_ids: Option<Vec<i64>>,
    /// The IDs included in a configuration.
    #[serde(rename="includedIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub included_ids: Option<Vec<i64>>,
}

impl client::Part for NumericTargetingDimension {}


/// A request to open a specified user list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user lists open buyers](BuyerUserListOpenCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OpenUserListRequest { _never_set: Option<bool> }

impl client::RequestValue for OpenUserListRequest {}


/// Policy compliance of the creative for a transaction type or a region.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyCompliance {
    /// Serving status for the given transaction type (for example, open auction, deals) or region (for example, China, Russia). Can be used to filter the response of the creatives.list method.
    
    pub status: Option<PolicyComplianceStatusEnum>,
    /// Topics related to the policy compliance for this transaction type (for example, open auction, deals) or region (for example, China, Russia). Topics may be present only if status is DISAPPROVED.
    
    pub topics: Option<Vec<PolicyTopicEntry>>,
}

impl client::Part for PolicyCompliance {}


/// Each policy topic entry will represent a violation of a policy topic for a creative, with the policy topic information and optional evidence for the policy violation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyTopicEntry {
    /// Pieces of evidence associated with this policy topic entry.
    
    pub evidences: Option<Vec<PolicyTopicEvidence>>,
    /// URL of the help center article describing this policy topic.
    #[serde(rename="helpCenterUrl")]
    
    pub help_center_url: Option<String>,
    /// Policy topic this entry refers to. For example, "ALCOHOL", "TRADEMARKS_IN_AD_TEXT", or "DESTINATION_NOT_WORKING". The set of possible policy topics is not fixed for a particular API version and may change at any time. Can be used to filter the response of the creatives.list method
    #[serde(rename="policyTopic")]
    
    pub policy_topic: Option<String>,
}

impl client::Part for PolicyTopicEntry {}


/// Evidence associated with a policy topic entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyTopicEvidence {
    /// The creative's destination URL was not crawlable by Google.
    #[serde(rename="destinationNotCrawlable")]
    
    pub destination_not_crawlable: Option<DestinationNotCrawlableEvidence>,
    /// The creative's destination URL did not function properly or was incorrectly set up.
    #[serde(rename="destinationNotWorking")]
    
    pub destination_not_working: Option<DestinationNotWorkingEvidence>,
    /// URL of the actual landing page.
    #[serde(rename="destinationUrl")]
    
    pub destination_url: Option<DestinationUrlEvidence>,
    /// Number of HTTP calls made by the creative, broken down by domain.
    #[serde(rename="domainCall")]
    
    pub domain_call: Option<DomainCallEvidence>,
    /// Total download size and URL-level download size breakdown for resources in a creative.
    #[serde(rename="downloadSize")]
    
    pub download_size: Option<DownloadSizeEvidence>,
    /// HTTP calls made by the creative that resulted in policy violations.
    #[serde(rename="httpCall")]
    
    pub http_call: Option<HttpCallEvidence>,
    /// Evidence for HTTP cookie-related policy violations.
    #[serde(rename="httpCookie")]
    
    pub http_cookie: Option<HttpCookieEvidence>,
}

impl client::Part for PolicyTopicEvidence {}


/// Pretargeting configuration: a set of targeting dimensions applied at the pretargeting stage of the RTB funnel. These control which inventory a bidder will receive bid requests for.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs activate bidders](BidderPretargetingConfigActivateCall) (response)
/// * [pretargeting configs add targeted apps bidders](BidderPretargetingConfigAddTargetedAppCall) (response)
/// * [pretargeting configs add targeted publishers bidders](BidderPretargetingConfigAddTargetedPublisherCall) (response)
/// * [pretargeting configs add targeted sites bidders](BidderPretargetingConfigAddTargetedSiteCall) (response)
/// * [pretargeting configs create bidders](BidderPretargetingConfigCreateCall) (request|response)
/// * [pretargeting configs get bidders](BidderPretargetingConfigGetCall) (response)
/// * [pretargeting configs patch bidders](BidderPretargetingConfigPatchCall) (request|response)
/// * [pretargeting configs remove targeted apps bidders](BidderPretargetingConfigRemoveTargetedAppCall) (response)
/// * [pretargeting configs remove targeted publishers bidders](BidderPretargetingConfigRemoveTargetedPublisherCall) (response)
/// * [pretargeting configs remove targeted sites bidders](BidderPretargetingConfigRemoveTargetedSiteCall) (response)
/// * [pretargeting configs suspend bidders](BidderPretargetingConfigSuspendCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PretargetingConfig {
    /// Targeting modes included by this configuration. A bid request must allow all the specified targeting modes. An unset value allows all bid requests to be sent, regardless of which targeting modes they allow.
    #[serde(rename="allowedUserTargetingModes")]
    
    pub allowed_user_targeting_modes: Option<Vec<PretargetingConfigAllowedUserTargetingModesEnum>>,
    /// Targeting on a subset of app inventory. If APP is listed in targeted_environments, the specified targeting is applied. A maximum of 30,000 app IDs can be targeted. An unset value for targeting allows all app-based bid requests to be sent. Apps can either be targeting positively (bid requests will be sent only if the destination app is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination app is not listed in the targeting dimension).
    #[serde(rename="appTargeting")]
    
    pub app_targeting: Option<AppTargeting>,
    /// Output only. The identifier that corresponds to this pretargeting configuration that helps buyers track and attribute their spend across their own arbitrary divisions. If a bid request matches more than one configuration, the buyer chooses which billing_id to attribute each of their bids.
    #[serde(rename="billingId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub billing_id: Option<i64>,
    /// The diplay name associated with this configuration. This name must be unique among all the pretargeting configurations a bidder has.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The sensitive content category label IDs excluded in this configuration. Bid requests for inventory with any of the specified content label IDs will not be sent. Refer to this file https://storage.googleapis.com/adx-rtb-dictionaries/content-labels.txt for category IDs.
    #[serde(rename="excludedContentLabelIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub excluded_content_label_ids: Option<Vec<i64>>,
    /// The geos included or excluded in this configuration defined in https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv
    #[serde(rename="geoTargeting")]
    
    pub geo_targeting: Option<NumericTargetingDimension>,
    /// Creative dimensions included by this configuration. Only bid requests eligible for at least one of the specified creative dimensions will be sent. An unset value allows all bid requests to be sent, regardless of creative dimension.
    #[serde(rename="includedCreativeDimensions")]
    
    pub included_creative_dimensions: Option<Vec<CreativeDimensions>>,
    /// Environments that are being included. Bid requests will not be sent for a given environment if it is not included. Further restrictions can be applied to included environments to target only a subset of its inventory. An unset value includes all environments.
    #[serde(rename="includedEnvironments")]
    
    pub included_environments: Option<Vec<PretargetingConfigIncludedEnvironmentsEnum>>,
    /// Creative formats included by this configuration. Only bid requests eligible for at least one of the specified creative formats will be sent. An unset value will allow all bid requests to be sent, regardless of format.
    #[serde(rename="includedFormats")]
    
    pub included_formats: Option<Vec<PretargetingConfigIncludedFormatsEnum>>,
    /// The languages included in this configuration, represented by their language code. See https://developers.google.com/adwords/api/docs/appendix/languagecodes.
    #[serde(rename="includedLanguages")]
    
    pub included_languages: Option<Vec<String>>,
    /// The mobile operating systems included in this configuration as defined in https://storage.googleapis.com/adx-rtb-dictionaries/mobile-os.csv
    #[serde(rename="includedMobileOperatingSystemIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub included_mobile_operating_system_ids: Option<Vec<i64>>,
    /// The platforms included by this configration. Bid requests for devices with the specified platform types will be sent. An unset value allows all bid requests to be sent, regardless of platform.
    #[serde(rename="includedPlatforms")]
    
    pub included_platforms: Option<Vec<PretargetingConfigIncludedPlatformsEnum>>,
    /// User identifier types included in this configuration. At least one of the user identifier types specified in this list must be available for the bid request to be sent.
    #[serde(rename="includedUserIdTypes")]
    
    pub included_user_id_types: Option<Vec<PretargetingConfigIncludedUserIdTypesEnum>>,
    /// The interstitial targeting specified for this configuration. The unset value will allow bid requests to be sent regardless of whether they are for interstitials or not.
    #[serde(rename="interstitialTargeting")]
    
    pub interstitial_targeting: Option<PretargetingConfigInterstitialTargetingEnum>,
    /// Output only. Existing included or excluded geos that are invalid. Previously targeted geos may become invalid due to privacy restrictions.
    #[serde(rename="invalidGeoIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub invalid_geo_ids: Option<Vec<i64>>,
    /// The maximum QPS threshold for this configuration. The bidder should receive no more than this number of bid requests matching this configuration per second across all their bidding endpoints among all trading locations. Further information available at https://developers.google.com/authorized-buyers/rtb/peer-guide
    #[serde(rename="maximumQps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_qps: Option<i64>,
    /// The targeted minimum viewability decile, ranging in values [0, 10]. A value of 5 means that the configuration will only match adslots for which we predict at least 50% viewability. Values > 10 will be rounded down to 10. An unset value or a value of 0 indicates that bid requests will be sent regardless of viewability.
    #[serde(rename="minimumViewabilityDecile")]
    
    pub minimum_viewability_decile: Option<i32>,
    /// Output only. Name of the pretargeting configuration that must follow the pattern `bidders/{bidder_account_id}/pretargetingConfigs/{config_id}`
    
    pub name: Option<String>,
    /// Targeting on a subset of publisher inventory. Publishers can either be targeted positively (bid requests will be sent only if the publisher is listed in the targeting dimension) or negatively (bid requests will be sent only if the publisher is not listed in the targeting dimension). A maximum of 10,000 publisher IDs can be targeted. Publisher IDs are found in [ads.txt](https://iabtechlab.com/ads-txt/) / [app-ads.txt](https://iabtechlab.com/app-ads-txt/) and in bid requests in the `BidRequest.publisher_id` field on the [Google RTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) or the `BidRequest.site.publisher.id` / `BidRequest.app.publisher.id` field on the [OpenRTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto). Publisher IDs will be returned in the order that they were entered.
    #[serde(rename="publisherTargeting")]
    
    pub publisher_targeting: Option<StringTargetingDimension>,
    /// Output only. The state of this pretargeting configuration.
    
    pub state: Option<PretargetingConfigStateEnum>,
    /// The remarketing lists included or excluded in this configuration as defined in UserList.
    #[serde(rename="userListTargeting")]
    
    pub user_list_targeting: Option<NumericTargetingDimension>,
    /// The verticals included or excluded in this configuration as defined in https://developers.google.com/authorized-buyers/rtb/downloads/publisher-verticals
    #[serde(rename="verticalTargeting")]
    
    pub vertical_targeting: Option<NumericTargetingDimension>,
    /// Targeting on a subset of site inventory. If WEB is listed in included_environments, the specified targeting is applied. A maximum of 50,000 site URLs can be targeted. An unset value for targeting allows all web-based bid requests to be sent. Sites can either be targeting positively (bid requests will be sent only if the destination site is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination site is not listed in the pretargeting configuration).
    #[serde(rename="webTargeting")]
    
    pub web_targeting: Option<StringTargetingDimension>,
}

impl client::RequestValue for PretargetingConfig {}
impl client::ResponseResult for PretargetingConfig {}


/// An Open Bidding exchange’s connection to a publisher. This is initiated by the publisher for the bidder to review. If approved by the bidder, this means that the bidder agrees to receive bid requests from the publisher.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [publisher connections get bidders](BidderPublisherConnectionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublisherConnection {
    /// Whether the publisher has been approved by the bidder.
    #[serde(rename="biddingState")]
    
    pub bidding_state: Option<PublisherConnectionBiddingStateEnum>,
    /// Output only. The time at which the publisher initiated a connection with the bidder (irrespective of if or when the bidder approves it). This is subsequently updated if the publisher revokes and re-initiates the connection.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Publisher display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Name of the publisher connection. This follows the pattern `bidders/{bidder}/publisherConnections/{publisher}`, where `{bidder}` represents the account ID of the bidder, and `{publisher}` is the ads.txt/app-ads.txt publisher ID.
    
    pub name: Option<String>,
    /// Output only. Whether the publisher is an Ad Manager or AdMob publisher.
    #[serde(rename="publisherPlatform")]
    
    pub publisher_platform: Option<PublisherConnectionPublisherPlatformEnum>,
}

impl client::ResponseResult for PublisherConnection {}


/// A request to stop targeting the provided apps in a specific pretargeting configuration. The pretargeting configuration itself specifies how these apps are targeted. in PretargetingConfig.appTargeting.mobileAppTargeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs remove targeted apps bidders](BidderPretargetingConfigRemoveTargetedAppCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveTargetedAppsRequest {
    /// A list of app IDs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted app IDs in PretargetingConfig.appTargeting.mobileAppTargeting.values.
    #[serde(rename="appIds")]
    
    pub app_ids: Option<Vec<String>>,
}

impl client::RequestValue for RemoveTargetedAppsRequest {}


/// A request to stop targeting publishers in a specific configuration. The pretargeting configuration itself specifies how these publishers are targeted in PretargetingConfig.publisherTargeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs remove targeted publishers bidders](BidderPretargetingConfigRemoveTargetedPublisherCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveTargetedPublishersRequest {
    /// A list of publisher IDs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted publisher IDs in PretargetingConfig.publisherTargeting.values. Publishers are identified by their publisher ID from ads.txt / app-ads.txt. See https://iabtechlab.com/ads-txt/ and https://iabtechlab.com/app-ads-txt/ for more details.
    #[serde(rename="publisherIds")]
    
    pub publisher_ids: Option<Vec<String>>,
}

impl client::RequestValue for RemoveTargetedPublishersRequest {}


/// A request to stop targeting sites in a specific pretargeting configuration. The pretargeting configuration itself specifies how these sites are targeted in PretargetingConfig.webTargeting.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs remove targeted sites bidders](BidderPretargetingConfigRemoveTargetedSiteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveTargetedSitesRequest {
    /// A list of site URLs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted URLs in PretargetingConfig.webTargeting.values.
    
    pub sites: Option<Vec<String>>,
}

impl client::RequestValue for RemoveTargetedSitesRequest {}


/// Generic targeting with string values used in app, website and publisher targeting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringTargetingDimension {
    /// How the items in this list should be targeted.
    #[serde(rename="targetingMode")]
    
    pub targeting_mode: Option<StringTargetingDimensionTargetingModeEnum>,
    /// The values specified.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for StringTargetingDimension {}


/// A request to suspend a pretargeting configuration. Sets the configuration’s state to SUSPENDED.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [pretargeting configs suspend bidders](BidderPretargetingConfigSuspendCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuspendPretargetingConfigRequest { _never_set: Option<bool> }

impl client::RequestValue for SuspendPretargetingConfigRequest {}


/// The URL-level breakdown for the download size.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlDownloadSize {
    /// Download size of the URL in kilobytes.
    #[serde(rename="downloadSizeKb")]
    
    pub download_size_kb: Option<i32>,
    /// The normalized URL with query parameters and fragment removed.
    #[serde(rename="normalizedUrl")]
    
    pub normalized_url: Option<String>,
}

impl client::Part for UrlDownloadSize {}


/// Represents the URL restriction (for the URL captured by the pixel callback) for a user list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlRestriction {
    /// End date (if specified) of the URL restriction. End date should be later than the start date for the date range to be valid.
    #[serde(rename="endDate")]
    
    pub end_date: Option<Date>,
    /// The restriction type for the specified URL.
    #[serde(rename="restrictionType")]
    
    pub restriction_type: Option<UrlRestrictionRestrictionTypeEnum>,
    /// Start date (if specified) of the URL restriction.
    #[serde(rename="startDate")]
    
    pub start_date: Option<Date>,
    /// Required. The URL to use for applying the restriction on the user list.
    
    pub url: Option<String>,
}

impl client::Part for UrlRestriction {}


/// Represents an Authorized Buyers user list. Authorized Buyers can create/update/list user lists. Once a user list is created in the system, Authorized Buyers can add users to the user list using the bulk uploader API. Alternatively, users can be added by hosting a tag on the advertiser’s page.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user lists close buyers](BuyerUserListCloseCall) (response)
/// * [user lists create buyers](BuyerUserListCreateCall) (request|response)
/// * [user lists get buyers](BuyerUserListGetCall) (response)
/// * [user lists open buyers](BuyerUserListOpenCall) (response)
/// * [user lists update buyers](BuyerUserListUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserList {
    /// The description for the user list.
    
    pub description: Option<String>,
    /// Required. Display name of the user list. This must be unique across all user lists for a given account.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The number of days a user's cookie stays on the user list. The field must be between 0 and 540 inclusive.
    #[serde(rename="membershipDurationDays")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub membership_duration_days: Option<i64>,
    /// Output only. Name of the user list that must follow the pattern `buyers/{buyer}/userLists/{user_list}`, where `{buyer}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyer}` represents the account ID of the child seat buyer. `{user_list}` is an int64 identifier assigned by Google to uniquely identify a user list.
    
    pub name: Option<String>,
    /// Output only. The status of the user list. A new user list starts out as open.
    
    pub status: Option<UserListStatusEnum>,
    /// Required. The URL restriction for the user list.
    #[serde(rename="urlRestriction")]
    
    pub url_restriction: Option<UrlRestriction>,
}

impl client::RequestValue for UserList {}
impl client::ResponseResult for UserList {}


/// Video content for a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoContent {
    /// Output only. Video metadata.
    #[serde(rename="videoMetadata")]
    
    pub video_metadata: Option<VideoMetadata>,
    /// The URL to fetch a video ad.
    #[serde(rename="videoUrl")]
    
    pub video_url: Option<String>,
    /// The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard.
    #[serde(rename="videoVastXml")]
    
    pub video_vast_xml: Option<String>,
}

impl client::Part for VideoContent {}


/// Video metadata for a creative.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoMetadata {
    /// The duration of the ad. Can be used to filter the response of the creatives.list method.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Is this a valid VAST ad? Can be used to filter the response of the creatives.list method.
    #[serde(rename="isValidVast")]
    
    pub is_valid_vast: Option<bool>,
    /// Is this a VPAID ad? Can be used to filter the response of the creatives.list method.
    #[serde(rename="isVpaid")]
    
    pub is_vpaid: Option<bool>,
    /// The list of all media files declared in the VAST. If there are multiple VASTs in a wrapper chain, this includes the media files from the deepest one in the chain.
    #[serde(rename="mediaFiles")]
    
    pub media_files: Option<Vec<MediaFile>>,
    /// The minimum duration that the user has to watch before being able to skip this ad. If the field is not set, the ad is not skippable. If the field is set, the ad is skippable. Can be used to filter the response of the creatives.list method.
    #[serde(rename="skipOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub skip_offset: Option<client::chrono::Duration>,
    /// The maximum VAST version across all wrapped VAST documents. Can be used to filter the response of the creatives.list method.
    #[serde(rename="vastVersion")]
    
    pub vast_version: Option<VideoMetadataVastVersionEnum>,
}

impl client::Part for VideoMetadata {}


/// A request to receive push notifications when any of the creatives belonging to the bidder changes status.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives watch bidders](BidderCreativeWatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WatchCreativesRequest { _never_set: Option<bool> }

impl client::RequestValue for WatchCreativesRequest {}


/// A response for the request to receive push notification when a bidder’s creatives change status.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [creatives watch bidders](BidderCreativeWatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WatchCreativesResponse {
    /// The Pub/Sub subscription that can be used to pull creative status notifications. This would be of the format `projects/{project_id}/subscriptions/{subscription_id}`. Subscription is created with pull delivery. All service accounts belonging to the bidder will have read access to this subscription. Subscriptions that are inactive for more than 90 days will be disabled. Use watchCreatives to re-enable the subscription.
    
    pub subscription: Option<String>,
    /// The Pub/Sub topic that will be used to publish creative serving status notifications. This would be of the format `projects/{project_id}/topics/{topic_id}`.
    
    pub topic: Option<String>,
}

impl client::ResponseResult for WatchCreativesResponse {}


